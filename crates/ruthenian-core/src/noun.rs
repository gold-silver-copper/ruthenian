//! Noun declension: **eight cases × three numbers over three declensions**
//! (`RUTHENIAN.md` §§3.1–3.8).
//!
//! The ending tables live in [`crate::case_endings`] as pure data; this module
//! owns only how an ending attaches to a stem. `spec_paradigms_match` checks the
//! result against `docs/RUTHENIAN.md`'s own tables, which is what makes this
//! correct-by-definition rather than correct-by-opinion: where the two disagree,
//! this module is wrong.
//!
//! # Three things Russian has that are absent here
//!
//! * **No accent patterns.** Stress is fixed per word (§2.1), so the stem keeps
//!   the mark it arrived with and no ending ever pulls it away.
//! * **No fleeting vowel in the genitive plural.** It is uniformly `-ov`
//!   (§3.9), so the zero-ending environment that produced `okno` → `okon` no
//!   longer occurs in this paradigm.
//! * **No indeclinables and no number defectiveness.** Every noun has all three
//!   numbers and declines (§3.9, §12.3).

use crate::case_endings::{self, Cell, Ending};
use crate::phono::{self, Palatal};
use crate::types::{Animacy, Case, Declension, Gender, NounClass, Number, StemHardness};
use crate::variant::{Prediction, Trace};

/// Decline a noun.
///
/// `stem` is the citation form's stem — see [`stem_of`] — in Ruthenian, carrying
/// its own stress mark. Stress is fixed, so whatever mark the stem arrives with
/// is the mark every form in the paradigm carries.
///
/// Returns `None` only when the cell genuinely does not exist for this class.
/// Every cell of every Ruthenian noun paradigm does exist, so in practice this
/// is `Some` throughout — the signature keeps the shape law 8 requires.
///
/// ```
/// use ruthenian_core::{noun, Animacy::*, Case::*, Declension::*, Gender::*, NounClass, Number::*};
///
/// let m = NounClass::hard(II);
/// let d = |case, number| noun("dom", m, Masculine, Inanimate, case, number).unwrap().text;
///
/// // the two endings the ablative decision restored, side by side
/// assert_eq!(d(Gen, Singular), "domogo");   // OF the house  (PIE *-osyo)
/// assert_eq!(d(Abl, Singular), "doma");     // FROM the house (PIE *-ōd)
///
/// assert_eq!(d(Voc, Singular), "domje");
/// assert_eq!(d(Loc, Singular), "domi");
/// assert_eq!(d(Nom, Dual), "doma");
/// assert_eq!(d(Dat, Dual), "domoma");
/// assert_eq!(d(Nom, Plural), "domy");
/// ```
///
/// A velar stem shows both palatalizations, which is where Ruthenian parts
/// company with Russian most visibly:
///
/// ```
/// use ruthenian_core::{noun, Animacy::*, Case::*, Declension::*, Gender::*, NounClass, Number::*};
///
/// let g = |case, number| {
///     noun("drug", NounClass::hard(II), Masculine, Animate, case, number).unwrap().text
/// };
/// assert_eq!(g(Voc, Singular), "druzzje");  // first palatalization:  g -> zz
/// assert_eq!(g(Loc, Singular), "druzi");    // second palatalization: g -> z
/// assert_eq!(g(Acc, Singular), "druga");    // animate: accusative = genitive
/// assert_eq!(g(Nom, Plural), "drugi");      // spelling rule only: y -> i
/// ```
pub fn noun(
    stem: &str,
    class: NounClass,
    gender: Gender,
    animacy: Animacy,
    case: Case,
    number: Number,
) -> Option<Prediction> {
    let cell = case_endings::nominal(class.declension, class.hardness, gender, case, number)?;
    match cell {
        Cell::Has(end) => Some(build(stem, class, end)),
        Cell::Same(source) => {
            let mut p = noun(stem, class, gender, animacy, source, number)?;
            p.trace = p.trace.then(match (source, case) {
                (Case::Nom, _) => "direct case: copies the nominative",
                (Case::Gen, Case::Abl) => "feminine singular: ablative = genitive",
                (Case::Dat, _) => "dual and plural: ablative = dative",
                _ => "syncretic cell",
            });
            Some(p)
        }
        Cell::ByAnimacy => {
            // §3.7's prose says an animate accusative "takes the genitive
            // form", but every paradigm table in §§3.3-3.5 shows the ABLATIVE
            // form in the singular: `dom`/`doma` against genitive `domogo`,
            // `konja` against `konjego`, `druga` against `drugogo`.
            //
            // The tables are followed, and they are also the historically
            // coherent reading: Slavic's animate accusative has always used the
            // `-a` form, and §3.1's whole argument is that this `-a` IS the
            // inherited ablative — Ruthenian merely gives it back its name. The
            // prose is using "genitive" in the traditional Slavic sense, which
            // this language has redefined out from under it.
            //
            // In the plural the question does not arise: ablative = dative
            // there, and the tables show the genitive (`drugov`, `zzenov`).
            let source = match (animacy, number) {
                (Animacy::Inanimate, _) => Case::Nom,
                (Animacy::Animate, Number::Singular) => Case::Abl,
                (Animacy::Animate, _) => Case::Gen,
            };
            let mut p = noun(stem, class, gender, animacy, source, number)?;
            p.trace = p.trace.then(match source {
                Case::Nom => "inanimate: accusative = nominative",
                Case::Abl => "animate singular: accusative = ablative",
                _ => "animate plural: accusative = genitive",
            });
            Some(p)
        }
    }
}

fn build(stem: &str, class: NounClass, end: Ending) -> Prediction {
    let mut trace = Trace::new("noun ending by declension, hardness, case and number");

    // A soft or declension-III citation form ends in `j`, but that `j` belongs
    // to the ENDING, not the stem: `konj` is `kon` + `j`, `noczj` is `nocz` +
    // `j`. Stripping it here lets each cell spell itself exactly — `nocz` +
    // `jju` for the instrumental, where the soft sign survives — instead of a
    // blanket `jj` -> `j` collapse that cannot tell the two apart.
    let stem = match (class.declension, class.hardness) {
        (Declension::III, _) | (_, StemHardness::Soft) => stem.strip_suffix('j').unwrap_or(stem),
        _ => stem,
    };

    // Palatalize BEFORE the spelling rules: the spelling rules read the stem's
    // final consonant, and palatalization is what changes it.
    let palatalized = phono::palatalize(stem, end.palatal);
    if palatalized != stem {
        trace = trace.then(match end.palatal {
            Palatal::First => "first palatalization before the vocative -je",
            Palatal::Second => "second palatalization before yat-derived -i",
            Palatal::None => unreachable!("no palatalization cannot change the stem"),
        });
    }

    // Stress is fixed (§2.1), so the ending never carries it and rule 2's
    // "unstressed o" condition is always met.
    let bare = phono::unstress(&palatalized);
    let ending = phono::spell_after_stem(&bare, end.text, false);
    if ending != end.text {
        trace = trace.then("automatic spelling adjustment after a velar or sibilant");
    }

    Prediction::new(format!("{palatalized}{ending}"), trace)
}

/// Strip a nominative-singular ending to get the stem the rules want.
///
/// ```
/// use ruthenian_core::noun::stem_of;
/// assert_eq!(stem_of("kniga"), "knig");
/// assert_eq!(stem_of("zjemlja"), "zjeml");
/// assert_eq!(stem_of("okno"), "okn");
/// assert_eq!(stem_of("polje"), "pol");
/// assert_eq!(stem_of("dom"), "dom");
/// // the soft sign belongs to the ending
/// assert_eq!(stem_of("konj"), "kon");
/// assert_eq!(stem_of("noczj"), "nocz");
/// // a stressed ending leaves the stem unmarked
/// assert_eq!(stem_of("okno\u{301}"), "okn");
/// ```
pub fn stem_of(nominative: &str) -> String {
    // A stress mark sits *after* its vowel, so a stressed ending would defeat a
    // plain `strip_suffix`. Strip segmentally, then put the stress back on the
    // vowel it belonged to if that vowel survived.
    let idx = phono::stressed_index(nominative);
    let bare = phono::unstress(nominative);
    let mut stem = bare.clone();
    for suffix in ["ja", "je", "a", "o", "j"] {
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
