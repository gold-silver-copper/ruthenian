# Spec: `ruthenian-core`

Phase 2. Depends on `ruthenian-orthography`.

## 1. Purpose

The productive morphology **of Ruthenian**, written as pure rules, plus the
grammatical vocabulary the rest of the workspace shares.

The language it implements is specified in [`../RUTHENIAN.md`](../RUTHENIAN.md),
which is normative: eight cases, three numbers, three declensions, six
conjugation classes, three past tenses. Where this crate's output disagrees with
that document, this crate is wrong.

It has two jobs at once, and the second is what makes the whole system small:

1. the **runtime fallback** for any lemma the generated tables do not list;
2. the extractor's **predictor** — at table-generation time, any attested form
   this crate already produces is dropped, so the tables hold exactly the
   exceptions.

That duality is a hard contract: **changing a rule here changes what counts as
irregular and requires regenerating the tables.** This is not a convention; it is
guarded (see `ruthenian.md` §9, `rule_table_sync`).

Wrong to put here: any lexical data. No lemma lists, no exception tables, no
dictionary. If a fact is about one word rather than about a class of words, it
belongs in `ruthenian-lexicon` and arrives as an argument.

## 2. The problem this crate exists to solve

A Ruthenian verb has **two stems** — the infinitive stem and the present stem —
and the second is not always derivable from the first. The forms you must be
told, because no rule recovers them, are the verb's **principal parts**.

Ruthenian deliberately removes most of the unpredictability the East Slavic
languages carry (`../RUTHENIAN.md` §1, "Removed"), and what survives is a short
list:

- **present-stem mutation** at the stem boundary, which is *regular per class*
  (§7.11): `pisatj` → `pisz-`, `voditj` → `vozz-`. Class 6 mutates by rule;
  class 4 mutates in the 1sg only. This is predictable and belongs to the rules,
  not to the lexicon.
- **epenthetic `-l-` after labials**: `ljubitj` → `ljublju`. Regular, and
  exceptionless in the measured source data.
- **which of the six classes a lemma belongs to** — not derivable from the
  infinitive alone, since `-atj` is class 1 or class 6.

Three things that make Russian hard are **absent by specification** and must not
be modelled here:

- **mobile stress.** Ruthenian stress is fixed per word (`../RUTHENIAN.md` §2.1).
  There are no accent paradigms, no `a`/`b`/`c`/`c″` letters, and no stress
  alternation across a paradigm. The lexicon stores one position per lemma.
- **lexical aspect pairing.** Aspect is derived from surface shape (§7.2), so no
  entry stores an aspect partner.
- **heteroclitics, eight declensions, soft adjective stems, indeclinables.** All
  removed; three declensions with an automatic hard/soft alternation replace
  them (§3.2).

So the shape of the rule engine is: **class + stem + slot → form**, with
principal parts supplied only where the class is not enough.

**Source-language classifications do not appear in this crate.** Zaliznyak's
sixteen classes, his stress letters and Russian stem classes are how a cognate is
read out of the dump and mapped onto one of Ruthenian's six classes; that mapping
lives in `ruthenian-extract` (`DIRECTION.md`, "Three structural decisions").
Ruthenian's six classes correspond to Zaliznyak's 1–6 (`../RUTHENIAN.md` §7.3),
which is what makes the mapping possible — and is also why the full apparatus,
which exists to encode irregularity Ruthenian has removed, must not leak past the
boundary.

## 3. Public API sketch

The grammatical vocabulary is **Ruthenian's**, taken directly from
`../RUTHENIAN.md`. This is the part of the crate most easily written as though
the target were Russian, and getting it wrong is unrecoverable: a six-case `Case`
cannot represent `domogo` or `doma`, and a two-value `Number` cannot represent
`domoma` at all.

```rust
// ---- grammatical vocabulary (owned here; every crate imports it from here) ----

/// Eight cases (§3.1). Ordered as the spec's paradigm tables order them, so a
/// generated table and a printed table cannot disagree about column order.
pub enum Case { Nom, Voc, Acc, Gen, Abl, Dat, Ins, Loc }

/// Three numbers (§3.1). The dual is obligatory with `dva` (§6.1).
pub enum Number { Singular, Dual, Plural }

pub enum Gender { Masculine, Feminine, Neuter }
pub enum Person { First, Second, Third }

/// Six tenses (§7.1). The three past tenses are independent of aspect, as in
/// OCS: a perfective verb has an imperfect, an imperfective has an aorist.
pub enum Tense { Present, Aorist, Imperfect, Perfect, Pluperfect, Future }

/// Two values. There is no `Biaspectual`: §7.2 abolishes it, along with
/// suppletive pairs. Aspect is DERIVED from surface shape, never stored — see
/// `aspect_of` below.
pub enum Aspect { Imperfective, Perfective }

pub enum Animacy { Animate, Inanimate }

/// The three declensions (§3.2), each with a hard and a soft variant. The velar,
/// sibilant, `c` and vowel stem-classes are NOT declensions — they are the same
/// endings with automatic spelling adjustments (§3.8).
pub enum Declension { I, II, III }
pub enum StemHardness { Hard, Soft }

/// The six conjugation classes (§7.3).
pub enum VerbClass { One, Two, Three, Four, Five, Six }

/// Every addressable cell of every paradigm. Exhaustive by construction:
/// a new slot is a new variant, so no code can quietly ignore one.
pub enum Slot {
    Noun    { case: Case, number: Number },
    Adj     { case: Case, number: Number, gender: Gender, animacy: Animacy, form: AdjForm },
    Verb(VerbSlot),
    /// Own variant: the post-prepositional n- series (`u njego`) is not a case
    /// of a noun, and modelling it as one produces wrong forms.
    Pronoun { case: Case, number: Number, gender: Gender, style: PronounStyle },
    /// Own variant: numeral government (2–4 + gen sg, 5+ + gen pl, the masculine
    /// animate accusative going genitive) is a property of the numeral, not of
    /// the noun it counts.
    Numeral { case: Case, gender: Gender, animacy: Animacy },
}

pub enum VerbSlot {
    Infinitive,
    /// The supine (§7.10a), if §13's open question closes in its favour.
    Supine,
    /// Present, aorist, imperfect and future are all synthetic and
    /// person-marked. Dual agreement is a `Number`, not a special case.
    Finite { person: Person, number: Number, tense: Tense },
    /// The l-participle, which the perfect and pluperfect are built from.
    LParticiple { gender: Option<Gender>, number: Number },  // None gender = plural
    /// Second person only in the singular and dual; §7.10.
    Imperative { person: Person, number: Number },
    Participle { kind: ParticipleKind, voice: Voice, tense: Tense },
    Gerund { tense: Tense },
}

// ---- classes ----
/// Declension plus hardness is the whole classification. There is no accent
/// pattern (stress is fixed, §2.1) and no `reducible` flag: the fleeting vowel
/// is derived from the stem shape (§3.9), never stored — law 5.
pub struct NounClass { pub declension: Declension, pub hardness: StemHardness }

// ---- the rule engine ----
pub struct Rules;
impl Rules {
    /// The productive answer, or None if this class genuinely has no such form.
    pub fn noun(stem: &Ruthenian, class: NounClass, g: Gender, a: Animacy, slot: Slot)
        -> Option<Prediction>;
    pub fn adjective(stem: &Ruthenian, slot: Slot) -> Option<Prediction>;
    pub fn verb(parts: &PrincipalPartsRef<'_>, class: VerbClass, slot: VerbSlot)
        -> Option<Prediction>;

    /// Aspect from surface shape alone (§7.2): bare stem imperfective, any
    /// prefix perfective, prefix + `-yva-`/`-iva-` imperfective again. Returns a
    /// `Prediction` so the trace names which of the three rules fired — the
    /// caller can then show *why* a verb is perfective.
    ///
    /// This is a function, not a stored field, and that is the point: no entry
    /// anywhere in the workspace carries an aspect value or an aspect partner.
    pub fn aspect_of(infinitive: &Ruthenian) -> Prediction<Aspect>;
}

pub struct Prediction {
    pub text: Ruthenian,
    pub trace: Trace,          // which rules fired, in order — never empty
}

/// Stable identifier for one rule. Used by the trace, by the regularization
/// register, and by the evaluator to attribute a mismatch.
pub struct RuleId(&'static str);   // "iotation.labial-epenthesis", "gap.fill-1sg"

/// Which OPTIONAL features of the specification are enabled. This does not
/// switch between Ruthenian and any other language — that axis does not exist.
pub struct Variant { .. }
impl Variant {
    /// The language exactly as `../RUTHENIAN.md` specifies it. The default, and
    /// the conformance baseline.
    pub fn standard() -> Self;
    pub fn with(self, rule: RuleId) -> Self;
    pub fn without(self, rule: RuleId) -> Self;
}
```

`Prediction` carries a non-empty `Trace` rather than a bare string because law 12
says return structure: the evaluator must be able to attribute every mismatch to
the rule that caused it, and the CLI's `--show-deviations` reads the same trace.

Note what the verb entry point takes: `PrincipalPartsRef`, not a lemma. The rule
engine never guesses a present stem it cannot derive — if the class does not
determine it, the caller supplies it. This is the type-level expression of law 8.

## 4. Inputs and outputs

In: a stem, a class, a slot, a variant. Out: `Option<Prediction>`. No files, no
lookups, no state.

## 5. Data owned

- The grammatical vocabulary (§3) — the only definition in the workspace.
- The productive endings for all three declensions and all six conjugation
  classes, in all three numbers, including the dual.
- The morphophonology: **all three palatalizations** with their conditioning
  environments (§2.4), iotation, labial epenthesis, the fleeting vowel, and the
  `zz`/`sz`/`cz`/`szcz`/`c` spelling adjustments (§3.8).
- The `RuleId` registry and the optional-feature rules.

**The second palatalization is load-bearing here, not decorative.** Russian
levelled it to 0 %; Ruthenian keeps it (Ukrainian 99 %, OCS 66 %), and it is what
distinguishes the locative `druzi` from the vocative `druzzje` in the consonant
while `-i` against `-je` distinguishes them in the vowel. A morphophonology
module ported from a Russian implementation will not have it, and every velar
stem will be silently wrong in two cells.

**One morphophonology module, used by every part of speech.** A second copy of a
seam rule means it is in the wrong place — this is exactly the duplication that
produced root cause R3 in `interslavic-phrase`.

## 6. Dependencies allowed

`ruthenian-orthography` only. **Zero** third-party dependencies; a
`[dependencies]` entry beyond the workspace path fails the phase.

## 7. Optional features

**The regularizations are not switchable.** Ruthenian's departures from its
source languages — three declensions instead of eight, six verb classes instead
of sixteen, fixed stress, derived aspect, no indeclinables — are not options. They
are what the language *is* (`../RUTHENIAN.md` §1). There is no policy that turns
them off, because the thing on the other side of that switch would be Russian,
and this crate does not generate Russian.

What `Variant` switches is the set of questions `../RUTHENIAN.md` §13 still calls
**open**. Each is a feature the spec might adopt, each is coherent on its own, and
none may ship enabled while the spec still lists it as undecided.

| RuleId | Spec §13 item | What it adds |
|---|---|---|
| `abl.plural-distinct` | 2 | Revives a distinct ablative plural (PIE `*-ios`). No attested language distinguishes it; the standard variant keeps abl = dat. |
| `pron.clitic-series` | 3 | A full/clitic pronoun opposition, as OCS, Sanskrit and Interslavic have. Adds a `PronounStyle` value, not a case. |
| `voice.middle` | 4 | The middle voice, lost in all Slavic and done by `-sja`. The most radical available conservatism. |

> **Spec inconsistency, unresolved here.** `../RUTHENIAN.md` §13 item 7 lists the
> supine as "not yet specified", but §7.10a specifies it completely — infinitive
> `-tj` against supine `-t`, governing the genitive (`idu lovit zvjerjej`) — and
> §"Written in this revision" claims it as newly written. §13 item 7 appears to
> be the stale entry. This crate therefore treats the supine as **standard**, not
> optional, and `VerbSlot::Supine` is unconditional. If that reading is wrong,
> the fix belongs in the spec first.

Three properties, none negotiable:

1. `Variant::standard()` is the language as specified. It is the conformance
   baseline; if it drifts from `../RUTHENIAN.md`, every conformance number
   becomes meaningless.
2. Every form produced under a non-standard variant is distinguishable **through
   the API** — the trace names the `RuleId`. Documentation alone does not satisfy
   this.
3. A feature graduates from `Variant` to the standard language by being written
   into `../RUTHENIAN.md` §13's settled list, not by being switched on by
   default. The spec moves first; the code follows.

### Gaps are structural, and therefore derived

A gap in Ruthenian is a property of the grammar, not a fact looked up per lemma.
`verb::slot_exists` derives every one from `(aspect, transitivity, slot)` and
returns `Ok(None)`; nothing consults data to decide it. This is law 5, and it is
also I4: `None` is a claim about the language, never about the code.

What the specification makes structural:

- **A perfective verb has no present-tense meaning.** Its present endings carry
  future sense (§7.8), so `poczitaju` fills the future, and the present is not a
  separate cell to fill.
- **A perfective verb has no present participle and no present gerund**, for the
  same reason (§7.12).
- **An intransitive verb has no passive participle.**
- **No noun has a vocative plural** — `nom = voc` in the plural runs 90–100 %
  across the family, so §3.1 gives Ruthenian none.
- **The ablative is distinct only in the masculine and neuter singular** (§3.1).
  Feminine singular is syncretic with the genitive; dual and plural with the
  dative. Those are syncretisms, not gaps: the cell exists and returns a form.

The distinction matters because the two are easy to confuse and behave
differently. A **gap** returns `None`. A **syncretism** returns `Some`, with the
same string another cell returns, and the trace says which rule produced it.

**Lexical defectiveness is a source-language phenomenon and does not cross the
boundary.** Russian's `победить` has no accepted 1sg; that is a fact about
Russian, recorded where Russian is described (`docs/sources/RUSSIAN_GRAMMAR.md`),
and it does not make the corresponding Ruthenian verb defective. Ruthenian's
paradigms are specified, regular and complete. If a Ruthenian lemma is ever to
carry a genuine lexical gap, the spec must say so first; there is no rule here
for inferring one from a cognate.

*Where the measured Russian figures went.* Earlier revisions of this section
reasoned from a count over 2 941 Russian verbs — 13 922 perfective gap slots
against 2 509 imperfective, each present-tense person/number slot appearing
~1 519 times. That measurement was right, and its conclusion (the gaps are
structural, not lexical) is the one kept above. It is evidence about a source
language, so it now lives with the other source-language evidence rather than in
this crate's specification.

`docs/VARIANTS.md` is **generated** from the rule registry, including the count of
lexicon entries each optional feature touches. A hand-written register would drift
from the rules within one release; the ecosystem has a worked example of exactly
that failure.

## 8. Invariants

1. For every (class, slot) pair, `Rules` either returns a form or returns `None`
   with a documented reason. There is no third outcome and no panic.
2. `None` means "this class has no such form", never "unimplemented".
3. Every `Prediction` has a non-empty trace.
4. `Variant::standard()` output is a function of (stem, class, slot) alone — no
   hidden state, no ambient configuration.
5. Rules are total on their declared input domain: any `Ruthenian` stem is
   accepted; garbage in yields a form, not a panic.
6. Applying an optional-feature rule never changes a form the rule does not claim
   to affect (checkable: diff `standard()` against `standard().with(r)` and
   confirm every difference carries `r` in its trace).
7. Every output string is valid Ruthenian per `ruthenian-orthography`.
8. **`Variant::standard()` reproduces `../RUTHENIAN.md`.** Every form the spec
   states, for every paradigm the spec tabulates, is produced exactly. This is
   the invariant the whole crate exists to satisfy, and it is checkable directly
   against the document.

## 9. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `slot_exhaustive` | Inv. 1 — every class × slot resolves or declares a gap | Add a `Slot` variant without handling it (must fail to compile or fail the test) | required | <1 s | crate |
| `regular_rules_golden` | The predictor's output for a fixed sample is stable | Change any ending; the golden diff shows exactly what moved | required | ms | crate |
| `trace_non_empty` | Inv. 3 | Return a `Prediction` with `Trace::default()` | required | ms | crate |
| `variant_isolation` | Inv. 6 | Make `pron.clitic-series` also alter a noun cell; the diff shows an untraced change | required | seconds | crate |
| `standard_is_pure` | Inv. 4 | Read an environment variable inside a rule | required | ms | crate |
| `output_is_valid_ruthenian` | Inv. 7 | Emit a raw `ъ` or a stray uppercase mid-word | required | seconds | crate |
| `spec_paradigms_match` | **Inv. 8** — every cell `../RUTHENIAN.md` tabulates is reproduced | Change any ending in the crate; the failing row names the spec table and cell it contradicts | required | seconds | crate |
| `stress_placed` | Every form carries exactly one U+0301 | Emit a form with no stress mark, or with two | required | seconds | crate |
| `morphophonology_single_owner` | §5 — one seam module | Copy `palatalize` into `verb.rs`; the check greps for a second definition **and** the duplicate-behaviour test diverges | required | ms | crate |
| `no_lexical_data` | §1 — no word lists | Add a `const IRREGULARS: &[(&str, &str)]` | required | ms | crate |
| `no_source_language_types` | §2 — no Zaliznyak class, accent pattern or stem class in any public type | Add `pub struct ZaliznyakVerbClass`; the check fails on the public-API surface | required | ms | crate |
| `no_dependencies` | §6 | Add any third-party dependency | required | ms | workspace |

Eleven guards. Two are worth their cost specifically because of what this crate
is:

`spec_paradigms_match` is the important one. It reads the tables out of
`../RUTHENIAN.md` and asserts the engine reproduces them, which makes the
normative document executable rather than aspirational. It is the same device as
`interslavic-phrase`'s Steen conformance corpus, and it is why a disagreement
between code and spec cannot survive a test run.

`morphophonology_single_owner` is deliberately belt-and-braces: a grep is a
lexical policy (weak on its own, per the lessons), so it is paired with a
behavioural test that fails when two implementations disagree.

## 10. Out of scope

- Lexical exceptions, irregular stems, per-word data → `ruthenian-lexicon`, and
  the facade's tables.
- Reading the dump, or any knowledge that Wiktionary exists →
  `ruthenian-extract`.
- Script conversion → `ruthenian-orthography`.
- Choosing *which* lemma to inflect, or resolving homographs → the facade.
- Mapping a source-language class code onto a Ruthenian class →
  `ruthenian-extract`. This crate never sees a Zaliznyak index.
- Measuring conformance over the whole lexicon → `ruthenian-eval`. This crate
  checks itself against the spec's own tables (`spec_paradigms_match`), which is
  a guard, not a metric; it publishes no numbers.

## 11. Done criteria

- Nouns, adjectives, verbs, pronouns and numerals all resolve every slot for
  every class in all three numbers, or declare the gap.
- Every paradigm table in `../RUTHENIAN.md` §§3–7 is reproduced exactly, checked
  by `spec_paradigms_match`. A cell the spec does not state is reported as a spec
  gap, not filled by inference.
- The `RuleId` registry exists, with every optional feature off by default and
  each reachable through `Variant::standard().with(rule)`.
- Eleven guards present, each demonstrated to fail under its witness.
- Doc test on every public function, in the style of `interslavic-core`.
- Zero third-party dependencies; `#![forbid(unsafe_code)]`; no panic on any
  public path.
- Stated in the crate docs: the predictor/fallback duality and the regeneration
  requirement, in the imperative, where a future contributor will read it.

## 12. Open questions

- **Which optional features `Variant` offers** — governed by `../RUTHENIAN.md`
  §13, not by this crate. A feature ships enabled only once the spec settles it,
  and until then is reachable through `Variant::standard().with(rule)`. (The
  separate question of *which variant is the default* is closed: `standard`.)
- ~~Whether stress is modelled~~ — **closed: yes, and it is simple.** Ruthenian
  stress is **fixed per word** (`../RUTHENIAN.md` §2.1), so there is no accent
  paradigm to model and no `StressPattern` enum: an entry stores one position and
  every form in the paradigm keeps it. Loans take the position stressed in the
  source and keep it throughout (§12.3). Running text never marks stress; the
  orthography carries it as a combining acute only when asked. Every rule that
  produces a form is responsible for carrying that position through; a form
  emitted without it is a bug, not a formatting choice.

  This is a place where inheriting the Russian model would have cost real
  complexity for nothing. Zaliznyak's accent letters (`a`, `b`, `c`, `c″`) encode
  mobile stress across a paradigm — a system Ruthenian removes outright. The
  letters are still useful at extraction time, to find where the stress *sits* in
  a cognate, and are discarded at that boundary.
- ~~Pronoun and numeral classes~~ — **closed: own `Slot` variants.** See §3.
  Two things follow. `PronounStyle` becomes part of the slot, and a style the
  language does not have returns `None` meaning "this cell does not exist" — the
  `interslavic` convention, and law 8. (The full/clitic opposition itself is
  `pron.clitic-series`, an optional feature; under the standard variant there is
  one series.)

  Numeral government is exposed as **structure, not a string**: the caller asks
  what case and number a count imposes and gets those back, so nobody re-derives
  the rules locally. That is law 12, and the mistake `interslavic` shipped
  `quantified_parts` to fix.

  In Ruthenian the rule being returned is genuinely simple, and that is the dual
  paying for itself (`../RUTHENIAN.md` §6.1): `odin` agrees in the singular,
  `dva` governs the **dual**, `tri`/`czjetyrje` the nominative plural, `pjatj`
  and above the genitive plural. There is no 11–14 exception and no last-digit
  rule; a compound numeral is governed by its last word. Russian's genitive
  singular after 2–4 is petrified dual agreement, so restoring the dual does not
  simplify that rule — it removes it. An implementation carrying a `2..=4` special
  case has imported a Russian irregularity the language does not have.
