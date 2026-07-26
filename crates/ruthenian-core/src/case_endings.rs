//! The nominal ending tables, as **pure data**.
//!
//! Every entry here is transcribed from a paradigm table in `RUTHENIAN.md`
//! §§3.3–3.6, and `spec_paradigms_match` reads those same tables at test time to
//! check them. Keeping the tables in their own module, with no logic, is the
//! `interslavic-core` layout: `noun.rs` decides *how* an ending attaches, this
//! module decides *which* ending it is, and the two can be reviewed separately.
//!
//! # Why an ending carries its palatalization
//!
//! `-i` is spelled the same in the genitive `knigi` and the dative `knizi`, but
//! only the dative's continues **yat**, and only yat triggered the second
//! palatalization (§3.8 rule 5). The trigger therefore travels with the ending
//! rather than being inferred from its spelling — inferring it is impossible,
//! and that impossibility is exactly the distinction Russian lost.

use crate::phono::Palatal;
use crate::types::{Case, Declension, Gender, Number, StemHardness};

/// One ending, plus the palatalization it triggers on a preceding velar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ending {
    pub text: &'static str,
    pub palatal: Palatal,
}

/// Where a cell takes its form from, when it is not an ending of its own.
///
/// A syncretism is **not** a gap: the cell exists and returns a form. Modelling
/// the two the same way is how `None` stops being trustworthy (I4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// A real ending.
    Has(Ending),
    /// Copies another case in the same number.
    Same(Case),
    /// Nominative for an inanimate; for an animate, the ablative in the singular
    /// and the genitive in the plural (§3.7 — see `noun.rs` for why the tables
    /// and the prose disagree, and which wins).
    ByAnimacy,
}

const fn e(text: &'static str) -> Cell {
    Cell::Has(Ending {
        text,
        palatal: Palatal::None,
    })
}
/// An ending that triggers the first palatalization — the vocative `-je`.
const fn e1(text: &'static str) -> Cell {
    Cell::Has(Ending {
        text,
        palatal: Palatal::First,
    })
}
/// An ending whose `-i` continues yat, triggering the second palatalization.
const fn e2(text: &'static str) -> Cell {
    Cell::Has(Ending {
        text,
        palatal: Palatal::Second,
    })
}

use Case::*;
use Cell::{ByAnimacy, Same};
use Number::*;
use StemHardness::{Hard, Soft};

/// Declension II — masculine (`dom`, `konj`, `drug`; §3.3).
pub fn declension_ii_masculine(hardness: StemHardness, case: Case, number: Number) -> Option<Cell> {
    Some(match (hardness, number, case) {
        (Hard, Singular, Nom) => e(""),
        (Hard, Singular, Voc) => e1("je"),
        (Hard, Singular, Gen) => e("ogo"),
        (Hard, Singular, Abl) => e("a"),
        (Hard, Singular, Dat) => e("u"),
        (Hard, Singular, Ins) => e("om"),
        (Hard, Singular, Loc) => e2("i"),
        (Soft, Singular, Nom) => e("j"),
        (Soft, Singular, Voc) => e("ju"),
        (Soft, Singular, Gen) => e("jego"),
        (Soft, Singular, Abl) => e("ja"),
        (Soft, Singular, Dat) => e("ju"),
        (Soft, Singular, Ins) => e("jem"),
        (Soft, Singular, Loc) => e("ji"),
        (_, Singular, Acc) => ByAnimacy,

        // The dual has three forms and never more: direct, adnominal, oblique.
        (Hard, Dual, Nom) => e("a"),
        (Hard, Dual, Gen | Loc) => e("u"),
        (Hard, Dual, Dat | Ins) => e("oma"),
        (Soft, Dual, Nom) => e("ja"),
        (Soft, Dual, Gen | Loc) => e("ju"),
        (Soft, Dual, Dat | Ins) => e("jema"),
        (_, Dual, Voc | Acc) => Same(Nom),
        (_, Dual, Abl) => Same(Dat),

        (Hard, Plural, Nom) => e("y"),
        (Hard, Plural, Gen) => e("ov"),
        (Hard, Plural, Dat) => e("om"),
        (Hard, Plural, Ins) => e("ami"),
        (Hard, Plural, Loc) => e("ah"),
        (Soft, Plural, Nom) => e("ji"),
        (Soft, Plural, Gen) => e("jev"),
        (Soft, Plural, Dat) => e("jem"),
        (Soft, Plural, Ins) => e("jami"),
        (Soft, Plural, Loc) => e("jah"),
        (_, Plural, Voc) => Same(Nom),
        (_, Plural, Acc) => ByAnimacy,
        (_, Plural, Abl) => Same(Dat),
    })
}

/// Declension II — neuter (`okno`, `polje`; §3.4).
pub fn declension_ii_neuter(hardness: StemHardness, case: Case, number: Number) -> Option<Cell> {
    Some(match (hardness, number, case) {
        (Hard, Singular, Nom) => e("o"),
        (Hard, Singular, Gen) => e("ogo"),
        (Hard, Singular, Abl) => e("a"),
        (Hard, Singular, Dat) => e("u"),
        (Hard, Singular, Ins) => e("om"),
        (Hard, Singular, Loc) => e2("i"),
        (Soft, Singular, Nom) => e("je"),
        (Soft, Singular, Gen) => e("jego"),
        (Soft, Singular, Abl) => e("ja"),
        (Soft, Singular, Dat) => e("ju"),
        (Soft, Singular, Ins) => e("jem"),
        (Soft, Singular, Loc) => e("ji"),

        // The neuter dual `-i` continues OCS `-ě` (`dvě selě`), so it is
        // yat-derived and palatalizes.
        (Hard, Dual, Nom) => e2("i"),
        (Hard, Dual, Gen | Loc) => e("u"),
        (Hard, Dual, Dat | Ins) => e("oma"),
        (Soft, Dual, Nom) => e("ji"),
        (Soft, Dual, Gen | Loc) => e("ju"),
        (Soft, Dual, Dat | Ins) => e("jema"),

        (Hard, Plural, Nom) => e("a"),
        (Hard, Plural, Gen) => e("ov"),
        (Hard, Plural, Dat) => e("om"),
        (Hard, Plural, Ins) => e("ami"),
        (Hard, Plural, Loc) => e("ah"),
        (Soft, Plural, Nom) => e("ja"),
        (Soft, Plural, Gen) => e("jev"),
        (Soft, Plural, Dat) => e("jem"),
        (Soft, Plural, Ins) => e("jami"),
        (Soft, Plural, Loc) => e("jah"),

        // The neuter vocative is the nominative in every language measured
        // (Sanskrit 84 % zero, OCS 45 %), and the neuter accusative is the
        // nominative throughout — no animacy split.
        (_, _, Voc | Acc) => Same(Nom),
        (_, Dual | Plural, Abl) => Same(Dat),
    })
}

/// Declension I — feminine in `-a` (`zzena`, `kniga`, `zjemlja`; §3.5).
pub fn declension_i(hardness: StemHardness, case: Case, number: Number) -> Option<Cell> {
    Some(match (hardness, number, case) {
        (Hard, Singular, Nom) => e("a"),
        (Hard, Singular, Voc) => e("o"),
        (Hard, Singular, Acc) => e("u"),
        (Hard, Singular, Gen) => e("y"),
        // Dative = locative: both continue OCS `-ě`, and they are identical in
        // OCS, Russian and Ukrainian alike. Keeping them apart would be an
        // innovation, not a conservatism.
        (Hard, Singular, Dat | Loc) => e2("i"),
        (Hard, Singular, Ins) => e("oj"),
        (Soft, Singular, Nom) => e("ja"),
        (Soft, Singular, Voc) => e("jo"),
        (Soft, Singular, Acc) => e("ju"),
        (Soft, Singular, Gen) => e("i"),
        (Soft, Singular, Dat) => e("ji"),
        (Soft, Singular, Loc) => e("i"),
        (Soft, Singular, Ins) => e("joj"),
        // Ablative = genitive in the feminine singular, as in PIE and Sanskrit
        // (99 %). The one place the ablative is not distinct in the singular.
        (_, Singular, Abl) => Same(Gen),

        (Hard, Dual, Nom) => e2("i"),
        (Hard, Dual, Gen | Loc) => e("u"),
        (Hard, Dual, Dat | Ins) => e("ama"),
        (Soft, Dual, Nom) => e("ji"),
        (Soft, Dual, Gen | Loc) => e("ju"),
        (Soft, Dual, Dat | Ins) => e("jama"),

        (Hard, Plural, Nom) => e("y"),
        (Hard, Plural, Gen) => e("ov"),
        (Hard, Plural, Dat) => e("am"),
        (Hard, Plural, Ins) => e("ami"),
        (Hard, Plural, Loc) => e("ah"),
        (Soft, Plural, Nom) => e("i"),
        (Soft, Plural, Gen) => e("jev"),
        (Soft, Plural, Dat) => e("jam"),
        (Soft, Plural, Ins) => e("jami"),
        (Soft, Plural, Loc) => e("jah"),

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
pub fn declension_iii(case: Case, number: Number) -> Option<Cell> {
    Some(match (number, case) {
        (Singular, Nom | Acc) => e("j"),
        (Singular, Voc | Gen | Dat | Loc) => e("i"),
        // The instrumental keeps the soft sign AND takes the ending, as in
        // Russian `ночью`.
        (Singular, Ins) => e("jju"),
        (Singular, Abl) => Same(Gen),

        (Dual, Nom) => e("i"),
        (Dual, Gen | Loc) => e("ju"),
        (Dual, Dat | Ins) => e("jma"),

        (Plural, Nom) => e("i"),
        (Plural, Gen) => e("jev"),
        (Plural, Dat) => e("jam"),
        (Plural, Ins) => e("jami"),
        (Plural, Loc) => e("jah"),

        (Dual, Voc | Acc) => Same(Nom),
        (Plural, Voc) => Same(Nom),
        (Plural, Acc) => ByAnimacy,
        (Dual | Plural, Abl) => Same(Dat),
    })
}

/// The one dispatch point: which table applies to this class and gender.
pub fn nominal(
    declension: Declension,
    hardness: StemHardness,
    gender: Gender,
    case: Case,
    number: Number,
) -> Option<Cell> {
    match (declension, gender) {
        (Declension::I, _) => declension_i(hardness, case, number),
        (Declension::II, Gender::Neuter) => declension_ii_neuter(hardness, case, number),
        (Declension::II, _) => declension_ii_masculine(hardness, case, number),
        (Declension::III, _) => declension_iii(case, number),
    }
}
