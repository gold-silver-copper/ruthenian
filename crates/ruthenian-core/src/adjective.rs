//! Adjectives (`RUTHENIAN.md` §4): the OCS **long/short opposition**, which
//! carries definiteness — the only such contrast the language has, since there
//! is no article.
//!
//! | | Meaning | Declines |
//! |---|---|---|
//! | short | *a* good man — indefinite | as a **noun** |
//! | long | *the* good man — definite | **pronominally** |
//!
//! Unlike Russian, the short form is **not** restricted to the predicate: it is
//! the indefinite adjective and declines fully.
//!
//! The long form *is* the short form plus the anaphoric pronoun `j-` (OCS
//! `dobrъ` + `jь`), which is where the noun's `-ogo` genitive came from in the
//! first place (§3.1). The two are visibly one system, and the genitive and
//! ablative coincide between them because contraction merged those cells across
//! all of Slavic.

use crate::phono::{self, Palatal};
use crate::trace::{Prediction, Trace};
use crate::types::{AdjForm, Animacy, Case, Degree, Gender, Number};

/// Short (indefinite) endings — the noun's, exactly (§4.1).
fn short(case: Case, number: Number, gender: Gender) -> Option<Ending> {
    use Case::*;
    use Gender::*;
    use Number::*;
    Some(match (number, gender, case) {
        (Singular, Masculine, Nom | Acc) => plain(""),
        (Singular, Masculine, Voc) => first_pal("je"),
        (Singular, Neuter, Nom | Acc | Voc) => plain("o"),
        (Singular, Feminine, Nom) => plain("a"),
        (Singular, Feminine, Voc) => plain("o"),
        (Singular, Feminine, Acc) => plain("u"),
        (Singular, Masculine | Neuter, Gen) => plain("ogo"),
        (Singular, Feminine, Gen | Abl) => plain("y"),
        (Singular, Masculine | Neuter, Abl) => plain("a"),
        (Singular, Masculine | Neuter, Dat) => plain("u"),
        (Singular, Feminine, Dat) => plain("i"),
        (Singular, Masculine | Neuter, Ins) => plain("om"),
        (Singular, Feminine, Ins) => plain("oj"),
        (Singular, _, Loc) => plain("i"),

        (Dual, _, Nom | Acc | Voc) => plain("a"),
        (Dual, _, Gen | Loc) => plain("u"),
        (Dual, _, Dat | Ins | Abl) => plain("oma"),

        (Plural, _, Nom | Acc | Voc) => plain("y"),
        (Plural, _, Gen) => plain("ov"),
        (Plural, _, Dat | Abl) => plain("om"),
        (Plural, _, Ins) => plain("ami"),
        (Plural, _, Loc) => plain("ah"),
    })
}

/// Long (definite) endings — the pronoun `toj`'s (§4.2).
///
/// Long adjectives have **no vocative**; the nominative is used, as in every
/// language measured.
fn long(case: Case, number: Number, gender: Gender, animacy: Animacy) -> Option<Ending> {
    use Case::*;
    use Gender::*;
    use Number::*;
    Some(match (number, gender, case) {
        (Singular, Masculine, Nom) => plain("yj"),
        (Singular, Masculine, Acc) => match animacy {
            Animacy::Animate => plain("ogo"),
            Animacy::Inanimate => plain("yj"),
        },
        (Singular, Neuter, Nom | Acc) => plain("oje"),
        (Singular, Feminine, Nom) => plain("aja"),
        (Singular, Feminine, Acc) => plain("uju"),
        (Singular, Masculine | Neuter, Gen) => plain("ogo"),
        (Singular, Masculine | Neuter, Abl) => plain("a"),
        (Singular, Masculine | Neuter, Dat) => plain("omu"),
        (Singular, Masculine | Neuter, Ins) => plain("ym"),
        (Singular, Masculine | Neuter, Loc) => plain("om"),
        (Singular, Feminine, Gen | Abl | Dat | Ins | Loc) => plain("oj"),

        (Dual, _, Nom | Acc) => plain("aja"),
        (Dual, _, Gen | Loc) => plain("u"),
        (Dual, _, Dat | Ins | Abl) => plain("yma"),

        (Plural, _, Nom) => plain("yje"),
        (Plural, _, Acc) => match animacy {
            Animacy::Animate => plain("yh"),
            Animacy::Inanimate => plain("yje"),
        },
        (Plural, _, Gen | Loc) => plain("yh"),
        (Plural, _, Dat | Abl) => plain("ym"),
        (Plural, _, Ins) => plain("ymi"),

        (_, _, Voc) => return None,
    })
}

struct Ending {
    text: &'static str,
    palatal: Palatal,
}

fn plain(text: &'static str) -> Ending {
    Ending {
        text,
        palatal: Palatal::None,
    }
}
fn first_pal(text: &'static str) -> Ending {
    Ending {
        text,
        palatal: Palatal::First,
    }
}

/// Decline an adjective.
///
/// `stem` is the positive-degree stem: `dobr` for `dobr`/`dobryj`.
///
/// `None` means the cell does not exist — the only such cell is the **long
/// vocative** (§4.2), where the nominative is used instead.
///
/// ```
/// use ruthenian_core::{adjective, AdjForm::*, Animacy::*, Case::*, Degree::*, Gender::*, Number::*};
///
/// let s = |case, number, gender| {
///     adjective("dobr", case, number, gender, Inanimate, Short, Positive).unwrap().text
/// };
/// let l = |case, number, gender| {
///     adjective("dobr", case, number, gender, Inanimate, Long, Positive).unwrap().text
/// };
///
/// // short declines as a noun; long declines pronominally
/// assert_eq!(s(Nom, Singular, Masculine), "dobr");
/// assert_eq!(l(Nom, Singular, Masculine), "dobryj");
/// assert_eq!(s(Dat, Singular, Masculine), "dobru");
/// assert_eq!(l(Dat, Singular, Masculine), "dobromu");
///
/// // ...but the genitive and ablative coincide, an inherited contraction
/// assert_eq!(s(Gen, Singular, Masculine), l(Gen, Singular, Masculine));
/// assert_eq!(s(Abl, Singular, Masculine), l(Abl, Singular, Masculine));
///
/// // the long form has no vocative
/// assert_eq!(s(Voc, Singular, Masculine), "dobrje");
/// assert!(adjective("dobr", Voc, Singular, Masculine, Inanimate, Long, Positive).is_none());
/// ```
///
/// Degree is regular, with no suppletion, and the comparative triggers the first
/// palatalization (§4.3):
///
/// ```
/// use ruthenian_core::{adjective, AdjForm::*, Animacy::*, Case::*, Degree::*, Gender::*, Number::*};
/// let d = |degree| {
///     adjective("dobr", Nom, Singular, Masculine, Inanimate, Long, degree).unwrap().text
/// };
/// assert_eq!(d(Positive), "dobryj");
/// assert_eq!(d(Comparative), "dobrjejszij");
/// assert_eq!(d(Superlative), "najdobrjejszij");
/// ```
#[allow(clippy::too_many_arguments)]
pub fn adjective(
    stem: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
    form: AdjForm,
    degree: Degree,
) -> Option<Prediction> {
    let mut trace = Trace::new(match form {
        AdjForm::Short => "short (indefinite) adjective: nominal declension",
        AdjForm::Long => "long (definite) adjective: pronominal declension",
    });

    // Degree first: the comparative suffix becomes part of the stem, and the
    // endings then attach to it. Building it the other way round would put the
    // suffix after the ending.
    let stem = match degree {
        Degree::Positive => phono::unstress(stem),
        Degree::Comparative | Degree::Superlative => {
            let base = phono::palatalize(&phono::unstress(stem), Palatal::First);
            trace = trace.then("comparative -jejsz-, with the first palatalization");
            format!("{base}jejsz")
        }
    };
    let (stem, prefix) = match degree {
        Degree::Superlative => {
            trace = trace.then("superlative naj- on the comparative");
            (stem, "naj")
        }
        _ => (stem, ""),
    };

    let end = match form {
        AdjForm::Short => short(case, number, gender)?,
        AdjForm::Long => long(case, number, gender, animacy)?,
    };

    let palatalized = phono::palatalize(&stem, end.palatal);
    if palatalized != stem {
        trace = trace.then("first palatalization before the vocative -je");
    }
    // Stress is fixed (§2.1), so an ending never carries it.
    let ending = phono::spell_after_stem(&palatalized, end.text, false);
    if ending != end.text {
        trace = trace.then("automatic spelling adjustment after a velar or sibilant");
    }

    Some(Prediction::new(
        format!("{prefix}{palatalized}{ending}"),
        trace,
    ))
}
