# Lessons from the ecosystem

Research pass over every prior project, 2026-07-25, to establish what Ruthenian
should copy and what it must not repeat. Each lesson cites where it was learned.

## What was reviewed

| Project | What it is | Highest-signal artifacts |
|---|---|---|
| `interslavic-rs` | 4-crate Interslavic inflector, published, downstream consumers | `CHANGELOG.md` (259 l), `INTEGRATION.md` (163 l), `IMPROVEMENT_PROMPT.md` (the PR #34 root-cause brief), `review/pr34-code-review.txt`, git log `66d03b8..187c621` |
| `english` | English inflector, ~1 MB data, 100%/99.5% measured | `crates/english/src/lib.rs` header, `english-core/src/lib.rs` header, `extractor/src/checks.rs` header, `xtask/src/main.rs` |
| `slovowiki` | Interslavic candidate generator + 6.9 GB static site, ~39 k lines Rust | `docs/ARCHITECTURE-ACCURACY-REVIEW.md` (919 l, dated 2026-07-24), `docs/GUARD-REGISTRY.md` (290 l), `ATTRIBUTION.md` |
| `mrzavec` | Roguelike translated to Interslavic; the first deep downstream consumer | `RUNTIME_INFLECTION_PROMPT.md`, `GLOSSARY.md` (517 l), `BUG_FIXES.md` |
| `ruthenian` (gold-silver-copper) | The reference Latin orthography | `src/translit_ru.rs`, `src/phonetics.rs`, `failed_roundtrips_ru.txt` |
| `interslavic-phrase` | Typed syntax trees → sentences | `phrase-improvement.md`, the PR #34 review |

## A. Architecture

**A1. One generation path. Everything else is a projection.**
Slovowiki's Finding 1: the benchmark and the website ran *materially different*
generation pipelines, so the published accuracy number did not describe the
published site. Its target architecture collapses to one `GenerationEngine`
feeding evaluation, site, and API alike. Ruthenian: one engine; the CLI, the
eval, and any future consumer are thin adapters over the same result type.

**A2. Layer rules and tables so the tables hold only what the rules can't predict.**
`english`'s extractor drops any Wiktionary attestation the rule engine already
produces, so the tables *are* the irregular set — that is how a full-coverage
inflector fits in ~1 MB. The coupling is explicit and enforced: "changing any
rule here changes what counts as irregular and REQUIRES regenerating the
tables," guarded by `rule_table_sync` and `regular_rules_golden`, with
`cargo xtask accuracy` as the authoritative check. Ruthenian: this is the whole
storage strategy. The Zaliznyak class plus productive rules predict most forms;
the lexicon stores the residue.

**A3. Stay typed until the last possible moment; stringify exactly once.**
The PR #34 review's root cause R1 was premature flattening to `Vec<String>` plus
string-search placement — which produced misplaced clitics, split NPs, and
comma tokens counted as words. The fix: `Token`/`Constituent` types where the
verb complex's identity is *known, not re-discovered*, and "the join is the only
stringification."

**A4. No droppable side channels — make losing data a type error.**
Root cause R2: an `Option` field on a render result that callers could silently
forget, so clitics vanished. The rule that replaced it: a clitic renders as a
typed element the linearizer *must* place.

**A5. One builder per concept; a second copy will diverge.**
Root cause R3: relative clauses were rendered by a diverging copy of the verb
path, so they skipped valence checks and dropped adverbs; imperative logic was
duplicated three times. Slovowiki's Finding 12 is the same shape (form-record
construction with multiple owners). Ruthenian: if two code paths can produce the
same form, one of them is wrong and nobody knows which.

**A6. Derive state; don't hand-maintain it.**
Root cause R4: a hand-maintained `order_marked` boolean drifted from reality and
its dead disjuncts *were* the bug. Replaced by computing markedness from the
actual linearizer output — "this cannot drift from the linearizer because it
reads the linearizer's output."

**A7. Separate pure build logic from filesystem writing.**
Slovowiki Finding 11: export, generation, reconciliation, API construction,
rendering, and artifact updates were coupled in one huge orchestration function.
Ruthenian: a pure build plan, then a thin writer.

**A8. Do not let configuration become an experiment matrix in production.**
Slovowiki Finding 8: `ConsensusConfig` is 27 booleans embedded in production
code, with dead experiment residue (`let _ = cons_branch_cov;`) still carrying
more comment than the live decision. Rejected experiments belong in a report,
not in control flow.

**A9. Size is a symptom.** Slovowiki: `eval.rs` 3609 lines, `check.rs` 2909,
`forms.rs` 2190, ~39 k total. The review's remedy is boundaries, not smaller
files for their own sake.

## B. Data and provenance

**B1. Provenance is part of the API, not a footnote.**
`interslavic`'s `noun_info` returns `Provenance::Dictionary` vs
`Provenance::Guessed` — "the guess is exactly what `noun()` inflects with, so you
can decide whether to trust it." mrzavec's `GLOSSARY.md` goes further with a
per-word trust legend (**O**fficial / **G**enerated / **C**oined /
**S**ubstituted) and probabilities on generated items. Ruthenian: every form
knows whether it is attested, rule-derived, or regularized, and by which rule.

**B2. `None` must mean "no such form exists", never "unimplemented".**
The pronoun tables return `None` only for genuinely unattested cells: "A `None`
means 'no such form exists' — render the full form instead, don't invent one."
The reflexive having no nominative is documented as by design, not a gap.

**B3. One canonical owner, with machine-readable twins generated from it.**
mrzavec's `game-lexicon.tsv` is regenerated from `src/lang.rs` by a test — "edit
`lang.rs`, never the TSV". Slovowiki's Finding 7 is the counter-example: README,
machine summary, and diagnostic reports carry *different* accuracy numbers
because they were maintained separately.

**B4. Say which claims are sourced and which are policy.**
`interslavic` documents count government as "steen's base rule is cited … the
compound-numeral rule, the Nom/Acc-only override, and the animate-accusative
genitive are documented policy — the sources are silent and the docs say so."

**B5. Get licensing right in the commit that first vendors data.**
Slovowiki's `ATTRIBUTION.md` separates code (MIT) from bundled data (CC BY-SA)
from generated content (CC BY-SA + GFDL, machine-generated and unverified).

## C. Measurement

**C1. Define the metric's unit deliberately, and state what it does not measure.**
`english` scores **per slot, not per attested form**, because Wiktionary attests
multiple valid variants per slot and a per-form metric could never reach 100%.
It then publishes three companion numbers — slot accuracy, variant gap, and
bare-lemma correctness — precisely because the headline metric is blind to a
standard form demoted to a `_n` key. Its README states plainly that the
percentages "do not measure precision, nor whether the natural bare-lemma call
returns the primary attested form."

**C2. A validation set is consumed the moment it influences a decision.**
Slovowiki Finding 2: the long-lived "holdout" was repeatedly inspected during
rule selection, so it is a validation set wearing a test set's name. The
recommended first PR literally includes "rename the current holdout to validation
in code and reports."

**C3. Generate reported numbers from one canonical result.**
Finding 7 again, and design principle: "Generate README metric values from the
canonical summary." No hand-copied metrics anywhere.

**C4. Every accuracy movement must be explainable — keep deterministic traces.**
Slovowiki design principle 9. Paired fixed/broken counts per release beat a
coarse floor: its 39.5 % exact floor "permits approximately 411 regressions."

**C5. Measure before and after every rule change.** `english`: "Run
`cargo xtask accuracy` before and after any rule/table change so the change
carries a number, not an anecdote." Misses are written to per-POS CSVs.

## D. Guards

**D1. A guard without a failure witness is decoration.**
Slovowiki's guard-review methodology: name the minimal mutation that must make
the guard fail. "A check that survives the mutation it claims to detect is
stale. A check that fails on unrelated intentional changes is too broad. A check
with no owner or stated invariant should be removed or redefined." Registry
fields: name, owner, invariant, scope, failure witness, mutation, cost, overlap,
required vs scheduled, replacement plan.

**D2. Guards that don't run are worse than none.** Finding 3 catalogues
`debug_assert!` compiled out under `--release` in CI, warning-only semantic
failures, a probe "explicitly described in CI as a reported metric that always
succeeds", and — the worst — **silent input substitution**: the evaluator
quietly swapped in the default dataset when a custom input looked too small,
while still reporting the requested path. Fail loudly or don't check.

**D3. Byte-level pins are the wrong tool for semantic invariants.** Finding 6:
whole-tree hashes and full fingerprint pins "freeze every legitimate change" and
exist to compensate for weak boundaries. Keep the differ, replace the hard pin
with a categorized semantic diff. But note the counterweight — `interslavic`'s
whole-dictionary paradigm fingerprint genuinely works, because its scope is one
well-defined artifact and every delta is enumerated in the changelog.

**D4. Distinguish telemetry from gates.** Slovowiki design principles 5 and 10;
its registry marks each step `required` or `diagnostic`.

**D5. Prefer types and module boundaries over grep policies.** Design principle 1
(a wall-clock grep is listed as a brittle lexical policy to replace).

## E. Downstream ergonomics

**E1. Return structure, not strings for the caller to parse.**
The clearest arc in the whole ecosystem. `perfect_parts` exists so nobody parses
`"(je) ukradla"`; `quantified_parts` returns the resolved case and number so a
consumer "never re-derives the government rules locally"; `conditional_parts`
killed a hand-copied 8-slot map in the phrase crate that could "silently desync
from core". Every one of these was added *after* a consumer got it wrong.

**E2. Wrap variants over one implementation so they cannot disagree.**
`quantified`/`quantified_with_info` became thin wrappers over the parts path —
"one implementation, the variants cannot disagree."

**E3. Output ordering is API once anyone tests against it.**
`interslavic`'s changelog opens with it: multi-byform cells are `" / "`-joined,
consumers bless first-variant outputs into expectations, "reordering variants is
a breaking change," fenced by `tests/variant_order.rs`.

**E4. Write the integration guide from real integration pain.**
`INTEGRATION.md` is subtitled "Everything the first deep integration learned the
hard way, in one place."

**E5. Let the consumer drive the API.** Releases 0.10.0–0.14.0 were each pulled
by mrzavec's adoption, and mrzavec's "zero pre-inflected forms" policy — no
inflected form may appear in any source literal — is what forced the library to
be complete enough to be usable at runtime. A CLI that must produce every form
from a lemma is the same forcing function for Ruthenian.

**E6. Determinism ≠ immutability, and the difference must be documented.**
`english`: sense keys are "DETERMINISTIC but NOT immutable" — a newer dump can
renumber them; "there is no lockfile … the committed generated tables are the
whole artifact." Say which stability you offer.

## F. Process

**F1. Root-cause or reject.** The PR #34 brief: "a change that makes one probe
sentence pass while leaving its root mechanism in place is a rejected change."
Its validation step demands a *structural* proof — grep showing the mechanisms no
longer exist.

**F2. Enumerate intentional output changes; never discover them.**
"CHANGELOG: enumerate every output change these fixes cause … intentional
changes are enumerated, never discovered."

**F3. Fix the claim when the code won't match it.** The same brief requires that
a header comment claiming "punctuation is the one string operation" either state
its exception or the code move to match — "the claim and the code may not
disagree." Elsewhere it flags that a PR's "full-node round-trip" claim was simply
false and must be corrected in the PR.

**F4. Keep the reject histogram.** A count of what was kept is not a result
without a count of what was dropped and why (slovowiki's coverage JSON records
`dropped_redirect_no_senses`, `dropped_multiword`, `dropped_non_content_pos`,
`dropped_no_real_gloss`).

## The five things Ruthenian must do differently

1. **One engine, one result type, from day one** — the CLI and the eval read the
   same `Result`, so the published number always describes the shipped tool (A1).
2. **Rules predict, tables hold the residue** — Zaliznyak class + productive
   morphology as the compression scheme, with regeneration enforced (A2).
3. **Provenance and policy on every form** — attested / rule-derived /
   regularized-by-rule-X, queryable, not just documented (B1, and the whole
   regularization idea depends on it).
4. **Guards with failure witnesses, in a registry, from the first crate** — not
   accumulated around symptoms later (D1, D2).
5. **Structure over strings at every boundary** — the mistake `interslavic` paid
   for four releases in a row (E1).
