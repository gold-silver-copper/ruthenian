# Phase 1: `ruthenian-orthography` — execution prompt

Build the first crate of the Ruthenian workspace: the bijective Cyrillic↔Latin
writing system. This is implementation, not design — the design is settled in
`docs/specs/ruthenian-orthography.md` and every decision it once left open is
now closed.

Work in `~/Desktop/code/ruthenian`, which currently contains only documents.

## Read first

- `docs/specs/ruthenian-orthography.md` — the authority for this crate. If this
  prompt and the spec disagree, the spec wins and you say so in your report.
- `DIRECTION.md` — the laws (§"The laws") and the crate map.
- `LESSONS.md` §D — why every guard here needs a failure witness.

## Start state and the reference

The reference implementation is one `git clone` away, and you will need it: the
baseline numbers below were produced by executing it, and you must reproduce them
before claiming to beat them.

```bash
git clone --depth 1 https://github.com/gold-silver-copper/ruthenian /tmp/ruth-ref   # commit 49d3af7
# /tmp/ruth-ref/biblija_ru.txt is the 41 462-line corpus used below
```

## Deliverable 1: the workspace scaffold

This phase creates the repository, so set the shape the other seven crates will
inherit:

```toml
# Cargo.toml
[workspace]
members = ["crates/ruthenian-orthography"]
resolver = "3"

[workspace.package]
edition = "2024"
rust-version = "1.85"          # pin it; do not float
license = "MIT OR Apache-2.0"
authors = ["gold-silver-copper"]
repository = "https://github.com/gold-silver-copper/ruthenian"
```

`git init`, a `.gitignore` for `/target`, and a `CHANGELOG.md`. Do not add crates
for later phases, not even empty ones.

## Deliverable 2: the crate

Zero non-dev dependencies. `#![forbid(unsafe_code)]`. No `unwrap`, `expect`, or
`panic!` on any path reachable from public input. Doc test on every public
function.

### The API

Implement §2 of the spec as written: `Cyrillic` and `Ruthenian` newtypes with
`parse` returning `AlphabetError { offset, found, kind }`; `to_latin`,
`to_cyrillic`, `to_latin_mixed` returning skipped spans; `tokenize` exposing the
reader; `Alphabet::contains` and `Alphabet::digraphs`.

The newtypes are the point, not decoration: the round-trip contract is claimed
only over the declared alphabet, so there must be **no entry point that silently
accepts arbitrary text**. A caller either parses and handles the error, or calls
`to_latin_mixed` and sees what was skipped.

### The algorithm

Three mechanisms, in this order of authority.

**1. The reader defines the language.** A greedy longest-match tokenizer over the
ordered digraph table — `szcz`, `sz`, `cz`, `zz`, `ja`, `je`, `jo`, `ju`, then
singles — is the single source of truth for how a Ruthenian string is read.
Everything else is defined in terms of it. Write this first, and write it so it
can be called in isolation, because the writer's correctness is defined by it.

**2. The writer inserts a separator exactly where re-reading would diverge.**
Emit the naive mapping; wherever running the reader over the output would not
reproduce the input, insert `'`. Worked cases, all of which must pass:

| Source | Naive | Emitted | Why |
|---|---|---|---|
| Иён | `ijon` | `ijon` | reads back as и + ё |
| Ийон | `ijon` | `ij'on` | without the separator, `jo` reads as ё |
| щи | `szczi` | `szczi` | `szcz` is щ |
| шчи | `szczi` | `sz'czi` | without it, ш + ч reads as щ |
| сзади | `szadi` | `s'zadi` | `sz` would read as ш |
| зж… | `zzz…` | `z'zz…` | з + ж |
| жз… | `zzz…` | `zz'z…` | ж + з — same naive string, different separator position |
| подъезд | — | `pod'jezd` | ъ *is* the separator, at a morpheme boundary |
| подезд | `podjezd` | `podjezd` | no boundary, so no separator — the pair stays distinct |

Do **not** implement this as "emit, re-read the whole string, patch, repeat" —
that is quadratic and hides the invariant. The longest digraph is four
characters, so a single left-to-right pass with a bounded window decides each
position locally. The exhaustive **triples** guard is what proves the local rule
is sufficient; if a triple fails, the window is too small, not the rule.

Two writer invariants worth their own assertions: the writer never emits two
adjacent separators unless the source has two adjacent ъ, and a separator is
never emitted where the reader would not have been misled (no gratuitous `'`).

**3. Case is a separate layer, applied per unit.** Fold the token, map it, then
restore case. There is a subtlety here that dissolves a tension the spec only
gestures at, so implement it deliberately:

Decode case **per emitted unit** — a unit whose first character is uppercase came
from an uppercase Cyrillic letter. Encode digraph case by the token's pattern:
ALL-CAPS token → ALL-CAPS digraph (`SZCZ`), otherwise Title (`Szcz`). These two
rules agree in every case, which is what makes universal round-tripping and
correct-looking output compatible:

- `Щука` → `Szczuka` → `Щука`
- `ЩУКА` → `SZCZUKA` → `ЩУКА` (the reference produces `SzczUKA`; that is the bug)
- `ЩуКа` → `SzczuKa` → `ЩуКа` (mixed case round-trips; no special case needed)
- `ПОДЪЕЗД` → `POD'JEZD` → `ПОДЪЕЗД` (`'` is caseless — `Ъ` is `'`, never `''`)

**4. The alphabet is declared, and violations are typed.** Anything outside it is
an `AlphabetError` with a byte offset and an `Unmapped` reason — `PreReform`
(ѣ ѳ і ѵ), `ForeignCyrillic` (ґ є ї ў), `LatinInCyrillic`, `Control`. Never a
silent passthrough. This closes the defect where `мѣсто` becomes `mѣsto`:
Cyrillic embedded in Latin output, round-tripping "successfully" while producing
mixed-script garbage.

**5. Stress is part of the alphabet.** Combining acute U+0301 is carried in both
directions and attaches to the same vowel (`писа́ть` ↔ `pisátj`).
`Alphabet::contains` must accept it. A stressed and an unstressed spelling are
**different strings** and both must round-trip; nothing in this crate may
normalize one into the other.

Note for later: extraction normalizes ё→е, so no lexicon entry will contain the
`jo` digraph. `jo` still stays in the reader and the table — this crate's
contract is totality over the declared Cyrillic alphabet, which includes ё,
regardless of what the lexicon happens to hold. Do not optimize it out.

## The baseline to beat

Executed against the reference on 2026-07-25. Reproduce these numbers first, then
beat them; report both columns.

| Input | Reference | Required |
|---|---|---|
| `Ийон` | → `Ijon` → `Иён` ✗ | round-trips |
| `Йод` | → `Jod` → `Ёд` ✗ | round-trips |
| `шчи` | → `szczi` → `щи` ✗ | round-trips |
| `"cat дом"` | → `"цат дом"` ✗ | strict: `AlphabetError`; mixed: `"cat дом"` unchanged |
| `ЩУКА` | → `SzczUKA` ✓ but wrong | → `SZCZUKA`, round-trips |
| `мѣсто` | → `mѣsto` ✓ but mixed-script | `AlphabetError { kind: PreReform }` |
| `biblija_ru.txt` | **3 failures** in 38 623 non-empty lines (12695, 13444, 31725) | **0 failures** |

Already-working behaviour that must not regress: `сзади` → `s'zadi`, `изжить` →
`iz'zzitj`, `СЗАДИ` → `S'ZADI`, `подъезд`/`подезд` staying distinct, and stress
marks surviving.

## Test corpus

`roundtrip_corpus` in the spec assumes the Phase 4 lexicon, which does not exist
yet. For this phase, substitute the reference corpus, and do it in a way that
still runs in CI:

- **Vendor a sample** — around 2 000 lines from `biblija_ru.txt`, and it **must
  include lines 12695, 13444 and 31725**, the three the reference fails. That
  makes the head-to-head provable per-PR without a 7.4 MB file in the repo.
  Record provenance in `tests/corpus/README.md`.
- **Keep the full run as a command**, documented in the crate README, with its
  result (lines tested, failures) reported in this phase's summary.

A test that silently skips when a file is missing is a guard that does not run.
If the full corpus is absent, the sample test still runs and the full one is
reported as not-run, loudly.

## Deliverable 3: `docs/ORTHOGRAPHY.md`

The normative spec: the mapping table, the reader, the separator rule, the case
rule, the declared alphabet, stress handling, and a **decision record** covering
D1–D7 from the crate spec plus the three closed decisions (stress stored and
rendered on request; mixed-script handled by two entry points; one apostrophe
with one rule). For each defect: what the reference did, the input that proves
it, and what we do instead.

## The guards

All eleven from spec §9, and the requirement is stronger than "write a test":
**for each guard, perform the mutation named as its failure witness, confirm the
guard fails, then revert.** A guard that survives its own witness is stale and
must be fixed or deleted. Record the outcome for all eleven in your report.

```text
roundtrip_exhaustive_singles   roundtrip_exhaustive_pairs   roundtrip_exhaustive_triples
roundtrip_corpus               reference_defect_witnesses   alphabet_totality
case_restoration               stress_preserved             stress_is_distinguishing
no_dependencies                property_roundtrip
```

`property_roundtrip` may use a dev-dependency; nothing else may.

## Gates

- `cargo test`, `cargo test --doc`, `cargo clippy --all-targets -- -D warnings`,
  `cargo fmt --check` all green.
- Zero non-dev dependencies, enforced by a test, not by inspection.
- 0 round-trip failures on the vendored sample **and** on the full corpus, with
  both counts stated.
- No `unwrap`/`expect`/`panic!` reachable from public input.

## House rules

- **Assert nothing you have not executed.** Three claims in the earlier analysis
  of this same reference implementation were derived from reading the code and
  turned out to be wrong when run. Every claim in your report needs a command
  behind it.
- **Root cause, not symptom.** If a round-trip case fails, fix the reader or the
  separator rule — never add a special case for the failing string. A pinned
  string that passes because it is pinned is worthless.
- One commit per deliverable. Do not push, publish, tag, or open a PR.

## Report

When done, state: the reference's numbers and yours side by side; the full-corpus
result (lines tested, failures); the eleven guards with the outcome of each
mutation test; any place the spec turned out to be wrong or underspecified, with
what you did instead; and anything Phase 2 needs to know that the spec does not
already say.
