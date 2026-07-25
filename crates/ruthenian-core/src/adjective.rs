//! Adjective declension: long forms, short forms, and the comparative.
//!
//! The `ж ш ч щ ц` spelling rules are stated in Ruthenian, where those letters
//! are `zz sz cz szcz c` — the rule is about the alphabet we actually emit, not
//! about the one the source data happens to use.

use crate::phono;
use crate::policy::{Prediction, Trace};
use crate::types::{AdjForm, Animacy, Case, Gender, Number};

fn long_ending(case: Case, number: Number, gender: Gender, soft: bool) -> Option<&'static str> {
    use Case::*;
    use Gender::*;
    use Number::*;
    Some(match (number, gender, case) {
        (Plural, _, Nom) => {
            if soft {
                "ije"
            } else {
                "yje"
            }
        }
        (Plural, _, Gen) => {
            if soft {
                "ih"
            } else {
                "yh"
            }
        }
        (Plural, _, Dat) => {
            if soft {
                "im"
            } else {
                "ym"
            }
        }
        (Plural, _, Ins) => {
            if soft {
                "imi"
            } else {
                "ymi"
            }
        }
        (Plural, _, Loc) => {
            if soft {
                "ih"
            } else {
                "yh"
            }
        }
        (Plural, _, Acc) => return None,

        (Singular, Masculine, Nom) => {
            if soft {
                "ij"
            } else {
                "yj"
            }
        }
        (Singular, Masculine, Gen) => {
            if soft {
                "jego"
            } else {
                "ogo"
            }
        }
        (Singular, Masculine, Dat) => {
            if soft {
                "jemu"
            } else {
                "omu"
            }
        }
        (Singular, Masculine, Ins) => {
            if soft {
                "im"
            } else {
                "ym"
            }
        }
        (Singular, Masculine, Loc) => {
            if soft {
                "jem"
            } else {
                "om"
            }
        }
        (Singular, Masculine, Acc) => return None,

        (Singular, Neuter, Nom) => {
            if soft {
                "jeje"
            } else {
                "oje"
            }
        }
        (Singular, Neuter, Acc) => {
            if soft {
                "jeje"
            } else {
                "oje"
            }
        }
        (Singular, Neuter, Gen) => {
            if soft {
                "jego"
            } else {
                "ogo"
            }
        }
        (Singular, Neuter, Dat) => {
            if soft {
                "jemu"
            } else {
                "omu"
            }
        }
        (Singular, Neuter, Ins) => {
            if soft {
                "im"
            } else {
                "ym"
            }
        }
        (Singular, Neuter, Loc) => {
            if soft {
                "jem"
            } else {
                "om"
            }
        }

        (Singular, Feminine, Nom) => {
            if soft {
                "jaja"
            } else {
                "aja"
            }
        }
        (Singular, Feminine, Gen) => {
            if soft {
                "jej"
            } else {
                "oj"
            }
        }
        (Singular, Feminine, Dat) => {
            if soft {
                "jej"
            } else {
                "oj"
            }
        }
        (Singular, Feminine, Acc) => {
            if soft {
                "juju"
            } else {
                "uju"
            }
        }
        (Singular, Feminine, Ins) => {
            if soft {
                "jej"
            } else {
                "oj"
            }
        }
        (Singular, Feminine, Loc) => {
            if soft {
                "jej"
            } else {
                "oj"
            }
        }
    })
}

fn short_ending(number: Number, gender: Gender) -> &'static str {
    match (number, gender) {
        (Number::Plural, _) => "y",
        (_, Gender::Masculine) => "",
        (_, Gender::Feminine) => "a",
        (_, Gender::Neuter) => "o",
    }
}

/// Decline an adjective. `stem` is the masculine nominative singular with its
/// ending removed.
///
/// ```
/// use ruthenian_core::adjective::adjective;
/// use ruthenian_core::types::*;
/// let p = adjective("nov", Case::Gen, Number::Singular, Gender::Masculine,
///                   Animacy::Inanimate, AdjForm::Long).unwrap();
/// assert_eq!(p.text, "novogo");
/// ```
pub fn adjective(
    stem: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
    form: AdjForm,
) -> Option<Prediction> {
    let bare = phono::unstress(stem);
    let soft = bare.ends_with('n') && false; // soft-stem adjectives end in -nij; see below
    let soft = soft || phono::ends_sibilant(&bare) || phono::ends_velar(&bare);

    match form {
        AdjForm::Short => {
            let e = phono::spell_after_stem(&bare, short_ending(number, gender));
            Some(Prediction::new(
                format!("{stem}{e}"),
                Trace::new("short form"),
            ))
        }
        AdjForm::Comparative => Some(Prediction::new(
            format!("{}jeje", phono::mutate_present_stem(&bare)),
            Trace::new("comparative: mutated stem + jeje"),
        )),
        AdjForm::Superlative => Some(Prediction::new(
            format!("samyj {stem}yj"),
            Trace::new("superlative: samyj + long form"),
        )),
        AdjForm::Long => {
            if case == Case::Acc {
                let source = match (gender, number, animacy) {
                    (Gender::Feminine, Number::Singular, _) => Case::Acc,
                    (_, _, Animacy::Animate) => Case::Gen,
                    (Gender::Masculine, _, _) | (_, Number::Plural, _) => Case::Nom,
                    _ => Case::Acc,
                };
                if source != Case::Acc {
                    let mut p = adjective(stem, source, number, gender, animacy, form)?;
                    p.trace = p.trace.then("accusative copies nominative or genitive");
                    return Some(p);
                }
            }
            let raw = long_ending(case, number, gender, soft)?;
            let e = phono::spell_after_stem(&bare, raw);
            Some(Prediction::new(
                format!("{stem}{e}"),
                Trace::new("long form ending"),
            ))
        }
    }
}
