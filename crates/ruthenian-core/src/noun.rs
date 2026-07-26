//! Noun declension: **eight cases × three numbers over three declensions**
//! (`RUTHENIAN.md` §§3.1–3.8).
//!
//! The endings below are transcribed from the specification's paradigm tables
//! and are checked against them by `spec_paradigms_match`. That test is what
//! makes this module correct-by-definition rather than correct-by-opinion: where
//! it disagrees with `docs/RUTHENIAN.md`, this module is wrong.
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

use crate::phono::{self, Palatal};
use crate::types::{Animacy, Case, Declension, Gender, NounClass, Number, StemHardness};
use crate::variant::{Prediction, Trace};

/// One ending, plus the palatalization it triggers on a preceding velar.
///
/// Carrying the trigger with the ending is what keeps the genitive `knigi` and
/// the dative `knizi` apart: both surface as `-i`, but only the dative's `-i`
/// continues yat, and only yat triggered the second palatalization (§3.8 rule
/// 5). Deciding this from the ending's spelling alone is impossible — that is
/// precisely the distinction Russian lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct End {
    text: &'static str,
    palatal: Palatal,
}

const fn e(text: &'static str) -> End {
    End {
        text,
        palatal: Palatal::None,
    }
}
/// An ending that triggers the first palatalization — the vocative `-je`.
const fn e1(text: &'static str) -> End {
    End {
        text,
        palatal: Palatal::First,
    }
}
/// An ending whose `-i` continues yat, triggering the second palatalization.
const fn e2(text: &'static str) -> End {
    End {
        text,
        palatal: Palatal::Second,
    }
}

/// Where a cell takes its form from, when it is not an ending of its own.
enum Cell {
    /// A real ending.
    Has(End),
    /// Copies another case in the same number.
    Same(Case),
    /// Nominative for an inanimate, genitive for an animate (§3.7).
    ByAnimacy,
}

use Cell::{ByAnimacy, Has, Same};

/// Declension II — masculine (`dom`, `konj`, `drug`; §3.3).
fn decl_ii_masc(hardness: StemHardness, case: Case, number: Number) -> Option<Cell> {
    use Case::*;
    use Number::*;
    use StemHardness::*;
    Some(match (hardness, number, case) {
        // ---- hard singular -------------------------------------------------
        (Hard, Singular, Nom) => Has(e("")),
        (Hard, Singular, Voc) => Has(e1("je")),
        (Hard, Singular, Gen) => Has(e("ogo")),
        (Hard, Singular, Abl) => Has(e("a")),
        (Hard, Singular, Dat) => Has(e("u")),
        (Hard, Singular, Ins) => Has(e("om")),
        (Hard, Singular, Loc) => Has(e2("i")),
        // ---- soft singular -------------------------------------------------
        (Soft, Singular, Nom) => Has(e("j")),
        (Soft, Singular, Voc) => Has(e("ju")),
        (Soft, Singular, Gen) => Has(e("jego")),
        (Soft, Singular, Abl) => Has(e("ja")),
        (Soft, Singular, Dat) => Has(e("ju")),
        (Soft, Singular, Ins) => Has(e("jem")),
        (Soft, Singular, Loc) => Has(e("ji")),
        (_, Singular, Acc) => ByAnimacy,

        // ---- dual: direct / adnominal / oblique ----------------------------
        (Hard, Dual, Nom) => Has(e("a")),
        (Hard, Dual, Gen) => Has(e("u")),
        (Hard, Dual, Loc) => Has(e("u")),
        (Hard, Dual, Dat) => Has(e("oma")),
        (Hard, Dual, Ins) => Has(e("oma")),
        (Soft, Dual, Nom) => Has(e("ja")),
        (Soft, Dual, Gen) => Has(e("ju")),
        (Soft, Dual, Loc) => Has(e("ju")),
        (Soft, Dual, Dat) => Has(e("jema")),
        (Soft, Dual, Ins) => Has(e("jema")),
        (_, Dual, Voc | Acc) => Same(Nom),
        (_, Dual, Abl) => Same(Dat),

        // ---- plural --------------------------------------------------------
        (Hard, Plural, Nom) => Has(e("y")),
        (Hard, Plural, Gen) => Has(e("ov")),
        (Hard, Plural, Dat) => Has(e("om")),
        (Hard, Plural, Ins) => Has(e("ami")),
        (Hard, Plural, Loc) => Has(e("ah")),
        (Soft, Plural, Nom) => Has(e("ji")),
        (Soft, Plural, Gen) => Has(e("jev")),
        (Soft, Plural, Dat) => Has(e("jem")),
        (Soft, Plural, Ins) => Has(e("jami")),
        (Soft, Plural, Loc) => Has(e("jah")),
        (_, Plural, Voc) => Same(Nom),
        (_, Plural, Acc) => ByAnimacy,
        (_, Plural, Abl) => Same(Dat),
    })
}

/// Declension II — neuter (`okno`, `polje`; §3.4).
fn decl_ii_neut(hardness: StemHardness, case: Case, number: Number) -> Option<Cell> {
    use Case::*;
    use Number::*;
    use StemHardness::*;
    Some(match (hardness, number, case) {
        (Hard, Singular, Nom) => Has(e("o")),
        (Hard, Singular, Gen) => Has(e("ogo")),
        (Hard, Singular, Abl) => Has(e("a")),
        (Hard, Singular, Dat) => Has(e("u")),
        (Hard, Singular, Ins) => Has(e("om")),
        (Hard, Singular, Loc) => Has(e2("i")),
        (Soft, Singular, Nom) => Has(e("je")),
        (Soft, Singular, Gen) => Has(e("jego")),
        (Soft, Singular, Abl) => Has(e("ja")),
        (Soft, Singular, Dat) => Has(e("ju")),
        (Soft, Singular, Ins) => Has(e("jem")),
        (Soft, Singular, Loc) => Has(e("ji")),

        // The neuter dual `-i` continues OCS `-ě` (`dvě selě`), so it is
        // yat-derived and palatalizes.
        (Hard, Dual, Nom) => Has(e2("i")),
        (Hard, Dual, Gen) => Has(e("u")),
        (Hard, Dual, Loc) => Has(e("u")),
        (Hard, Dual, Dat) => Has(e("oma")),
        (Hard, Dual, Ins) => Has(e("oma")),
        (Soft, Dual, Nom) => Has(e("ji")),
        (Soft, Dual, Gen) => Has(e("ju")),
        (Soft, Dual, Loc) => Has(e("ju")),
        (Soft, Dual, Dat) => Has(e("jema")),
        (Soft, Dual, Ins) => Has(e("jema")),

        (Hard, Plural, Nom) => Has(e("a")),
        (Hard, Plural, Gen) => Has(e("ov")),
        (Hard, Plural, Dat) => Has(e("om")),
        (Hard, Plural, Ins) => Has(e("ami")),
        (Hard, Plural, Loc) => Has(e("ah")),
        (Soft, Plural, Nom) => Has(e("ja")),
        (Soft, Plural, Gen) => Has(e("jev")),
        (Soft, Plural, Dat) => Has(e("jem")),
        (Soft, Plural, Ins) => Has(e("jami")),
        (Soft, Plural, Loc) => Has(e("jah")),

        // The neuter vocative is the nominative in every language measured
        // (Sanskrit 84 % zero, OCS 45 %), and the neuter accusative is the
        // nominative throughout — no animacy split.
        (_, _, Voc | Acc) => Same(Nom),
        (_, Dual | Plural, Abl) => Same(Dat),
    })
}

/// Declension I — feminine in `-a` (`zzena`, `kniga`, `zjemlja`; §3.5).
fn decl_i(hardness: StemHardness, case: Case, number: Number) -> Option<Cell> {
    use Case::*;
    use Number::*;
    use StemHardness::*;
    Some(match (hardness, number, case) {
        (Hard, Singular, Nom) => Has(e("a")),
        (Hard, Singular, Voc) => Has(e("o")),
        (Hard, Singular, Acc) => Has(e("u")),
        (Hard, Singular, Gen) => Has(e("y")),
        // Dative = locative: both continue OCS `-ě`, and they are identical in
        // OCS, Russian and Ukrainian alike. Keeping them apart would be an
        // innovation, not a conservatism.
        (Hard, Singular, Dat) => Has(e2("i")),
        (Hard, Singular, Loc) => Has(e2("i")),
        (Hard, Singular, Ins) => Has(e("oj")),
        (Soft, Singular, Nom) => Has(e("ja")),
        (Soft, Singular, Voc) => Has(e("jo")),
        (Soft, Singular, Acc) => Has(e("ju")),
        (Soft, Singular, Gen) => Has(e("i")),
        (Soft, Singular, Dat) => Has(e("ji")),
        (Soft, Singular, Loc) => Has(e("i")),
        (Soft, Singular, Ins) => Has(e("joj")),
        // Ablative = genitive in the feminine singular, as in PIE and Sanskrit
        // (99 %). This is the one place the ablative is NOT distinct in the
        // singular.
        (_, Singular, Abl) => Same(Gen),

        (Hard, Dual, Nom) => Has(e2("i")),
        (Hard, Dual, Gen) => Has(e("u")),
        (Hard, Dual, Loc) => Has(e("u")),
        (Hard, Dual, Dat) => Has(e("ama")),
        (Hard, Dual, Ins) => Has(e("ama")),
        (Soft, Dual, Nom) => Has(e("ji")),
        (Soft, Dual, Gen) => Has(e("ju")),
        (Soft, Dual, Loc) => Has(e("ju")),
        (Soft, Dual, Dat) => Has(e("jama")),
        (Soft, Dual, Ins) => Has(e("jama")),

        (Hard, Plural, Nom) => Has(e("y")),
        (Hard, Plural, Gen) => Has(e("ov")),
        (Hard, Plural, Dat) => Has(e("am")),
        (Hard, Plural, Ins) => Has(e("ami")),
        (Hard, Plural, Loc) => Has(e("ah")),
        (Soft, Plural, Nom) => Has(e("i")),
        (Soft, Plural, Gen) => Has(e("jev")),
        (Soft, Plural, Dat) => Has(e("jam")),
        (Soft, Plural, Ins) => Has(e("jami")),
        (Soft, Plural, Loc) => Has(e("jah")),

        (_, Dual, Voc | Acc) => Same(Nom),
        (_, Plural, Voc) => Same(Nom),
        (_, Plural, Acc) => ByAnimacy,
        (_, Dual | Plural, Abl) => Same(Dat),
    })
}

/// Declension III — feminine in a consonant (`noczj`, `kostj`; §3.6).
///
/// The inherited PIE *i*-stem. Its singular is heavily syncretic — `-i` for
/// genitive, ablative, dative and locative — as it is in Russian, Ukrainian and
/// OCS. Hardness does not apply: the specification gives one paradigm.
fn decl_iii(case: Case, number: Number) -> Option<Cell> {
    use Case::*;
    use Number::*;
    Some(match (number, case) {
        (Singular, Nom) => Has(e("j")),
        (Singular, Voc) => Has(e("i")),
        (Singular, Acc) => Has(e("j")),
        (Singular, Gen) => Has(e("i")),
        (Singular, Dat) => Has(e("i")),
        (Singular, Loc) => Has(e("i")),
        // The instrumental keeps the soft sign AND takes the ending, as in
        // Russian `ночью`.
        (Singular, Ins) => Has(e("jju")),
        (Singular, Abl) => Same(Gen),

        (Dual, Nom) => Has(e("i")),
        (Dual, Gen) => Has(e("ju")),
        (Dual, Loc) => Has(e("ju")),
        (Dual, Dat) => Has(e("jma")),
        (Dual, Ins) => Has(e("jma")),

        (Plural, Nom) => Has(e("i")),
        (Plural, Gen) => Has(e("jev")),
        (Plural, Dat) => Has(e("jam")),
        (Plural, Ins) => Has(e("jami")),
        (Plural, Loc) => Has(e("jah")),

        (Dual, Voc | Acc) => Same(Nom),
        (Plural, Voc) => Same(Nom),
        (Plural, Acc) => ByAnimacy,
        (Dual | Plural, Abl) => Same(Dat),
    })
}

fn cell(class: NounClass, gender: Gender, case: Case, number: Number) -> Option<Cell> {
    match (class.declension, gender) {
        (Declension::I, _) => decl_i(class.hardness, case, number),
        (Declension::II, Gender::Neuter) => decl_ii_neut(class.hardness, case, number),
        (Declension::II, _) => decl_ii_masc(class.hardness, case, number),
        (Declension::III, _) => decl_iii(case, number),
    }
}

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
    match cell(class, gender, case, number)? {
        Has(end) => Some(build(stem, class, end)),
        Same(source) => {
            let mut p = noun(stem, class, gender, animacy, source, number)?;
            p.trace = p.trace.then(match (source, case) {
                (Case::Nom, _) => "direct case: copies the nominative",
                (Case::Gen, Case::Abl) => "feminine singular: ablative = genitive",
                (Case::Dat, _) => "dual and plural: ablative = dative",
                _ => "syncretic cell",
            });
            Some(p)
        }
        ByAnimacy => {
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

fn build(stem: &str, class: NounClass, end: End) -> Prediction {
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
