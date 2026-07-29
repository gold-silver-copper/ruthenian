//! Reading a citation form: everything §3.2 and §2.1 say a lemma carries.
//!
//! Law 3: **derive state; never store it.** This module is how that law is kept
//! affordable — the analysis is recomputed per call from the lemma alone and
//! never cached in a struct, so there is no field to drift out of step.
//!
//! Nothing here is public. The crate's entry points take a `&str` lemma, and a
//! caller never sees a `Declension` or an `Option`, which is what keeps guard 4
//! (`no_option_no_result`) true of the public API by construction.

use crate::grammar::{Animacy, Gender};

/// The plain vowels (§2.3). The iotated series `ja je jo ju` are `j` + vowel, so
/// their vowel is the second character and this list is complete.
const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

/// Is this a Ruthenian *word*, as opposed to something the alphabet happens to
/// tolerate?
///
/// `Ruthenian::parse` guards the alphabet, and it is deliberately permissive
/// about characters that are neutral in running text — punctuation, spaces,
/// digits. A lemma is not running text, so it must clear a higher bar: letters
/// and the separator only, and **at least one vowel**, since every Ruthenian
/// syllable has one (§2.5).
///
/// Without the vowel test `noun("!", ..)` returns `"!"` — a plausible-looking
/// answer to a question that was never a word. Law 4 asks every function to be
/// total, not to pretend a non-word inflects.
fn is_word(bare: &str) -> bool {
    !bare.is_empty()
        && bare.chars().all(|c| c.is_ascii_alphabetic() || c == '\'')
        && bare
            .chars()
            .any(|c| VOWELS.contains(&c.to_ascii_lowercase()))
}

/// The three declensions of §3.2. Derived, never supplied.
///
/// Named as the specification names them, in Roman numerals, which is worth one
/// lint exemption: renaming `III` to `Three` would make every reference here
/// disagree with every reference in §3.2 through §3.6.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Declension {
    /// Nouns in `-a`, of either gender: `zzena`, `zjemlja`, `sluga'`.
    I,
    /// Masculine and neuter: `dom`, `konj`, `okno`, `polje`.
    II,
    /// Feminine ending in a consonant: `noczj'`, `kostj'`.
    III,
}

/// A citation form, read.
///
/// Held only for the length of one call. Every field is a *function* of
/// [`Nominal::stem`] plus the original spelling, and the struct exists to avoid
/// computing them four times in one paradigm cell, not to remember them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Nominal {
    /// The stem: the citation form less its nominative ending, lowercased.
    ///
    /// §2.5 makes this invariant across the whole paradigm — there is no
    /// fleeting vowel — so it is computed once and never reconsidered.
    pub stem: String,
    pub gender: Gender,
    pub declension: Declension,
    /// Soft exactly when the citation form ends in `j`, `ja` or `je` (§3.2).
    pub soft: bool,
    /// Animate exactly when the lemma's first letter is a capital (§3.7).
    pub animacy: Animacy,
}

impl Nominal {
    /// Read a noun's citation form, or `None` if it is not one.
    ///
    /// `None` means the string is not a lemma at all — unparseable under the
    /// alphabet, empty, or nothing but an ending. It never means "a lemma whose
    /// class I could not decide", because §3.2's table is total over
    /// well-formed lemmas: every ending decides its own row, and the word-final
    /// `'` supplies the one bit two of them leave open.
    pub(crate) fn read(word: &str) -> Option<Self> {
        let parsed = ruthenian_orthography::Ruthenian::parse(word).ok()?;
        let marked = parsed.is_marked();
        let bare = parsed.word();
        if !is_word(bare) {
            return None;
        }

        // §3.7: a capital first letter marks an animate noun. Read before
        // folding, because the fold is what makes the output lowercase.
        let animacy = match bare.chars().next()?.is_uppercase() {
            true => Animacy::Animate,
            false => Animacy::Inanimate,
        };
        let lower = bare.to_lowercase();

        // §3.2's table, in its own order. Each arm names the ending it matches,
        // and the `marked` arms come first because the mark is what overrides
        // the prediction the ending would otherwise make.
        let (cut, gender, declension, soft) = if marked && lower.ends_with("ja") {
            // A marked `-ja`: masculine of declension I, soft. §3.2 tabulates
            // `-a` + mark; `junosza'` shows the soft variant is possible.
            (2, Gender::Masculine, Declension::I, true)
        } else if marked && lower.ends_with('a') {
            // `sluga'` — masculine in agreement, declension I in form (§3.5).
            (1, Gender::Masculine, Declension::I, false)
        } else if marked && lower.ends_with('j') {
            // `noczj'` — feminine, declension III. The pair with `konj` is what
            // makes the mark necessary: both end in `j` and nothing else tells
            // a soft masculine II from a feminine III.
            (1, Gender::Feminine, Declension::III, false)
        } else if lower.ends_with("ja") {
            (2, Gender::Feminine, Declension::I, true)
        } else if lower.ends_with('a') {
            (1, Gender::Feminine, Declension::I, false)
        } else if lower.ends_with("je") {
            (2, Gender::Neuter, Declension::II, true)
        } else if lower.ends_with('o') {
            (1, Gender::Neuter, Declension::II, false)
        } else if lower.ends_with('j') {
            (1, Gender::Masculine, Declension::II, true)
        } else {
            // Any other consonant: masculine, II, hard. A lemma ending in a
            // bare vowel other than `a`/`o` reaches here too — a loan like
            // `taksi` — and declines as a consonant stem, which is §12.3's
            // "no indeclinables" applied literally.
            (0, Gender::Masculine, Declension::II, false)
        };

        let stem = lower[..lower.len() - cut].to_string();
        if stem.is_empty() {
            return None;
        }
        Some(Self {
            stem,
            gender,
            declension,
            soft,
            animacy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(w: &str) -> Nominal {
        Nominal::read(w).expect("a well-formed lemma")
    }

    #[test]
    fn declension_and_gender_come_from_the_ending() {
        // §3.2's table, row by row.
        let dom = read("dom");
        assert_eq!(dom.stem, "dom");
        assert_eq!(dom.gender, Gender::Masculine);
        assert_eq!(dom.declension, Declension::II);
        assert!(!dom.soft);

        let konj = read("konj");
        assert_eq!(konj.stem, "kon");
        assert_eq!(konj.gender, Gender::Masculine);
        assert!(konj.soft, "a citation form in `j` is soft");

        let okno = read("okno");
        assert_eq!(okno.stem, "okn");
        assert_eq!(okno.gender, Gender::Neuter);

        let polje = read("polje");
        assert_eq!(polje.stem, "pol", "§3.4 gives the stem as pol-");
        assert!(polje.soft);

        let zzena = read("zzena");
        assert_eq!(zzena.stem, "zzen");
        assert_eq!(zzena.gender, Gender::Feminine);
        assert_eq!(zzena.declension, Declension::I);

        let zjemlja = read("zjemlja");
        assert_eq!(zjemlja.stem, "zjeml", "§3.5 gives the stem as zjeml-");
        assert!(zjemlja.soft);

        // §3.5's vowel-final stem.
        let nacija = read("nacija");
        assert_eq!(nacija.stem, "naci");
        assert_eq!(nacija.declension, Declension::I);
        assert!(nacija.soft);
    }

    #[test]
    fn the_mark_overrides_what_the_ending_predicts() {
        // `konj` against `noczj'` — the pair the mark exists for.
        let nocz = read("noczj'");
        assert_eq!(nocz.stem, "nocz");
        assert_eq!(nocz.gender, Gender::Feminine);
        assert_eq!(nocz.declension, Declension::III);

        // `sluga'` — masculine in agreement, declension I in form.
        let sluga = read("sluga'");
        assert_eq!(sluga.stem, "slug");
        assert_eq!(sluga.gender, Gender::Masculine);
        assert_eq!(sluga.declension, Declension::I);

        // Without the mark, each is what its ending predicts.
        assert_eq!(read("konj").gender, Gender::Masculine);
        assert_eq!(read("zzena").gender, Gender::Feminine);
    }

    #[test]
    fn a_capital_marks_animacy_and_never_survives_into_the_stem() {
        let animate = read("Drug");
        assert_eq!(animate.animacy, Animacy::Animate);
        assert_eq!(animate.stem, "drug", "the stem is lowercase");

        assert_eq!(read("drug").animacy, Animacy::Inanimate);

        // Both marks at once (§3.5): an animate masculine in `-a`.
        let sluga = read("Sluga'");
        assert_eq!(sluga.animacy, Animacy::Animate);
        assert_eq!(sluga.gender, Gender::Masculine);
        assert_eq!(sluga.stem, "slug");
    }

    #[test]
    fn a_non_lemma_is_refused_rather_than_guessed() {
        for bad in ["", "'", "''", "дом", "quiz", "a", "ja", "o"] {
            assert!(
                Nominal::read(bad).is_none(),
                "{bad:?} is not a well-formed citation form"
            );
        }
    }
}
