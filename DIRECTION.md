# Ruthenian — direction

> **The language itself is specified in [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md)**
> — eight cases, three numbers, three declensions, with the etymology of each
> restored category. The research behind it is in
> [`docs/COMPARATIVE_GRAMMAR.md`](docs/COMPARATIVE_GRAMMAR.md) and
> [`docs/RUSSIAN_GRAMMAR.md`](docs/RUSSIAN_GRAMMAR.md).

## What Ruthenian is

Ruthenian is **Russian written in a Latin alphabet, and slightly more regular
than Russian**. Russian phonology, Russian grammar, Russian vocabulary, rendered
in a bijective Latin orthography — plus a small, declared set of departures from
standard Russian where the standard language is irregular for historical rather
than systematic reasons.

It ships as a **command-line tool** over a library workspace. You give it a
lemma; it gives you every form, and tells you where each form came from.

What it is not:

- **Not a website.** No static site generation, no server, no HTML. The CLI is
  the product.
- **Not an Interslavic project.** No Interslavic data enters this repository —
  not the dictionary, not a derived cache, not a single gloss. Interslavic is a
  reference point in two narrow senses: a constructed standard may be more
  regular than the natural language it draws on, and `interslavic-rs` is an
  architecture worth learning from. Both are ideas, not data.
- **Not a conlang with invented vocabulary.** Every lemma comes from attested
  Russian.
- **Not a morphological analyser.** Generation only: lemma → forms. Surface →
  lemma is out of scope for every crate here.
- **Not multilingual.** Russian only. No Ukrainian profile, no Church Slavonic,
  no Proto-Slavic reconstruction.

The single vocabulary source is the English Wiktionary dump at
`~/Desktop/code/wikidata/raw-wiktextract-data.jsonl` — 23 622 298 877 bytes
(22.0 GiB), 10 667 129 lines, verified 2026-07-25.

## The shape of the system

```text
   raw-wiktextract-data.jsonl          (22 GiB, read once, never at runtime)
                │
                ▼
        ruthenian-extract               streaming; pure build plan, thin writer
                │
                ├──────────────► lexicon artifact  (entries: class, principal
                │                                    parts, gender, animacy,
                │                                    aspect, gaps, provenance)
                │                └─────► attested-forms artifact (eval input)
                ▼
         generated PHF tables           ONLY what the rules cannot predict
                │
                ▼
   ┌─────────────────────────────────────────────────┐
   │  ruthenian  (facade)                            │
   │    one generation path:                         │
   │    (entry, slot, policy) → Option<Form>         │
   │    Form = text + origin + trace                 │
   └──────┬───────────────────────────────────┬──────┘
          │                                   │
          ▼                                   ▼
    ruthenian-cli                       ruthenian-eval
    (`ruth`)                            (canonical summary)

   ruthenian-core         productive rules — predictor AND fallback
   ruthenian-lexicon      the types everything above agrees on
   ruthenian-orthography  the only script conversion in the system
```

The load-bearing property: **the CLI and the evaluator consume the same `Form`
from the same call.** The published accuracy number therefore describes the
shipped tool, not a parallel approximation of it. Slovowiki's first and largest
finding was that its benchmark and its website ran materially different
generation pipelines; that failure is designed out here rather than guarded
against later.

## The crate map

Eight crates, eight phases, in dependency order. Each phase leaves the workspace
green and is useful on its own.

| # | Crate | Goal | Owns | May depend on | Done when |
|---|---|---|---|---|---|
| 1 | `ruthenian-orthography` | Bijective Cyrillic↔Latin conversion and the declared Ruthenian alphabet. | The mapping table, the separator rule, the case layer, alphabet validation. | nothing | Round-trip holds by construction over the declared alphabet, with 0 failures on the extracted corpus and exhaustive letter/pair/triple coverage. |
| 2 | `ruthenian-core` | Productive Russian morphology as pure rules, plus the grammatical vocabulary the whole workspace shares. | `Case`/`Number`/`Gender`/`Person`/`Tense`/`Aspect`/`Slot`, `ZaliznyakClass`, the rule engine, `RuleId` and the regularization rules. | `ruthenian-orthography` | Every slot for every class either produces a form or is a declared gap; no lexical data anywhere in the crate. |
| 3 | `ruthenian-lexicon` | The lexical entry schema every other crate agrees on. | `Entry`, `PrincipalParts`, `Provenance`, `Origin`, the artifact formats and their versioning. | `ruthenian-core`, `ruthenian-orthography` | The extractor, the facade and the evaluator all speak these types and no crate defines a second entry representation. |
| 4 | `ruthenian-extract` | Turn the dump into the lexicon and the attested-forms artifact, once, deterministically. | The dump schema knowledge, the reject histogram, the dump fingerprint. | `ruthenian-lexicon`, `ruthenian-core`, `ruthenian-orthography`, serde | Same dump in → byte-identical artifacts out, with every rejected record counted by reason. |
| 5 | `ruthenian` | The facade: one generation path over rules + generated tables, every answer carrying its origin. | The generated PHF tables, the public API. | `ruthenian-core`, `ruthenian-lexicon`, `phf` | One call site produces every form; tables contain no row the rules already predict. |
| 6 | `ruthenian-eval` | Measure the facade against attested forms and produce the one canonical summary. | The sealed test split, the metric definitions, `summary.json`. | `ruthenian`, `ruthenian-lexicon` | Every number in the README is generated from `summary.json`; explained and unexplained mismatches are separated. |
| 7 | `ruthenian-cli` | The `ruth` binary. | Argument parsing, output formatting. | `ruthenian`, `ruthenian-orthography` | Contains no morphology; every subcommand is a thin adapter with `--json`. |
| 8 | `xtask` | `refresh-data`, `check-registry`, `accuracy`. | Nothing. | `ruthenian-extract`, `ruthenian-eval` | Orchestration only — no logic that belongs in the crate it invokes. |

Detailed specs live in `docs/specs/<crate>.md`. Each spec is authoritative for
its crate; this document is authoritative for the boundaries between them.

### Two structural decisions, made deliberately

**The regularization policy is not a crate.** It is tempting to build
`ruthenian-standard` alongside `ruthenian` and let it apply the departures. That
would recreate the exact failure slovowiki spent 919 pages of review on: two code
paths generating the same data, diverging quietly, with no way to know which one
the published numbers describe. Instead:

- `Policy` is a typed parameter on the single generation path;
- every departure from attested Russian is a `RuleId` in `ruthenian-core`,
  applied inside the one rule engine;
- `Policy::attested()` exists and reproduces Russian — it is the baseline the
  evaluator scores against;
- `docs/REGULARIZATION.md` is **generated** from the rule table, so the register
  cannot drift from the rules it describes.

**`ruthenian-lexicon` earns its place** because the extractor must not depend on
the artifact it generates. If the entry types lived in the facade, the extractor
would depend on the crate whose tables it writes — conceptually inverted, and a
trap the first refactor would spring. A zero-dependency schema crate lets
`extract`, `ruthenian` and `eval` agree without any of them owning the others.

## The laws

These hold in every crate. Each is a lesson someone already paid for; the
citations are in `LESSONS.md`.

1. **One generation path.** The CLI and the evaluator are adapters over the same
   call. If a second way to produce a form appears, one of them is wrong and
   nobody knows which. (A1, A5)
2. **Rules predict; tables store the residue.** The extractor drops any attested
   form the rule engine already produces, so the tables are exactly the
   exceptions. Changing a rule therefore changes what counts as irregular and
   **requires regenerating the tables** — enforced by a dump-free layering check,
   with the dump-driven accuracy run as the authority. (A2)
3. **Typed until the last moment; one stringification point.** Forms travel as
   typed values with their origin attached. Nothing is recovered by searching a
   string that was already structured. (A3)
4. **No droppable side channels.** If a caller can silently forget a field and
   still compile, that field will be silently forgotten. Losing information is a
   type error. (A4)
5. **Derive state; never hand-maintain it.** No boolean that duplicates something
   computable from the data. A hand-maintained flag drifts, and its dead branches
   become the bug. (A6)
6. **Pure build logic, thin writer.** Every artifact is produced by a pure
   function returning a plan; exactly one place writes bytes to disk. (A7)
7. **Provenance travels with the form.** Every form knows whether it is attested,
   rule-derived, or regularized, and by which rule. This is API, not
   documentation. (B1)
8. **`None` means "no such form exists".** Never "not implemented", never
   "unknown". A defective paradigm slot under `Policy::attested()` is `None` and
   is documented as by design. (B2)
9. **One canonical owner per artifact; docs and metrics are generated from it.**
   No number is hand-copied into prose. A README figure that disagrees with
   `summary.json` is a build failure, not a typo. (B3, C3)
10. **Label sourced vs policy.** Where Russian usage is genuinely variable or the
    sources disagree, say so and state which way we went and why. (B4)
11. **Every guard has a failure witness and an owner.** A check whose claimed
    mutation does not break it is stale and gets deleted. A guard that cannot
    fail loudly does not count. (D1, D2)
12. **Structure, not strings, at every boundary.** Return the parts, not a string
    for the caller to parse; return the resolved case and number, not just the
    word. This is the single mistake `interslavic` paid for across four
    consecutive releases. (E1)

## Phase order

The order is a dependency order, and each phase unlocks the next:

1. **Orthography** first because every later artifact is written in it. Getting
   it wrong means re-extracting 22 GiB.
2. **Core** next because it defines the grammatical vocabulary the schema needs,
   and because it is the extractor's predictor — the tables cannot be generated
   until the rules exist.
3. **Lexicon** third: the schema can only be designed once you know what the
   rules need supplied to them (which is exactly the set of things they cannot
   predict).
4. **Extract** fourth: now the dump can be read once and turned into artifacts.
5. **Facade** fifth: rules plus tables behind one call.
6. **Eval** sixth: the first point at which any accuracy claim may be made. No
   number is published before this phase exists.
7. **CLI** seventh: the product, assembled from finished parts.
8. **xtask** last, or grown incrementally — it is glue and owns nothing.

"Green" at every boundary means: `cargo test --workspace`, `cargo test --doc
--workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --check`, plus the guards that crate's spec declares.

## The stability contract

- **Deterministic, and keys are stable.** The same dump produces byte-identical
  artifacts. A *newer* dump may change entries and add lemmas, but because
  homograph keys are composite — derived from the lemma and its disambiguating
  class rather than from an arbitrary sort order — a key does not silently move
  to a different word when the data is refreshed. A key changes only when the
  linguistic facts behind it change, and that is a semantic change announced in
  the changelog. There is still no lockfile; the committed generated tables are
  the whole artifact.
- **Ordering is API.** Where a slot has several valid forms, they are returned in
  a defined order and the first is the recommended one. Consumers will bless
  first-variant outputs into their tests, so reordering is a breaking change and
  is announced as one.
- **One fingerprint, one artifact.** A hash over the generated lexicon pins the
  whole-paradigm output so that any change is visible. Its scope is deliberately
  narrow — one well-defined artifact, with every delta enumerated in the
  changelog. Whole-tree hashes over everything are the anti-pattern; a scoped
  fingerprint with an enumerated delta is the working version.
- **Regularization changes are semantic changes.** Adding, removing or altering a
  `RuleId` changes output under `Policy::regularized()`, and the changelog
  enumerates the affected slots. Intentional output changes are enumerated, never
  discovered.

## Open decisions

Exactly one remains, and it is deferred by design rather than undecided: it
cannot be answered responsibly without measurements that do not exist yet.

| Decision | Options | Close by |
|---|---|---|
| **Which rules `Policy::regularized()` enables** — the *content* of the regularized policy, not which policy is default (that is closed: `attested`). | Per-rule, priced by the evaluator: `gap.fill-1sg`, `iotation.uniform`, `stress.fixed-stem`, `numeral.regular`, `suppletion.level`. Until then every rule is off in both presets and reachable only via `Policy::attested().with(rule)`. | Phase 6 — **no rule ships enabled before its impact is measured** |

### Closed during specification

| Decision | Resolution | Recorded in |
|---|---|---|
| **Stress** | **Ruthenian stores stress and renders it on request.** The lexicon keeps it, the orthography carries it as a combining acute (`pisátj`), and the CLI prints it only when asked. `StressPattern` is therefore load-bearing in `core`, and `stress.fixed-stem` is implementable. | `docs/specs/ruthenian-orthography.md` §12, `docs/specs/ruthenian-core.md` §12 |
| **Homograph keying** | **Composite keys carrying the disambiguating class**, not `_n` sense suffixes. A key is derived from linguistic properties, so it is stable across dump refreshes — which upgrades the stability contract below. | `docs/specs/ruthenian-lexicon.md` §2, §10 |
| **ё/е normalization** | **Normalize ё → е at extraction** — with the implicit stress transferred to an explicit U+0301, because the dump never marks stress on ё (verified). Ruthenian therefore spells these words `je`, and the `jo` digraph survives in the orthography's reader for round-trip totality but appears in no lexicon entry. | `docs/specs/ruthenian-extract.md` §10 |
| **The apostrophe** | **One glyph, one rule**: `'` means "the next character starts a new letter". Russian ъ is that rule at a morpheme boundary (`pod'jezd`), so the hard sign and the digraph separator are one idea, handled uniformly by the writer's separator-insertion pass. | `docs/specs/ruthenian-orthography.md` §7, §12 |
| **Default policy** | `Policy::attested()` is the default; `regularized` is opt-in for v1, so default output is checkable against a real language. Constructed as presets plus `.with(rule)` / `.without(rule)`, so the evaluator can price one rule at a time. | `docs/specs/ruthenian.md` §10 |
| **Sense storage** | Full structured senses (gloss, tags, topics), stored in a generated `senses.rdb` blob — **not** inlined in `Entry` and **not** compiled into any crate. Consumers embed it with `include_bytes!` or read it at runtime; the CLI embeds it and stays self-contained. | `docs/specs/ruthenian-lexicon.md` §2a |
| **`Form` type location** | `ruthenian-lexicon`, so the evaluator can consume it without depending on generated tables. | `docs/specs/ruthenian-lexicon.md` §2 |
| **Pronouns and numerals** | Their own `Slot` variants. Numeral government is returned as structure (case + number), never re-derived by callers. | `docs/specs/ruthenian-core.md` §3, §12 |
| **CI fixture** | A few hundred **real** dump records vendored with provenance, chosen to cover the hard cases. Never hand-written fixtures. | `docs/specs/ruthenian-extract.md` §10 |
| **No sealed test set in v1** | Score against everything attested; publish the generated caveat that the figure is coverage of known data, not generalization. Per-rule impact counts and paired diffs replace the held-out set. | `docs/specs/ruthenian-eval.md` §3 |
| **Stress in evaluation** | Scored twice — segmental (headline) and strict (including stress placement) — so neither number hides the other. | `docs/specs/ruthenian-eval.md` §2 |
| **`resolve` does not rank** | All candidates in deterministic key order; no manufactured primacy score. | `docs/specs/ruthenian.md` §10 |
| **CLI surface** | `paradigm` prints the full table by default, `principal-parts` is its own subcommand, and `ruth` carries **no** developer commands — `extract` and `eval` live only in `cargo xtask`. | `docs/specs/ruthenian-cli.md` §2, §10 |
| **Accuracy gating** | `cargo xtask accuracy` fails on a net-negative paired diff; everything else reports without gating. | `docs/specs/xtask.md` §9 |
| **No publication threshold** | Publish the first number the evaluator produces, with the coverage caveat and the unexplained-mismatch count beside it. The release gate is the paired diff, not an absolute floor. | `docs/specs/ruthenian-eval.md` §11 |
| **CLI formatting** | Tables always aligned; colour only on a TTY and never carrying information absent from the text. | `docs/specs/ruthenian-cli.md` §10 |
| **Latin-in-Cyrillic input** — verified breakage: `"cat дом"` round-trips to `"цат дом"` in the reference | Two entry points. `to_latin` is strict and returns `AlphabetError` with a byte offset; `to_latin_mixed` transliterates only declared-alphabet runs and reports the spans it skipped. The CLI uses the strict one, so a mixed-script argument is an error, never a silent guess. | `docs/specs/ruthenian-orthography.md` §2, §12; `docs/specs/ruthenian-cli.md` §3 |
