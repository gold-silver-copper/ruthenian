# Spec: `ruthenian-eval`

Phase 6. Depends on `ruthenian`, `ruthenian-lexicon`.

## 1. Purpose

Measure the shipped facade against attested Russian, and produce the one
canonical summary that every reported number is generated from.

This crate exists to make one specific failure impossible: publishing an accuracy
figure that describes something other than the tool people use. It therefore
scores by calling `ruthenian::form` — the same entry point the CLI calls — and
never by reconstructing a parallel approximation of the pipeline.

It is also the crate that keeps the regularization honest. A departure from
standard Russian looks exactly like a bug unless the evaluator can tell them
apart, so the headline number is accompanied by a count of **unexplained**
mismatches, which is the number that must go to zero.

Wrong to put here: anything that changes output. The evaluator observes; it never
patches, never post-processes, never "fixes up" a form before comparing.

## 2. What gets measured, and how

**Per slot, not per form.** A slot counts as a hit when the facade's output is in
the attested set for that slot. Wiktionary attests several valid variants for one
slot, so a per-form metric could never reach 100 % regardless of which valid
variant the library picks. `english` established this and its README states the
consequence plainly; copy both the metric and the plain statement.

Every mismatch is then classified — this is the part that is specific to
Ruthenian:

| Class | Meaning | Target |
|---|---|---|
| **Hit** | Output ∈ attested set | maximize |
| **Explained** | Output ≠ attested, and the trace names a `RuleId` that claims this slot | expected; report per rule |
| **Unexplained** | Output ≠ attested, no rule accounts for it | **must go to zero** |
| **Gap agreement** | Facade returns `None`, source marks the slot `"-"` | correct behaviour, counted separately |
| **Gap disagreement** | One says gap, the other does not | a bug in either direction |

**Stress is scored twice.** Since Ruthenian stores stress, a form can be
segmentally right and prosodically wrong, and one number cannot say both. So
every figure is reported in two variants:

- **segmental** — letters only, diacritics ignored. The headline.
- **strict** — the full string including stress placement.

Reporting both is the `english` pattern of publishing slot accuracy and
bare-lemma accuracy side by side: neither number is allowed to hide inside the
other. A large gap between them is itself the finding — it means the endings are
right and the accent patterns are not.

Reported per part of speech, and separately under `Policy::attested()` and
`Policy::regularized()`. The attested run is the correctness measurement; the
regularized run measures how far Ruthenian has moved from Russian, which is a
*description*, not a score.

## 3. No split in v1 — and what that costs

**v1 scores against every attested form.** There is no sealed test set, and the
reason is that Ruthenian has no learned parameters: the rules are written by
hand, the tables are mechanically derived, and nothing is fit by an optimizer
that could memorize a training set.

The honesty requirement that replaces the split is therefore blunt and must
appear next to every published number:

> The accuracy figure is **coverage of known data**, not generalization
> performance. Every attested form was available while the rules were written.

That sentence is generated into the README from `summary.json` along with the
number, so the two cannot drift apart.

What is genuinely given up: rules are still fit to the data by a human, which is
the same leakage as training, only slower. A rule added because it fixes twelve
observed forms may not generalize, and this design cannot detect that. The
mitigations that remain are cheap and are required:

- **Per-rule impact counts** (§4) — a rule touching three entries is visible as
  overfitting in a way an aggregate percentage is not.
- **Paired diffs** on every change — fixed and broken counted separately, so a
  rule that trades five regressions for six fixes cannot hide inside a rising
  average.
- **The unexplained-mismatch list** — enumerated in full, never summarized.

If a sealed set is added later, it must be **grouped by lemma, not by form**
(otherwise a lemma's 1sg lands in train and its 2sg in test and the paradigm
leaks across the boundary), and it must be called what it is: the ecosystem has a
worked example of a "holdout" that was inspected during rule selection and
quietly became a validation set wearing a test set's name.

## 4. Public API sketch

```rust
pub fn run(attested: &Path, policy: &Policy) -> Result<Summary, EvalError>;

pub struct Summary {
    pub facade_fingerprint: String,   // which build produced this
    pub dump_fingerprint: String,     // which data
    pub policy: PolicyId,
    pub by_pos: BTreeMap<Pos, PosScore>,
    pub unexplained: Vec<Mismatch>,   // the actionable list
    pub by_rule: BTreeMap<RuleId, RuleImpact>,
}

pub struct PosScore {
    pub segmental: Counts,     // letters only — the headline
    pub strict: Counts,        // including stress placement
}

pub struct Counts {
    pub slots: u64, pub hits: u64,
    pub explained: u64, pub unexplained: u64,
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

In: `attested.tsv` from Phase 4; the facade as a library dependency.

Out: **`eval/summary.json`** — the single canonical result. Every number in the
README, the changelog and any report is generated from this file. Nothing is
hand-copied; a figure in prose that disagrees with `summary.json` is a build
failure.

Also out: per-part-of-speech CSVs of misses, for working on, mirroring
`english`'s `data/intermediate/*_check.csv`.

## 6. Data owned

The split definition (which lemmas are in which set), the metric definitions, and
`summary.json`.

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
2. `Policy::attested()` is the baseline for correctness claims.
3. Every mismatch is classified; "other" is not a category.
4. Every published number is accompanied by the generated coverage caveat (§3).
   A number without it is a build failure.
5. Every `RuleId` active under the measured policy has an impact count in the
   summary — the small-n overfitting signal that replaces a held-out set.
6. `summary.json` records the facade fingerprint, the dump fingerprint and the
   policy — a summary that cannot say what it measured is invalid.
7. Reported numbers are generated from `summary.json`, never transcribed.
8. Failures fail. No warning-only outcome, no silent input substitution: if the
   requested input is missing or unsuitable, the run errors and says why.

## 8. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `eval_uses_public_api` | Inv. 1 | Import a private/internal path from the facade; the check on the dependency surface fails | required | ms | crate |
| `mismatch_totally_classified` | Inv. 3 | Add a mismatch path that increments no class counter; hits + explained + unexplained ≠ slots | required | seconds | crate |
| `coverage_caveat_published` | Inv. 4 | Emit a README number without the "coverage of known data, not generalization" sentence generated beside it | required | ms | crate |
| `per_rule_impact_reported` | Inv. 5 | Add a `RuleId` with no impact count in the summary; a rule affecting 3 entries must be visible as such | required | seconds | crate |
| `summary_self_describing` | Inv. 6 | Emit a summary without the facade fingerprint | required | ms | crate |
| `readme_numbers_generated` | Inv. 7 | Edit a number in the README by hand; regeneration diffs against it | required | seconds | workspace |
| `no_silent_substitution` | Inv. 8 | Point the run at a missing/short input; it must error, never fall back to a default dataset while reporting the requested path | required | ms | crate |
| `no_debug_only_assertions` | Inv. 8 | Put an invariant in `debug_assert!` — it compiles out under the `--release` runs CI uses | required | ms | crate |
| `paired_diff_gate` | §4 | Land a change that fixes 3 slots and breaks 5; the paired diff blocks it where an absolute floor would not | required | seconds | crate |
| `both_stress_variants_reported` | §2 — segmental and strict both present | Emit a summary with one variant; a stress-only regression becomes invisible | required | ms | crate |

Nine guards. Two of them — `no_silent_substitution` and `no_debug_only_assertions`
— exist purely because the ecosystem has already shipped both bugs, in the
evaluator, in the project this one is modelled on.

## 9. Out of scope

- Changing any form. The evaluator is read-only with respect to output.
- Ranking, calibration, or probability estimation. There is nothing to calibrate:
  the facade is deterministic and rule-driven, and a confidence score with no
  decision hanging off it is decoration.
- Judging whether a *regularization* is a good idea. It reports impact; the
  decision is a human one recorded in `docs/REGULARIZATION.md`.
- Evaluating the orthography — that is a round-trip property, guarded in Phase 1.

## 10. Done criteria

- `summary.json` produced, with per-part-of-speech scores under both policies.
- **Unexplained mismatches enumerated in full**, not summarized. This list is the
  Phase 6 deliverable; the accuracy percentage is secondary to it.
- Every rule's impact counted, feeding the generated
  `docs/REGULARIZATION.md`.
- README numbers generated, with a statement of what the metric does *not*
  measure — in the same paragraph as the number, not in a footnote.
- The paired-diff gate wired so a release cannot silently trade regressions for
  improvements.
- Nine guards present, each demonstrated to fail under its witness.

## 11. Closed decisions

- **No sealed test set in v1.** Score against everything attested, and publish the
  coverage caveat with every number. See §3 for what is given up and what
  replaces it.
- **Stress is scored both ways** — segmental headline, strict companion. See §2.
- **There is no publication threshold.** Publish whatever the first run produces,
  with the coverage caveat and the unexplained-mismatch count printed beside it.
  A withheld number helps nobody, an honest low one sets the baseline the paired
  diff then protects, and a fixed floor would reintroduce exactly the
  coarse-gate trap this spec avoids elsewhere. The release gate is the *paired
  diff*, not an absolute figure.

## 12. Open questions

None. Every question this spec opened is closed above.
