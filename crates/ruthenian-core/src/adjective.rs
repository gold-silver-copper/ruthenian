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

/// Split a citation form into its stem and whether that stem is soft.
///
/// Soft iff the form ends `-ij` **and** the stem-final consonant is not a velar
/// or a sibilant. Velar and sibilant stems take *hard* endings with the `y` ->
/// `i` spelling rule (`russkogo`, never `*russkjego`); only the `-nij` type is
/// genuinely soft, and it is 155 of 9 999 adjectives in the dump — 1.6 %.
///
/// Stripped segmentally, so a stressed ending does not defeat the suffix match.
pub fn split_citation(citation: &str) -> (String, bool) {
    let idx = phono::stressed_index(citation);
    let bare = phono::unstress(citation);
    let (stem, soft) = if let Some(rest) = bare.strip_suffix("ij") {
        let soft = !(phono::ends_velar(rest) || phono::ends_sibilant(rest));
        (rest.to_string(), soft)
    } else if let Some(rest) = bare.strip_suffix("yj").or_else(|| bare.strip_suffix("oj")) {
        (rest.to_string(), false)
    } else {
        (bare.clone(), false)
    };
    let stem = match idx {
        Some(i) if i < phono::vowel_count(&stem) => phono::apply_stress_at(&stem, i),
        _ => stem,
    };
    (stem, soft)
}

fn short_ending(number: Number, gender: Gender) -> &'static str {
    match (number, gender) {
        (Number::Plural, _) => "y",
        (_, Gender::Masculine) => "",
        (_, Gender::Feminine) => "a",
        (_, Gender::Neuter) => "o",
    }
}

/// Decline an adjective from its **citation form** — the masculine nominative
/// singular, ending included.
///
/// The citation form rather than a bare stem, because softness is a property of
/// the lemma and a stem cannot carry it: `sin` and `nov` look alike, but
/// `sinij` is soft and `novyj` is hard.
///
/// ```
/// use ruthenian_core::adjective::adjective;
/// use ruthenian_core::types::*;
/// let long = |w, c, g| adjective(w, c, Number::Singular, g, Animacy::Inanimate, AdjForm::Long)
///     .unwrap().text;
///
/// assert_eq!(long("novyj", Case::Gen, Gender::Masculine), "novogo");
/// // soft stem: -jego, not -ogo
/// assert_eq!(long("sinij", Case::Gen, Gender::Masculine), "sinjego");
/// // a velar stem is HARD with an i-spelling, not soft
/// assert_eq!(long("russkij", Case::Gen, Gender::Masculine), "russkogo");
/// ```
pub fn adjective(
    citation: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
    form: AdjForm,
) -> Option<Prediction> {
    let (stem, soft) = split_citation(citation);
    let stem = stem.as_str();
    let bare = phono::unstress(stem);

    match form {
        AdjForm::Short => {
            let e = phono::spell_after_stem(&bare, short_ending(number, gender), false);
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
            format!("samyj {citation}"),
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
                    let mut p = adjective(citation, source, number, gender, animacy, form)?;
                    p.trace = p.trace.then("accusative copies nominative or genitive");
                    return Some(p);
                }
            }
            let raw = long_ending(case, number, gender, soft)?;
            let e = phono::spell_after_stem(&bare, raw, false);
            Some(Prediction::new(
                format!("{stem}{e}"),
                Trace::new("long form ending"),
            ))
        }
    }
}
