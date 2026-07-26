# Ruthenian — direction

> **The language itself is specified in [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md)**
> — eight cases, three numbers, three declensions, six conjugation classes, with
> the etymology of each restored category. **That document is normative.** This
> one describes the software that realizes it, and is authoritative only for the
> boundaries between crates.
>
> The comparative evidence behind the language is in
> [`docs/COMPARATIVE_GRAMMAR.md`](docs/COMPARATIVE_GRAMMAR.md). The source
> languages the lexicon draws on are described in
> [`docs/sources/`](docs/sources/) — those are studies of *other* languages, not
> of Ruthenian.

## What Ruthenian is

Ruthenian is a **constructed Latin-script East Slavic literary language, more
conservative than Russian or Ukrainian and more regular than either**.

It restores three categories modern East Slavic lost — the **ablative**, the
**vocative** and the **dual** — along with the aorist, the imperfect, the OCS
long/short adjective and the full copula. It removes what makes Russian hard:
mobile stress, heteroclitics, soft adjective stems, indeclinables, fleeting
vowels in the genitive plural, irregular numeral government, ten of Russian's
sixteen verb classes, and lexical aspect pairing.

| | Ruthenian | Russian | Ukrainian | OCS |
|---|---|---|---|---|
| cases | **8** | 6 | 7 | 7 |
| numbers | **3** | 2 | 2 | 3 |
| noun declensions | **3** | 8 | 4 | 8 |
| verb classes | **6** | 16 | 13 | 5 |
| past tenses | **3** | 1 | 1 | 3 |
| stress | fixed | 10 patterns | mobile | mobile |
| 2nd palatalization | **kept** | lost (0 %) | kept (99 %) | kept (66 %) |

**Every restored feature is attested in the family and every regularization has a
precedent in a sister language.** Nothing is invented; the novelty is in the
combination. That is the design brief, and it is what separates this from both a
relexification and an *a priori* conlang.

It ships as a **command-line tool** over a library workspace. You give it a
lemma; it gives you every form, and tells you where each form came from.

What it is not:

- **Not Russian in Latin letters.** Russian has no dual, no ablative, no aorist,
  no productive vocative, and 0 % second palatalization. A Ruthenian paradigm is
  mostly cells Russian does not have. Anything that treats Russian as the target
  output is measuring the wrong language.
- **Not a website.** No static site generation, no server, no HTML. The CLI is
  the product.
- **Not an Interslavic project.** No Interslavic data enters this repository —
  not the dictionary, not a derived cache, not a single gloss. Interslavic is a
  reference point in two narrow senses: a constructed standard may be more
  regular than the natural languages it draws on, and `interslavic-rs` is an
  architecture worth learning from. Both are ideas, not data.
- **Not a morphological analyser.** Generation only: lemma → forms. Surface →
  lemma is out of scope for every crate here.

## The specification is the ground truth

This is the load-bearing decision of the whole project, and everything below
follows from it.

Ruthenian is a **specified** language. Its paradigms are fixed by
`docs/RUTHENIAN.md`, not discovered from a corpus. There is no body of attested
Ruthenian text and there never will be, so:

- **The engine is measured against the specification**, not against any natural
  language. `docs/RUTHENIAN.md`'s paradigm tables (`dom`, `konj`, `drug`, `okno`,
  `polje`, `zzena`, `zjemlja`, `noczj`, `kostj`, and the verb tables of §7) are
  the conformance fixtures. A cell the engine gets wrong is a bug in the engine;
  a cell the spec does not state is a hole in the spec.
- **Attested forms are evidence, not answers.** Russian, Ukrainian, Belarusian,
  Polish and OCS supply the etyma and the cognates from which a Ruthenian lemma
  is *reconstructed* (`docs/RUTHENIAN.md` §12.2). They do not supply the target
  output, and agreement with any one of them is neither required nor a score.
- **Comparing output to Russian is a description, never a metric.** Such a
  comparison answers "how far has Ruthenian moved from Russian here", which is
  interesting and is reported as a distance. It is not accuracy, is never called
  accuracy, and never gates a release.

The failure this rules out is the one the project would otherwise have shipped:
an "accuracy" figure computed against Russian, which for `domogo`, `doma` (abl),
`domje` (voc), `druzi`, `druzzje` and the whole dual column has no counterpart to
compare against at all. That is not a low score; it is a category error.

## The lexicon is multi-source

Ruthenian draws its vocabulary from **all of East Slavic plus Polish and Old
Church Slavonic**, and adapts international vocabulary through a declared
borrowing system (`docs/RUTHENIAN.md` §9, §12.3).

| Tier | Languages | Role |
|---|---|---|
| **primary** | Russian, Ukrainian, Belarusian | the East Slavic core |
| **secondary** | Polish, Old Church Slavonic | recover what East Slavic levelled |
| **borrowing** | Latin, Greek, Sanskrit, English, French, German | adapted by rule |

Measured lemma inventories, all from full scans of the same dump: Russian
419 283, Polish 152 325, Ukrainian 52 223, Belarusian 6 899, Old Church Slavonic
4 311.

**Why more than one source is necessary, not merely nice.** Russian alone cannot
supply the language specified above. Yat is the clearest case: it is a phoneme in
~15 % of the inherited vocabulary, Russian merged it into `e`, and Ukrainian
(`i`), Polish (`ie`/`ia`) and OCS (`ě`) each preserve it. The same holds for the
nasal vowels, which only Polish keeps (`ą`, `ę`); for the second palatalization,
which Russian levelled to 0 % while Ukrainian keeps at 99 %; for the vocative,
where Russian has 40 relic forms against Ukrainian's 25 180; and for the dual,
which only OCS attests at all (77 714 forms). Every one of those is information
the specification needs and Russian has destroyed.

A Ruthenian lemma is therefore **the reflex the Proto-Slavic etymon would have in
Ruthenian's phonology**, with the attested cognates as evidence — not the Russian
word transliterated. How confidently that reconstruction follows is recorded per
entry, because a form derived from one language's reflex is a weaker claim than
one attested across four.

The corpus is the same English Wiktionary dump —
`~/Desktop/code/wikidata/raw-wiktextract-data.jsonl`, 23 622 298 877 bytes,
10 667 129 lines — read in full for every language (`INVARIANTS.md` I1).

**Interslavic remains excluded.** It is a grammar reference, not a lexical
source: no Interslavic data enters the crate.

## The shape of the system

```text
   raw-wiktextract-data.jsonl          (22 GiB, read once, never at runtime)
                │                       SOURCE-LANGUAGE cognates and etyma
                ▼
        ruthenian-extract               streaming; pure build plan, thin writer
                │                       reconstructs Ruthenian lemmas by rule
                │
                ├──────────────► lexicon artifact  (entries: declension, class,
                │                                    principal parts, gender,
                │                                    animacy, aspect, gaps,
                │                                    reconstruction provenance)
                ▼
         generated PHF tables           ONLY what the rules cannot predict
                │
                ▼
   ┌─────────────────────────────────────────────────┐
   │  ruthenian  (facade)                            │
   │    one generation path:                         │
   │    (entry, slot, variant) → Option<Form>        │
   │    Form = text + origin + trace                 │
   └──────┬───────────────────────────────────┬──────┘
          │                                   │
          ▼                                   ▼
    ruthenian-cli                       ruthenian-eval
    (`ruth`)                            conformance vs docs/RUTHENIAN.md

   ruthenian-core         productive RUTHENIAN rules — predictor AND fallback
   ruthenian-lexicon      the types everything above agrees on
   ruthenian-orthography  the only script conversion in the system
```

The load-bearing property: **the CLI and the evaluator consume the same `Form`
from the same call.** The published conformance number therefore describes the
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
| 2 | `ruthenian-core` | **Ruthenian** morphology as pure rules, plus the grammatical vocabulary the whole workspace shares. | `Case` (8), `Number` (3), `Gender`, `Person`, `Tense` (6), `Aspect`, `Slot`, the three declensions, the six conjugation classes, the rule engine, `RuleId`, `Variant`. | `ruthenian-orthography` | Every slot for every class either produces a form or is a declared gap; output matches `docs/RUTHENIAN.md`'s tables; no lexical data anywhere in the crate. |
| 3 | `ruthenian-lexicon` | The lexical entry schema every other crate agrees on. | `Entry`, `PrincipalParts`, `Provenance`, `Origin`, the artifact formats and their versioning. | `ruthenian-core`, `ruthenian-orthography` | The extractor, the facade and the evaluator all speak these types and no crate defines a second entry representation. |
| 4 | `ruthenian-extract` | Turn the dump into the lexicon, once, deterministically — grouping cognates by etymon and reconstructing the Ruthenian form. | The dump schema knowledge, the source-language class codes, the reject histogram, the dump fingerprint. | `ruthenian-lexicon`, `ruthenian-core`, `ruthenian-orthography`, serde | Same dump in → byte-identical artifacts out, with every rejected record counted by reason and every reconstruction carrying its evidence. |
| 5 | `ruthenian` | The facade: one generation path over rules + generated tables, every answer carrying its origin. | The generated PHF tables, the public API. | `ruthenian-core`, `ruthenian-lexicon`, `phf` | One call site produces every form; tables contain no row the rules already predict. |
| 6 | `ruthenian-eval` | Measure the facade against **the specification** and produce the one canonical summary. | The conformance corpus derived from `docs/RUTHENIAN.md`, the metric definitions, `summary.json`. | `ruthenian`, `ruthenian-lexicon` | Every number in the README is generated from `summary.json`; spec conformance, spec coverage and source-language distance are reported separately and never averaged. |
| 7 | `ruthenian-cli` | The `ruth` binary. | Argument parsing, output formatting. | `ruthenian`, `ruthenian-orthography` | Contains no morphology; every subcommand is a thin adapter with `--json`. |
| 8 | `xtask` | `refresh-data`, `check-registry`, `conformance`. | Nothing. | `ruthenian-extract`, `ruthenian-eval` | Orchestration only — no logic that belongs in the crate it invokes. |

Detailed specs live in `docs/specs/<crate>.md`. Each spec is authoritative for
its crate; this document is authoritative for the boundaries between them; and
`docs/RUTHENIAN.md` outranks all of them on any question about the language.

### Three structural decisions, made deliberately

**The grammatical vocabulary is Ruthenian's, not any source language's.**
`ruthenian-core` defines eight cases, three numbers and six past/present tenses
because that is what `docs/RUTHENIAN.md` specifies. A six-case `Case` enum cannot
represent `domogo` or `doma`; a two-value `Number` cannot represent `domoma`.
Source-language classifications — Zaliznyak's sixteen verb classes and their
stress letters, Russian stem classes — are **extraction-time input**, used to
read a cognate out of the dump and map it onto a Ruthenian class. They do not
appear in `ruthenian-core`'s public API, in `Entry`, or in any type the facade
returns. Ruthenian's six conjugation classes correspond to Zaliznyak's 1–6
(`docs/RUTHENIAN.md` §7.3), which is exactly why the mapping belongs at the
boundary where source data enters and nowhere else.

**The variant policy is not a crate.** It is tempting to build
`ruthenian-standard` alongside `ruthenian` and let it apply optional features.
That would recreate the exact failure slovowiki spent 919 pages of review on: two
code paths generating the same data, diverging quietly, with no way to know which
one the published numbers describe. Instead:

- `Variant` is a typed parameter on the single generation path;
- every optional feature is a `RuleId` in `ruthenian-core`, applied inside the
  one rule engine;
- `Variant::standard()` is the language as specified — the conformance baseline;
- `docs/VARIANTS.md` is **generated** from the rule table, so it cannot drift
  from the rules it describes.

What `Variant` switches is the set of **genuinely open questions in the
specification** (`docs/RUTHENIAN.md` §13): the ablative plural, clitic pronouns,
the middle voice, the supine, the etymological alphabet. It does *not* switch
between "Ruthenian" and "Russian" — that axis does not exist, because reproducing
Russian is not a thing this system is for.

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
2. **Rules predict; tables store the residue.** The extractor drops any form the
   rule engine already produces, so the tables are exactly the unpredictable
   residue — the reconstructed stems and principal parts no rule recovers.
   Changing a rule therefore changes what counts as irregular and **requires
   regenerating the tables** — enforced by a dump-free layering check. (A2)
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
7. **Provenance travels with the form.** Every form knows whether it is
   rule-derived, stored, or produced under an optional variant, and by which
   rule; every *lemma* knows which source languages attest it and how confidently
   its reconstruction follows. This is API, not documentation. (B1)
8. **`None` means "no such form exists".** Never "not implemented", never
   "unknown". A cell the specification declares absent is `None` and is
   documented as by design. (B2)
9. **One canonical owner per artifact; docs and metrics are generated from it.**
   No number is hand-copied into prose. A README figure that disagrees with
   `summary.json` is a build failure, not a typo. (B3, C3)
10. **The spec decides; the code conforms.** Where `docs/RUTHENIAN.md` states a
    form, that form is correct by definition and a disagreeing engine is wrong.
    Where the spec is silent, the gap is reported as a spec gap and closed there
    — never patched with a guess in the code. Where a source language is
    genuinely variable, say so and state which way the spec went and why. (B4)
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
   until the rules exist. Core is also the first phase that can be checked
   against `docs/RUTHENIAN.md` directly, and it should be.
3. **Lexicon** third: the schema can only be designed once you know what the
   rules need supplied to them (which is exactly the set of things they cannot
   predict).
4. **Extract** fourth: now the dump can be read once and turned into artifacts.
5. **Facade** fifth: rules plus tables behind one call.
6. **Eval** sixth: the first point at which any conformance claim may be made
   over the whole lexicon. No number is published before this phase exists.
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
- **A spec change is a semantic change.** `docs/RUTHENIAN.md` is normative, so
  amending it changes the language and therefore the output. Every such amendment
  enumerates the affected slots in the changelog, and the conformance corpus is
  regenerated in the same change. Intentional output changes are enumerated,
  never discovered.

## Open decisions

Two remain. Both are deferred by design rather than undecided.

| Decision | Options | Close by |
|---|---|---|
| **Which optional features `Variant` offers**, and which ship enabled. The candidates are `docs/RUTHENIAN.md` §13's open questions: the ablative plural, clitic pronouns, the middle voice, the supine. | Per-feature, each a `RuleId`. Until the spec settles them, every one is off in the standard variant and reachable only via `Variant::standard().with(rule)`. | When §13 closes — **no optional feature ships enabled while the spec still calls it open** |
| **Cognate grouping method** — the unsolved lexicon problem (`docs/RUTHENIAN.md` §12.2). Explicit Proto-Slavic links cover 5 517 etyma, only 88 of them attested across all five source languages. | Phonological matching plus English-gloss pivoting, as slovowiki does. Scoped as its own phase, not assumed away. | Phase 4 — it is the gate on lexicon quality, not a detail of it |

### Closed during specification

| Decision | Resolution | Recorded in |
|---|---|---|
| **The measurement baseline** | **The specification, not any natural language.** `docs/RUTHENIAN.md`'s paradigm tables are the conformance corpus. Attested forms are reconstruction evidence; comparison to Russian is reported as distance and is never called accuracy. | `docs/specs/ruthenian-eval.md` §1, §2 |
| **Source-language classifications** | **Extraction-time only.** Zaliznyak classes and stress letters are how a cognate is read out of the dump and mapped onto one of Ruthenian's six classes. They appear in no public type. | `docs/specs/ruthenian-extract.md` §4; `docs/specs/ruthenian-core.md` §3 |
| **Stress** | **Ruthenian stores stress and renders it on request.** Stress is fixed per word (`docs/RUTHENIAN.md` §2.1), so there are no mobile patterns to model — but the lexicon keeps the position, the orthography carries it as a combining acute (`pisátj`), and the CLI prints it only when asked. Running text never marks it. | `docs/specs/ruthenian-orthography.md` §12, `docs/specs/ruthenian-core.md` §12 |
| **Homograph keying** | **Composite keys carrying the disambiguating class**, not `_n` sense suffixes. A key is derived from linguistic properties, so it is stable across dump refreshes. | `docs/specs/ruthenian-lexicon.md` §2, §10 |
| **ё/е normalization** | **Normalize ё → е at extraction** — with the implicit stress transferred to an explicit U+0301, because the dump never marks stress on ё (verified). This is a *source-reading* rule; Ruthenian's own orthography is settled by `docs/ORTHOGRAPHY.md`. | `docs/specs/ruthenian-extract.md` §10 |
| **The apostrophe** | **One glyph, one rule**: `'` means "the next character starts a new letter" (`docs/RUTHENIAN.md` §2.1). | `docs/specs/ruthenian-orthography.md` §7, §12 |
| **Default variant** | `Variant::standard()` — the language exactly as specified — is the default and the conformance baseline. Optional features are opt-in, constructed as presets plus `.with(rule)` / `.without(rule)`. | `docs/specs/ruthenian.md` §10 |
| **Sense storage** | Full structured senses (gloss, tags, topics), stored in a generated `senses.rdb` blob — **not** inlined in `Entry` and **not** compiled into any crate. The CLI embeds it and stays self-contained. | `docs/specs/ruthenian-lexicon.md` §2a |
| **`Form` type location** | `ruthenian-lexicon`, so the evaluator can consume it without depending on generated tables. | `docs/specs/ruthenian-lexicon.md` §2 |
| **Pronouns and numerals** | Their own `Slot` variants. Numeral government is returned as structure (case + number), never re-derived by callers — and it is regular in Ruthenian (`docs/RUTHENIAN.md` §6.1), with `dva` governing the dual. | `docs/specs/ruthenian-core.md` §3, §12 |
| **CI fixture** | A few hundred **real** dump records vendored with provenance, chosen to cover the hard cases. Never hand-written fixtures. | `docs/specs/ruthenian-extract.md` §10 |
| **Conformance corpus** | Derived from `docs/RUTHENIAN.md`'s tables, not hand-written, so it cannot drift from the spec. Coverage of the spec is reported alongside conformance to it. | `docs/specs/ruthenian-eval.md` §3 |
| **Stress in evaluation** | Scored twice — segmental (headline) and strict (including stress placement) — so neither number hides the other. | `docs/specs/ruthenian-eval.md` §2 |
| **`resolve` does not rank** | All candidates in deterministic key order; no manufactured primacy score. | `docs/specs/ruthenian.md` §10 |
| **CLI surface** | `paradigm` prints the full table by default, `principal-parts` is its own subcommand, and `ruth` carries **no** developer commands — `extract` and `eval` live only in `cargo xtask`. | `docs/specs/ruthenian-cli.md` §2, §10 |
| **Conformance gating** | `cargo xtask conformance` fails on any regression against the spec corpus; everything else reports without gating. | `docs/specs/xtask.md` §9 |
| **Latin-in-Cyrillic input** — verified breakage: `"cat дом"` round-trips to `"цат дом"` in the reference | Two entry points. `to_latin` is strict and returns `AlphabetError` with a byte offset; `to_latin_mixed` transliterates only declared-alphabet runs and reports the spans it skipped. The CLI uses the strict one, so a mixed-script argument is an error, never a silent guess. | `docs/specs/ruthenian-orthography.md` §2, §12; `docs/specs/ruthenian-cli.md` §3 |
