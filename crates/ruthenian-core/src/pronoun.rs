//! Pronouns (`RUTHENIAN.md` §5). All have dual forms.
//!
//! Pronouns get their own [`Slot`](crate::Slot) variant because the
//! post-prepositional `n-` series is not a case of a noun: modelling `u njego`
//! as a case would produce wrong forms everywhere else.

use crate::types::{Case, Gender, Number, Person, PronounStyle};
use crate::variant::{Prediction, Trace};

/// First and second person (§5.1). `vje` "we two" and `va` "you two" are the
/// OCS duals, restored.
fn first_second(person: Person, case: Case, number: Number) -> Option<&'static str> {
    use Case::*;
    use Number::*;
    use Person::*;
    Some(match (person, number, case) {
        (First, Singular, Nom) => "ja",
        (First, Singular, Acc | Abl) => "mjenja",
        (First, Singular, Gen) => "mjenjego",
        (First, Singular, Dat | Loc) => "mnje",
        (First, Singular, Ins) => "mnoj",
        (Second, Singular, Nom) => "ty",
        (Second, Singular, Acc | Abl) => "tjebja",
        (Second, Singular, Gen) => "tjebjego",
        (Second, Singular, Dat | Loc) => "tjebje",
        (Second, Singular, Ins) => "toboj",

        (First, Dual, Nom) => "vje",
        (First, Dual, Acc) => "na",
        (First, Dual, Gen | Loc) => "naju",
        (First, Dual, Dat | Ins | Abl) => "nama",
        (Second, Dual, Nom | Acc) => "va",
        (Second, Dual, Gen | Loc) => "vaju",
        (Second, Dual, Dat | Ins | Abl) => "vama",

        (First, Plural, Nom) => "my",
        (First, Plural, Acc | Gen | Loc) => "nas",
        (First, Plural, Dat | Abl) => "nam",
        (First, Plural, Ins) => "nami",
        (Second, Plural, Nom) => "vy",
        (Second, Plural, Acc | Gen | Loc) => "vas",
        (Second, Plural, Dat | Abl) => "vam",
        (Second, Plural, Ins) => "vami",

        // Pronouns have no vocative, and the third person is not this table's.
        (_, _, Voc) | (Third, _, _) => return None,
    })
}

/// Third person (§5.1). These are the forms the `n-` prefix applies to.
fn third(case: Case, number: Number, gender: Gender) -> Option<&'static str> {
    use Case::*;
    use Gender::*;
    use Number::*;
    Some(match (number, gender, case) {
        (Singular, Masculine, Nom) => "on",
        (Singular, Neuter, Nom) => "ono",
        (Singular, Feminine, Nom) => "ona",
        (Singular, Masculine | Neuter, Acc | Gen | Abl) => "jego",
        (Singular, Masculine | Neuter, Dat) => "jemu",
        (Singular, Masculine | Neuter, Ins) => "jim",
        (Singular, Masculine | Neuter, Loc) => "jem",
        (Singular, Feminine, Acc) => "ju",
        (Singular, Feminine, Gen | Abl) => "jeje",
        (Singular, Feminine, Dat | Loc) => "jej",
        (Singular, Feminine, Ins) => "jeju",

        (Dual, _, Nom) => "ona",
        (Dual, _, Acc) => "ja",
        (Dual, _, Gen | Loc) => "jeju",
        (Dual, _, Dat | Ins | Abl) => "jima",

        (Plural, _, Nom) => "oni",
        (Plural, _, Acc | Gen | Loc) => "jih",
        (Plural, _, Dat | Abl) => "jim",
        (Plural, _, Ins) => "jimi",

        (_, _, Voc) => return None,
    })
}

/// The reflexive (§5.2). It has **no nominative** — the cell does not exist,
/// which is a fact about the language and therefore a `None`, not an error.
///
/// ```
/// use ruthenian_core::{pronoun::reflexive, Case};
/// assert_eq!(reflexive(Case::Acc).unwrap().text, "sjebja");
/// assert_eq!(reflexive(Case::Gen).unwrap().text, "sjebjego");
/// assert_eq!(reflexive(Case::Ins).unwrap().text, "soboj");
/// assert!(reflexive(Case::Nom).is_none());
/// ```
pub fn reflexive(case: Case) -> Option<Prediction> {
    let text = match case {
        Case::Acc | Case::Abl => "sjebja",
        Case::Gen => "sjebjego",
        Case::Dat | Case::Loc => "sjebje",
        Case::Ins => "soboj",
        Case::Nom | Case::Voc => return None,
    };
    Some(Prediction::new(text, Trace::new("reflexive pronoun")))
}

/// Apply the post-prepositional `n-` prefix.
///
/// After a preposition every `j-`-initial third-person form takes `nj-`: `u
/// njego`, `s njim`, `k njej`, `o njih`. Pan-Slavic, from a reanalysed
/// preposition-final nasal, and it applies to prepositions **only** — never to a
/// bare oblique.
fn prefix_n(form: &str) -> Option<String> {
    form.strip_prefix('j').map(|rest| format!("nj{rest}"))
}

/// A personal pronoun.
///
/// ```
/// use ruthenian_core::{personal, Case, Gender, Number, Person, PronounStyle};
///
/// let p = |person, case, number, gender, style| {
///     personal(person, case, number, gender, style).map(|p| p.text)
/// };
/// use PronounStyle::{AfterPreposition, Full};
///
/// // the restored duals
/// assert_eq!(p(Person::First, Case::Nom, Number::Dual, Gender::Masculine, Full).as_deref(), Some("vje"));
/// assert_eq!(p(Person::Second, Case::Nom, Number::Dual, Gender::Masculine, Full).as_deref(), Some("va"));
///
/// // the n- series applies only to j-initial third-person forms
/// assert_eq!(p(Person::Third, Case::Gen, Number::Singular, Gender::Masculine, Full).as_deref(), Some("jego"));
/// assert_eq!(
///     p(Person::Third, Case::Gen, Number::Singular, Gender::Masculine, AfterPreposition).as_deref(),
///     Some("njego"),
/// );
/// // `on` does not begin with j-, so it is unchanged
/// assert_eq!(
///     p(Person::Third, Case::Nom, Number::Singular, Gender::Masculine, AfterPreposition).as_deref(),
///     Some("on"),
/// );
///
/// // pronouns have no vocative: the cell does not exist
/// assert!(p(Person::First, Case::Voc, Number::Singular, Gender::Masculine, Full).is_none());
/// ```
pub fn personal(
    person: Person,
    case: Case,
    number: Number,
    gender: Gender,
    style: PronounStyle,
) -> Option<Prediction> {
    let base = match person {
        Person::Third => third(case, number, gender)?,
        other => first_second(other, case, number)?,
    };
    let mut trace = Trace::new("personal pronoun");

    let text = match style {
        PronounStyle::Full => base.to_string(),
        PronounStyle::AfterPreposition => match prefix_n(base) {
            Some(prefixed) => {
                trace = trace.then("post-prepositional n- prefix");
                prefixed
            }
            // Not every cell has an n- variant, and that is not a gap: `on`,
            // `nas`, `mnje` never took the prefix because they never began with
            // `j-`. Returning the plain form is correct, not a fallback.
            None => base.to_string(),
        },
    };
    Some(Prediction::new(text, trace))
}
