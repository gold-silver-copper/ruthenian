//! Nouns: §3.3–§3.6, all eight cases in all three numbers.
//!
//! The **hard** endings below are transcribed cell by cell from the
//! specification, which is normative. The soft series is not transcribed: §3.2
//! gives it as `j` + the hard ending, with `o` written `e` after the `j`, and
//! [`soften`] is that sentence. The vocative singular is the one cell the rule
//! does not reach (`domje` against `konju`), and §3.1 states those two endings
//! outright.
//!
//! Two tables were the earlier arrangement, on the reasoning that a rule right
//! in fourteen cells and wrong in one is harder to check than a transcription.
//! That was wrong twice over: the exception is two cells and stated, and while
//! both series were written out they *did* drift — the same softened `-y` was
//! spelled `-i` in §3.5 and `-ji` in §3.3, which nothing caught.

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
    let Some((ending, palatal)) = ending(set, n.soft, case, number) else {
        return UNREADABLE.to_string();
    };
    join(&palatalize(&n.stem, palatal), &ending)
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
            // The dual accusative is the nominative in every paradigm (§3.3).
            (_, Dual, _) => Nominative,
            // §3.7: the singular's oblique accusative belongs to **declension
            // II masculine alone**. §3.5 states it outright for `sluga'`
            // ("vizzu slugu — declension I accusative -u, not the masculine
            // ablative"), the neuter keeps its nominative (`vizzu okno`, as
            // every Slavic language has it), and declension III patterns with
            // the other feminines (`vizzu myszj`, Russian `вижу мышь`).
            (Set::I, Singular, _) => Accusative,
            (Set::IiNeuter | Set::Iii, Singular, _) => Nominative,
            (Set::IiMasculine, Singular, Animacy::Animate) => Ablative,
            // §3.7: the plural genitive-accusative reaches **every** animate
            // noun, the neuter included — the adjective's one plural column
            // could not agree with a neuter noun that opted out.
            (_, Plural, Animacy::Animate) => Genitive,
            (_, _, Animacy::Inanimate) => Nominative,
        };
    }
    case
}

/// The **soft** counterpart of a hard ending (§3.2).
///
/// One rule: `j` + the hard ending, with `o` written `e` after the `j`. That
/// replaces the three substitutions §3.2 used to list — `je` for `o`, `ju` for
/// `u`, `i` for `y` — with a single operation, and it is why the soft series
/// needs no table of its own.
///
/// ```text
/// ""  -> j        ogo -> jego     oj -> jej      y -> jy
/// a   -> ja       om  -> jem      i  -> ji       ov -> jev
/// u   -> ju       oma -> jema     o  -> je       am -> jam
/// ```
///
/// The vocative singular is the one cell it does not reach; §3.1 states those
/// endings outright (`konju`, `zjemljo`) and they are not derivable from the
/// hard ones.
fn soften(hard: &str) -> String {
    match hard.strip_prefix('o') {
        _ if hard.is_empty() => "j".to_string(),
        Some(rest) => format!("je{rest}"),
        None => format!("j{hard}"),
    }
}

crate::dsl::paradigm! {
    /// §3.3 — declension II masculine, hard series. `dom`, `drug`.
    ///
    /// Only the hard series is tabulated, here and below; [`soften`] derives
    /// the soft one. That is not a space saving — it is what stops the two
    /// drifting apart, which they did while both were written out.
    ///
    /// A `Second` goes with a **yat-derived** `-i` and nothing else (§3.8 rule
    /// 5). An `-i` that is merely rule 1's respelling of `-y` — the velar
    /// nominative plural `drugi`, the genitive `knigi` — is not yat-derived and
    /// must not palatalize; that distinction is the whole reason `knigi`
    /// (genitive) and `knizi` (dative) differ. The vocative `-je` is the one
    /// first-palatalization environment in the nominal system: `druzze`,
    /// `otjecze`.
    pub const II_MASCULINE = [
        //   singular        dual      plural
        nom: "",             "a",      "y";
        voc: (First "je"),   -,        -;
        acc: -,              -,        -;
        gen: "ogo",          "u",      "ov";
        abl: "a",            -,        -;
        dat: "u",            "oma",    "om";
        ins: "om",           "oma",    "ami";
        loc: (Second "i"),   "u",      "ah";
    ];

    /// §3.4 — declension II neuter. `okno`.
    ///
    /// The dual `-i` continues OCS `-ě` (`dvje selje`), so it is yat-derived
    /// and palatalizes.
    pub const II_NEUTER = [
        //   singular        dual           plural
        nom: "o",            (Second "i"),  "a";
        voc: "o",            -,             -;
        acc: -,              -,             -;
        gen: "ogo",          "u",           "ov";
        abl: "a",            -,             -;
        dat: "u",            "oma",         "om";
        ins: "om",           "oma",         "ami";
        loc: (Second "i"),   "u",           "ah";
    ];

    /// §3.5 — declension I. `zzena`, `kniga`, and (soft) `zjemlja`.
    ///
    /// Genitive `-y` is not yat: `kniga` -> `knigi` by rule 1 alone, against
    /// the dative/locative `knizi`. Declension I keeps its own accusative
    /// (`slugu`), so it is the one table with an `acc` cell.
    pub const I = [
        //   singular        dual           plural
        nom: "a",            (Second "i"),  "y";
        voc: "o",            -,             -;
        acc: "u",            -,             -;
        gen: "y",            "u",           "ov";
        abl: "y",            -,             -;
        dat: (Second "i"),   "ama",         "am";
        ins: "oj",           "ama",         "ami";
        loc: (Second "i"),   "u",           "ah";
    ];

    /// §3.6 — declension III, the inherited PIE i-stem. `noczj'`, `kostj'`.
    ///
    /// No hard/soft pair: its endings are its own, and [`soften`] is never
    /// applied to them. The plural nominative `-i` is the i-stem's own vowel,
    /// not yat, so it does not palatalize — unlike the dual's.
    pub const III = [
        //   singular        dual           plural
        nom: "j",            (Second "i"),  "i";
        voc: "i",            -,             -;
        acc: -,              -,             -;
        gen: "i",            "ju",          "jev";
        abl: "i",            -,             -;
        dat: (Second "i"),   "jma",         "jam";
        ins: "jju",          "jma",         "jami";
        loc: (Second "i"),   "ju",          "jah";
    ];
}

/// The table a set declines by.
fn table(set: Set) -> &'static crate::dsl::Paradigm {
    match set {
        Set::IiMasculine => &II_MASCULINE,
        Set::IiNeuter => &II_NEUTER,
        Set::I => &I,
        Set::Iii => &III,
    }
}

/// The ending for one cell, hard or soft.
///
/// §3.1's soft vocatives are the only cells [`soften`] cannot reach. `None` is
/// a resolved case landing on a `-` cell, which the `paradigm_totality` guard
/// proves never happens — the caller's fallback is declared, not exercised.
fn ending(set: Set, soft: bool, case: Case, number: Number) -> Option<(String, Palatal)> {
    let (hard, palatal) = table(set).cell(case, number)?;
    if !soft || set == Set::Iii {
        return Some((hard.to_string(), palatal));
    }
    if case == Case::Vocative && number == Number::Singular {
        let voc = match set {
            Set::IiMasculine => "ju",
            Set::I => "jo",
            // The neuter vocative is the nominative, so it never arrives here.
            _ => return Some((soften(hard), palatal)),
        };
        return Some((voc.to_string(), Palatal::None));
    }
    Some((soften(hard), palatal))
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
) -> Option<(&'static str, crate::spelling::Palatal)> {
    let set = match gender {
        Gender::Masculine => Set::IiMasculine,
        Gender::Neuter => Set::IiNeuter,
        Gender::Feminine => Set::I,
    };
    table(set).cell(resolve(set, case, number, animacy), number)
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
