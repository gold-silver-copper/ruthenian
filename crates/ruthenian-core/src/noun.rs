//! Nouns: §3.3–§3.6, all eight cases in all three numbers.
//!
//! The endings below are transcribed cell by cell from the specification, which
//! is normative. They are deliberately *not* derived from one another — the soft
//! series is very nearly `j` + the hard series, but the vocative is not
//! (`domje` against `konju`), and a transformation that is right in fourteen
//! cells and wrong in one is harder to check than two tables.

use crate::fallback::UNREADABLE;
use crate::grammar::{Animacy, Case, Gender, Number};
use crate::lemma::{Declension, Nominal};
use crate::spelling::{Palatal, join, palatalize};

/// Which of the four ending sets a lemma takes.
///
/// Declension II splits by gender and the other two do not, so this is the
/// declension crossed with just the distinction that matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Set {
    /// §3.3 — `dom`, `konj`, `drug`.
    IiMasculine,
    /// §3.4 — `okno`, `polje`.
    IiNeuter,
    /// §3.5 — `zzena`, `kniga`, `zjemlja`, `nacija`, `sluga'`.
    I,
    /// §3.6 — `noczj'`, `kostj'`.
    Iii,
}

/// Inflect a noun.
///
/// The lemma carries its own gender, animacy, declension and hardness (§2.1,
/// §3.2), so nothing else is supplied.
///
/// ```
/// use ruthenian_core::{noun, Case, Number};
///
/// // §3.3, the hard masculine
/// assert_eq!(noun("dom", Case::Nominative, Number::Singular), "dom");
/// assert_eq!(noun("dom", Case::Vocative, Number::Singular), "domje");
/// assert_eq!(noun("dom", Case::Genitive, Number::Singular), "domogo");
/// assert_eq!(noun("dom", Case::Ablative, Number::Singular), "doma");
/// assert_eq!(noun("dom", Case::Locative, Number::Singular), "domi");
/// assert_eq!(noun("dom", Case::Nominative, Number::Plural), "domy");
///
/// // The velar stem distinguishes the vocative from the locative twice over —
/// // first palatalization against second (§3.1).
/// assert_eq!(noun("Drug", Case::Vocative, Number::Singular), "druzze");
/// assert_eq!(noun("Drug", Case::Locative, Number::Singular), "druzi");
///
/// // §3.6, and the mark that makes it feminine rather than a soft masculine
/// assert_eq!(noun("noczj'", Case::Instrumental, Number::Singular), "noczjju");
/// assert_eq!(noun("konj", Case::Instrumental, Number::Singular), "konjem");
/// ```
pub fn noun(word: &str, case: Case, number: Number) -> String {
    let Some(n) = Nominal::read(word) else {
        return UNREADABLE.to_string();
    };
    let set = match (n.declension, n.gender) {
        (Declension::I, _) => Set::I,
        (Declension::III, _) => Set::Iii,
        (Declension::II, Gender::Neuter) => Set::IiNeuter,
        (Declension::II, _) => Set::IiMasculine,
    };
    let case = resolve(set, case, number, n.animacy);
    let (ending, palatal) = ending(set, n.soft, case, number);
    join(&palatalize(&n.stem, palatal), ending)
}

/// Collapse the syncretisms so the table below has one row per *form*.
///
/// §3.1: the vocative is singular-only (the nominative is used elsewhere), the
/// dual has three forms (nom=voc=acc, gen=loc, dat=ins=abl), and the ablative
/// merges with the dative in the dual and plural. §3.7 then puts an animate
/// accusative on the ablative in the singular and the genitive in the plural.
pub(crate) fn resolve(set: Set, case: Case, number: Number, animacy: Animacy) -> Case {
    use Case::*;
    use Number::*;

    // §3.1: no vocative plural, and the dual vocative is the nominative.
    if case == Vocative && number != Singular {
        return Nominative;
    }
    // §3.1: the ablative is distinct only in the singular.
    if case == Ablative && number != Singular {
        return Dative;
    }
    if case == Accusative {
        return match (set, number, animacy) {
            // The neuter accusative is the nominative throughout (§3.4).
            (Set::IiNeuter, _, _) => Nominative,
            // The dual accusative is the nominative in every paradigm (§3.3).
            (_, Dual, _) => Nominative,
            // §3.5, stated outright for `sluga'`: "vizzu slugu — declension I
            // accusative -u, not the masculine ablative". Declension I has its
            // own accusative singular, so animacy cannot reach it.
            (Set::I, Singular, _) => Accusative,
            // §3.7: animate accusative = ablative in the singular, genitive in
            // the plural. The plural has no distinct ablative, so the genitive
            // is the oblique the paradigm makes available.
            (_, Singular, Animacy::Animate) => Ablative,
            (_, Plural, Animacy::Animate) => Genitive,
            (_, _, Animacy::Inanimate) => Nominative,
        };
    }
    case
}

/// The ending for one resolved cell, and the palatalization it triggers.
///
/// The `Second` palatalization goes with a **yat-derived** `-i` and nothing else
/// (§3.8 rule 5): the locative singular, the feminine dative singular, and the
/// neuter and feminine dual. An `-i` that is merely rule 1's respelling of `-y`
/// — the velar nominative plural `drugi`, the genitive `knigi` — is not
/// yat-derived and must not palatalize. That distinction is the whole reason
/// `knigi` (genitive) and `knizi` (dative) differ.
pub(crate) fn ending(set: Set, soft: bool, case: Case, number: Number) -> (&'static str, Palatal) {
    use Case::*;
    use Number::*;
    let n = Palatal::None;

    match set {
        // ---- §3.3, declension II masculine -----------------------------------
        Set::IiMasculine if !soft => match (case, number) {
            (Nominative, Singular) => ("", n),
            // The vocative `-je` is the one first-palatalization environment in
            // the nominal system: `drug` -> `druzze`, `otjec` -> `otjecze`.
            (Vocative, Singular) => ("je", Palatal::First),
            (Accusative, Singular) => ("", n),
            (Genitive, Singular) => ("ogo", n),
            (Ablative, Singular) => ("a", n),
            (Dative, Singular) => ("u", n),
            (Instrumental, Singular) => ("om", n),
            (Locative, Singular) => ("i", Palatal::Second),
            (Nominative | Vocative | Accusative, Dual) => ("a", n),
            (Genitive | Locative, Dual) => ("u", n),
            (Dative | Instrumental | Ablative, Dual) => ("oma", n),
            (Nominative | Vocative | Accusative, Plural) => ("y", n),
            (Genitive, Plural) => ("ov", n),
            (Dative | Ablative, Plural) => ("om", n),
            (Instrumental, Plural) => ("ami", n),
            (Locative, Plural) => ("ah", n),
        },
        Set::IiMasculine => match (case, number) {
            (Nominative, Singular) => ("j", n),
            (Vocative, Singular) => ("ju", n),
            (Accusative, Singular) => ("j", n),
            (Genitive, Singular) => ("jego", n),
            (Ablative, Singular) => ("ja", n),
            (Dative, Singular) => ("ju", n),
            (Instrumental, Singular) => ("jem", n),
            (Locative, Singular) => ("ji", n),
            (Nominative | Vocative | Accusative, Dual) => ("ja", n),
            (Genitive | Locative, Dual) => ("ju", n),
            (Dative | Instrumental | Ablative, Dual) => ("jema", n),
            (Nominative | Vocative | Accusative, Plural) => ("ji", n),
            (Genitive, Plural) => ("jev", n),
            (Dative | Ablative, Plural) => ("jem", n),
            (Instrumental, Plural) => ("jami", n),
            (Locative, Plural) => ("jah", n),
        },

        // ---- §3.4, declension II neuter --------------------------------------
        Set::IiNeuter if !soft => match (case, number) {
            (Nominative | Vocative | Accusative, Singular) => ("o", n),
            (Genitive, Singular) => ("ogo", n),
            (Ablative, Singular) => ("a", n),
            (Dative, Singular) => ("u", n),
            (Instrumental, Singular) => ("om", n),
            (Locative, Singular) => ("i", Palatal::Second),
            // The neuter dual `-i` continues OCS `-ě` (`dvě selě`), so it is
            // yat-derived and palatalizes.
            (Nominative | Vocative | Accusative, Dual) => ("i", Palatal::Second),
            (Genitive | Locative, Dual) => ("u", n),
            (Dative | Instrumental | Ablative, Dual) => ("oma", n),
            (Nominative | Vocative | Accusative, Plural) => ("a", n),
            (Genitive, Plural) => ("ov", n),
            (Dative | Ablative, Plural) => ("om", n),
            (Instrumental, Plural) => ("ami", n),
            (Locative, Plural) => ("ah", n),
        },
        Set::IiNeuter => match (case, number) {
            (Nominative | Vocative | Accusative, Singular) => ("je", n),
            (Genitive, Singular) => ("jego", n),
            (Ablative, Singular) => ("ja", n),
            (Dative, Singular) => ("ju", n),
            (Instrumental, Singular) => ("jem", n),
            (Locative, Singular) => ("ji", n),
            (Nominative | Vocative | Accusative, Dual) => ("ji", n),
            (Genitive | Locative, Dual) => ("ju", n),
            (Dative | Instrumental | Ablative, Dual) => ("jema", n),
            (Nominative | Vocative | Accusative, Plural) => ("ja", n),
            (Genitive, Plural) => ("jev", n),
            (Dative | Ablative, Plural) => ("jem", n),
            (Instrumental, Plural) => ("jami", n),
            (Locative, Plural) => ("jah", n),
        },

        // ---- §3.5, declension I ----------------------------------------------
        Set::I if !soft => match (case, number) {
            (Nominative, Singular) => ("a", n),
            (Vocative, Singular) => ("o", n),
            (Accusative, Singular) => ("u", n),
            // Genitive `-y` is not yat: `kniga` -> `knigi` by rule 1 alone,
            // against the dative/locative `knizi`.
            (Genitive | Ablative, Singular) => ("y", n),
            (Dative, Singular) => ("i", Palatal::Second),
            (Instrumental, Singular) => ("oj", n),
            (Locative, Singular) => ("i", Palatal::Second),
            (Nominative | Vocative | Accusative, Dual) => ("i", Palatal::Second),
            (Genitive | Locative, Dual) => ("u", n),
            (Dative | Instrumental | Ablative, Dual) => ("ama", n),
            (Nominative | Vocative | Accusative, Plural) => ("y", n),
            (Genitive, Plural) => ("ov", n),
            (Dative | Ablative, Plural) => ("am", n),
            (Instrumental, Plural) => ("ami", n),
            (Locative, Plural) => ("ah", n),
        },
        Set::I => match (case, number) {
            (Nominative, Singular) => ("ja", n),
            (Vocative, Singular) => ("jo", n),
            (Accusative, Singular) => ("ju", n),
            (Genitive | Ablative, Singular) => ("i", n),
            (Dative, Singular) => ("ji", n),
            (Instrumental, Singular) => ("joj", n),
            (Locative, Singular) => ("i", n),
            (Nominative | Vocative | Accusative, Dual) => ("ji", n),
            (Genitive | Locative, Dual) => ("ju", n),
            (Dative | Instrumental | Ablative, Dual) => ("jama", n),
            (Nominative | Vocative | Accusative, Plural) => ("i", n),
            (Genitive, Plural) => ("jev", n),
            (Dative | Ablative, Plural) => ("jam", n),
            (Instrumental, Plural) => ("jami", n),
            (Locative, Plural) => ("jah", n),
        },

        // ---- §3.6, declension III --------------------------------------------
        // The inherited PIE i-stem. Its singular is heavily syncretic (`-i` for
        // genitive, ablative, dative and locative) as it is in Russian,
        // Ukrainian and OCS alike, and the instrumental keeps the soft sign
        // *and* takes the ending, as in Russian `ночью`.
        Set::Iii => match (case, number) {
            (Nominative | Accusative, Singular) => ("j", n),
            (Vocative, Singular) => ("i", n),
            (Genitive | Ablative, Singular) => ("i", n),
            (Dative, Singular) => ("i", Palatal::Second),
            (Instrumental, Singular) => ("jju", n),
            (Locative, Singular) => ("i", Palatal::Second),
            (Nominative | Vocative | Accusative, Dual) => ("i", Palatal::Second),
            (Genitive | Locative, Dual) => ("ju", n),
            (Dative | Instrumental | Ablative, Dual) => ("jma", n),
            (Nominative | Vocative | Accusative, Plural) => ("i", n),
            (Genitive, Plural) => ("jev", n),
            (Dative | Ablative, Plural) => ("jam", n),
            (Instrumental, Plural) => ("jami", n),
            (Locative, Plural) => ("jah", n),
        },
    }
}

/// The nominal declension, keyed by the gender it agrees in.
///
/// §4.1: the short adjective's "endings are the noun's, **exactly** — including
/// the animacy syncretism, which belongs to the nominal declension rather than
/// to nouns as a word class". This is that entry point, and it is why
/// `short_adjective` restates nothing: it *is* this table.
///
/// Adjective stems are always hard — §1 removes soft adjective stems — so there
/// is no hardness parameter.
pub(crate) fn nominal(
    gender: Gender,
    case: Case,
    number: Number,
    animacy: Animacy,
) -> (&'static str, crate::spelling::Palatal) {
    let set = match gender {
        Gender::Masculine => Set::IiMasculine,
        Gender::Neuter => Set::IiNeuter,
        Gender::Feminine => Set::I,
    };
    ending(set, false, resolve(set, case, number, animacy), number)
}

/// A noun with its lexical facts bound, so the per-call signature is the grammar
/// alone.
///
/// Holds the lemma and nothing else — law 3. Gender, animacy, declension,
/// hardness and stem are all recomputed from it, because a stored copy is a
/// field that can drift.
///
/// ```
/// use ruthenian_core::{Noun, Case, Number};
///
/// let dom = Noun::new("dom");
/// assert_eq!(dom.form(Case::Genitive, Number::Singular), "domogo");
/// assert_eq!(dom.form(Case::Nominative, Number::Dual), "doma");
/// assert_eq!(dom.paradigm().len(), 24);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Noun {
    lemma: String,
}

impl Noun {
    /// Bind a lemma.
    ///
    /// ```
    /// use ruthenian_core::Noun;
    /// assert_eq!(Noun::new("Drug").lemma(), "Drug");
    /// ```
    pub fn new(lemma: &str) -> Self {
        Self {
            lemma: lemma.to_string(),
        }
    }

    /// The lemma as given, marks and capitalisation intact.
    ///
    /// Both are morphology (§2.1), so neither is normalised away: `Sluga'` is an
    /// animate masculine of declension I and all three facts are in the string.
    ///
    /// ```
    /// use ruthenian_core::Noun;
    /// assert_eq!(Noun::new("Sluga'").lemma(), "Sluga'");
    /// ```
    pub fn lemma(&self) -> &str {
        &self.lemma
    }

    /// One cell. The same code path as [`noun`], because it *is* [`noun`].
    ///
    /// ```
    /// use ruthenian_core::{Noun, Case, Number};
    /// assert_eq!(Noun::new("zzena").form(Case::Vocative, Number::Singular), "zzeno");
    /// ```
    pub fn form(&self, case: Case, number: Number) -> String {
        noun(&self.lemma, case, number)
    }

    /// Every cell of the paradigm: 8 cases × 3 numbers, in §3.1's order.
    ///
    /// Law 2 — this calls [`Noun::form`] rather than computing anything, so the
    /// two can never disagree. Syncretism is visible as repeated forms rather
    /// than hidden by omission: `doma` appears as both the ablative singular and
    /// the nominative dual, which is inherited from OCS and not a defect.
    ///
    /// ```
    /// use ruthenian_core::{Noun, Case, Number};
    ///
    /// let dom = Noun::new("dom");
    /// let table = dom.paradigm();
    /// assert_eq!(table.len(), 24);
    /// assert!(table.contains(&(Case::Ablative, Number::Singular, "doma".to_string())));
    /// assert!(table.contains(&(Case::Nominative, Number::Dual, "doma".to_string())));
    /// ```
    pub fn paradigm(&self) -> Vec<(Case, Number, String)> {
        let mut out = Vec::with_capacity(24);
        for number in Number::ALL {
            for case in Case::ALL {
                out.push((case, number, self.form(case, number)));
            }
        }
        out
    }
}
