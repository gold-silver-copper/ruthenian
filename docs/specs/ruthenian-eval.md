# Spec: `ruthenian-eval`

Phase 6. Depends on `ruthenian`, `ruthenian-lexicon`.

## 1. Purpose

Measure the shipped facade against **the specification**, and produce the one
canonical summary that every reported number is generated from.

This crate exists to make one specific failure impossible: publishing a figure
that describes something other than the tool people use. It therefore scores by
calling `ruthenian::form` — the same entry point the CLI calls — and never by
reconstructing a parallel approximation of the pipeline.

### The baseline is `docs/RUTHENIAN.md`, not any natural language

Ruthenian is specified, not attested (`DIRECTION.md`, "The specification is the
ground truth"). There is no corpus of Ruthenian text, so there is nothing to
score against except the document that defines the language — and that is
sufficient, because the document is normative: a form it states is correct by
definition.

An earlier version of this spec said "measure the shipped facade against attested
Russian". That cannot work, and the reason is worth stating so it is not
reintroduced. A Ruthenian paradigm is mostly cells Russian does not have:
`domogo` (gen), `doma` (abl), `domje` (voc), `druzi`, `druzzje`, and every cell
of the dual. Scoring against Russian would silently restrict the measurement to
the minority of cells that happen to overlap — and worse, the score would *rise*
as Ruthenian moved closer to Russian, which is backwards for a language specified
to be more conservative than Russian and to keep what Russian levelled away.

Attested forms keep a real job, one level up: they are the **evidence** a
Ruthenian lemma is reconstructed from (`../RUTHENIAN.md` §12.2). That is
`ruthenian-extract`'s concern, and its quality is reported here as
reconstruction confidence, never as accuracy.

Wrong to put here: anything that changes output. The evaluator observes; it never
patches, never post-processes, never "fixes up" a form before comparing. Equally
wrong: treating a disagreement with the spec as a spec bug. If the engine and
`../RUTHENIAN.md` disagree, the engine is wrong — that is what normative means.
Amending the spec to match the code is how the baseline stops meaning anything.

## 2. Three quantities, never averaged

Per `INVARIANTS.md` I3, the specification baseline yields three distinct numbers.
Collapsing any two produces a figure that means nothing, so they are reported
separately and no arithmetic combines them.

### Conformance — the headline

Of the cells `../RUTHENIAN.md` states, what fraction does the facade reproduce?

**Per slot, not per form.** A slot counts as a hit when the facade's output
matches the spec's stated form for that slot. Where the spec states alternatives
(`dom` / `doma` in the accusative, by animacy), the whole set is the target and
matching any declared member is a hit.

| Class | Meaning | Target |
|---|---|---|
| **Hit** | Output = the form the spec states | maximize — **this must reach 100 %** |
| **Variant-explained** | Output ≠ standard, and the trace names an enabled optional `RuleId` claiming this slot | expected; report per rule |
| **Unexplained** | Output ≠ spec, no rule accounts for it | **a bug; must go to zero** |
| **Gap agreement** | Facade returns `None`, spec declares no such cell | correct behaviour, counted separately |
| **Gap disagreement** | One says gap, the other does not | a bug in either direction |

Conformance differs from an accuracy figure in a way that matters: **100 % is
attainable and is the actual target.** The spec is finite and authoritative, so
any shortfall is a defect with a known fix, not an asymptote to creep toward.

### Coverage — the honesty companion

Of the cells the language *has*, what fraction does the spec state? 100 %
conformance over a spec that tabulates nine paradigms is not a claim about the
language as a whole, and reporting it alone would imply otherwise.

A miss here is a **hole in the specification**, closed by amending
`../RUTHENIAN.md` — never by inferring a form in code. Coverage is what turns
"the engine agrees with the spec" into a statement about how much of the language
that covers, and it is the number that says where the spec should grow next.

### Distance — descriptive only

For each generated form, how far is it from its source-language cognates? This
answers "how much has Ruthenian moved from Russian here", which is genuinely
interesting — the second palatalization, the yat reflex and the dual should all
show large distances, and a *small* distance in those places is a signal the
engine has quietly reverted to Russian.

It is never called accuracy, never gates a release, and never appears without the
word "distance" attached.

**Stress is scored twice.** Since Ruthenian stores stress, a form can be
segmentally right and prosodically wrong, and one number cannot say both. So
every figure is reported in two variants:

- **segmental** — letters only, diacritics ignored. The headline.
- **strict** — the full string including stress placement.

Reporting both is the `english` pattern of publishing slot accuracy and
bare-lemma accuracy side by side: neither number is allowed to hide inside the
other. A large gap between them is itself the finding — it means the endings are
right and the accent patterns are not.

Reported per part of speech, and separately under `Variant::standard()` and under
each enabled optional feature. The standard run is the conformance measurement;
an optional-feature run shows what that feature changes.

## 3. The conformance corpus is generated from the spec

**The corpus is derived from `../RUTHENIAN.md`, not hand-written.** Its paradigm
tables (§§3–7: `dom`, `konj`, `drug`, `okno`, `polje`, `zzena`, `zjemlja`,
`noczj`, `kostj`, the adjective, the pronouns, the numerals, the verb tables and
`byti`) are parsed into expected cells. Hand-transcribing them would create a
second copy of the language that drifts from the first, which is law 9 and the
mistake this project has already watched happen elsewhere.

A consequence worth being explicit about: **amending the spec changes the
corpus in the same commit**, and that is the intended workflow. The spec moves
first; the corpus follows mechanically; the engine is then measured against it.

### There is no held-out set, and here that is sound

Ruthenian has no learned parameters and no attested corpus to hold out. The rules
implement a document; the document is the answer key; there is nothing to
generalize *to*. The train/test distinction does not apply, and importing it
would be cargo-culting a machine-learning ritual into a setting with no
inference.

What replaces it is the coverage number (§2), and the honesty requirement is
correspondingly different from the old one. Every published conformance figure
carries:

> Conformance is measured against `docs/RUTHENIAN.md`, which specifies N of the
> language's paradigms. It states how faithfully the engine implements the
> specification, not how complete the specification is — see coverage.

That sentence is generated into the README from `summary.json` along with the
number, so the two cannot drift apart.

What is genuinely given up: the spec itself is written by a human who can be
wrong, and no amount of conformance detects a mis-specified paradigm. That is a
real limit and the mitigation is comparative, not statistical — `COMPARATIVE_GRAMMAR.md`
is where a proposed paradigm is checked against the family. The mitigations below
remain cheap and are required:

- **Per-rule impact counts** (§4) — a rule touching three entries is visible as
  overfitting in a way an aggregate percentage is not.
- **Paired diffs** on every change — fixed and broken counted separately, so a
  rule that trades five regressions for six fixes cannot hide inside a rising
  average.
- **The unexplained-mismatch list** — enumerated in full, never summarized.

## 4. Public API sketch

```rust
pub fn run(spec_corpus: &Path, variant: &Variant) -> Result<Summary, EvalError>;

pub struct Summary {
    pub facade_fingerprint: String,   // which build produced this
    pub spec_fingerprint: String,     // which revision of docs/RUTHENIAN.md
    pub dump_fingerprint: String,     // which lexicon data
    pub variant: VariantId,
    pub by_pos: BTreeMap<Pos, PosScore>,
    pub unexplained: Vec<Mismatch>,   // the actionable list
    pub by_rule: BTreeMap<RuleId, RuleImpact>,
    /// Of the cells the language has, how many does the spec state? A spec
    /// gap, not an engine defect — reported so conformance cannot be read as
    /// a claim about the whole language.
    pub coverage: Coverage,
    /// Descriptive only. Never combined with the above.
    pub distance: BTreeMap<SourceLang, DistanceStats>,
}

pub struct PosScore {
    pub segmental: Counts,     // letters only — the headline
    pub strict: Counts,        // including stress placement
}

pub struct Counts {
    pub slots: u64, pub hits: u64,
    pub variant_explained: u64, pub unexplained: u64,
    pub gap_agreement: u64, pub gap_disagreement: u64,
}

/// Diff two summaries. This, not an absolute floor, is the release gate.
pub fn compare(base: &Summary, head: &Summary) -> PairedDiff;   // fixed / broken, per slot
```

`compare` returning paired fixed/broken counts is the lesson from slovowiki's
39.5 % floor, which "permits approximately 411 regressions": an absolute floor
cannot distinguish a change that fixes 50 slots and breaks 40 from one that fixes
50 and breaks none. The gate is the pair.

## 5. Inputs and outputs

In: the conformance corpus generated from `../RUTHENIAN.md`; the lexicon artifact
from Phase 4 (for coverage and distance); the facade as a library dependency.

Out: **`eval/summary.json`** — the single canonical result. Every number in the
README, the changelog and any report is generated from this file. Nothing is
hand-copied; a figure in prose that disagrees with `summary.json` is a build
failure.

Also out: per-part-of-speech CSVs of misses, for working on, mirroring
`english`'s `data/intermediate/*_check.csv`.

## 6. Data owned

The corpus generator (spec tables → expected cells), the metric definitions, and
`summary.json`. It does **not** own the expected forms themselves — those belong
to `../RUTHENIAN.md`, and a form written here rather than derived from there is a
second copy of the language.

## 6a. Dependencies allowed

`ruthenian` (as a normal downstream consumer, through its public API only),
`ruthenian-lexicon`, plus `serde`/`serde_json` for the summary and a CSV writer
for the miss lists. Explicitly **not** `ruthenian-core` — the evaluator must
reach the rules the way a user does, through the facade, or it stops measuring
the shipped tool. Explicitly **not** `ruthenian-extract` — it consumes the
artifact, not the dump.

## 7. Invariants

1. The evaluator calls the public facade. It never reimplements lookup or
   inflection.
2. `Variant::standard()` measured against `../RUTHENIAN.md` is the baseline for
   correctness claims.
3. Every mismatch is classified; "other" is not a category.
4. Every published conformance number is accompanied by the generated coverage
   statement (§3). A number without it is a build failure.
5. Every `RuleId` enabled in the measured variant has an impact count in the
   summary.
6. `summary.json` records the facade fingerprint, the **spec fingerprint**, the
   dump fingerprint and the variant — a summary that cannot say what it measured
   is invalid.
7. Reported numbers are generated from `summary.json`, never transcribed.
8. Failures fail. No warning-only outcome, no silent input substitution: if the
   requested input is missing or unsuitable, the run errors and says why.
9. **Conformance, coverage and distance are reported separately and never
   combined.** No arithmetic produces a single "score" from them (I3).
10. The expected forms are derived from `../RUTHENIAN.md` at build time. No
    expected form is written by hand in this crate.

## 8. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `eval_uses_public_api` | Inv. 1 | Import a private/internal path from the facade; the check on the dependency surface fails | required | ms | crate |
| `mismatch_totally_classified` | Inv. 3 | Add a mismatch path that increments no class counter; hits + explained + unexplained ≠ slots | required | seconds | crate |
| `coverage_statement_published` | Inv. 4 | Emit a README conformance number without the generated coverage sentence beside it | required | ms | crate |
| `per_rule_impact_reported` | Inv. 5 | Add a `RuleId` with no impact count in the summary; a rule affecting 3 entries must be visible as such | required | seconds | crate |
| `summary_self_describing` | Inv. 6 | Emit a summary without the facade or spec fingerprint | required | ms | crate |
| `three_quantities_separate` | Inv. 9 | Add a field averaging conformance with coverage, or label a distance figure "accuracy"; the check on `Summary`'s shape and on generated prose fails | required | ms | crate |
| `expected_forms_are_derived` | Inv. 10 | Hand-write an expected cell in the crate instead of deriving it; the corpus regeneration diffs against it | required | seconds | crate |
| `readme_numbers_generated` | Inv. 7 | Edit a number in the README by hand; regeneration diffs against it | required | seconds | workspace |
| `no_silent_substitution` | Inv. 8 | Point the run at a missing/short input; it must error, never fall back to a default dataset while reporting the requested path | required | ms | crate |
| `no_debug_only_assertions` | Inv. 8 | Put an invariant in `debug_assert!` — it compiles out under the `--release` runs CI uses | required | ms | crate |
| `paired_diff_gate` | §4 | Land a change that fixes 3 slots and breaks 5; the paired diff blocks it where an absolute floor would not | required | seconds | crate |
| `both_stress_variants_reported` | §2 — segmental and strict both present | Emit a summary with one variant; a stress-only regression becomes invisible | required | ms | crate |

Eleven guards. Two of them — `no_silent_substitution` and
`no_debug_only_assertions` — exist purely because the ecosystem has already
shipped both bugs, in the evaluator, in the project this one is modelled on. Two
more — `three_quantities_separate` and `expected_forms_are_derived` — exist
because the pressure to produce one headline number, and to write the expected
forms down "just this once", is what turned the baseline into Russian the first
time.

## 9. Out of scope

- Changing any form. The evaluator is read-only with respect to output.
- **Amending the specification.** If the engine and `../RUTHENIAN.md` disagree,
  this crate reports it; it never edits the document to make the number better.
- Ranking, calibration, or probability estimation. There is nothing to calibrate:
  the facade is deterministic and rule-driven, and a confidence score with no
  decision hanging off it is decoration.
- Judging whether an optional feature is a good idea. It reports impact; the
  decision is a human one, recorded in `../RUTHENIAN.md` §13 and the generated
  `docs/VARIANTS.md`.
- Judging the *quality of a reconstruction* — whether the right cognate was
  chosen for a lemma. That is `ruthenian-extract`'s provenance data; this crate
  reports the confidence distribution without second-guessing it.
- Evaluating the orthography — that is a round-trip property, guarded in Phase 1.

## 10. Done criteria

- `summary.json` produced, with per-part-of-speech conformance under the standard
  variant, plus coverage and per-source distance.
- **Conformance at 100 % under `Variant::standard()`, or every shortfall
  enumerated.** Unlike an accuracy figure, this target is reachable, and a
  shortfall is a defect list rather than a percentage to improve on.
- **Unexplained mismatches enumerated in full**, not summarized. This list is the
  Phase 6 deliverable.
- Coverage reported against the language's full paradigm inventory, so the
  conformance figure cannot be read as a claim about the whole language.
- Every rule's impact counted, feeding the generated `docs/VARIANTS.md`.
- README numbers generated, with a statement of what the metric does *not*
  measure — in the same paragraph as the number, not in a footnote.
- The paired-diff gate wired so a release cannot silently trade regressions for
  improvements.
- Eleven guards present, each demonstrated to fail under its witness.

## 11. Closed decisions

- **The baseline is the specification.** See §1 for why attested Russian cannot
  serve, and what attested forms are used for instead.
- **The corpus is generated from `../RUTHENIAN.md`**, never hand-written, so it
  cannot drift from the language it checks.
- **No held-out set**, and here that is sound rather than a compromise: there are
  no learned parameters and no corpus to hold out. See §3.
- **Stress is scored both ways** — segmental headline, strict companion. See §2.
- **There is no publication threshold.** Publish whatever the first run produces,
  with the coverage statement and the unexplained-mismatch count printed beside
  it. A withheld number helps nobody, an honest low one sets the baseline the
  paired diff then protects, and a fixed floor would reintroduce exactly the
  coarse-gate trap this spec avoids elsewhere. The release gate is the *paired
  diff*, not an absolute figure.

## 12. Open questions

- **How coverage is denominated.** Conformance is well-defined; coverage needs a
  denominator — "the cells the language has" — and that is only knowable once the
  lexicon exists. Until Phase 4, coverage is reported as paradigms-stated rather
  than cells-covered, and the imprecision is stated with it.
