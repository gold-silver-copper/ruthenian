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
classes, three synthetic tenses — and can enumerate all of them.

Everything is computed from rules. There is no dictionary here, no data files, no
lookup tables, no network, no I/O of any kind. A word the crate has never seen
inflects exactly as well as one it has.

## The API

Typed arguments, string in, string out. The types are the interface: you cannot
ask for a case that does not exist, and you cannot get a form back without having
said which cell you wanted.

```rust
// Nominals. Every one of these is total: the language has no gap here.
pub fn noun(word: &str, gender: Gender, animacy: Animacy,
            case: Case, number: Number) -> String;

pub fn adjective(word: &str, form: AdjectiveForm, degree: Degree,
                 case: Case, number: Number, gender: Gender,
                 animacy: Animacy) -> String;

pub fn numeral(value: u64, case: Case, gender: Gender, animacy: Animacy) -> String;

// Verbs. `NonPast` is present for an imperfective and future for a perfective —
// one slot, one form, meaning fixed by aspect (§7.8). Aspect is therefore NOT a
// parameter: it changes what a form means, never what it looks like.
pub fn verb(word: &str, class: VerbClass,
            person: Person, number: Number, tense: FiniteTense) -> String;

// Person × Number. §7.10 has synthetic forms for five of the nine; the other
// four return the present indicative, which is what the language's periphrastic
// third-person imperative is built from (`da idjet`).
pub fn imperative(word: &str, class: VerbClass,
                  person: Person, number: Number) -> String;

pub fn participle(word: &str, class: VerbClass, kind: ParticipleKind,
                  voice: Voice, case: Case, number: Number,
                  gender: Gender, animacy: Animacy) -> String;

// The parts the periphrastic tenses are built from.
pub fn l_participle(word: &str, gender: Gender, number: Number) -> String;
pub fn infinitive(word: &str) -> String;

// `byti` is suppletive (§7.9) and belongs to no class, so it gets its own
// function rather than a `VerbClass::Irregular` variant that every other call
// site would have to handle and could never hit.
pub fn byti(person: Person, number: Number, tense: FiniteTense) -> String;

// `budu` is a different root from `jes-` — suppletion, not a tense of one stem —
// and its only job is to build the imperfective future, so it is named for that.
pub fn future_auxiliary(person: Person, number: Number) -> String;

// Pronouns. Personal pronouns are total: they have no distinct vocative, so the
// vocative returns the nominative — the language's own convention, already used
// for the vocative plural (§3.1) and the long adjective (§4.2).
pub fn pronoun(p: Pronoun, case: Case, number: Number,
               gender: Gender, style: PronounStyle) -> String;

// The reflexive has no gender or number. §5.2 gives it no nominative either;
// asking for one returns `sjebja`, the form the pronoun is cited by.
pub fn reflexive(case: Case, style: PronounStyle) -> String;
```

**Every function is total.** No `Option`, no `Result`, no panic: any combination
of arguments the types permit returns a string. Where the language has no form
for a cell, the function returns a defined fallback rather than nothing — see
below.

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

### Totality, and what fills the empty cells

**The crate generates forms, not meanings.** A cell is "missing" only when the
morphology has nothing to produce — not when the result would be semantically
odd. On that test almost nothing is missing:

| Supposed gap | Real? | |
|---|---|---|
| perfective has no **present** | no | The morphology produces `poczitaju`; §7.8 says perfective present endings carry future sense. The form exists — only the label `Present` was wrong. Hence `NonPast`. |
| imperfective has no synthetic **future** | no | `budu czitatj` is two words. Composition, not inflection. |
| **perfect**, **pluperfect** | no | Participle + copula. Same. |
| intransitive has no **passive participle** | no | The suffix applies regardless; the oddness is semantic, not formal. |
| pronouns have no **vocative** | no | The nominative is used — the same convention §3.1 applies to the vocative plural. |
| imperative has no **3rd person** or **1sg** | partly | No *synthetic* form; the language uses particle + present indicative (`da idjet`, §7.10). |
| **reflexive has no nominative** | yes | §5.2: a reflexive cannot be a subject. |

The last two are filled rather than excluded:

| Call | Returns | Why |
|---|---|---|
| `imperative(w, c, Third, Singular)` | the present indicative, `idjet` | It is exactly the form §7.10's periphrastic imperative is built from; the caller prefixes `da`, `nehaj` or `pustj`. |
| `imperative(w, c, First, Singular)` | the present indicative, `czitaju` | Same rule, though the construction is rarer. |
| `reflexive(Nominative, ..)` | `sjebja` | The form the reflexive is cited by, standing in for a cell the language lacks. |

**The cost, stated once.** A caller can ask a question the language does not have
an answer to and get a plausible-looking string back. That is a deliberate trade:
these combinations do not arise in real use, and paying for them with `Option` at
every call site — or with three extra enums a caller must learn — is worse than
paying for them with a documented fallback. Each fallback is a *defined* value,
not a guess, and each is listed above.

### Why `String` and not something richer

An earlier design returned a `Prediction` carrying the form plus a trace of which
rules fired. That existed to serve an evaluator and a CLI, neither of which is in
this crate's scope. With no consumer, the trace was structure nobody read, so the
return type is the form itself.

This is a deliberate relaxation of "return structure, not strings", and the cost
is real: a caller cannot ask *why* a form came out as it did. If a consumer ever
needs that, it is added as a second function (`noun_traced`) beside the simple
one — never by complicating the simple one.

### The periphrastic tenses are out of scope

The language has six tenses (§7.1); three are synthetic and three are built from
a participle plus a copula. This crate returns **words**, so it provides the
parts and the caller composes:

```rust
// perfect: jesmj czital
format!("{} {}", byti(First, Singular, NonPast),
                 l_participle("czitatj", Masculine, Singular));

// pluperfect: bjeh czital (aorist aux) / bjah czital (imperfect aux) — §7.7
format!("{} {}", byti(First, Singular, Aorist),
                 l_participle("czitatj", Masculine, Singular));

// imperfective future: budu czitatj
format!("{} {}", future_auxiliary(First, Singular), infinitive("czitatj"));
```

`byti`'s `NonPast` is the present (`jesmj`), not a present/future blend: the
future uses a different root altogether (`bǫd-` against `jes-`), which is
suppletion rather than a tense of one stem. That is why `future_auxiliary` is its
own function and there is no fourth tense variant for one verb's sake.

Doing the composition here would mean doing agreement and word order, which is
syntax, and the return value would stop being a word.

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

// The three SYNTHETIC tenses. NonPast is present for an imperfective and future
// for a perfective (§7.8). The perfect, pluperfect and imperfective future are
// periphrastic and are composed by the caller.
pub enum FiniteTense { NonPast, Aorist, Imperfect }

// A grammatical category of the language, but NOT an inflection parameter:
// aspect decides what NonPast means, never what it looks like.
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

**Aspect is not on that list**, which is worth stating because it was on an
earlier version. The endings are identical for both aspects, so aspect never
changes a form — only what `NonPast` means. A caller reasoning about *meaning*
still needs it (and §7.2's closed perfective class still has to be stored
somewhere), but that somewhere is not this crate.

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
4. **Every function is total, and every fallback is declared.** No `Option`, no
   `Result`, no panic. Where the language has no form for a cell, the function
   returns a *named* substitute listed in "Totality" above — never an
   undocumented guess, and never nothing. Adding a fallback means adding a row to
   that table, not quietly picking something in the code.
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
- Nouns, adjectives, verbs, pronouns and numerals each resolve **every** slot for
  every class, in all three numbers. A guard asserts the public API contains no
  `Option` and no `Result`, and a second asserts that every fallback in
  "Totality" is exercised by a test — so a substitute cannot be added without
  being written down.
- `paradigm()` enumerates a complete table for each part of speech.
- Every public function carries a doc test showing a real form.
- Zero third-party dependencies; `#![forbid(unsafe_code)]`; no panic on any
  public path, for any input including empty and non-Ruthenian strings.
- Every guard demonstrated to fail under its stated witness.
