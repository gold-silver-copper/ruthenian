//! The grammatical vocabulary. Owned here; every other crate imports it from
//! here, which is why this module holds no logic.

use ruthenian_orthography::Ruthenian;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Case {
    Nom,
    Gen,
    Dat,
    Acc,
    Ins,
    Loc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
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
    P1,
    P2,
    P3,
}

impl PersonNumber {
    pub fn of(person: Person, number: Number) -> Self {
        match (person, number) {
            (Person::First, Number::Singular) => Self::S1,
            (Person::Second, Number::Singular) => Self::S2,
            (Person::Third, Number::Singular) => Self::S3,
            (Person::First, Number::Plural) => Self::P1,
            (Person::Second, Number::Plural) => Self::P2,
            (Person::Third, Number::Plural) => Self::P3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tense {
    Present,
    Past,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Aspect {
    Imperfective,
    Perfective,
    Biaspectual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Animacy {
    Animate,
    Inanimate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AdjForm {
    Long,
    Short,
    Comparative,
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

/// How a personal pronoun is realized. A style with no attested form returns
/// `None`, meaning "this cell does not exist" — never "unimplemented".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PronounStyle {
    Full,
    AfterPreposition,
}

/// Every addressable cell of every paradigm.
///
/// Exhaustive by construction: a new slot is a new variant, so no code can
/// quietly ignore one. `Pronoun` and `Numeral` are their own variants because
/// the post-prepositional `n-` series is not a case of a noun and numeral
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
    Finite {
        person: Person,
        number: Number,
        tense: Tense,
    },
    /// `gender: None` is the plural l-participle (`byli`), not a missing value.
    Past {
        gender: Option<Gender>,
        number: Number,
    },
    Imperative {
        number: Number,
    },
    Participle {
        kind: ParticipleKind,
        voice: Voice,
        tense: Tense,
    },
}

/// Stem class of a noun, in the naming the source data uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StemClass {
    Hard,
    Soft,
    Velar,
    Sibilant,
    Ts,
    I,
    Vowel,
}

/// Zaliznyak accent pattern. `a` is fixed stem stress, `b` fixed ending stress;
/// together they are ~93 % of attested nouns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccentPattern {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounClass {
    pub stem: StemClass,
    pub accent: AccentPattern,
    /// A fleeting vowel in the stem (`ru-noun+` argument `2` = `*`).
    pub reducible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjClass {
    pub stem: StemClass,
    pub accent: AccentPattern,
    /// Fleeting vowel in the short form, written `*` in the source notation.
    pub reducible: bool,
}

/// What the rules need supplied because they cannot derive it.
///
/// This is the **borrowed input shape**. `ruthenian-lexicon` (phase 3) depends on
/// this crate, so it owns the stored shape and converts into this one; defining
/// an owned version here would invert the dependency.
#[derive(Debug, Clone, Copy, Default)]
pub struct PrincipalPartsRef<'a> {
    pub infinitive: Option<&'a Ruthenian>,
    /// The present (non-past) stem, when the class does not determine it.
    pub present_stem: Option<&'a Ruthenian>,
    /// The past stem, when it is not the infinitive stem.
    pub past_stem: Option<&'a Ruthenian>,
}
