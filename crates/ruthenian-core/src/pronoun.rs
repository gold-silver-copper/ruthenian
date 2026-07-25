//! Personal pronouns, including the post-prepositional `n-` series.
//!
//! `PronounStyle::AfterPreposition` is not a case of a noun, which is why
//! pronouns have their own `Slot` variant: `u njego`, not `*u jego`.

use crate::policy::{Prediction, Trace};
use crate::types::{Case, Gender, Number, PronounStyle};

/// The third-person forms, which are the ones the `n-` prefix applies to.
fn third(case: Case, number: Number, gender: Gender) -> Option<&'static str> {
    use Case::*;
    use Gender::*;
    use Number::*;
    Some(match (number, gender, case) {
        (Singular, Masculine | Neuter, Nom) => "on",
        (Singular, Masculine | Neuter, Gen | Acc) => "jego",
        (Singular, Masculine | Neuter, Dat) => "jemu",
        (Singular, Masculine | Neuter, Ins) => "jim",
        (Singular, Masculine | Neuter, Loc) => "jem",
        (Singular, Feminine, Nom) => "ona",
        (Singular, Feminine, Gen) => "jeje",
        (Singular, Feminine, Dat) => "jej",
        (Singular, Feminine, Acc) => "jeje",
        (Singular, Feminine, Ins) => "jeju",
        (Singular, Feminine, Loc) => "njej",
        (Plural, _, Nom) => "oni",
        (Plural, _, Gen | Acc) => "jih",
        (Plural, _, Dat) => "jim",
        (Plural, _, Ins) => "jimi",
        (Plural, _, Loc) => "jih",
    })
}

/// A third-person personal pronoun.
///
/// ```
/// use ruthenian_core::pronoun::third_person;
/// use ruthenian_core::types::*;
/// let full = third_person(Case::Gen, Number::Singular, Gender::Masculine, PronounStyle::Full);
/// assert_eq!(full.unwrap().text, "jego");
/// let after = third_person(Case::Gen, Number::Singular, Gender::Masculine,
///                          PronounStyle::AfterPreposition);
/// assert_eq!(after.unwrap().text, "njego");
/// ```
pub fn third_person(
    case: Case,
    number: Number,
    gender: Gender,
    style: PronounStyle,
) -> Option<Prediction> {
    let base = third(case, number, gender)?;
    match style {
        PronounStyle::Full => Some(Prediction::new(base, Trace::new("third-person full form"))),
        PronounStyle::AfterPreposition => {
            if case == Case::Nom {
                // There is no post-prepositional nominative: the cell does not
                // exist, rather than being unimplemented.
                return None;
            }
            let text = if let Some(rest) = base.strip_prefix('j') {
                format!("nj{rest}")
            } else {
                base.to_string()
            };
            Some(Prediction::new(
                text,
                Trace::new("n- prefix after a preposition"),
            ))
        }
    }
}
