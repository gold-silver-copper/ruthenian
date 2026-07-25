# Spec: `ruthenian` (the facade)

Phase 5. Depends on `ruthenian-core`, `ruthenian-lexicon`, `phf`.

## 1. Purpose

The public library. One generation path: give it an entry key and a slot, get a
`Form` back — text, origin, and the trace of how it was produced.

It is the join point of the two halves of the system. The generated tables hold
the attested exceptions; `ruthenian-core` holds the productive rules; this crate
consults them in that order and labels the result. Everything else in the
workspace is either upstream of it (producing its tables) or downstream of it
(the CLI, the evaluator).

Wrong to put here: new morphology, table-building logic, or any second way to
produce a form. If the CLI needs a form the facade cannot give it, the fix is in
the facade or in `core` — never in the CLI.

## 2. Public API sketch

```rust
/// The one generation path. Everything else in this crate is a convenience
/// wrapper over it, so the variants cannot disagree.
pub fn form(key: &EntryKey, slot: Slot, policy: &Policy) -> Option<Form>;

/// The whole paradigm, in a defined order.
pub fn paradigm(key: &EntryKey, policy: &Policy) -> Option<Paradigm>;

/// Metadata without inflecting anything.
pub fn entry(key: &EntryKey) -> Option<&'static Entry>;

/// Resolve a surface lemma to zero, one, or several composite keys. Returns all
/// candidates — the caller decides, and cannot accidentally get "the first one".
/// Accepts a bare lemma ("voda"), a full key ("voda.f.1a"), and either script;
/// stress marks in the input are optional and do not affect matching.
pub fn resolve(lemma: &str) -> Candidates;

/// Structured accessors, so no consumer parses a string to recover structure.
pub fn principal_parts(key: &EntryKey) -> Option<PrincipalPartsView>;
pub fn aspect_pair(key: &EntryKey) -> Option<(EntryKey, EntryKey)>;   // (ipf, pf)
```

Three deliberate shapes:

- **`resolve` returns `Candidates`, not `Option<Entry>`.** Homographs are real;
  an API that silently picks one teaches consumers to depend on which one.
- **`paradigm` returns a typed `Paradigm`, not a map of strings.** Law 12. A
  consumer wanting the 1sg present asks for the slot, it does not index a table
  by a stringly-typed key.
- **Convenience wrappers delegate to `form`.** `interslavic` learned this the
  explicit way — "one implementation, the variants cannot disagree."

For an unknown lemma, the facade still answers: the rules run, and the returned
`Form` carries `Origin::RuleDerived` with a provenance of `Inferred`. A guess is
always distinguishable from a lookup, at the type level, without reading docs.

## 3. Inputs and outputs

In: the generated tables (compiled in via `include!`), plus caller arguments.
Out: `Form`, `Paradigm`, `Entry` views. No I/O, no runtime data loading, no
environment.

## 4. Data owned

The generated PHF tables in `generated/` — committed, machine-written, never
hand-edited. They are the whole shipped artifact; there is no lockfile and no
separate override file.

## 5. Dependencies allowed

`ruthenian-core`, `ruthenian-lexicon`, `phf`. Nothing else. `crate-type =
["cdylib", "rlib"]`, matching the ecosystem's other published inflectors.

## 6. Invariants

1. **Two-tier lookup, always in the same order**: table first, rules second. No
   third source, no special-cased lemma anywhere in the crate.
2. **Tables contain no row the rules already predict.** Enforced here as well as
   in the extractor, because this is where a stale table would do damage.
3. Every returned `Form` carries an `Origin` that accurately describes how it was
   produced, and a non-empty trace.
4. `None` means "no such form exists" — a genuine paradigm gap under the active
   policy. It never means "not in the tables".
5. `policy` is the only thing that changes output for a fixed (key, slot). No
   ambient state, no feature flag that alters forms.
6. Byform ordering is stable and defined; the first is the recommended form.
7. Case-insensitive convenience is an *ergonomic* affordance with documented
   limits, not semantic proper-noun handling.
8. A whole-lexicon fingerprint pins every cell of every paradigm; any change to
   any output is visible as a fingerprint delta.

## 7. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `rule_table_sync` | Inv. 2 — no table row duplicates the rule engine | Change any productive ending in `core` without regenerating; a now-redundant row is detected | required, **dump-free** | seconds | crate |
| `single_path` | Inv. 1 | Add a lookup shortcut in `paradigm` that bypasses `form` | required | ms | crate |
| `origin_accurate` | Inv. 3 | Return a table hit labelled `RuleDerived`, or vice versa | required | seconds | crate |
| `none_means_gap` | Inv. 4 | Return `None` for a lemma merely absent from the tables | required | ms | crate |
| `policy_is_the_only_variable` | Inv. 5 | Read a `static mut` or env var in the lookup path | required | ms | crate |
| `byform_order` | Inv. 6 | Reorder a multi-form cell; the pinned first-variant expectations fail | required | ms | crate |
| `paradigm_fingerprint` | Inv. 8 | Change any cell anywhere; the pinned hash moves and the delta must be enumerated in the changelog | required | seconds | crate |
| `wrappers_delegate` | §2 | Reimplement `principal_parts` instead of delegating; a differential test finds the divergence | required | ms | crate |
| `unknown_is_labelled` | §2 | Return `Origin::Attested` for an out-of-vocabulary lemma | required | ms | crate |

Nine guards. `paradigm_fingerprint` is the one that needs care: whole-tree hashes
are an anti-pattern, but this one is scoped to a single well-defined artifact and
every delta is enumerated in the changelog. That is the difference between the
version that works and the version that freezes all legitimate change.

## 8. Out of scope

- Building tables → `ruthenian-extract`.
- New morphological rules → `ruthenian-core`.
- Measuring correctness → `ruthenian-eval`. The facade has goldens and a
  fingerprint; it does not have an accuracy number.
- Surface → lemma analysis. `resolve` matches citation forms only.
- Text-level anything: no tokenization, no sentences, no agreement across words.

## 9. Done criteria

- Every part of speech answerable through `form`, with `paradigm` covering every
  slot the class defines.
- Table size and rule-coverage ratio reported: what fraction of attested forms the
  rules predicted, and therefore how small the tables are. This is the phase's
  headline number and the proof that law 2 is working.
- Nine guards present, each demonstrated to fail under its witness.
- `INTEGRATION.md` started — the downstream guide, in the style of
  `interslavic-rs`'s: citation-form conventions, what `None` means, byform
  ordering as API, and the determinism-not-immutability property spelled out.
- Doc tests on every public function.

## 10. Closed decisions

- ~~Which policy is the default~~ — **closed**: `Policy::attested()` is the
  default and `regularized` is opt-in for v1, so default output is checkable
  against a real language. Still open, and owned by Phase 6: *which rules*
  `Policy::regularized()` enables, since no rule should be turned on before the
  evaluator has priced it.
- **`resolve` does not rank.** It returns every candidate in a deterministic
  order (by key), and the caller chooses. There is no evidence base for calling
  one homograph "primary", and manufacturing a score would conflate rank with
  probability — a distinction the ecosystem already had to write down after
  getting it wrong.
- **`Policy` is presets plus adjustment**: `attested()`, `regularized()`, and
  `.with(rule)` / `.without(rule)`. The common cases are one call, and the
  evaluator can isolate a single rule — which is exactly what Phase 6 needs in
  order to price rules one at a time. `PolicyId` in the eval summary must
  therefore identify an arbitrary combination, not just the two presets.
- **The facade embeds no sense text.** Senses ship as `senses.rdb`
  (`ruthenian-lexicon.md` §2a); this crate is inflection only, and stays in the
  size class of the ecosystem's comparable inflector.

## 11. Open questions

One, deferred by design: **which rules `Policy::regularized()` enables**. Owned
by Phase 6, because no rule should ship enabled before the evaluator has priced
it. Until then every rule is off in both presets and reachable only through
`Policy::attested().with(rule)`.
