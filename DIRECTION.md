# `ruthenian-core` — direction

> The language itself is specified in [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md),
> which is **normative**. This document is about the crate that implements it,
> and about nothing else.

## One crate, one job

**Give it a word and some grammar; it gives you the form.**

```rust
use ruthenian_core::*;

assert_eq!(noun("dom", Masculine, Inanimate, Genitive, Singular), "domogo");
assert_eq!(noun("dom", Masculine, Inanimate, Ablative, Singular), "doma");
assert_eq!(noun("dom", Masculine, Inanimate, Nominative, Dual),   "doma");
assert_eq!(noun("drug", Masculine, Animate, Locative, Singular),  "druzi");
```

The goal is complete coverage: **every form of every word.** Given a citation
form and the grammatical facts a citation form cannot carry, the crate produces
any cell of any paradigm — eight cases, three numbers, three genders, six verb
classes, six tenses — and can enumerate all of them.

Everything is computed from rules. There is no dictionary here, no data files, no
lookup tables, no network, no I/O of any kind. A word the crate has never seen
inflects exactly as well as one it has.

## The API

Typed arguments, string in, string out. The types are the interface: you cannot
ask for a case that does not exist, and you cannot get a form back without having
said which cell you wanted.

```rust
// Nouns — total. Every noun has every cell (RUTHENIAN.md §3.9).
pub fn noun(word: &str, gender: Gender, animacy: Animacy,
            case: Case, number: Number) -> String;

// Adjectives — total. The long form's "missing" vocative is the nominative (§4.2).
pub fn adjective(word: &str, form: AdjectiveForm, degree: Degree,
                 case: Case, number: Number, gender: Gender,
                 animacy: Animacy) -> String;

// Verbs — NOT total. A perfective has no present tense (§7.8).
pub fn verb(word: &str, class: VerbClass, aspect: Aspect,
            person: Person, number: Number, tense: Tense) -> Option<String>;

// Pronouns — NOT total. The reflexive has no nominative (§5.2).
pub fn pronoun(pronoun: Pronoun, case: Case, number: Number,
               gender: Gender, style: PronounStyle) -> Option<String>;

pub fn numeral(value: u64, case: Case, gender: Gender, animacy: Animacy) -> String;
```

For repeated use, bind the lexical facts once and the per-call signature reduces
to the grammar alone:

```rust
let dom = Noun::new("dom", Masculine, Inanimate);
assert_eq!(dom.form(Genitive, Singular), "domogo");
assert_eq!(dom.form(Nominative, Dual),   "doma");

for (case, number, form) in dom.paradigm() {
    println!("{case:?} {number:?}: {form}");
}
```

`paradigm()` is how "every possible form" is reached in practice, and it is the
same code path as `form()` — not a second implementation that could disagree.

### Why `String` and not something richer

An earlier design returned a `Prediction` carrying the form plus a trace of which
rules fired. That existed to serve an evaluator and a CLI, neither of which is in
this crate's scope. With no consumer, the trace was structure nobody read, so the
return type is the form itself.

This is a deliberate relaxation of "return structure, not strings", and the cost
is real: a caller cannot ask *why* a form came out as it did. If a consumer ever
needs that, it is added as a second function (`noun_traced`) beside the simple
one — never by complicating the simple one.

## The grammar types

The whole vocabulary of the language, and the only thing other crates would ever
import from here.

```rust
pub enum Case { Nominative, Vocative, Accusative, Genitive,
                Ablative, Dative, Instrumental, Locative }   // 8 — §3.1

pub enum Number { Singular, Dual, Plural }                   // 3 — §3.1

pub enum Gender { Masculine, Feminine, Neuter }
pub enum Animacy { Animate, Inanimate }
pub enum Person { First, Second, Third }

pub enum Tense { Present, Aorist, Imperfect,
                 Perfect, Pluperfect, Future }               // 6 — §7.1

pub enum Aspect { Imperfective, Perfective }                 // no biaspectual — §7.2
pub enum Mood { Indicative, Imperative, Conditional }
pub enum Voice { Active, Passive }

pub enum AdjectiveForm { Short, Long }                       // definiteness — §4
pub enum Degree { Positive, Comparative, Superlative }       // §4.3

pub enum PronounStyle { Full, Clitic }                       // §5.1a
pub enum Declension { First, Second, Third }                 // §3.2
pub enum VerbClass { One, Two, Three, Four, Five, Six }      // §7.3
```

Each is exhaustive and each maps to a numbered section of the specification. A
category the language does not have does not appear: there is no `Biaspectual`,
because §7.2 abolishes it; no `AfterPreposition` pronoun style, because §5.1
drops the `n-` prefix; no accent pattern, because stress is fixed (§2.1).

## What must be supplied, and why

The argument lists above are not arbitrary. **Every argument beyond the word and
the grammar is something a Ruthenian citation form genuinely cannot tell you**,
and the list is short:

| Supplied | Why it cannot be derived |
|---|---|
| **gender** (nouns) | `konj` "horse" is masculine declension II; `noczj` "night" is feminine declension III. Both end in `j`. No rule separates them. |
| **animacy** (nouns) | The accusative depends on it — `vizzu dom` against `vizzu druga` (§3.7) — and nothing in the string marks it. |
| **class** (verbs) | `-atj` is class 1 or class 6 (§7.3): `czitatj` → `czitaj-`, `pisatj` → `pisz-`. |
| **aspect** (verbs) | A closed class is inherently perfective — `datj`, `kupitj`, `sjestj` — and no surface property identifies it (§7.2). `datj` is perfective while `pitj`, `mytj`, `bitj` are identically shaped and imperfective. |

Everything else **is** derived, and storing any of it would be a bug:

- **declension and hardness**, from gender plus the word's ending (§3.2);
- **the stem**, which is the citation form itself, since there is no fleeting
  vowel (§3.9) — `son`, `sona`, `sonu`;
- **the palatalizations**, from the ending's own trigger (§2.4, §3.8);
- **paradigm gaps**, from aspect and transitivity (§7.8);
- **numeral government**, from the numeral's last word (§6.1).

## What this crate is not

- **Not a dictionary.** No lemma lists, no exception tables, no word data of any
  kind. A fact about one word rather than a class of words does not belong here;
  it is an argument.
- **Not an analyser.** Generation only: grammar → form. Form → grammar is a
  different problem and is out of scope.
- **Not a corpus tool.** Nothing here reads Wiktionary, or any file. The two
  stored classes the language does have — inherently perfective verbs and
  determinate/indeterminate motion pairs (§7.2, §7.2a) — arrive as arguments.
- **Not a text engine.** No agreement across words, no sentences, no
  tokenization. One word at a time.

## The laws

Short, and each falsifiable by a test.

1. **The specification decides; the code conforms.** Where `docs/RUTHENIAN.md`
   states a form, that form is correct by definition and a disagreeing engine is
   wrong. Where the spec is silent, the gap is reported as a **spec** gap and
   closed there — never patched with a guess in code.
2. **One generation path.** `paradigm()` calls `form()`. A convenience wrapper
   that computes a form its own way is two implementations that will disagree.
3. **Derive state; never store it.** No field duplicating something computable.
   Declension, hardness, stem, gaps and palatalization are all derived. A stored
   flag drifts, and its dead branch becomes the bug.
4. **`None` means "no such form exists".** Never "not implemented". A perfective
   verb's present tense is `None` because the language has no such cell (§7.8);
   an unimplemented rule is a panic-free error or a compile failure, not a
   `None`.
5. **No droppable arguments.** If a caller can omit a lexical fact and still get
   a plausible-looking wrong answer, that fact belongs in the type signature.
   This is why `gender` is a parameter and not an `Option` with a guess behind it.
6. **Pure functions.** Same arguments, same output, always. No I/O, no ambient
   state, no configuration, no global mutable anything.
7. **Zero dependencies.** `ruthenian-orthography` for the alphabet, and nothing
   else. A `[dependencies]` entry beyond the workspace path fails review.
8. **Every guard has a verified failure witness.** A guard ships only after its
   named mutation has been applied, observed to fail it, and reverted. A guard
   that survives its own witness is stale and is deleted, not left in place
   looking reassuring.

## Correctness

The crate is measured against **the specification**, because there is no other
authority: Ruthenian is specified, not attested, and no corpus of it exists or
ever will.

`docs/RUTHENIAN.md`'s paradigm tables — `dom`, `konj`, `drug`, `okno`, `polje`,
`zzena`, `zjemlja`, `noczj`, `kostj`, the adjective, the pronouns, the numerals,
`byti` and the verb tables — are extracted **once** into a committed corpus, and
the conformance test asserts the engine against that file.

Extraction is deliberately a separate step from assertion. Parsing the
specification inside the test was tried and failed silently: a heading match
found `noczj` in §3.2's declension summary rather than §3.6's paradigm and
compared `dom`'s forms against it while reporting a clean run. A committed
artifact makes prose reformatting a reviewable diff instead of a test that
quietly checks less, and a currency check fails when the two drift.

## Done when

- Every cell of every paradigm the specification tabulates is reproduced exactly,
  checked against the committed corpus.
- Nouns, adjectives, verbs, pronouns and numerals each resolve every slot for
  every class, in all three numbers, or declare the gap.
- `paradigm()` enumerates a complete table for each part of speech.
- Every public function carries a doc test showing a real form.
- Zero third-party dependencies; `#![forbid(unsafe_code)]`; no panic on any
  public path, for any input including empty and non-Ruthenian strings.
- Every guard demonstrated to fail under its stated witness.
