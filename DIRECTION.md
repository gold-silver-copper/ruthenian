# `ruthenian-core` — direction

> The language itself is specified in [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md),
> which is **normative**. This document is about the crate that implements it,
> and about nothing else.

## One crate, one job

**Give it a word and some grammar; it gives you the form.**

```rust
use ruthenian_core::*;

assert_eq!(noun("dom", Genitive, Singular),  "domogo");   // OF the house
assert_eq!(noun("dom", Ablative, Singular),  "doma");     // FROM the house
assert_eq!(noun("dom", Nominative, Dual),    "doma");     // two houses

// Everything a shape cannot predict is in the lemma itself:
assert_eq!(noun("Drug", Locative, Singular), "druzi");    // capital = animate
assert_eq!(noun("noczj'", Genitive, Singular), "noczi");  // ' = not the predicted gender
assert_eq!(verb("pisatj'", First, Singular), "piszu");           // ' = class 6

// Output is always lowercase; sentence capitalisation is the caller's business.
assert_eq!(noun("Drug", Nominative, Singular), "drug");
```

The goal is complete coverage: **every form of every word.** Given a citation
form and the grammatical facts a citation form cannot carry, the crate produces
any cell of any paradigm — eight cases, three numbers, three genders and six
verb classes — and can enumerate all of them.

Everything is computed from rules. There is no dictionary here, no data files, no
lookup tables, no network, no I/O of any kind. A word the crate has never seen
inflects exactly as well as one it has.

## The API

Typed arguments, string in, string out. The types are the interface: you cannot
ask for a case that does not exist, and you cannot get a form back without having
said which cell you wanted.

```rust
// ---- nouns -----------------------------------------------------------------
pub fn noun(word: &str, case: Case, number: Number) -> String;

// ---- adjectives: two paradigms, so two functions ---------------------------
// Long = definite, declining pronominally; short = indefinite, declining as a
// noun (§4). They are different tables, not two cells of one.
pub fn adjective(word: &str, case: Case, number: Number,
                 gender: Gender, animacy: Animacy) -> String;
pub fn short_adjective(word: &str, case: Case, number: Number,
                       gender: Gender, animacy: Animacy) -> String;

// ---- derivation: word in, word out ----------------------------------------
// Degree builds a new stem (§4.3), so it is a derivation rather than a cell.
// The result declines through the two functions above like any other adjective.
pub fn comparative(word: &str) -> String;    // "dobr" -> "dobrjejsz"
pub fn superlative(word: &str) -> String;    // "dobr" -> "najdobrjejsz"

// ---- verbs -----------------------------------------------------------------
pub fn verb(word: &str, person: Person, number: Number) -> String;
pub fn imperative(word: &str, person: Person, number: Number) -> String;
pub fn infinitive(word: &str) -> String;
pub fn l_participle(word: &str, gender: Gender, number: Number) -> String;

// `bytj` is suppletive (§7.9) and belongs to no class, so it gets its own
// function rather than an irregular-class escape hatch that every other call
// site would have to handle and could never hit. It has no tense parameter
// because the language has one synthetic tense (§7.1) — and no past form,
// since the copula is not exempt from that.
pub fn bytj(person: Person, number: Number) -> String;

// `budu` is a different root from `jes-` — suppletion, not a tense of one stem —
// and its only job is to build the imperfective future, so it is named for that.
pub fn future_auxiliary(person: Person, number: Number) -> String;

// ---- participles: verb in, adjective lemma out ------------------------------
// §7.12: "participles decline as adjectives and have both long and short forms".
// So each is a derivation, and the result goes through adjective() or
// short_adjective(). This is what removes ParticipleKind and Voice.
// §7.12's past passive `n` is single, so one stem serves both.
pub fn present_active_participle(word: &str) -> String;
pub fn past_active_participle(word: &str) -> String;
pub fn present_passive_participle(word: &str) -> String;
pub fn past_passive_participle(word: &str) -> String;   // "poczitatj" -> "poczitan"

// Gerunds are indeclinable, so these return a finished form rather than a stem.
pub fn present_gerund(word: &str) -> String;   // "czitatj" -> "czitaja"
pub fn past_gerund(word: &str) -> String;      // "czitatj" -> "czitav"

// ---- pronouns: full and clitic are two paradigms ---------------------------
// A pronoun is named by the features it agrees in, not by a variant of its own:
// person × number × gender covers §5.1 and its third-person table exactly. See
// "A pronoun has no name" below. Gender is inert outside the third singular.
pub fn pronoun(person: Person, number: Number, gender: Gender,
               case: Case) -> String;
pub fn clitic_pronoun(person: Person, number: Number, gender: Gender,
                      case: Case) -> String;

// The reflexive has no gender and no number. §5.2 gives it no nominative
// either; asking for one returns `sjebja`, the form the pronoun is cited by.
pub fn reflexive(case: Case) -> String;
pub fn clitic_reflexive(case: Case) -> String;

// ---- numerals --------------------------------------------------------------
pub fn numeral(value: u64, case: Case, gender: Gender, animacy: Animacy) -> String;
```

### One rule decides what is a function and what is a parameter

**An enum that selects a paradigm becomes a function. An enum that indexes
within one stays a parameter.**

Long and short adjectives are two declensions, so they are two functions; case
and number index within either, so they are arguments. Full and clitic pronouns
are two series, so they are two functions. Degree and the participles build new
*words*, so they are derivations that hand their result back to the declension
functions — which is what keeps the adjective API at two entry points instead of
the twenty-four that `form × degree × participle-kind × voice` would produce.

Applying it removed seven enums. `AdjectiveForm` and `PronounStyle` became
function pairs; `Degree`, `ParticipleKind` and `Voice` became derivations, and
`Voice` had no other use since Ruthenian's passive is participle + copula rather
than a synthetic form; `Mood` and `Aspect` were already unused, the imperative
being its own function, the conditional periphrastic, and aspect changing what a
form means rather than what it looks like.

**The rule has a third clause, and it removes an eighth enum.** An enum that
names a *word* is neither: it is a lexical identity, and this API already has
two mechanisms for that — `word: &str` and the function name. `Pronoun { Ja, Ty,
On, Ona, Ono, My, Vy, Oni, Vje, Va }` was the one type here that was not a
dimension of anything, and its own justification said so: *which pronoun, the
way `word: &str` says which noun.*

What survives is five types, each a genuine dimension of a paradigm: `Case`,
`Number`, `Gender`, `Animacy`, `Person`.

`FiniteTense` was a sixth until §7.1 dropped both synthetic pasts. With one
synthetic tense left it indexed nothing, so the same rule that removed `Pronoun`
removed it: an enum with one value is not a dimension.

### A pronoun has no name

It is exhausted by the features it agrees in, so `pronoun` takes those and the
enum is gone. The mapping is exact — every former variant is one cell of
person × number × gender, and every cell is a variant:

| | Singular | Dual | Plural |
|---|---|---|---|
| **1st** | `ja` | `vje` | `my` |
| **2nd** | `ty` | `va` | `vy` |
| **3rd** | `on` / `ono` / `ona` by gender | `ona` | `oni` |

The enum was not merely redundant. It let the identity and the parameters
contradict each other — `pronoun(Ja, .., Plural, ..)` asked for a first-person
*singular* pronoun in the plural, and something had to be returned — and it gave
§5.1's third-person **dual** three names, since that column has no gender and
`On`, `Ono` and `Ona` each plus `Dual` denoted it with nothing marking one
canonical. Both defects are unstateable in the new signature, which is why
neither needs a fallback in "Totality" below.

**Gender is inert outside the third-person singular**, and that is the language
rather than a wart: Ruthenian's first and second persons do not inflect for
gender, and §5.1's third-person dual and plural do not either. `pronoun(First,
Singular, Feminine, Nominative)` is `ja`. The argument is ignored, not guessed,
so law 5 is untouched — and neither `Gender::Unspecified` nor an `Option` is
added to say what the table already says.

This is the one place the crate names a closed class by feature rather than by
string, and it is worth being clear why that is not a precedent for `bytj` or
the reflexive: those are single words, so there is nothing to index. §5's other
series — `toj`, `sjej`, `kto`, `czto`, `izzje` and the §5.6 prefixed forms —
have no entry point at all yet, which is an open hole rather than a decision;
see "What this crate is not".

**Every function is total.** No `Option`, no `Result`, no panic: any combination
of arguments the types permit returns a string. Where the language has no form
for a cell, the function returns a defined fallback rather than nothing — see
below.

For repeated use, bind the lexical facts once and the per-call signature reduces
to the grammar alone:

```rust
let dom = Noun::new("dom");
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
| `imperative(w, Third, Singular)` | the present indicative, `idjet` | It is exactly the form §7.10's periphrastic imperative is built from; the caller prefixes `da`, `nehaj` or `pustj`. |
| `imperative(w, First, Singular)` | the present indicative, `czitaju` | Same rule, though the construction is rarer. |
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
format!("{} {}", bytj(First, Singular),
                 l_participle("czitatj", Masculine, Singular));

// pluperfect: jesmj byl czital — the copula's own participle between them
format!("{} {} {}", bytj(First, Singular),
                    l_participle("bytj", Masculine, Singular),
                    l_participle("czitatj", Masculine, Singular));

// imperfective future: budu czitatj
format!("{} {}", future_auxiliary(First, Singular), infinitive("czitatj"));
```

`bytj` returns the present (`jesmj`), not a present/future blend: the future uses
a different root altogether (`bǫd-` against `jes-`), which is suppletion rather
than a tense of one stem. That is why `future_auxiliary` is its own function.

### Why `bytj` is a function and not five special cases

It is the language's only suppletive verb (§7.9), so it has to be handled
somewhere. The alternative — threading it through the general path with checks at
each stage — is what `interslavic-rs` does, and it is worth knowing how that
turned out.

There, `bytj` has no entry point of its own. Its irregularity is spread across
**five** places in one file: the prefix splitter carries it in a `NON_REGULAR`
list; the stem deriver swaps `by` → `jes` with a string comparison; a hardcoded
six-slot table supplies the present; the future builder special-cases it to emit
the auxiliary with an empty lexical verb; and the passive-participle builder
returns `—` for it. Its own `is_irregular_stem` predicate lists `da | je | jě |
ja | vě` and **does not include `jes`** — so the most irregular verb in the
language bypasses the crate's irregular-verb mechanism entirely.

None of that is wrong, and it produces correct forms. But a reader asking "how
does `bytj` work" has to find all five sites, and nothing tells them there are
five. A dedicated function collects the same facts where they can be read at
once.

The cost is real and is accepted deliberately: **`bytj` is a second generation
path**, which law 2 otherwise forbids. It is tolerable only because the verb is a
closed, nine-cell paradigm that the specification tabulates in full, so the
function is checked against §7.9 by the same conformance corpus as everything
else. If it ever grows a rule rather than a table, that exemption stops holding.

### Where `budu` lives, and why it is a choice rather than a constraint

`interslavic-rs` gets `bytj`'s future for free: its future is periphrastic for
*every* verb, so `bųdų bytj` is emitted with the lexical verb elided and comes
out as `bųdų`. One table serves both the auxiliary and the verb's own future.

**That trick transfers to Ruthenian.** `bytj` is imperfective, and Ruthenian's
imperfective future *is* periphrastic — `budu` + infinitive (§7.8) — so `bytj`'s
future is the same rule with the infinitive elided. §7.8's synthetic *perfective*
future is irrelevant here, because `bytj` is not perfective; what it prevents is
a single universal future rule for regular verbs, which is a different claim.

So the `budu…` forms are one table, and the only question is what to call the
function that holds it. `future_auxiliary` rather than `bytj(.., Future)` because
there is no `Future` tense, and none is needed because a regular verb's
future is either **identical to its `NonPast`** (perfective) or **two words**
(imperfective) — neither needs a slot of its own. `bytj`'s future is the one
one-word future form with nowhere else to live.

That is a naming decision, not a structural one. Putting the table inside `bytj`
under a fourth tense value would work equally well and would cost one enum
variant that only one verb can use; putting it in its own function costs one
function name. Either is defensible, and `bǫd-` being a different root from
`jes-` is the tiebreaker: the two are suppletively unified, not one stem inflected
two ways, so naming them separately reflects what they are.

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
```

**Five types, and every one is a dimension of a paradigm.** Each is exhaustive
and each maps to a numbered section of the specification.

A category the language does not have does not appear: there is no
`Biaspectual`, because §7.2 abolishes it; no `AfterPreposition` pronoun style,
because §5.1 drops the `n-` prefix; no accent pattern, because stress is fixed
(§2.1).

And a category the language *does* have does not appear either, if the API never
indexes by it. `Mood`, `Voice`, `Aspect`, `Degree`, `AdjectiveForm`,
`ParticipleKind` and `PronounStyle` are all real (§7.1, §4, §5.1a) and none is a
type here — they became functions, derivations, or nothing. §7.1 remains the
place the language's categories are enumerated; this list is only what the
inflector needs to be told.

Nor does a **word** appear as a type. `Pronoun` did, and was removed for it: the
personal pronouns are a closed class, but so are the prepositions and the
conjunctions, and enumerating a closed class in the type system is a lexicon
inside a crate that declares it has none. A pronoun is reached the way every
other word is — by the features that select it.

## What must be supplied, and why

The argument lists above are not arbitrary. **Every argument beyond the word and
the grammar is something a Ruthenian citation form genuinely cannot tell you**,
and the list is short:

| Supplied | Why it cannot be derived |
|---|---|
| **gender, animacy** (adjectives, numerals) | They agree with a **head noun**, so these come from elsewhere in the sentence rather than from the word being inflected. A noun's own gender and animacy are in its lemma. |

**Nothing about a verb is on that list.** The conjugation class used to be, but
§7.3 now derives it from the citation form: every ending decides its own class,
and `-atj` — the one genuinely ambiguous ending — is disambiguated by the
word-final `'` that marks a class-6 lemma (§2.1). `pisatj'` carries its class in
its spelling, so the engine is told nothing.

**Aspect is not on that list either**, which is worth stating because it was on
an earlier version. It changes what a form means, never what it looks like. The endings are identical for both aspects, so aspect never
changes a form — only what `NonPast` means. A caller reasoning about *meaning*
still needs it (and §7.2's closed perfective class still has to be stored
somewhere), but that somewhere is not this crate.

A noun's own gender and animacy are **not** on that list, because the lemma
carries them: a capital first letter is animate, and the word-final `'` supplies
the one bit where the ending leaves gender open (§2.1, §3.2). `noun` therefore
takes a word, a case and a number, and nothing else.

Everything else **is** derived, and storing any of it would be a bug:

- **gender**, from the ending plus the mark: `-o`/`-je` is neuter, a non-`j`
  consonant masculine, and only `-j` and `-a` are ambiguous — each binary, never
  three-way (§3.2);
- **animacy**, from the lemma's first letter (§3.7);
- **the conjugation class**, from the citation form's ending, plus the word-final
  `'` where the ending alone is ambiguous (§7.3);
- **declension and hardness**, from the ending: a feminine in `-a` is declension
  I, a feminine in `-j` is III, masculines and neuters are II; a stem is soft
  exactly when the citation form ends in `j`, `ja` or `je` (§3.2);
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

### One thing it is not yet, and should be

**§5's non-personal pronouns have no entry point.** `toj` and `sjej` (§5.4),
`kto`, `czto` and `izzje` (§5.5) and the `ni-`/`nje-`/`-libo` series (§5.6) are
all specified, and `toj`'s paradigm is tabulated in full, but no function above
produces a cell of any of them. This is a gap rather than a decision, and it
predates the removal of the `Pronoun` enum, which never covered them either.

`adjective()` does not absorb them: it builds the long adjective, whose
masculine nominative singular is stem + `-yj`, so `toj` is not `adjective("t",
..)`. The resolution that follows the rules already stated here is to expose the
**pronominal declension itself** — one entry point that `toj`, `sjej`, `izzje`
and the long adjective all route through, since §4.2's whole claim is that
`dobryj`'s endings *are* `toj`'s. That makes one paradigm one function, and one
table serve both. It is not implemented, and until it is, the conformance corpus
will carry §5.4's rows with nothing to check them against.

## The laws

Short, and each falsifiable by a test.

1. **The specification decides; the code conforms.** Where `docs/RUTHENIAN.md`
   states a form, that form is correct by definition and a disagreeing engine is
   wrong. Where the spec is silent, the gap is reported as a **spec** gap and
   closed there — never patched with a guess in code.
2. **One generation path.** `paradigm()` calls `form()`. A convenience wrapper
   that computes a form its own way is two implementations that will disagree.
3. **Derive state; never store it.** No field duplicating something computable.
   Gender, animacy, class, declension, hardness, stem, gaps and palatalization
   are all derived from the lemma. A stored
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
`bytj` and the verb tables — are extracted **once** into a committed corpus, and
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
