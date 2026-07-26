# ruthenian-core

The productive morphology of **Ruthenian** as pure rules — eight cases, three
numbers, three declensions, six conjugation classes — with zero third-party
dependencies.

The language is defined by [`docs/RUTHENIAN.md`](../../docs/RUTHENIAN.md), which
is **normative**. Where this crate's output disagrees with that document, this
crate is wrong, and `spec_paradigms_match` fails — it reads the document's own
paradigm tables at test time rather than a transcription of them.

```rust
use ruthenian_core::{noun_forms, Animacy, Case, Declension, Gender, NounClass, Number};

let p = noun_forms("dom", NounClass::hard(Declension::II), Gender::Masculine, Animacy::Inanimate);

assert_eq!(p.get(Case::Gen, Number::Singular).unwrap().text, "domogo"); // OF the house
assert_eq!(p.get(Case::Abl, Number::Singular).unwrap().text, "doma");   // FROM the house
assert_eq!(p.get(Case::Nom, Number::Dual).unwrap().text, "doma");       // two houses
```

## This is not Russian morphology

Most of a Ruthenian paradigm is cells Russian does not have. Anything ported from
a Russian implementation will be silently wrong, and these are the places it
happens:

| | Ruthenian | Russian |
|---|---|---|
| cases | **8** — with the ablative and a productive vocative | 6 |
| numbers | **3** — the dual throughout, including verb agreement | 2 |
| declensions | **3**, hard/soft | 8 |
| verb classes | **6** | 16 |
| past tenses | **3** — aorist, imperfect, perfect | 1 |
| 2nd palatalization | **kept**: `drug` → loc. `druzi` | lost (0 %) |
| stress | fixed, one position per word | 10 mobile patterns |
| aspect | **derived** from surface shape | lexical, stored in pairs |

The ablative is the clearest case. Slavic did not lose it: it lost the *genitive*
and reassigned the ablative form to genitive function. Ruthenian returns both
endings to their inherited jobs, so `doma` means *from the house* — its original
meaning — and `domogo` means *of the house*.

## Layout

| Module | Owns |
|---|---|
| `types` | the grammatical vocabulary every crate imports |
| `case_endings` | the nominal ending tables, as **pure data** |
| `phono` | morphophonology: all three palatalizations, spelling rules, stress |
| `noun`, `adjective`, `verb`, `pronoun`, `numeral` | how endings attach |
| `paradigm` | whole-table accessors — `noun_forms`, `adj_forms`, `verb_forms` |
| `variant` | the `RuleId` registry and optional features |

`case_endings` holds no logic and `noun` holds no tables, so the two are
reviewable separately: one says *which* ending, the other says *how* it attaches.
This is `interslavic-core`'s split, adopted for the same reason.

## Three outcomes, kept distinct

| Result | Meaning |
|---|---|
| `Some(_)` / `Ok(Some(_))` | the form |
| `None` / `Ok(None)` | **the cell does not exist** — a perfective verb has no present tense |
| `Err(Unsupported)` | the rules do not cover this input; never a wrong form |

`None` never means "unimplemented". That is what `Err` is for, and keeping them
apart is what lets a caller trust a `None`.

## What is deliberately absent

- **No accent patterns.** Stress is fixed per word, so there is nothing to model
  beyond one stored position.
- **No stored aspect.** `aspect_of` derives it from surface shape, and its trace
  names which of §7.2's three rules fired.
- **No `reducible` flag, no `indeclinable` flag.** The fleeting vowel is derived;
  Ruthenian has no indeclinables at all.
- **No source-language classification.** Zaliznyak classes, accent letters and
  Russian stem classes stop at `ruthenian-extract`, which uses them to read a
  cognate and map it onto a Ruthenian class. The `no_source_language_types` guard
  keeps them out.
- **No lexical data.** If a fact is about one word rather than a class of words,
  it belongs in the lexicon and arrives as an argument.

## Tests

```
cargo test -p ruthenian-core
```

- `spec_paradigms.rs` — **144 cells across six paradigms**, parsed out of
  `docs/RUTHENIAN.md` and compared against the engine, plus §11's paradigm-size
  table. No expected form is transcribed, so the corpus cannot drift from the
  specification.
- `guards.rs` — 15 guards, each with a failure witness that has been applied,
  observed to fail, and reverted (`INVARIANTS.md` I5).
- 24 doc tests.
