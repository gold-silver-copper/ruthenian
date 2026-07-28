//! The grammatical categories, and nothing else.
//!
//! **Five types, and every one is a dimension of a paradigm** (`DIRECTION.md`,
//! "The grammar types"). Each is exhaustive and each maps to a numbered section
//! of `docs/RUTHENIAN.md`.
//!
//! A category the language does not have does not appear, and neither does a
//! category it *does* have if the API never indexes by it: `Mood`, `Voice`,
//! `Aspect`, `Degree`, `AdjectiveForm`, `ParticipleKind` and `PronounStyle` are
//! all real and none is a type here — they became functions, derivations, or
//! nothing. Nor does a *word* appear as a type.

/// The eight cases (§3.1).
///
/// The **ablative** is the restored PIE `*-ōd`, distinct only in the masculine
/// and neuter singular; elsewhere it is syncretic with the genitive (feminine
/// singular) or the dative (dual and plural).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Case {
    Nominative,
    Vocative,
    Accusative,
    Genitive,
    Ablative,
    Dative,
    Instrumental,
    Locative,
}

/// The three numbers (§3.1). The **dual** is restored, with three distinct
/// forms: nom=voc=acc, gen=loc, dat=ins=abl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    Singular,
    Dual,
    Plural,
}

/// The three genders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

/// Animacy (§3.7). An animate noun takes an oblique form in the accusative: the
/// **ablative** in the singular, the **genitive** in the plural.
///
/// A noun's own animacy is in its lemma — a capital first letter (§2.1) — so
/// this is a parameter only where a word agrees with a head noun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Animacy {
    Animate,
    Inanimate,
}

/// The three persons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Person {
    First,
    Second,
    Third,
}

impl Case {
    /// Every case, in the order §3.1 lists them.
    pub const ALL: [Case; 8] = [
        Case::Nominative,
        Case::Vocative,
        Case::Accusative,
        Case::Genitive,
        Case::Ablative,
        Case::Dative,
        Case::Instrumental,
        Case::Locative,
    ];
}

impl Number {
    /// Every number, in the order §3.1 lists them.
    pub const ALL: [Number; 3] = [Number::Singular, Number::Dual, Number::Plural];
}

impl Gender {
    /// Every gender.
    pub const ALL: [Gender; 3] = [Gender::Masculine, Gender::Feminine, Gender::Neuter];
}

impl Animacy {
    /// Both values.
    pub const ALL: [Animacy; 2] = [Animacy::Animate, Animacy::Inanimate];
}

impl Person {
    /// Every person.
    pub const ALL: [Person; 3] = [Person::First, Person::Second, Person::Third];
}
