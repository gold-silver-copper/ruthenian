//! Verb conjugation, driven by the Zaliznyak class.
//!
//! # Structural gaps are derived, never read
//!
//! A perfective verb has **no present tense** — its non-past morphology realizes
//! the future — no present participles and no present gerund; an intransitive
//! verb has no passive participle. In the source data these appear as `"-"`
//! forms, and they are the overwhelming majority of them: 13 922 gap slots
//! across 1 459 perfective verbs against 2 509 across 1 477 imperfective ones.
//!
//! They are grammar, not defectiveness, so this module computes them from
//! `(aspect, transitivity, slot)` and returns `Ok(None)`. Filling them would
//! invent a present tense for perfective verbs and destroy the aspect
//! distinction.
//!
//! Genuine lexical defectiveness is a different, much smaller thing — the source
//! marks it with an explicit override (`pobjeditj` carries `futr_1sg: "-"`), it
//! belongs to the lexicon, and only [`crate::policy::GAP_FILL_DEFECTIVE_1SG`]
//! may fill it.

use crate::class::{Conjugation, ZaliznyakVerbClass};
use crate::phono;
use crate::policy::{Prediction, Trace};
use crate::types::{
    Aspect, Gender, ParticipleKind, PersonNumber, PrincipalPartsRef, Tense, VerbSlot, Voice,
};

/// The engine could not answer, and says so rather than guessing.
///
/// Distinct from `Ok(None)`, which means the cell does not exist. This is "the
/// rules do not cover this", and it is always an error the caller can act on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unsupported {
    pub reason: &'static str,
    pub class: String,
}

impl core::fmt::Display for Unsupported {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "class {:?}: {}", self.class, self.reason)
    }
}

impl core::error::Error for Unsupported {}

/// `Ok(Some)` = a form. `Ok(None)` = the cell does not exist. `Err` = the rules
/// do not cover this class.
pub type Resolved = Result<Option<Prediction>, Unsupported>;

/// Everything about a verb that is not its stem.
#[derive(Debug, Clone, Copy)]
pub struct VerbInfo {
    pub aspect: Aspect,
    /// `None` where the source carries no marker.
    pub transitive: Option<bool>,
    pub reflexive: bool,
}

/// Does this slot exist at all for a verb with these properties?
///
/// Pure, and the reason it exists: the answer is a function of the grammar, so
/// nothing may consult data to decide it.
pub fn slot_exists(slot: VerbSlot, info: VerbInfo) -> bool {
    let perfective = info.aspect == Aspect::Perfective;
    let transitive = info.transitive.unwrap_or(true);
    match slot {
        VerbSlot::Finite { tense, .. } => match tense {
            // A perfective verb has no present tense.
            Tense::Present => !perfective,
            // An imperfective future is periphrastic but it exists.
            Tense::Future => true,
            Tense::Past => true,
        },
        VerbSlot::Participle { voice, tense, .. } => match (voice, tense) {
            // No present participles or gerund for a perfective.
            (_, Tense::Present) if perfective => false,
            // No passive participle without an object to promote.
            (Voice::Passive, _) if !transitive => false,
            (Voice::Passive, Tense::Future) => false,
            _ => true,
        },
        VerbSlot::Infinitive | VerbSlot::Past { .. } | VerbSlot::Imperative { .. } => true,
    }
}

/// Non-past endings. The first conjugation is `-u -jeszj -jet -jem -jetje -ut`,
/// the second `-u -iszj -it -im -itje -at`; the iotated variants are chosen by
/// the stem's final letter.
fn nonpast_ending(conj: Conjugation, pn: PersonNumber, soft: bool) -> &'static str {
    use Conjugation::*;
    use PersonNumber::*;
    match (conj, pn) {
        (First, S1) => "u",
        (First, S2) => "jeszj",
        (First, S3) => "jet",
        (First, P1) => "jem",
        (First, P2) => "jetje",
        (First, P3) => "ut",
        (Second, S1) => {
            if soft {
                "ju"
            } else {
                "u"
            }
        }
        (Second, S2) => "iszj",
        (Second, S3) => "it",
        (Second, P1) => "im",
        (Second, P2) => "itje",
        (Second, P3) => {
            if soft {
                "jat"
            } else {
                "at"
            }
        }
    }
}

/// The imperfective future auxiliary, `byti` in its non-past forms.
fn future_auxiliary(pn: PersonNumber) -> &'static str {
    use PersonNumber::*;
    match pn {
        S1 => "budu",
        S2 => "budjeszj",
        S3 => "budjet",
        P1 => "budjem",
        P2 => "budjetje",
        P3 => "budut",
    }
}

/// The infinitive stem: the citation form with `-tj`/`-ti` removed.
///
/// ```
/// use ruthenian_core::verb::infinitive_stem;
/// assert_eq!(infinitive_stem("citatj"), "cita");
/// assert_eq!(infinitive_stem("govoritj"), "govori");
/// ```
pub fn infinitive_stem(infinitive: &str) -> String {
    for suffix in ["tjsja", "tj", "ti", "cz"] {
        if let Some(rest) = infinitive.strip_suffix(suffix) {
            return rest.to_string();
        }
    }
    infinitive.to_string()
}

/// The present (non-past) stem, derived from the class where the class
/// determines it.
///
/// Mutation is conditioned on the **class**, never on the stem's final
/// consonant: class 1 verbs in `-ivatj`/`-yvatj` end in a labial and take no
/// epenthesis at all, and keying on the consonant corrupts hundreds of them.
fn present_stem(
    infinitive: &str,
    class: &ZaliznyakVerbClass,
    parts: &PrincipalPartsRef<'_>,
) -> Result<(String, Trace), Unsupported> {
    if let Some(given) = parts.present_stem {
        return Ok((
            given.as_str().to_string(),
            Trace::new("present stem supplied as a principal part"),
        ));
    }
    let unsupported = |reason| Unsupported {
        reason,
        class: class.raw.clone(),
    };
    if class.needs_principal_parts() {
        return Err(unsupported(
            "irregular or unclassified: the lexicon must supply the present stem",
        ));
    }
    // Segmental first: a stress mark sits *after* its vowel, so suffix surgery
    // on a stressed stem silently fails. Stress is re-placed by accent pattern
    // once the form is built.
    let stem = phono::unstress(&infinitive_stem(infinitive));
    let index = class.index.ok_or_else(|| unsupported("no class index"))?;

    Ok(match index {
        // -atj / -jatj: the theme vowel stays and a glide is added. No mutation.
        1 => (
            format!("{stem}j"),
            Trace::new("class 1: present stem = infinitive stem + j"),
        ),
        // -ovatj / -irovatj: `ov` -> `u`, then the glide. The commonest mutation.
        2 => {
            let m = phono::mutate_present_stem(&stem);
            (format!("{m}j"), Trace::new("class 2: ov -> u, then + j"))
        }
        // -nutj: the theme vowel drops.
        3 => {
            let s = stem.strip_suffix("nu").unwrap_or(&stem);
            (
                format!("{s}n"),
                Trace::new("class 3: -nu- drops before non-past endings"),
            )
        }
        // -itj: the theme vowel drops; the first person singular mutates.
        4 => {
            let s = stem.strip_suffix('i').unwrap_or(&stem);
            (s.to_string(), Trace::new("class 4: theme vowel drops"))
        }
        // -etj / -atj, second conjugation: theme vowel drops, no glide.
        5 => {
            let s = stem
                .strip_suffix('e')
                .or_else(|| stem.strip_suffix('a'))
                .unwrap_or(&stem);
            (s.to_string(), Trace::new("class 5: theme vowel drops"))
        }
        // -atj with a mutated present stem throughout: pisatj -> pisz-.
        6 => {
            let s = stem
                .strip_suffix('a')
                .or_else(|| stem.strip_suffix("ja"))
                .unwrap_or(&stem);
            (
                phono::mutate_present_stem(s),
                Trace::new("class 6: theme vowel drops, stem mutates"),
            )
        }
        other => {
            let _ = other;
            return Err(unsupported(
                "class not implemented; classes 1-6 are covered",
            ));
        }
    })
}

/// Conjugate.
///
/// `infinitive` is the citation form in Ruthenian. Where the class does not
/// determine the present stem, supply it through `parts` — the engine never
/// guesses a stem it cannot derive.
///
/// ```
/// use ruthenian_core::verb::{verb, VerbInfo};
/// use ruthenian_core::class::ZaliznyakVerbClass;
/// use ruthenian_core::types::*;
///
/// let cls = ZaliznyakVerbClass::parse("1a").unwrap();
/// let info = VerbInfo { aspect: Aspect::Imperfective, transitive: Some(true), reflexive: false };
/// let parts = PrincipalPartsRef::default();
/// let slot = VerbSlot::Finite { person: Person::First, number: Number::Singular, tense: Tense::Present };
/// assert_eq!(verb("citatj", &cls, info, &parts, slot).unwrap().unwrap().text, "citaju");
///
/// // A perfective verb has no present tense: the cell does not exist.
/// let pf = VerbInfo { aspect: Aspect::Perfective, ..info };
/// assert!(verb("citatj", &cls, pf, &parts, slot).unwrap().is_none());
/// ```
pub fn verb(
    infinitive: &str,
    class: &ZaliznyakVerbClass,
    info: VerbInfo,
    parts: &PrincipalPartsRef<'_>,
    slot: VerbSlot,
) -> Resolved {
    if !slot_exists(slot, info) {
        return Ok(None);
    }
    let refl = |s: String| {
        if info.reflexive {
            let tail = if s.ends_with(['a', 'e', 'i', 'o', 'u', 'y']) {
                "sj"
            } else {
                "sja"
            };
            format!("{s}{tail}")
        } else {
            s
        }
    };

    match slot {
        VerbSlot::Infinitive => Ok(Some(Prediction::new(
            infinitive.to_string(),
            Trace::new("infinitive is the citation form"),
        ))),

        VerbSlot::Finite {
            person,
            number,
            tense,
        } => {
            let pn = PersonNumber::of(person, number);
            // An imperfective future is periphrastic: auxiliary + infinitive.
            if tense == Tense::Future && info.aspect != Aspect::Perfective {
                return Ok(Some(Prediction::new(
                    format!("{} {}", future_auxiliary(pn), infinitive),
                    Trace::new("imperfective future: byti auxiliary + infinitive"),
                )));
            }
            let (stem, mut trace) = present_stem(infinitive, class, parts)?;
            let conj = class.conjugation();
            // The first person singular is where the class-4 mutation surfaces.
            let stem = if conj == Conjugation::Second && pn == PersonNumber::S1 {
                trace = trace.then("first person singular mutates");
                phono::mutate_present_stem(&stem)
            } else {
                stem
            };
            let soft = !phono::ends_sibilant(&stem);
            let ending = nonpast_ending(conj, pn, soft);
            let ending = phono::spell_after_stem(&stem, ending);
            trace = trace.then(if conj == Conjugation::First {
                "first conjugation endings"
            } else {
                "second conjugation endings"
            });
            if tense == Tense::Future {
                trace = trace.then("perfective non-past realizes the future");
            }
            let built = collapse(&format!("{stem}{ending}"));
            let (built, trace) = place_nonpast_stress(&built, infinitive, class, pn, trace);
            Ok(Some(Prediction::new(refl(built), trace)))
        }

        VerbSlot::Past { gender, number } => {
            let stem = phono::unstress(
                &parts
                    .past_stem
                    .map(|s| s.as_str().to_string())
                    .unwrap_or_else(|| infinitive_stem(infinitive)),
            );
            let ending = match (number, gender) {
                (crate::types::Number::Plural, _) => "li",
                (_, Some(Gender::Feminine)) => "la",
                (_, Some(Gender::Neuter)) => "lo",
                (_, _) => "l",
            };
            // The past keeps the infinitive's stressed vowel for the common
            // patterns; the shifting feminine (`bylá`) is not modelled.
            let built = match phono::stressed_index(infinitive) {
                Some(i) => phono::apply_stress_at(&format!("{stem}{ending}"), i),
                None => format!("{stem}{ending}"),
            };
            Ok(Some(Prediction::new(
                refl(built),
                Trace::new("past: infinitive stem + l-participle ending"),
            )))
        }

        VerbSlot::Imperative { number } => {
            let (stem, trace) = present_stem(infinitive, class, parts)?;
            let base = if stem.ends_with('j') {
                stem.clone()
            } else {
                format!("{stem}i")
            };
            let text = match number {
                crate::types::Number::Singular => base,
                crate::types::Number::Plural => format!("{base}tje"),
            };
            Ok(Some(Prediction::new(
                refl(collapse(&text)),
                trace.then("imperative from the present stem"),
            )))
        }

        VerbSlot::Participle { kind, voice, tense } => {
            participle(infinitive, class, info, parts, kind, voice, tense).map(|o| {
                o.map(|p| Prediction {
                    text: refl(p.text),
                    trace: p.trace,
                })
            })
        }
    }
}

fn participle(
    infinitive: &str,
    class: &ZaliznyakVerbClass,
    _info: VerbInfo,
    parts: &PrincipalPartsRef<'_>,
    kind: ParticipleKind,
    voice: Voice,
    tense: Tense,
) -> Resolved {
    let past_stem = parts
        .past_stem
        .map(|s| s.as_str().to_string())
        .unwrap_or_else(|| infinitive_stem(infinitive));

    match (kind, voice, tense) {
        (ParticipleKind::Adjectival, Voice::Active, Tense::Present) => {
            let (stem, trace) = present_stem(infinitive, class, parts)?;
            let suffix = if class.conjugation() == Conjugation::First {
                "uszczij"
            } else if phono::ends_sibilant(&stem) {
                "aszczij"
            } else {
                "jaszczij"
            };
            Ok(Some(Prediction::new(
                collapse(&format!("{stem}{suffix}")),
                trace.then("present active participle"),
            )))
        }
        (ParticipleKind::Adjectival, Voice::Active, _) => Ok(Some(Prediction::new(
            format!("{past_stem}vszij"),
            Trace::new("past active participle: past stem + vszij"),
        ))),
        (ParticipleKind::Adverbial, _, Tense::Present) => {
            let (stem, trace) = present_stem(infinitive, class, parts)?;
            let suffix = if phono::ends_sibilant(&stem) {
                "a"
            } else {
                "ja"
            };
            Ok(Some(Prediction::new(
                collapse(&format!("{stem}{suffix}")),
                trace.then("present gerund"),
            )))
        }
        (ParticipleKind::Adverbial, _, _) => Ok(Some(Prediction::new(
            format!("{past_stem}v"),
            Trace::new("past gerund: past stem + v"),
        ))),
        (ParticipleKind::Adjectival, Voice::Passive, Tense::Present) => {
            let (stem, trace) = present_stem(infinitive, class, parts)?;
            let suffix = if class.conjugation() == Conjugation::First {
                "jemyj"
            } else {
                "imyj"
            };
            Ok(Some(Prediction::new(
                collapse(&format!("{stem}{suffix}")),
                trace.then("present passive participle"),
            )))
        }
        (ParticipleKind::Adjectival, Voice::Passive, _) => {
            // `+p` in the class code is what says this participle exists, and it
            // predicts an attested one with ~99.9 % precision.
            if !class.ppp {
                return Ok(None);
            }
            let mutated = match &class.ppp_mutation {
                Some(m) if !m.is_empty() => {
                    let _ = m;
                    phono::mutate_present_stem(&past_stem)
                }
                _ => past_stem.clone(),
            };
            let suffix = if class.index == Some(4) || class.index == Some(5) {
                "jonnyj"
            } else if past_stem.ends_with('a') || past_stem.ends_with('e') {
                "nnyj"
            } else {
                "tyj"
            };
            let base = if class.index == Some(4) || class.index == Some(5) {
                mutated.trim_end_matches('i').to_string()
            } else {
                mutated
            };
            Ok(Some(Prediction::new(
                collapse(&format!("{base}{suffix}")),
                Trace::new("past passive participle (+p in the class code)"),
            )))
        }
    }
}

/// Place stress on a non-past form according to the class's accent pattern.
///
/// `a` keeps the infinitive's stressed vowel; `b` moves it to the ending; `c`
/// stresses the ending in the first person singular and the stem elsewhere.
/// Patterns `d`-`f` are not modelled and say so in the trace rather than
/// inventing a position.
fn place_nonpast_stress(
    built: &str,
    infinitive: &str,
    class: &ZaliznyakVerbClass,
    pn: PersonNumber,
    trace: Trace,
) -> (String, Trace) {
    use crate::types::AccentPattern::*;
    let from_infinitive = phono::stressed_index(infinitive);
    match class.accent {
        Some(A) => match from_infinitive {
            Some(i) => (
                phono::apply_stress_at(built, i),
                trace.then("accent a: stress stays on the stem vowel"),
            ),
            None => (built.to_string(), trace),
        },
        Some(B) => (
            phono::stress_last_vowel(built),
            trace.then("accent b: ending stress"),
        ),
        Some(C) => {
            if pn == PersonNumber::S1 {
                (
                    phono::stress_last_vowel(built),
                    trace.then("accent c: ending stress in the 1sg"),
                )
            } else {
                match from_infinitive {
                    Some(i) => (
                        phono::apply_stress_at(built, i),
                        trace.then("accent c: stem stress outside the 1sg"),
                    ),
                    None => (built.to_string(), trace),
                }
            }
        }
        _ => (
            built.to_string(),
            trace.then("accent pattern d-f: stress not modelled"),
        ),
    }
}

/// `j` + `j`-initial ending collapses: `citaj` + `jet` is `citajet`.
fn collapse(s: &str) -> String {
    let mut out = s.to_string();
    while let Some(at) = out.find("jj") {
        out.replace_range(at..at + 2, "j");
    }
    out
}
