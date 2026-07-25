# Spec: `ruthenian-core`

Phase 2. Depends on `ruthenian-orthography`.

## 1. Purpose

The productive morphology of Russian, written as pure rules over Ruthenian
strings, plus the grammatical vocabulary the rest of the workspace shares.

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

A Russian verb has **two stems** — the infinitive/past stem and the present
(non-past) stem — and the second is frequently not derivable from the first. The
forms you must be told, because no rule recovers them, are the verb's **principal
parts**. Four mechanisms produce the unpredictability:

- **consonant mutation (iotation)** at the present-stem boundary: `pisatj` →
  `piszu` (писа́ть → пишу́), `voditj` → `vozzu` (води́ть → вожу́);
- **epenthetic `-l-` after labials**: `ljubitj` → `ljublju` (люби́ть → люблю́);
- **stress mobility** across the paradigm: `piszú` / `píszeszj`;
- **suppletion** and defectiveness: `idti` → `szol`; `pobjeditj` has no accepted
  1sg at all.

Russian lexicography encodes all of this compactly as the **Zaliznyak class**: a
digit for the morphological class, a letter for the stress pattern, `°` and
suffixes for irregularity — `1a`, `4a+p`, `4c+p`, `6°b`. Nouns get the parallel
treatment (stem class plus accent pattern). The English Wiktionary dump carries
these codes directly, which is why the extractor can hand this crate a class and
expect a full paradigm back.

So the shape of the rule engine is: **class + stem + slot → form**, with
principal parts supplied only where the class is not enough.

## 3. Public API sketch

```rust
// ---- grammatical vocabulary (owned here; every crate imports it from here) ----
pub enum Case { Nom, Gen, Dat, Acc, Ins, Loc }
pub enum Number { Singular, Plural }
pub enum Gender { Masculine, Feminine, Neuter }
pub enum Person { First, Second, Third }
pub enum Tense { Present, Past, Future }
pub enum Aspect { Imperfective, Perfective, Biaspectual }
pub enum Animacy { Animate, Inanimate }

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
    Finite { person: Person, number: Number, tense: Tense },
    Past { gender: Option<Gender>, number: Number },   // None gender = plural
    Imperative { number: Number },
    Participle { kind: ParticipleKind, voice: Voice, tense: Tense },
    Gerund { tense: Tense },
}

// ---- classes ----
pub struct ZaliznyakVerbClass { pub index: u8, pub stress: StressPattern, pub irregular: bool, .. }
pub struct NounClass { pub stem: StemClass, pub accent: AccentPattern, pub reducible: bool }

// ---- the rule engine ----
pub struct Rules;
impl Rules {
    /// The productive answer, or None if this class genuinely has no such form.
    pub fn noun(stem: &Ruthenian, class: NounClass, g: Gender, a: Animacy, slot: Slot)
        -> Option<Prediction>;
    pub fn adjective(stem: &Ruthenian, slot: Slot) -> Option<Prediction>;
    pub fn verb(parts: &PrincipalPartsRef<'_>, class: ZaliznyakVerbClass, slot: VerbSlot)
        -> Option<Prediction>;
}

pub struct Prediction {
    pub text: Ruthenian,
    pub trace: Trace,          // which rules fired, in order — never empty
}

/// Stable identifier for one rule. Used by the trace, by the regularization
/// register, and by the evaluator to attribute a mismatch.
pub struct RuleId(&'static str);   // "iotation.labial-epenthesis", "gap.fill-1sg"

pub struct Policy { .. }
impl Policy {
    pub fn attested() -> Self;      // reproduces standard Russian; the eval baseline
    pub fn regularized() -> Self;   // every departure enabled
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

In: a stem, a class, a slot, a policy. Out: `Option<Prediction>`. No files, no
lookups, no state.

## 5. Data owned

- The grammatical vocabulary (§3) — the only definition in the workspace.
- The productive endings for every declension and conjugation class.
- The morphophonology: palatalization, iotation, labial epenthesis, fleeting
  vowels, the ж/ш/ч/щ/ц (`zz`/`sz`/`cz`/`szcz`/`c`) spelling rules.
- The `RuleId` registry and the regularization rules.

**One morphophonology module, used by every part of speech.** A second copy of a
seam rule means it is in the wrong place — this is exactly the duplication that
produced root cause R3 in `interslavic-phrase`.

## 6. Dependencies allowed

`ruthenian-orthography` only. **Zero** third-party dependencies; a
`[dependencies]` entry beyond the workspace path fails the phase.

## 7. Regularization

Every departure from standard Russian lives here, next to the rule it modifies,
identified by a `RuleId`, and switched by `Policy`. There is no separate
"standard Ruthenian" code path — see `DIRECTION.md` for why that would be a
mistake.

Candidate rules for v1, in descending order of value-for-risk:

| RuleId | What it does | Why it is cheap |
|---|---|---|
| `gap.fill-1sg` | Generates the missing 1sg for defective verbs (`pobjeditj`) | The dump marks defective slots `"-"`, so the affected set is enumerated exactly by Phase 4 |
| `iotation.uniform` | Applies the class's mutation to the exceptions that escape it | Mechanical; the trace shows every affected lemma |
| `stress.fixed-stem` | Removes mobile stress in favour of fixed stem stress | Only meaningful if stress is stored (Phase 1 decision) |
| `numeral.regular` | Regularizes Russian's most irregular subsystem | Small closed class; high visibility |
| `suppletion.level` | Levels `idti`/`szol` onto one stem | Highest semantic cost — recommend leaving off by default |

Three constraints, none negotiable:

1. `Policy::attested()` must reproduce Russian. It is the evaluator's baseline; if
   it drifts, every accuracy number becomes meaningless.
2. Every regularized form is distinguishable **through the API** — the trace names
   the `RuleId`. Documentation alone does not satisfy this.
3. `docs/REGULARIZATION.md` is **generated** from the rule registry, including the
   count of lexicon entries each rule touches. A hand-written register would
   drift from the rules within one release; the whole ecosystem has a worked
   example of exactly that failure.

## 8. Invariants

1. For every (class, slot) pair, `Rules` either returns a form or returns `None`
   with a documented reason. There is no third outcome and no panic.
2. `None` means "this class has no such form", never "unimplemented".
3. Every `Prediction` has a non-empty trace.
4. `Policy::attested()` output is a function of (stem, class, slot) alone — no
   hidden state, no ambient configuration.
5. Rules are total on their declared input domain: any `Ruthenian` stem is
   accepted; garbage in yields a form, not a panic.
6. Applying a policy rule never changes a form the rule does not claim to affect
   (checkable: diff `attested()` against `attested().with(r)` and confirm every
   difference carries `r` in its trace).
7. Every output string is valid Ruthenian per `ruthenian-orthography`.

## 9. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `slot_exhaustive` | Inv. 1 — every class × slot resolves or declares a gap | Add a `Slot` variant without handling it (must fail to compile or fail the test) | required | <1 s | crate |
| `regular_rules_golden` | The predictor's output for a fixed sample is stable | Change any ending; the golden diff shows exactly what moved | required | ms | crate |
| `trace_non_empty` | Inv. 3 | Return a `Prediction` with `Trace::default()` | required | ms | crate |
| `policy_isolation` | Inv. 6 | Make `gap.fill-1sg` also alter 3pl; the diff shows an untraced change | required | seconds | crate |
| `attested_is_pure` | Inv. 4 | Read an environment variable inside a rule | required | ms | crate |
| `output_is_valid_ruthenian` | Inv. 7 | Emit a raw `ъ` or a stray uppercase mid-word | required | seconds | crate |
| `stress_placed` | Every form whose class determines a stress position carries exactly one U+0301 | Emit a form with no stress mark, or with two | required | seconds | crate |
| `morphophonology_single_owner` | §5 — one seam module | Copy `iotate` into `verb.rs`; the check greps for a second definition **and** the duplicate-behaviour test diverges | required | ms | crate |
| `no_lexical_data` | §1 — no word lists | Add a `const IRREGULARS: &[(&str, &str)]` | required | ms | crate |
| `no_dependencies` | §6 | Add any third-party dependency | required | ms | workspace |

Nine guards. `morphophonology_single_owner` is deliberately belt-and-braces: a
grep is a lexical policy (weak on its own, per the lessons), so it is paired with
a behavioural test that fails when two implementations disagree.

## 10. Out of scope

- Lexical exceptions, irregular stems, per-word data → `ruthenian-lexicon`, and
  the facade's tables.
- Reading the dump, or any knowledge that Wiktionary exists →
  `ruthenian-extract`.
- Script conversion → `ruthenian-orthography`.
- Choosing *which* lemma to inflect, or resolving homographs → the facade.
- Measuring whether the rules are right → `ruthenian-eval`. This crate has
  goldens; it does not have accuracy numbers.

## 11. Done criteria

- Nouns, adjectives, verbs, pronouns and numerals all resolve every slot for
  every class, or declare the gap.
- The `RuleId` registry exists with at least `gap.fill-1sg` implemented and its
  affected-entry count reported (from Phase 4 data, so this closes after Phase 4;
  the mechanism ships in Phase 2).
- Nine guards present, each demonstrated to fail under its witness.
- Doc test on every public function, in the style of `interslavic-core`.
- Zero third-party dependencies; `#![forbid(unsafe_code)]`; no panic on any
  public path.
- Stated in the crate docs: the predictor/fallback duality and the regeneration
  requirement, in the imperative, where a future contributor will read it.

## 12. Open questions

- **Which rules `Policy::regularized()` enables** — deferred to Phase 6, when the
  evaluator can price each one; until then every rule is off in both presets and
  reachable only through `Policy::attested().with(rule)`. (The separate question
  of *which policy is the default* is closed: `attested`.)
- ~~Whether stress is modelled~~ — **closed: yes.** Ruthenian stores stress and
  renders it on request, so `StressPattern` is load-bearing: the accent letter of
  the Zaliznyak class (`a`, `b`, `c`, `c″`…) drives real stress placement in
  generated forms rather than sitting in the entry as inert metadata, and
  `stress.fixed-stem` is implementable. Every rule that produces a form is
  responsible for placing its stress; a form emitted without stress where the
  class determines one is a bug, not a formatting choice.
- ~~Pronoun and numeral classes~~ — **closed: own `Slot` variants.** See §3.
  Two things follow. `PronounStyle` (full / clitic-if-any / after-preposition)
  becomes part of the slot, and a style with no attested form returns `None`
  meaning "this cell does not exist" — the `interslavic` convention, and law 8.
  Numeral government is exposed as **structure, not a string**: the caller asks
  what case and number a count imposes and gets those back, so nobody
  re-derives the 2–4 / 5+ rules locally. That is law 12, and the mistake
  `interslavic` shipped `quantified_parts` to fix.
