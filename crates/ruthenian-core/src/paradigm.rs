//! Whole-paradigm structs: one call returns a complete table.
//!
//! The single-cell functions ([`noun()`](crate::noun()), [`verb()`](crate::verb()),
//! [`adjective()`](crate::adjective())) are the primitive; these are the shape a consumer
//! actually wants. `ruth paradigm dom` prints one of these, and the conformance
//! evaluator walks one rather than reconstructing the cell list itself.
//!
//! This is `interslavic-core`'s `noun_forms`/`verb_forms` pattern, adopted for
//! the reason that crate adopted it: **returning the parts, not a string for the
//! caller to parse** (law 12). A consumer that has to enumerate cases and
//! numbers for itself will eventually enumerate them differently.
//!
//! # Ordering is API
//!
//! Cells are stored in [`Case::ALL`] × [`Number::ALL`] order — the order
//! `RUTHENIAN.md`'s own tables use. Consumers will bless a printed table into
//! their expectations, so reordering is a breaking change and is announced as
//! one (`DIRECTION.md`, "Ordering is API").

use crate::types::{
    AdjForm, Animacy, Case, Degree, Gender, NounClass, Number, Person, PersonNumber, Tense,
    VerbClass, VerbSlot,
};
use crate::variant::Prediction;
use crate::verb::{Resolved, Unsupported, VerbInfo};

/// A noun's full table: 8 cases × 3 numbers.
///
/// Every cell of a Ruthenian noun paradigm exists (§3.9 removes number
/// defectiveness and indeclinables), so in practice no cell is `None` — but the
/// `Option` is kept so a future declared gap is representable rather than
/// papered over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounParadigm {
    pub lemma: String,
    pub class: NounClass,
    pub gender: Gender,
    pub animacy: Animacy,
    cells: Vec<Option<Prediction>>,
}

impl NounParadigm {
    fn index(case: Case, number: Number) -> usize {
        let c = Case::ALL.iter().position(|x| *x == case).unwrap();
        let n = Number::ALL.iter().position(|x| *x == number).unwrap();
        c * Number::ALL.len() + n
    }

    /// One cell. Identical to calling [`noun()`](crate::noun()) directly — the whole point
    /// is that there is one implementation, not two (law 1).
    pub fn get(&self, case: Case, number: Number) -> Option<&Prediction> {
        self.cells[Self::index(case, number)].as_ref()
    }

    /// Every cell in specification order, with its coordinates.
    pub fn iter(&self) -> impl Iterator<Item = (Case, Number, Option<&Prediction>)> {
        Case::ALL.into_iter().flat_map(move |case| {
            Number::ALL
                .into_iter()
                .map(move |number| (case, number, self.get(case, number)))
        })
    }

    /// Distinct surface forms **within each number, summed** — the count §11
    /// tabulates: 15 for `dom`, 14 for `okno` and `zzena`, 11 for `noczj`.
    ///
    /// This is the honest measure of a paradigm's size, because 24 cells never
    /// yield 24 forms: the dual contributes three regardless of how many cases
    /// exist, and the ablative contributes one, in two paradigms out of four.
    /// That is the whole design — conservatism in the inventory, regularity in
    /// the realization.
    pub fn distinct_forms(&self) -> usize {
        Number::ALL
            .into_iter()
            .map(|number| {
                let mut seen: Vec<&str> = Case::ALL
                    .into_iter()
                    .filter_map(|case| self.get(case, number).map(|p| p.text.as_str()))
                    .collect();
                seen.sort_unstable();
                seen.dedup();
                seen.len()
            })
            .sum()
    }

    /// Distinct forms across the **whole** paradigm, ignoring number.
    ///
    /// Lower than [`distinct_forms`](Self::distinct_forms), and the gap is
    /// meaningful rather than an inconsistency: some forms are shared between
    /// numbers. `doma` is both ablative singular and nominative dual, `domu`
    /// both dative singular and genitive/locative dual, `domom` both
    /// instrumental singular and dative plural.
    ///
    /// §3.3 addresses the first of those directly — it is inherited, not a
    /// defect, because OCS has exactly the same collision (genitive singular
    /// `-a` = nominative dual `-a`), both continuing different PIE endings that
    /// fell together regularly. Agreement disambiguates: a dual noun takes dual
    /// modifiers and a dual verb.
    pub fn distinct_forms_overall(&self) -> usize {
        let mut seen: Vec<&str> = self
            .cells
            .iter()
            .filter_map(|c| c.as_ref().map(|p| p.text.as_str()))
            .collect();
        seen.sort_unstable();
        seen.dedup();
        seen.len()
    }
}

/// Build a noun's full paradigm.
///
/// ```
/// use ruthenian_core::{noun_forms, Animacy, Case, Declension, Gender, NounClass, Number};
///
/// let p = noun_forms("dom", NounClass::hard(Declension::II), Gender::Masculine, Animacy::Inanimate);
/// assert_eq!(p.get(Case::Gen, Number::Singular).unwrap().text, "domogo");
/// assert_eq!(p.get(Case::Abl, Number::Singular).unwrap().text, "doma");
/// assert_eq!(p.get(Case::Dat, Number::Dual).unwrap().text, "domoma");
///
/// // 24 cells, but §11 says fifteen distinct forms — the dual and the ablative
/// // are heavily syncretic by design, which is why adding them costs so little.
/// assert_eq!(p.iter().count(), 24);
/// assert_eq!(p.distinct_forms(), 15);
///
/// // Twelve across the whole paradigm: three forms are shared between numbers,
/// // `doma` (abl sg = nom du) being the collision §3.3 calls inherited.
/// assert_eq!(p.distinct_forms_overall(), 12);
/// ```
pub fn noun_forms(lemma: &str, class: NounClass, gender: Gender, animacy: Animacy) -> NounParadigm {
    let stem = crate::noun::stem_of(lemma);
    let mut cells = Vec::with_capacity(Case::ALL.len() * Number::ALL.len());
    for case in Case::ALL {
        for number in Number::ALL {
            cells.push(crate::noun(&stem, class, gender, animacy, case, number));
        }
    }
    NounParadigm {
        lemma: lemma.to_string(),
        class,
        gender,
        animacy,
        cells,
    }
}

/// An adjective's full table, for one form and degree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjParadigm {
    pub lemma: String,
    pub form: AdjForm,
    pub degree: Degree,
    cells: Vec<Option<Prediction>>,
}

impl AdjParadigm {
    fn index(case: Case, number: Number, gender: Gender) -> usize {
        let c = Case::ALL.iter().position(|x| *x == case).unwrap();
        let n = Number::ALL.iter().position(|x| *x == number).unwrap();
        let g = Gender::ALL.iter().position(|x| *x == gender).unwrap();
        (c * Number::ALL.len() + n) * Gender::ALL.len() + g
    }

    pub fn get(&self, case: Case, number: Number, gender: Gender) -> Option<&Prediction> {
        self.cells[Self::index(case, number, gender)].as_ref()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Case, Number, Gender, Option<&Prediction>)> {
        Case::ALL.into_iter().flat_map(move |case| {
            Number::ALL.into_iter().flat_map(move |number| {
                Gender::ALL
                    .into_iter()
                    .map(move |gender| (case, number, gender, self.get(case, number, gender)))
            })
        })
    }
}

/// Build an adjective's full paradigm for one form and degree.
///
/// ```
/// use ruthenian_core::{adj_forms, AdjForm, Case, Degree, Gender, Number};
///
/// let short = adj_forms("dobr", AdjForm::Short, Degree::Positive);
/// let long = adj_forms("dobr", AdjForm::Long, Degree::Positive);
///
/// assert_eq!(short.get(Case::Nom, Number::Singular, Gender::Masculine).unwrap().text, "dobr");
/// assert_eq!(long.get(Case::Nom, Number::Singular, Gender::Masculine).unwrap().text, "dobryj");
///
/// // the long form has no vocative; the short one does
/// assert!(long.get(Case::Voc, Number::Singular, Gender::Masculine).is_none());
/// assert_eq!(short.get(Case::Voc, Number::Singular, Gender::Masculine).unwrap().text, "dobrje");
/// ```
pub fn adj_forms(lemma: &str, form: AdjForm, degree: Degree) -> AdjParadigm {
    let mut cells = Vec::new();
    for case in Case::ALL {
        for number in Number::ALL {
            for gender in Gender::ALL {
                cells.push(crate::adjective(
                    lemma,
                    case,
                    number,
                    gender,
                    Animacy::Inanimate,
                    form,
                    degree,
                ));
            }
        }
    }
    AdjParadigm {
        lemma: lemma.to_string(),
        form,
        degree,
        cells,
    }
}

/// A verb's finite forms: the synthetic tenses × 9 person/number combinations.
///
/// The perfect and pluperfect are absent by construction, not by omission: they
/// are the l-participle plus a copula (§7.7), so they are phrases rather than
/// cells and belong to whatever composes them. Fusing them here would create a
/// second way to build the same form.
#[derive(Debug, Clone)]
pub struct VerbParadigm {
    pub lemma: String,
    pub class: VerbClass,
    pub info: VerbInfo,
    /// The synthetic tenses, in `Tense::ALL` order minus the periphrastic ones.
    pub tenses: Vec<Tense>,
    cells: Vec<Option<Prediction>>,
    /// The l-participle, which the periphrastic tenses are built from.
    pub l_participle: Vec<Option<Prediction>>,
}

impl VerbParadigm {
    /// The tenses a verb paradigm actually tabulates.
    pub fn synthetic_tenses() -> Vec<Tense> {
        Tense::ALL
            .into_iter()
            .filter(|t| !t.is_periphrastic())
            .collect()
    }

    pub fn get(&self, tense: Tense, person: Person, number: Number) -> Option<&Prediction> {
        let t = self.tenses.iter().position(|x| *x == tense)?;
        let pn = PersonNumber::ALL
            .iter()
            .position(|x| *x == PersonNumber::of(person, number))?;
        self.cells[t * PersonNumber::ALL.len() + pn].as_ref()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Tense, Person, Number, Option<&Prediction>)> {
        self.tenses.clone().into_iter().flat_map(move |tense| {
            [Person::First, Person::Second, Person::Third]
                .into_iter()
                .flat_map(move |person| {
                    Number::ALL
                        .into_iter()
                        .map(move |number| (tense, person, number, self.get(tense, person, number)))
                })
        })
    }
}

/// Build a verb's finite paradigm.
///
/// ```
/// use ruthenian_core::{verb_forms, Number, Person, Tense, VerbClass, VerbInfo};
///
/// let p = verb_forms("czitatj", VerbClass::One, VerbInfo::default()).unwrap();
///
/// assert_eq!(p.get(Tense::Present, Person::First, Number::Singular).unwrap().text, "czitaju");
/// assert_eq!(p.get(Tense::Present, Person::First, Number::Dual).unwrap().text, "czitajevje");
/// assert_eq!(p.get(Tense::Aorist,  Person::Second, Number::Singular).unwrap().text, "czita");
/// assert_eq!(p.get(Tense::Imperfect, Person::Second, Number::Singular).unwrap().text, "czitajasze");
///
/// // an imperfective has no synthetic future: `budu czitatj` is a phrase
/// assert!(p.get(Tense::Future, Person::First, Number::Singular).is_none());
/// ```
pub fn verb_forms(
    lemma: &str,
    class: VerbClass,
    info: VerbInfo,
) -> Result<VerbParadigm, Unsupported> {
    let tenses = VerbParadigm::synthetic_tenses();
    let mut cells = Vec::new();
    for tense in &tenses {
        for pn in PersonNumber::ALL {
            let (person, number) = decompose(pn);
            let slot = VerbSlot::Finite {
                person,
                number,
                tense: *tense,
            };
            cells.push(resolve(lemma, class, info, slot)?);
        }
    }
    let mut l_participle = Vec::new();
    for number in Number::ALL {
        for gender in Gender::ALL {
            let slot = VerbSlot::LParticiple {
                gender: Some(gender),
                number,
            };
            l_participle.push(resolve(lemma, class, info, slot)?);
        }
    }
    Ok(VerbParadigm {
        lemma: lemma.to_string(),
        class,
        info,
        tenses,
        cells,
        l_participle,
    })
}

fn resolve(
    lemma: &str,
    class: VerbClass,
    info: VerbInfo,
    slot: VerbSlot,
) -> Result<Option<Prediction>, Unsupported> {
    let r: Resolved = crate::verb(lemma, class, info, slot);
    r
}

fn decompose(pn: PersonNumber) -> (Person, Number) {
    use PersonNumber::*;
    match pn {
        S1 => (Person::First, Number::Singular),
        S2 => (Person::Second, Number::Singular),
        S3 => (Person::Third, Number::Singular),
        D1 => (Person::First, Number::Dual),
        D2 => (Person::Second, Number::Dual),
        D3 => (Person::Third, Number::Dual),
        P1 => (Person::First, Number::Plural),
        P2 => (Person::Second, Number::Plural),
        P3 => (Person::Third, Number::Plural),
    }
}
