//! Noun declension: six cases × two numbers over seven stem classes.
//!
//! The endings are keyed on `(gender, stem class, case, number)`; the accent
//! pattern then decides where the stress lands. Patterns `a` (fixed stem) and
//! `b` (fixed ending) are implemented and cover ~93 % of attested nouns; `c`–`f`
//! are mobile and return their form with the *segmental* answer and the stem's
//! own stress, which the trace records — see [`NounClass`] and the coverage note
//! in the crate docs.

use crate::phono;
use crate::policy::{Prediction, Trace};
use crate::types::{AccentPattern, Animacy, Case, Gender, NounClass, Number, StemClass};

/// Endings for the singular, by gender and stem class.
fn ending_sg(g: Gender, stem: StemClass, case: Case) -> Option<&'static str> {
    use Case::*;
    use Gender::*;
    use StemClass::*;
    Some(match (g, stem, case) {
        // --- masculine -------------------------------------------------
        (Masculine, I, _) => return None, // the i-stem declension is feminine
        (Masculine, Soft, Nom) => "j",
        (Masculine, Soft, Gen) => "ja",
        (Masculine, Soft, Dat) => "ju",
        (Masculine, Soft, Ins) => "jem",
        (Masculine, Soft, Loc) => "je",
        (Masculine, Vowel, Nom) => "j",
        (Masculine, Vowel, Gen) => "ja",
        (Masculine, Vowel, Dat) => "ju",
        (Masculine, Vowel, Ins) => "jem",
        (Masculine, Vowel, Loc) => "i",
        (Masculine, _, Nom) => "",
        (Masculine, _, Gen) => "a",
        (Masculine, _, Dat) => "u",
        (Masculine, _, Ins) => "om",
        (Masculine, _, Loc) => "je",
        (Masculine, _, Acc) => return None, // decided by animacy

        // --- feminine --------------------------------------------------
        (Feminine, I, Nom) => "j",
        (Feminine, I, Gen) => "i",
        (Feminine, I, Dat) => "i",
        (Feminine, I, Acc) => "j",
        (Feminine, I, Ins) => "jju",
        (Feminine, I, Loc) => "i",
        (Feminine, Soft, Nom) => "ja",
        (Feminine, Soft, Gen) => "i",
        (Feminine, Soft, Dat) => "je",
        (Feminine, Soft, Acc) => "ju",
        (Feminine, Soft, Ins) => "joj",
        (Feminine, Soft, Loc) => "je",
        (Feminine, Vowel, Nom) => "ja",
        (Feminine, Vowel, Gen) => "i",
        (Feminine, Vowel, Dat) => "i",
        (Feminine, Vowel, Acc) => "ju",
        (Feminine, Vowel, Ins) => "jej",
        (Feminine, Vowel, Loc) => "i",
        (Feminine, _, Nom) => "a",
        (Feminine, _, Gen) => "y",
        (Feminine, _, Dat) => "je",
        (Feminine, _, Acc) => "u",
        (Feminine, _, Ins) => "oj",
        (Feminine, _, Loc) => "je",

        // --- neuter ----------------------------------------------------
        (Neuter, I, _) => return None,
        (Neuter, Soft, Nom) => "je",
        (Neuter, Soft, Gen) => "ja",
        (Neuter, Soft, Dat) => "ju",
        (Neuter, Soft, Acc) => "je",
        (Neuter, Soft, Ins) => "jem",
        (Neuter, Soft, Loc) => "je",
        (Neuter, Vowel, Nom) => "je",
        (Neuter, Vowel, Gen) => "ja",
        (Neuter, Vowel, Dat) => "ju",
        (Neuter, Vowel, Acc) => "je",
        (Neuter, Vowel, Ins) => "jem",
        (Neuter, Vowel, Loc) => "i",
        (Neuter, _, Nom) => "o",
        (Neuter, _, Gen) => "a",
        (Neuter, _, Dat) => "u",
        (Neuter, _, Acc) => "o",
        (Neuter, _, Ins) => "om",
        (Neuter, _, Loc) => "je",
    })
}

fn ending_pl(g: Gender, stem: StemClass, case: Case) -> Option<&'static str> {
    use Case::*;
    use Gender::*;
    use StemClass::*;
    let soft = matches!(stem, Soft | I | Vowel);
    Some(match (g, case) {
        (_, Dat) => {
            if soft {
                "jam"
            } else {
                "am"
            }
        }
        (_, Ins) => {
            if soft {
                "jami"
            } else {
                "ami"
            }
        }
        (_, Loc) => {
            if soft {
                "jah"
            } else {
                "ah"
            }
        }
        (Masculine, Nom) => {
            if soft {
                "i"
            } else {
                "y"
            }
        }
        (Masculine, Gen) => match stem {
            Soft | Vowel => "jej",
            Sibilant => "jej",
            _ => "ov",
        },
        (Feminine, Nom) => {
            if soft {
                "i"
            } else {
                "y"
            }
        }
        (Feminine, Gen) => match stem {
            I => "jej",
            Soft => "j",
            Vowel => "j",
            _ => "",
        },
        (Neuter, Nom) => {
            if soft {
                "ja"
            } else {
                "a"
            }
        }
        (Neuter, Gen) => match stem {
            Soft | Vowel => "j",
            _ => "",
        },
        (_, Acc) => return None, // decided by animacy
    })
}

/// Decline a noun.
///
/// `stem` is the citation form's stem — the nominative singular with its ending
/// removed — in Ruthenian, carrying its own stress mark.
///
/// Returns `None` only when the cell genuinely does not exist for this class.
///
/// ```
/// use ruthenian_core::{noun, AccentPattern, Animacy, Case, Gender, NounClass, Number, StemClass};
/// let hard = NounClass { stem: StemClass::Hard, accent: AccentPattern::A, reducible: false };
/// let p = noun("stól", hard, Gender::Masculine, Animacy::Inanimate, Case::Gen, Number::Singular).unwrap();
/// assert_eq!(p.text, "stóla");
///
/// let velar = NounClass { stem: StemClass::Velar, accent: AccentPattern::A, reducible: false };
/// // the spelling rule: after k/g/h, y is written i
/// let p = noun("kníg", velar, Gender::Feminine, Animacy::Inanimate, Case::Gen, Number::Singular).unwrap();
/// assert_eq!(p.text, "knígi");
/// ```
pub fn noun(
    stem: &str,
    class: NounClass,
    gender: Gender,
    animacy: Animacy,
    case: Case,
    number: Number,
) -> Option<Prediction> {
    // The accusative is not an ending of its own: it copies the nominative or
    // the genitive, and which one is what animacy means.
    if case == Case::Acc {
        let source = match (gender, number, animacy) {
            (Gender::Masculine, _, Animacy::Animate) => Case::Gen,
            (_, Number::Plural, Animacy::Animate) => Case::Gen,
            (Gender::Masculine, _, Animacy::Inanimate) => Case::Nom,
            (Gender::Feminine, Number::Singular, _) => {
                // feminine singular has its own accusative
                return build(stem, class, gender, Case::Acc, number, true);
            }
            _ => Case::Nom,
        };
        let mut p = noun(stem, class, gender, animacy, source, number)?;
        p.trace = p.trace.then(match source {
            Case::Gen => "accusative = genitive (animate)",
            _ => "accusative = nominative (inanimate)",
        });
        return Some(p);
    }
    build(stem, class, gender, case, number, false)
}

fn build(
    stem: &str,
    class: NounClass,
    gender: Gender,
    case: Case,
    number: Number,
    direct_acc: bool,
) -> Option<Prediction> {
    let raw = match number {
        Number::Singular => ending_sg(gender, class.stem, case)?,
        Number::Plural => ending_pl(gender, class.stem, case)?,
    };
    let _ = direct_acc;

    // A soft or i-stem noun's citation form ends in `ь`, but that `ь` belongs to
    // the *ending*, not the stem: `konj` is `kon` + `j`, `boljeznj` is `boljezn`
    // + `j`. Stripping it here means the endings can spell each cell exactly —
    // `boljezn` + `jju` for the instrumental, where the soft sign survives —
    // instead of a blanket `jj` -> `j` collapse that cannot tell the two apart.
    let stem = match class.stem {
        StemClass::Soft | StemClass::I => stem.strip_suffix('j').unwrap_or(stem),
        _ => stem,
    };
    let bare_stem = phono::unstress(stem);
    // Accent `b` puts the stress on the ending, which is what decides the
    // sibilant `o`/`je` alternation.
    let ending_stressed = class.accent == AccentPattern::B;
    let ending = phono::spell_after_stem(&bare_stem, raw, ending_stressed);

    let mut trace = Trace::new("noun ending by gender and stem class");
    if ending != raw {
        trace = trace.then("spelling rule after velar/sibilant stem");
    }

    // A zero ending on a reducible stem grows a fleeting vowel: the genitive
    // plural of `sbjerknizzka` is `sbjerknizzjek`, not `*sbjerknizzk`.
    let (stem, bare_stem, trace) = if ending.is_empty() && class.reducible {
        let filled = phono::insert_fleeting_vowel(&bare_stem);
        let stressed = match phono::stressed_index(stem) {
            Some(i) => phono::apply_stress_at(&filled, i),
            None => filled.clone(),
        };
        (
            std::borrow::Cow::Owned(stressed),
            filled,
            trace.then("fleeting vowel before a zero ending"),
        )
    } else {
        (std::borrow::Cow::Borrowed(stem), bare_stem, trace)
    };
    let mut trace = trace;
    let stem: &str = &stem;

    let text = match class.accent {
        // Fixed stem stress: the stem keeps the mark it arrived with.
        AccentPattern::A => {
            trace = trace.then("accent a: fixed stem stress");
            format!("{stem}{ending}")
        }
        // Fixed ending stress: the stem loses its mark, the ending takes it.
        AccentPattern::B => {
            trace = trace.then("accent b: fixed ending stress");
            let e = if ending.is_empty() {
                // A null ending cannot carry stress; it stays on the stem's last
                // vowel, which is what Russian does (`stól` / `stolá` / `stolóv`).
                return Some(Prediction::new(
                    phono::stress_last_vowel(&bare_stem),
                    trace.then("null ending: stress retracts to the stem"),
                ));
            } else {
                phono::stress_first_vowel(&ending)
            };
            format!("{bare_stem}{e}")
        }
        // Mobile patterns. The segmental form is correct; the stress is the
        // stem's own, which is right for roughly half their cells. The trace
        // says so, so a consumer can tell, and phase 6 will price it.
        other => {
            trace = trace.then(match other {
                AccentPattern::C => "accent c: mobile stress not modelled",
                AccentPattern::D => "accent d: mobile stress not modelled",
                AccentPattern::E => "accent e: mobile stress not modelled",
                _ => "accent f: mobile stress not modelled",
            });
            format!("{stem}{ending}")
        }
    };

    Some(Prediction::new(text, trace))
}

/// Strip a nominative-singular ending to get the stem the rules want.
///
/// ```
/// use ruthenian_core::noun::stem_of;
/// assert_eq!(stem_of("kníga"), "kníg");
/// assert_eq!(stem_of("stól"), "stól");
/// // the stressed vowel went with the ending, so the stem carries no mark
/// assert_eq!(stem_of("okno\u{301}"), "okn");
/// ```
pub fn stem_of(nominative: &str) -> String {
    // A stress mark sits *after* its vowel, so a stressed ending would defeat a
    // plain `strip_suffix`. Strip segmentally, then put the stress back on the
    // vowel it belonged to if that vowel survived.
    let idx = phono::stressed_index(nominative);
    let bare = phono::unstress(nominative);
    let mut stem = bare.clone();
    for suffix in ["ja", "je", "a", "o", "e"] {
        if let Some(rest) = bare.strip_suffix(suffix) {
            if !rest.is_empty() {
                stem = rest.to_string();
                break;
            }
        }
    }
    match idx {
        Some(i) if i < phono::vowel_count(&stem) => phono::apply_stress_at(&stem, i),
        _ => stem,
    }
}
