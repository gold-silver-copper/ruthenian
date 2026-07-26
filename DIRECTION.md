# Ruthenian — direction

> **The language itself is specified in [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md)**
> — eight cases, three numbers, three declensions, six conjugation classes, with
> the etymology of each restored category. **That document is normative.** This
> one describes the software that realizes it, and is authoritative only for the
> boundaries between crates.
>
> The comparative evidence behind the language — PIE, Sanskrit, OCS, Russian,
> Ukrainian, Belarusian, Interslavic, measured — is in
> [`docs/COMPARATIVE_GRAMMAR.md`](docs/COMPARATIVE_GRAMMAR.md). What the
> specification still owes is in
> [`docs/OPEN_QUESTIONS.md`](docs/OPEN_QUESTIONS.md).

**The whole repository is five documents.** This one, the three above, and
`CHANGELOG.md`. Everything else is either a licence notice (`ATTRIBUTION.md`) or
belongs to a single crate or tool, and lives beside it. If a seventh guiding
document seems necessary, one of these five is doing its job badly.

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
10 667 129 lines — read in full for every language (law 3).

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
   │    (entry, slot) → Option<Form>                 │
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
| 2 | `ruthenian-core` | **Ruthenian** morphology as pure rules, plus the grammatical vocabulary the whole workspace shares. | `Case` (8), `Number` (3), `Gender`, `Person`, `Tense` (6), `Aspect`, `Slot`, the three declensions, the six conjugation classes, the rule engine. | `ruthenian-orthography` | Every slot for every class either produces a form or is a declared gap; output matches `docs/RUTHENIAN.md`'s tables; no lexical data anywhere in the crate. |
| 3 | `ruthenian-lexicon` | The lexical entry schema every other crate agrees on. | `Entry`, `PrincipalParts`, `Provenance`, `Origin`, the artifact formats and their versioning. | `ruthenian-core`, `ruthenian-orthography` | The extractor, the facade and the evaluator all speak these types and no crate defines a second entry representation. |
| 4 | `ruthenian-extract` | Turn the dump into the lexicon, once, deterministically — grouping cognates by etymon and reconstructing the Ruthenian form. | The dump schema knowledge, the source-language class codes, the reject histogram, the dump fingerprint. | `ruthenian-lexicon`, `ruthenian-core`, `ruthenian-orthography`, serde | Same dump in → byte-identical artifacts out, with every rejected record counted by reason and every reconstruction carrying its evidence. |
| 5 | `ruthenian` | The facade: one generation path over rules + generated tables, every answer carrying its origin. | The generated PHF tables, the public API. | `ruthenian-core`, `ruthenian-lexicon`, `phf` | One call site produces every form; tables contain no row the rules already predict. |
| 6 | `ruthenian-eval` | Measure the facade against **the specification** and produce the one canonical summary. | The conformance corpus derived from `docs/RUTHENIAN.md`, the metric definitions, `summary.json`. | `ruthenian`, `ruthenian-lexicon` | Every number in the README is generated from `summary.json`; spec conformance, spec coverage and source-language distance are reported separately and never averaged. |
| 7 | `ruthenian-cli` | The `ruth` binary. | Argument parsing, output formatting. | `ruthenian`, `ruthenian-orthography` | Contains no morphology; every subcommand is a thin adapter with `--json`. |
| 8 | `xtask` | `refresh-data`, `check-registry`, `conformance`. | Nothing. | `ruthenian-extract`, `ruthenian-eval` | Orchestration only — no logic that belongs in the crate it invokes. |

**A crate's own documentation lives in the crate**, as
`crates/<name>/README.md`, and is written when the crate is — a specification
for an unbuilt crate is a guess with a version number. This document is
authoritative for the boundaries *between* crates and for the decisions that
span them; `docs/RUTHENIAN.md` outranks it on any question about the language.

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

**There is no configuration axis.** Generation is a function:
`(entry, slot) → Option<Form>`. No policy, no variant, no feature flags, and
nothing that changes an answer for a fixed input.

An earlier design had one — a `Variant` switching `docs/RUTHENIAN.md` §13's open
questions (the ablative plural, clitic pronouns, the middle voice) — and it was
wrong twice over. It carried rules that were *permanently disabled*, since none
may ship enabled while the specification still calls the question open, so every
rule in the engine would have grown a dead branch waiting on a decision that had
not been made. And it mistook the category: §13's entries are **language-design
questions awaiting an answer**, not options a caller picks between. Treating them
as runtime configuration presupposes Ruthenian ships as several simultaneous
dialects, which nothing here calls for.

The language is fixed by `docs/RUTHENIAN.md`. When it changes, the specification
changes and the code changes with it — a source edit and a changelog entry, not a
switch. There is also emphatically no axis running between "Ruthenian" and
"Russian": reproducing Russian is not a thing this system is for.

What survives is **provenance**, which is not configuration. Every form carries a
trace naming the rules that produced it, because a caller is entitled to know
*why* a form looks the way it does, and that is as true of one fixed grammar as
of a configurable one.

**`ruthenian-lexicon` earns its place** because the extractor must not depend on
the artifact it generates. If the entry types lived in the facade, the extractor
would depend on the crate whose tables it writes — conceptually inverted, and a
trap the first refactor would spring. A zero-dependency schema crate lets
`extract`, `ruthenian` and `eval` agree without any of them owning the others.

## The laws

These hold in every crate. Each is falsifiable: the **check** is the command or
test that catches a violation, and a law without one is an aspiration rather than
a law. Where a law was learned the hard way, the source is named — most were paid
for by `slovowiki`, `interslavic` or `english` before this project started.

### About the language

1. **The specification decides; the code conforms.** Where `docs/RUTHENIAN.md`
   states a form, that form is correct by definition and a disagreeing engine is
   wrong. Where the spec is silent, the gap is reported as a spec gap and closed
   *there* — never patched with a guess in code.
   *Check:* the conformance corpus, extracted from the spec and asserted against
   the engine.

2. **Claims about a source language are measured; claims about Ruthenian are
   specified.** A statement about how Russian, Ukrainian, Belarusian, Polish or
   OCS works is backed by a count over the dump, not by a grammar reference
   alone — references say what to look for, the data says what is there. A
   statement about *Ruthenian* is backed by the specification, because there is
   no Ruthenian corpus and never will be.
   Conflating the two is how a constructed language turns back into a description
   of Russian. The converse error is as bad: taking a measured fact about Russian
   as though it settled a question the spec had already answered differently.
   *Learned:* a published summary described Russian noun accent pattern `f` as
   stem-stressed in the singular. It is ending-stressed, as ~285 000 attested
   forms show. Implementing the quoted version would have been wrong for every
   `f` noun.
   *Check:* `tools/measure.py` regenerates every source-language figure; a diff
   against the committed docs is empty.

3. **Every measurement scans the entire dump. No sampling.** Any number
   describing a source language — class distributions, mutation counts, gap
   counts, lemma inventories, the reflex percentages the spec reasons from — is
   computed over **all 10 667 129 lines** of `raw-wiktextract-data.jsonl`, per
   language code.
   *Learned:* sampling was used early and was wrong three separate times, in ways
   a bigger sample would not have fixed. A sample said 117 verb class codes; the
   truth is **226**, and it missed 9 the parser could not parse. A sample said
   670 class-1 labial stems take no epenthesis; the truth is **1 977, and not one
   takes it** — the rule is exceptionless, which only a full scan can establish,
   and "always" is what lets an engine key on a rule instead of hedging around
   it. Even a full-file `grep` is not automatically a full scan: an early census
   used a pattern assuming JSON key order and found 183 of the 226. **Parse the
   records; do not pattern-match them.**
   *Check:* no `dd if=`, `skip=`, `count=`, `| head -` or "sampled" appears in any
   tracked file.

4. **Conformance, coverage and distance are three numbers. Never averaged, never
   substituted.** Conformance: of the cells the spec states, how many does the
   engine produce (a miss is an engine bug). Coverage: of the cells the language
   has, how many does the spec state (a miss is a spec hole). Distance: how far a
   form sits from its source-language cognates — descriptive only, never called
   accuracy, never gating.
   *Why:* the single number this project was originally specified to publish —
   accuracy against attested Russian — cannot exist. Most Ruthenian cells have no
   Russian counterpart, so the figure would silently score the overlapping
   minority and would *improve* as the language moved closer to Russian.
   *Check:* `Summary`'s shape, plus a check that no generated prose calls a
   distance an accuracy.

### About the code

5. **One generation path.** The CLI and the evaluator are adapters over the same
   call. If a second way to produce a form appears, one of them is wrong and
   nobody knows which.
   *Learned:* slovowiki's benchmark and its website ran materially different
   pipelines, so its published accuracy described neither.
   *Check:* the evaluator depends on the facade's public API only.

6. **Rules predict; tables store the residue.** The extractor drops any form the
   rule engine already produces, so the tables hold exactly what no rule
   recovers. Changing a rule therefore changes what counts as irregular and
   **requires regenerating the tables**.
   *Check:* for every table row, the predictor's output differs from it.

7. **Derive state; never hand-maintain it.** No field that duplicates something
   computable. A hand-maintained flag drifts, and its dead branch becomes the
   bug. Aspect, paradigm gaps, the fleeting vowel and reducibility are all
   derived; none is stored.
   *Check:* a schema check paired with a differential test against the deriving
   function — the schema check alone would not catch the value smuggled in under
   another name.

8. **Typed until the last moment; one stringification point.** Forms travel as
   typed values with their origin attached. Nothing is recovered by searching a
   string that was already structured.

9. **Structure, not strings, at every boundary.** Return the parts, not a string
   for the caller to parse; return the resolved case and number, not just the
   word.
   *Learned:* the single mistake `interslavic` paid for across four consecutive
   releases, and shipped `quantified_parts` to fix.

10. **No droppable side channels.** If a caller can silently forget a field and
    still compile, that field will be silently forgotten. Losing information is a
    type error.

11. **Provenance travels with the form.** Every form knows whether it is
    rule-derived or stored, and carries the trace of the rules that built it;
    every *lemma* knows which source languages attest it and how confidently its
    reconstruction follows. API, not documentation.
    *Check:* every `Prediction` carries a non-empty trace; every entry carries
    non-empty provenance.

12. **`None` means "no such form exists".** Never "not implemented", never
    "unknown". `Ok(None)` is a claim about the language; `Err(Unsupported)` is a
    claim about the code. Conflating them makes every `None` untrustworthy.
    *Check:* structural gaps are derived from the grammar, not read from data.

13. **Pure build logic, thin writer.** Every artifact is produced by a pure
    function returning a plan; exactly one place writes bytes to disk.
    *Check:* no file is opened inside a `plan` function.

14. **One canonical owner per artifact; docs and metrics are generated from it.**
    No number is hand-copied into prose. A README figure that disagrees with
    `summary.json` is a build failure, not a typo.
    *Learned:* one verb-coverage figure was published three times three ways —
    `~76 %`, `73.2 %`, `87.6 %` — before the true value (`90.7 %`) was measured.
    *Check:* regeneration diffs against the committed docs.

15. **Every guard has a failure witness, and the witness is verified.** A guard
    ships only after its named mutation has been applied, observed to fail it,
    and reverted. A guard that survives its own witness is stale and is deleted,
    not left in place looking reassuring.
    *Learned:* phase 1 found two stale guards this way; phase 2 found a third,
    plus two witnesses that were themselves wrong. Guards are not
    self-evidently correct.
    *Check:* the mutation table in each phase's report.

16. **The dependency-free crates stay dependency-free.** `ruthenian-orthography`
    has zero dependencies; `ruthenian-core` depends on it and nothing else.
    *Check:* a `no_dependencies` test in each crate's guard suite — a test, not a
    review habit.

17. **Redistributed data carries its licence.** Any third-party data committed
    here is recorded in `ATTRIBUTION.md` with its source and licence, **in the
    same change that adds it**. Wiktionary-derived content is CC BY-SA 4.0 +
    GFDL and requires attribution and share-alike from anyone redistributing it
    further.

Breaking one of these is not a trade-off to be weighed. It is a bug, and the fix
is to restore the law, not to document the exception.

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
| **`docs/RUTHENIAN.md` §13's open questions** — the ablative plural, clitic pronouns, the middle voice. Each changes the language, so each changes the engine. | Answer them in the specification. They are not runtime options and no code models them until the spec settles them; the supine is the worked example, specified in §7.10a and therefore simply implemented. | Before phase 2 ships the affected paradigms — **a question left open is a paradigm that cannot be finished** |
| **Cognate grouping method** — the unsolved lexicon problem (`docs/RUTHENIAN.md` §12.2). Explicit Proto-Slavic links cover 5 517 etyma, only 88 of them attested across all five source languages. | Phonological matching plus English-gloss pivoting, as slovowiki does. Scoped as its own phase, not assumed away. | Phase 4 — it is the gate on lexicon quality, not a detail of it |

### Closed during specification

Recorded here because they span crate boundaries or predate the crates they
constrain. When a crate is built, its README restates the ones it implements.


| Decision | Resolution | Applies to |
|---|---|---|
| **The measurement baseline** | **The specification, not any natural language.** `docs/RUTHENIAN.md`'s paradigm tables are the conformance corpus. Attested forms are reconstruction evidence; comparison to Russian is reported as distance and is never called accuracy. | `ruthenian-eval` |
| **Source-language classifications** | **Extraction-time only.** Zaliznyak classes and stress letters are how a cognate is read out of the dump and mapped onto one of Ruthenian's six classes. They appear in no public type. | `ruthenian-extract`, `ruthenian-core` |
| **Stress** | **Ruthenian stores stress and renders it on request.** Stress is fixed per word (`docs/RUTHENIAN.md` §2.1), so there are no mobile patterns to model — but the lexicon keeps the position, the orthography carries it as a combining acute (`pisátj`), and the CLI prints it only when asked. Running text never marks it. | `ruthenian-orthography`, `ruthenian-core` |
| **Homograph keying** | **Composite keys carrying the disambiguating class**, not `_n` sense suffixes. A key is derived from linguistic properties, so it is stable across dump refreshes. | `ruthenian-lexicon` |
| **ё/е normalization** | **Normalize ё → е at extraction** — with the implicit stress transferred to an explicit U+0301, because the dump never marks stress on ё (verified). This is a *source-reading* rule; Ruthenian's own orthography is settled by `docs/RUTHENIAN.md` §2.1. | `ruthenian-extract` |
| **The apostrophe** | **One glyph, one rule**: `'` means "the next character starts a new letter" (`docs/RUTHENIAN.md` §2.1). | `ruthenian-orthography` |
| **No configuration axis** | Generation is `(entry, slot) → Option<Form>`. There is no policy, variant or feature flag: the language is fixed by the specification, so changing it is a source edit rather than a runtime switch. Provenance survives as the trace. | `ruthenian-core`, `ruthenian` |
| **Sense storage** | Full structured senses (gloss, tags, topics), stored in a generated `senses.rdb` blob — **not** inlined in `Entry` and **not** compiled into any crate. The CLI embeds it and stays self-contained. | `ruthenian-lexicon` |
| **`Form` type location** | `ruthenian-lexicon`, so the evaluator can consume it without depending on generated tables. | `ruthenian-lexicon` |
| **Pronouns and numerals** | Their own `Slot` variants. Numeral government is returned as structure (case + number), never re-derived by callers — and it is regular in Ruthenian (`docs/RUTHENIAN.md` §6.1), with `dva` governing the dual. | `ruthenian-core` |
| **CI fixture** | A few hundred **real** dump records vendored with provenance, chosen to cover the hard cases. Never hand-written fixtures. | `ruthenian-extract` |
| **Conformance corpus** | **Extracted once into a committed artifact**, not parsed live inside the assertion. Amending the spec regenerates it and the diff is reviewed; a currency check fails if the two drift. Coverage of the spec is reported alongside conformance to it. | `ruthenian-core`, `ruthenian-eval` |
| **Stress in evaluation** | Scored twice — segmental (headline) and strict (including stress placement) — so neither number hides the other. | `ruthenian-eval` |
| **`resolve` does not rank** | All candidates in deterministic key order; no manufactured primacy score. | `ruthenian` |
| **CLI surface** | `paradigm` prints the full table by default, `principal-parts` is its own subcommand, and `ruth` carries **no** developer commands — `extract` and `eval` live only in `cargo xtask`. | `ruthenian-cli` |
| **Conformance gating** | `cargo xtask conformance` fails on any regression against the spec corpus; everything else reports without gating. | `xtask` |
| **Latin-in-Cyrillic input** — verified breakage: `"cat дом"` round-trips to `"цат дом"` in the reference | Two entry points. `to_latin` is strict and returns `AlphabetError` with a byte offset; `to_latin_mixed` transliterates only declared-alphabet runs and reports the spans it skipped. The CLI uses the strict one, so a mixed-script argument is an error, never a silent guess. | `ruthenian-orthography`, `ruthenian-cli` |
