//! The grammatical vocabulary of **Ruthenian**. Owned here; every other crate
//! imports it from here, which is why this module holds no logic.
//!
//! Every enum below is taken from [`docs/RUTHENIAN.md`], which is normative. The
//! categories are the language's own, not any source language's: a six-case
//! `Case` could not express `domogo` or `doma`, and a two-value `Number` could
//! not express `domoma` at all.
//!
//! [`docs/RUTHENIAN.md`]: https://github.com/gold-silver-copper/ruthenian/blob/main/docs/RUTHENIAN.md

use ruthenian_orthography::Ruthenian;

/// The eight cases (`RUTHENIAN.md` §3.1).
///
/// Declared in the order the specification's paradigm tables use, so a generated
/// table and a printed one cannot disagree about column order. That order is
/// API — see `DIRECTION.md`, "Ordering is API".
///
/// The **ablative** is the inherited PIE `*-ōd` returned to its original
/// function: `doma` is *from the house*, `domogo` is *of the house*. It is a
/// distinct form only in the masculine and neuter singular; elsewhere it is
/// syncretic (feminine singular with the genitive, dual and plural with the
/// dative), which is why adding a case costs one cell in two paradigms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Case {
    Nom,
    Voc,
    Acc,
    Gen,
    Abl,
    Dat,
    Ins,
    Loc,
}

impl Case {
    /// Every case, in specification order.
    pub const ALL: [Case; 8] = [
        Case::Nom,
        Case::Voc,
        Case::Acc,
        Case::Gen,
        Case::Abl,
        Case::Dat,
        Case::Ins,
        Case::Loc,
    ];
}

/// Three numbers (`RUTHENIAN.md` §3.1). The dual is used for exactly two of
/// something and is **obligatory** with the numeral `dva` (§6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Singular,
    Dual,
    Plural,
}

impl Number {
    pub const ALL: [Number; 3] = [Number::Singular, Number::Dual, Number::Plural];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

impl Gender {
    pub const ALL: [Gender; 3] = [Gender::Masculine, Gender::Feminine, Gender::Neuter];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Person {
    First,
    Second,
    Third,
}

/// Person and number as one value, because every finite ending is selected by
/// the pair and splitting them invites a mismatched combination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PersonNumber {
    S1,
    S2,
    S3,
    D1,
    D2,
    D3,
    P1,
    P2,
    P3,
}

impl PersonNumber {
    pub fn of(person: Person, number: Number) -> Self {
        use Number::*;
        use Person::*;
        match (person, number) {
            (First, Singular) => Self::S1,
            (Second, Singular) => Self::S2,
            (Third, Singular) => Self::S3,
            (First, Dual) => Self::D1,
            (Second, Dual) => Self::D2,
            (Third, Dual) => Self::D3,
            (First, Plural) => Self::P1,
            (Second, Plural) => Self::P2,
            (Third, Plural) => Self::P3,
        }
    }

    pub const ALL: [PersonNumber; 9] = [
        Self::S1,
        Self::S2,
        Self::S3,
        Self::D1,
        Self::D2,
        Self::D3,
        Self::P1,
        Self::P2,
        Self::P3,
    ];
}

/// Six tenses (`RUTHENIAN.md` §7.1).
///
/// Ruthenian has **three past tenses**, as OCS did, and they divide by function
/// rather than by aspect: a perfective verb has an imperfect (`poczitaszje`), an
/// imperfective has an aorist (`czita`). Aspect and past tense are independent
/// axes — modelling them as one is the Russian collapse this language undoes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tense {
    Present,
    /// A single completed event — the narrative past. Synthetic, `-h-`/`-s-`.
    Aorist,
    /// Ongoing or repeated past. Synthetic, `-jah-`.
    Imperfect,
    /// A past state still relevant now. `l`-participle + present copula.
    Perfect,
    /// Past before the past. `l`-participle + past copula.
    Pluperfect,
    Future,
}

impl Tense {
    pub const ALL: [Tense; 6] = [
        Tense::Present,
        Tense::Aorist,
        Tense::Imperfect,
        Tense::Perfect,
        Tense::Pluperfect,
        Tense::Future,
    ];

    /// Is this tense built analytically, from the l-participle plus a copula?
    pub fn is_periphrastic(self) -> bool {
        matches!(self, Tense::Perfect | Tense::Pluperfect)
    }
}

/// Two values. There is deliberately **no** `Biaspectual`: `RUTHENIAN.md` §7.2
/// abolishes it along with suppletive pairs.
///
/// Aspect is never stored. It is a function of the verb's surface shape, and
/// [`crate::verb::aspect_of`] is the one implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Aspect {
    Imperfective,
    Perfective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Animacy {
    Animate,
    Inanimate,
}

/// The long/short opposition (`RUTHENIAN.md` §4), which carries **definiteness**
/// — the only such contrast the language has, since there is no article.
///
/// Unlike Russian, the short form is not restricted to the predicate: it is the
/// indefinite adjective and declines fully, as a noun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AdjForm {
    /// Indefinite. Declines with the noun endings.
    Short,
    /// Definite. Declines pronominally.
    Long,
}

/// Degree of comparison (`RUTHENIAN.md` §4.3). Regular, with no suppletion;
/// both degrees exist in both [`AdjForm`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Degree {
    Positive,
    /// `-jejsz-`, triggering the first palatalization.
    Comparative,
    /// `naj-` + comparative, following OCS, Ukrainian, Belarusian and Polish
    /// against Russian's analytic `самый`.
    Superlative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ParticipleKind {
    Adjectival,
    Adverbial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Voice {
    Active,
    Passive,
}

/// How a personal pronoun is realized.
///
/// There is no `Clitic` variant. `RUTHENIAN.md` §13 item 3 records a full/clitic
/// opposition as an open question, and until the specification answers it the
/// language has one series — an unanswered question is not a feature to be
/// switched on. A style the language does not have returns `None`, meaning
/// "this cell does not exist", never "unimplemented".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PronounStyle {
    Full,
    AfterPreposition,
}

/// The three declensions (`RUTHENIAN.md` §3.2).
///
/// Russian's velar, sibilant, `c` and vowel stem-classes are **not** separate
/// declensions here — they are the same endings with the automatic spelling
/// adjustments of §3.8, which is why three replace eight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Declension {
    /// Feminine in `-a`: `zzena`, `zjemlja`.
    I,
    /// Masculine and neuter: `dom`, `konj`, `okno`, `polje`.
    II,
    /// Feminine ending in a consonant: `noczj`, `kostj`. The inherited PIE
    /// *i*-stem.
    III,
}

/// Each declension has a hard and a soft variant; the soft substitutes `je` for
/// `o`, `ju` for `u`, `i` for `y` — a single alternation, applied everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StemHardness {
    Hard,
    Soft,
}

/// A noun's class: which declension, and hard or soft.
///
/// That is the whole classification. There is **no** accent pattern — stress is
/// fixed per word (`RUTHENIAN.md` §2.1) — and **no** `reducible` flag, because
/// the fleeting vowel is derived from the stem's shape (§3.9). Both were
/// hand-maintained state duplicating something computable, which law 5 forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NounClass {
    pub declension: Declension,
    pub hardness: StemHardness,
}

impl NounClass {
    pub const fn new(declension: Declension, hardness: StemHardness) -> Self {
        Self {
            declension,
            hardness,
        }
    }
    pub const fn hard(declension: Declension) -> Self {
        Self::new(declension, StemHardness::Hard)
    }
    pub const fn soft(declension: Declension) -> Self {
        Self::new(declension, StemHardness::Soft)
    }
}

/// An adjective's class. Adjectives have no soft stems in Ruthenian — §1 lists
/// "soft adjective stems" among what was removed — so the only parameter is
/// which form is being built, which is a [`Slot`] property rather than a class
/// one. The struct is kept as the place a future class distinction would land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct AdjClass;

/// The six conjugation classes (`RUTHENIAN.md` §7.3).
///
/// These correspond to Zaliznyak's 1–6, with his 7–16 regularized onto them —
/// which is exactly what lets `ruthenian-extract` map a source cognate onto a
/// Ruthenian class. His classification does not appear here: it encodes
/// irregularity this language removed, and the `no_source_language_types` guard
/// keeps it out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerbClass {
    /// `-atj`, present stem + `j`: `czitatj` → `czitaj-`.
    One,
    /// `-ovatj`, `ov` → `uj`: `njegodovatj` → `njegoduj-`.
    Two,
    /// `-nutj`, theme drops: `dvinutj` → `dvin-`.
    Three,
    /// `-itj`, theme drops and the 1sg mutates: `govoritj` → `govor-`.
    Four,
    /// `-jetj`, theme drops: `vidjetj` → `vid-`.
    Five,
    /// `-atj`, theme drops and the stem mutates: `pisatj` → `pisz-`.
    Six,
}

impl VerbClass {
    pub const ALL: [VerbClass; 6] = [
        VerbClass::One,
        VerbClass::Two,
        VerbClass::Three,
        VerbClass::Four,
        VerbClass::Five,
        VerbClass::Six,
    ];

    /// Which set of present endings the class takes (§7.4).
    pub fn conjugation(self) -> Conjugation {
        match self {
            VerbClass::Four | VerbClass::Five => Conjugation::Second,
            _ => Conjugation::First,
        }
    }
}

/// The two present-tense ending sets (`RUTHENIAN.md` §7.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Conjugation {
    First,
    Second,
}

/// Every addressable cell of every paradigm.
///
/// Exhaustive by construction: a new slot is a new variant, so no code can
/// quietly ignore one. `Pronoun` and `Numeral` are their own variants because
/// the post-prepositional `n-` series is not a case of a noun, and numeral
/// government is a property of the numeral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Slot {
    Noun {
        case: Case,
        number: Number,
    },
    Adj {
        case: Case,
        number: Number,
        gender: Gender,
        animacy: Animacy,
        form: AdjForm,
        degree: Degree,
    },
    Verb(VerbSlot),
    Pronoun {
        case: Case,
        number: Number,
        gender: Gender,
        style: PronounStyle,
    },
    Numeral {
        case: Case,
        gender: Gender,
        animacy: Animacy,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerbSlot {
    Infinitive,
    /// The supine (`RUTHENIAN.md` §7.10a): `-t` against the infinitive's `-tj`,
    /// used for purpose after verbs of motion and governing the **genitive**
    /// (`idu lovit zvjerjej`).
    ///
    /// Unconditional, not optional. §7.10a specifies it completely while §13
    /// item 7 still lists it as unspecified; §7.10a is treated as authoritative
    /// and the discrepancy is recorded in `docs/specs/ruthenian-core.md` §7.
    Supine,
    /// Present, aorist, imperfect and future are all synthetic and
    /// person-marked. Dual agreement is a `Number`, not a special case.
    Finite {
        person: Person,
        number: Number,
        tense: Tense,
    },
    /// The `l`-participle the perfect and pluperfect are built from.
    /// `gender: None` is the non-singular form (`byli`), not a missing value.
    LParticiple {
        gender: Option<Gender>,
        number: Number,
    },
    /// Second person, plus the first-person hortative in the dual and plural
    /// (§7.10). A cell the paradigm does not have returns `None`.
    Imperative {
        person: Person,
        number: Number,
    },
    Participle {
        kind: ParticipleKind,
        voice: Voice,
        tense: Tense,
    },
}

/// What the rules need supplied because they cannot derive it.
///
/// This is the **borrowed input shape**. `ruthenian-lexicon` (phase 3) depends
/// on this crate, so it owns the stored shape and converts into this one;
/// defining an owned version here would invert the dependency.
#[derive(Debug, Clone, Copy, Default)]
pub struct PrincipalPartsRef<'a> {
    pub infinitive: Option<&'a Ruthenian>,
    /// The present stem, when the class does not determine it.
    pub present_stem: Option<&'a Ruthenian>,
}
