# Invariants

Properties that must hold across the whole project, each with the command that
checks it. These are narrower than `DIRECTION.md`'s laws: a law is a design
principle you apply with judgement, an invariant is a fact you can falsify.

Breaking one is not a trade-off to be weighed. It is a bug, and the fix is to
restore the invariant, not to document the exception.

---

## I1 — Every measurement scans the entire dump. No sampling.

**There is no such thing as a sampled figure in this project.** Any number that
describes the language — class distributions, mutation counts, gap counts, stem
classes, accent patterns, accuracy — is computed over **all 10 667 129 lines** of
`raw-wiktextract-data.jsonl`. Windowed sampling with `dd`, `head`, `skip=`, or
"the first N records" is forbidden in code, in tests, in tooling, and in
documents.

**Why this is an invariant and not a preference.** Sampling was used early in this
project and it was wrong three separate times, in ways that a bigger sample would
not have fixed and only a full scan revealed:

| Claim from a sample | Truth from the full scan |
|---|---|
| 117 distinct verb class codes | **226** — the sample missed 109, including 9 the parser could not parse |
| classes 1–6 cover 87.6 % of verbs | **90.7 %** |
| 670 class-1 labial stems take no epenthesis | **1 977**, and *not one* takes it — the rule is exceptionless, which a sample could only ever suggest |
| `ov → u` occurs 146 times | **675** |
| noun accent `a`+`b` ≈ 93 % | **94.7 %**, and the sample never saw the primed patterns (`accent-dʹ`, `accent-fʺ`) at all |

The third row is the important one. A sample can show a rule holds *often*; only
a full scan can show it holds *always*, and "always" is what a morphology engine
needs in order to key on the rule rather than hedge around it.

Even a full-file `grep` is not automatically a full scan: an early attempt at the
class-code census used `grep -oE` with a pattern that assumed JSON key order, and
found 183 of the 226 codes. **Parse the records; do not pattern-match them.**

**How to do it.** One full scan, cached, then measure from the cache:

```bash
# One pass over the whole dump. The filter is a superset — every record whose
# lang_code is ru contains this literal — so the cache is provably complete.
LC_ALL=C grep -F '"lang_code": "ru"' raw-wiktextract-data.jsonl > ru_all.jsonl
python3 tools/measure.py ru_all.jsonl
```

That is 441 629 Russian records out of 517 691 matching lines; the difference is
nested occurrences, which is exactly why the JSON must be parsed rather than
grepped. The cache is a projection of a full scan, not a sample of one.

**Check:** no `dd if=`, `skip=`, `count=`, `| head -`, or "sampled" appears in any
tracked file.

```bash
! grep -rn 'dd if=\|skip=[0-9]\|sampled' --include='*.rs' --include='*.md' \
    --include='*.py' . --exclude-dir=target --exclude-dir=.git
```

## I2 — Every published number traces to one command, and no two disagree.

A figure in a doc comment, a README, a changelog or a PR description must be
reproducible by a stated command, and the same quantity must not appear with two
values anywhere in the tree.

**Why.** The verb-coverage figure was published three times, three different ways
— `~76 %`, `73.2 %`, `87.6 %` — before the true value (`90.7 %`) was measured.
Two of those were wrong for the same reason: they counted only the most frequent
class codes, so class-1–6 variants outside the top of the list went uncounted.
This is the metric-drift failure `LESSONS.md` records from slovowiki, reproduced
here.

**Check:** `tools/measure.py` regenerates every figure; a diff against the
committed docs is empty.

## I3 — Accuracy is measured on a random sample; coverage is measured on a targeted one.

Two fixtures, two purposes, never averaged and never substituted for one another:

- `tests/paradigms/fixture.tsv` — **targeted**. One lemma per class, per
  mutation, plus hand-picked hard cases. A regression net. Its pass rate is *not*
  an accuracy figure.
- `tests/paradigms/random_*.tsv` — **random**, fixed seed, no hand-picking and no
  class targeting. This is the accuracy figure.

**Why.** Quoting the targeted fixture as accuracy understated the crate badly:
nouns read 58 % when the honest figure was 79 %, adjectives 80 % when it was 93 %.
A fixture built to concentrate hard cases measures the hard tail, by construction.

## I4 — `None` means "does not exist". Never "not implemented".

`Ok(None)` is a claim about the language: this cell is absent. `Err(Unsupported)`
is a claim about the code: the rules do not cover this. Conflating them makes
every `None` untrustworthy.

**Check:** `structural_gaps_are_derived` and `slot_exhaustive` in
`crates/ruthenian-core/tests/guards.rs`.

## I5 — Every guard has a failure witness, and the witness is verified.

A guard ships only after its named mutation has been applied, observed to fail
the guard, and reverted. A guard that survives its own witness is stale and is
deleted or fixed — never left in place.

**Why.** Phase 1 found two stale guards this way; phase 2 found one more, plus
two witnesses of mine that were themselves wrong. Guards are not self-evidently
correct and cannot be assumed so.

**Check:** the mutation table in each phase's report.

## I6 — The dependency-free crates stay dependency-free.

`ruthenian-orthography` has zero dependencies. `ruthenian-core` depends on
`ruthenian-orthography` and nothing else.

**Check:** `no_dependencies` in each crate's guard suite — a test, not a review
habit.

## I7 — Redistributed data carries its licence.

Any third-party data committed to this repository is recorded in
`ATTRIBUTION.md` with its source and licence, in the same change that adds it.
Wiktionary-derived content is CC BY-SA 4.0 + GFDL and requires attribution and
share-alike from anyone who redistributes it further.
