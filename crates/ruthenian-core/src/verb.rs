//! Verb conjugation over Ruthenian's **six** classes (`RUTHENIAN.md` §7).
//!
//! # Aspect is derived, never stored
//!
//! Every Slavic language stores aspect pairs in the dictionary because pairing
//! is unpredictable. Ruthenian derives them from surface shape alone (§7.2), so
//! [`aspect_of`] is the single implementation and no entry anywhere in the
//! workspace carries an aspect field. There are no biaspectual verbs and no
//! suppletive pairs.
//!
//! # Three past tenses, independent of aspect
//!
//! The aorist, imperfect and perfect divide by **function**, not by aspect, as
//! in OCS: a perfective verb has an imperfect (`poczitaszje` "he kept
//! finishing"), an imperfective has an aorist (`czita` "he read, once"). Folding
//! the two axes together is the Russian collapse this language undoes, and a
//! single `Tense::Past` would re-impose it.
//!
//! # Structural gaps are derived, never read
//!
//! A perfective verb's present morphology realizes the **future** (§7.8), so it
//! has no present tense and no present participles; an intransitive verb has no
//! passive participle. [`slot_exists`] computes these from
//! `(aspect, transitivity, slot)` and returns `Ok(None)`.

use crate::phono;
use crate::trace::{Prediction, Trace};
use crate::types::{
    Aspect, Conjugation, Gender, Number, ParticipleKind, Person, PersonNumber, PrincipalPartsRef,
    Tense, VerbClass, VerbSlot, Voice,
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
///
/// Note what is **not** here: aspect. It is derived from the infinitive by
/// [`aspect_of`], so storing it would create a second answer that eventually
/// disagrees with the first.
#[derive(Debug, Clone, Copy)]
pub struct VerbInfo {
    /// `None` where the source carries no marker.
    pub transitive: Option<bool>,
    pub reflexive: bool,
}

impl Default for VerbInfo {
    fn default() -> Self {
        Self {
            transitive: Some(true),
            reflexive: false,
        }
    }
}

/// A derived grammatical value and the reasoning behind it.
///
/// Aspect is returned this way rather than bare so the trace can name the rule
/// that fired: law 12, return the structure rather than a value the caller must
/// explain for itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derived<T> {
    pub value: T,
    pub trace: Trace,
}

/// The lexical prefixes that perfectivize as a side effect (§7.2 rule 2), plus
/// `po-`, the empty perfectivizer (rule 1).
///
/// Ordered longest first so `pod-` is tried before `po-` and `pjerje-` before
/// `pje-`; a shorter prefix pre-empting a longer one is the same trap the
/// mutation table documents.
pub const PREFIXES: &[&str] = &[
    "pjerje", "raz", "pri", "pro", "pod", "iz", "vy", "do", "ot", "za", "na", "po", "s", "u", "v",
];

/// The secondary-imperfective suffixes (§7.2 rule 3).
pub const IMPERFECTIVIZERS: &[&str] = &["yva", "iva"];

/// Derive a verb's aspect from its surface shape.
///
/// Three rules, no exceptions, no lexical entry (`RUTHENIAN.md` §7.2):
///
/// 1. a bare stem is **imperfective**;
/// 2. any prefix makes it **perfective** — `po-` is the empty perfectivizer,
///    the others add meaning as well;
/// 3. `-yva-`/`-iva-` on a prefixed perfective makes it **imperfective** again.
///
/// Rule 3 outranks rule 2, which is why it is checked first inside the prefixed
/// branch: a secondary imperfective is built *on* a prefixed perfective, so both
/// conditions hold at once.
///
/// ```
/// use ruthenian_core::{aspect_of, Aspect};
/// assert_eq!(aspect_of("czitatj").value, Aspect::Imperfective);     // bare stem
/// assert_eq!(aspect_of("poczitatj").value, Aspect::Perfective);     // po- + stem
/// assert_eq!(aspect_of("napisatj").value, Aspect::Perfective);      // lexical prefix
/// assert_eq!(aspect_of("napisyvatj").value, Aspect::Imperfective);  // -yva- wins
/// ```
///
/// The trace names which rule fired, so a caller can explain *why* a verb is
/// perfective rather than merely assert it:
///
/// ```
/// # use ruthenian_core::aspect_of;
/// let d = aspect_of("napisyvatj");
/// assert!(d.trace.steps().iter().any(|s| s.contains("-yva-")));
/// ```
pub fn aspect_of(infinitive: &str) -> Derived<Aspect> {
    let bare = phono::unstress(infinitive);
    let stem = bare.strip_suffix("tj").unwrap_or(&bare);

    // A prefix must leave a recognizable stem behind; `potj` is not `po` + a
    // verb. Three characters is the shortest real Ruthenian verb stem.
    let prefixed = PREFIXES
        .iter()
        .any(|p| stem.strip_prefix(*p).is_some_and(|rest| rest.len() >= 3));

    if prefixed {
        if IMPERFECTIVIZERS.iter().any(|s| stem.ends_with(s)) {
            return Derived {
                value: Aspect::Imperfective,
                trace: Trace::new("prefixed stem")
                    .then("-yva-/-iva- re-imperfectivizes a prefixed perfective"),
            };
        }
        return Derived {
            value: Aspect::Perfective,
            trace: Trace::new("a prefix makes the verb perfective"),
        };
    }

    Derived {
        value: Aspect::Imperfective,
        trace: Trace::new("a bare stem is imperfective"),
    }
}

/// Does this slot exist for a verb with these properties?
///
/// Derived from the grammar, never looked up. `false` is a claim about the
/// language: a perfective verb has no present tense because its present
/// morphology realizes the future (§7.8).
pub fn slot_exists(aspect: Aspect, transitive: Option<bool>, slot: VerbSlot) -> bool {
    let perfective = aspect == Aspect::Perfective;
    match slot {
        // A perfective's present endings ARE its future, so exactly one of the
        // two synthetic cells exists for any given verb.
        VerbSlot::Finite {
            tense: Tense::Present,
            ..
        } => !perfective,
        VerbSlot::Finite {
            tense: Tense::Future,
            ..
        } => perfective,
        VerbSlot::Participle {
            voice: Voice::Passive,
            ..
        } if transitive == Some(false) => false,
        VerbSlot::Participle {
            tense: Tense::Present,
            ..
        } => !perfective,
        // First person is the hortative, which has no singular (§7.10); third
        // person has no imperative at all.
        VerbSlot::Imperative {
            person: Person::First,
            number,
        } => number != Number::Singular,
        VerbSlot::Imperative {
            person: Person::Third,
            ..
        } => false,
        _ => true,
    }
}

/// Present endings (§7.4), by conjugation and person/number.
fn present_ending(conj: Conjugation, pn: PersonNumber) -> &'static str {
    use Conjugation::*;
    use PersonNumber::*;
    match (conj, pn) {
        (First, S1) => "u",
        (First, S2) => "jeszj",
        (First, S3) => "jet",
        (First, D1) => "jevje",
        (First, D2) => "jeta",
        (First, D3) => "jetje",
        (First, P1) => "jem",
        (First, P2) => "jetje",
        (First, P3) => "ut",
        (Second, S1) => "ju",
        (Second, S2) => "iszj",
        (Second, S3) => "it",
        (Second, D1) => "ivje",
        (Second, D2) => "ita",
        (Second, D3) => "itje",
        (Second, P1) => "im",
        (Second, P2) => "itje",
        (Second, P3) => "jat",
    }
}

/// Aorist endings (§7.5) — the OCS sigmatic aorist, on the **infinitive** stem.
///
/// The second and third singular are the bare stem: the inherited shape, and the
/// reason the aorist is instantly recognizable.
fn aorist_ending(pn: PersonNumber) -> &'static str {
    use PersonNumber::*;
    match pn {
        S1 => "h",
        S2 | S3 => "",
        D1 => "hovje",
        D2 => "sta",
        D3 => "stje",
        P1 => "hom",
        P2 => "stje",
        P3 => "sza",
    }
}

/// Imperfect endings (§7.6), on the infinitive stem with `-jah-`.
fn imperfect_ending(pn: PersonNumber) -> &'static str {
    use PersonNumber::*;
    match pn {
        S1 => "jah",
        S2 | S3 => "jasze",
        D1 => "jahovje",
        D2 => "jaszeta",
        D3 => "jaszetje",
        P1 => "jahom",
        P2 => "jaszetje",
        P3 => "jahu",
    }
}

/// The infinitive stem — what the aorist, imperfect, l-participle, past
/// participles and supine are built on.
///
/// ```
/// use ruthenian_core::verb::infinitive_stem;
/// assert_eq!(infinitive_stem("czitatj").as_deref(), Some("czita"));
/// assert_eq!(infinitive_stem("govoritj").as_deref(), Some("govori"));
/// assert_eq!(infinitive_stem("dom"), None);
/// ```
pub fn infinitive_stem(infinitive: &str) -> Option<String> {
    let bare = phono::unstress(infinitive);
    bare.strip_suffix("tj").map(str::to_string)
}

/// The present stem, derived from the infinitive by **class** (§7.3).
///
/// Classes 4 and 6 mutate; 1, 2, 3 and 5 do not. The mutation is conditioned on
/// the class, never on the stem's final consonant — a rule keyed on "ends in a
/// labial" corrupts every class-1 verb, because the theme vowel intervenes.
///
/// ```
/// use ruthenian_core::{verb::present_stem, VerbClass};
/// assert_eq!(present_stem("czitatj", VerbClass::One).as_deref(), Some("czitaj"));
/// assert_eq!(present_stem("njegodovatj", VerbClass::Two).as_deref(), Some("njegoduj"));
/// assert_eq!(present_stem("dvinutj", VerbClass::Three).as_deref(), Some("dvin"));
/// assert_eq!(present_stem("govoritj", VerbClass::Four).as_deref(), Some("govor"));
/// assert_eq!(present_stem("vidjetj", VerbClass::Five).as_deref(), Some("vid"));
/// assert_eq!(present_stem("pisatj", VerbClass::Six).as_deref(), Some("pisz"));
/// ```
pub fn present_stem(infinitive: &str, class: VerbClass) -> Option<String> {
    present_stem_with(infinitive, class, PrincipalPartsRef::default())
}

/// The present stem, honouring a supplied principal part.
///
/// The rule engine never guesses a stem it cannot derive: if the class does not
/// determine it — the suppletive copula `byti` (§7.9) is the clear case — the
/// caller supplies it, and this is the type-level expression of law 8.
pub fn present_stem_with(
    infinitive: &str,
    class: VerbClass,
    parts: PrincipalPartsRef<'_>,
) -> Option<String> {
    if let Some(given) = parts.present_stem {
        return Some(phono::unstress(given.as_str()));
    }
    let stem = infinitive_stem(infinitive)?;
    Some(match class {
        VerbClass::One => format!("{stem}j"),
        VerbClass::Two => {
            let root = stem.strip_suffix("ova").unwrap_or(&stem);
            format!("{root}uj")
        }
        // `-nutj`: the THEME VOWEL drops, not the whole suffix.
        // `dvinutj` -> `dvinu` -> `dvin`, per §7.3's own example.
        VerbClass::Three => stem.strip_suffix('u').unwrap_or(&stem).to_string(),
        VerbClass::Four => stem.strip_suffix('i').unwrap_or(&stem).to_string(),
        VerbClass::Five => stem.strip_suffix("je").unwrap_or(&stem).to_string(),
        VerbClass::Six => {
            let root = stem.strip_suffix('a').unwrap_or(&stem);
            phono::mutate_present_stem(root)
        }
    })
}

/// Conjugate a verb.
///
/// `Ok(None)` means the cell does not exist for a verb of this aspect and
/// transitivity — see [`slot_exists`]. `Err` means the rules do not cover the
/// input, and never a wrong form.
///
/// ```
/// use ruthenian_core::{verb, Number, Person, Tense, VerbClass, VerbInfo, VerbSlot};
///
/// let info = VerbInfo::default();
/// let f = |person, number, tense| {
///     verb("czitatj", VerbClass::One, info, VerbSlot::Finite { person, number, tense })
///         .unwrap().unwrap().text
/// };
///
/// // present, including the dual
/// assert_eq!(f(Person::First,  Number::Singular, Tense::Present), "czitaju");
/// assert_eq!(f(Person::Second, Number::Singular, Tense::Present), "czitajeszj");
/// assert_eq!(f(Person::First,  Number::Dual,     Tense::Present), "czitajevje");
/// assert_eq!(f(Person::Third,  Number::Plural,   Tense::Present), "czitajut");
///
/// // the aorist: 2sg and 3sg are the bare stem
/// assert_eq!(f(Person::First,  Number::Singular, Tense::Aorist), "czitah");
/// assert_eq!(f(Person::Second, Number::Singular, Tense::Aorist), "czita");
/// assert_eq!(f(Person::First,  Number::Plural,   Tense::Aorist), "czitahom");
/// assert_eq!(f(Person::Third,  Number::Plural,   Tense::Aorist), "czitasza");
///
/// // the imperfect, an axis independent of aspect
/// assert_eq!(f(Person::Second, Number::Singular, Tense::Imperfect), "czitajasze");
/// ```
///
/// A perfective has no present tense — its present endings are its future:
///
/// ```
/// use ruthenian_core::{verb, Number, Person, Tense, VerbClass, VerbInfo, VerbSlot};
/// let slot = |tense| VerbSlot::Finite { person: Person::First, number: Number::Singular, tense };
/// let info = VerbInfo::default();
///
/// // the cell does not exist — `Ok(None)`, never an error
/// assert!(verb("poczitatj", VerbClass::One, info, slot(Tense::Present)).unwrap().is_none());
/// // and the future is where its present morphology lands
/// assert_eq!(
///     verb("poczitatj", VerbClass::One, info, slot(Tense::Future)).unwrap().unwrap().text,
///     "poczitaju",
/// );
/// ```
pub fn verb(infinitive: &str, class: VerbClass, info: VerbInfo, slot: VerbSlot) -> Resolved {
    verb_with(infinitive, class, info, PrincipalPartsRef::default(), slot)
}

/// Conjugate a verb, supplying the principal parts the class does not determine.
///
/// This is the general entry point; [`verb`] is the common case with none
/// supplied. The `_with` split follows `interslavic`'s convention for
/// explicit-metadata variants, and exists because one verb genuinely needs it:
/// the copula `byti` is suppletive (§7.9) and no class derives `jesmj` from it.
///
/// ```
/// use ruthenian_core::{verb_with, Number, Person, PrincipalPartsRef, Tense, VerbClass, VerbInfo, VerbSlot};
/// use ruthenian_orthography::Ruthenian;
///
/// // A supplied present stem overrides the class derivation entirely.
/// let stem = Ruthenian::parse("znaj").unwrap();
/// let parts = PrincipalPartsRef { present_stem: Some(&stem), ..Default::default() };
/// let slot = VerbSlot::Finite { person: Person::Third, number: Number::Singular, tense: Tense::Present };
///
/// let got = verb_with("znatj", VerbClass::One, VerbInfo::default(), parts, slot)
///     .unwrap().unwrap();
/// assert_eq!(got.text, "znajet");
/// assert!(got.trace.steps().iter().any(|s| s.contains("principal part")));
/// ```
pub fn verb_with(
    infinitive: &str,
    class: VerbClass,
    info: VerbInfo,
    parts: PrincipalPartsRef<'_>,
    slot: VerbSlot,
) -> Resolved {
    let aspect = aspect_of(infinitive).value;
    if !slot_exists(aspect, info.transitive, slot) {
        return Ok(None);
    }
    let unsupported = |reason| Unsupported {
        reason,
        class: format!("{class:?}"),
    };
    let inf_stem =
        || infinitive_stem(infinitive).ok_or_else(|| unsupported("infinitive must end in -tj"));
    let pres_stem = || {
        present_stem_with(infinitive, class, parts)
            .ok_or_else(|| unsupported("infinitive must end in -tj"))
    };
    let supplied = parts.present_stem.is_some();

    let p = match slot {
        VerbSlot::Infinitive => Prediction::new(
            phono::unstress(infinitive),
            Trace::new("the citation form itself"),
        ),

        // The supine (§7.10a) is the infinitive without its soft sign, and
        // governs the genitive rather than the accusative.
        VerbSlot::Supine => {
            let bare = phono::unstress(infinitive);
            let cut = bare
                .strip_suffix('j')
                .ok_or_else(|| unsupported("infinitive must end in -tj"))?;
            Prediction::new(
                cut.to_string(),
                Trace::new("supine: the infinitive without its soft sign"),
            )
        }

        VerbSlot::Finite {
            person,
            number,
            tense,
        } => {
            let pn = PersonNumber::of(person, number);
            match tense {
                Tense::Present | Tense::Future => {
                    let stem = pres_stem()?;
                    let raw = present_ending(class.conjugation(), pn);
                    let ending = phono::spell_after_stem(&stem, raw, false);
                    let mut trace = Trace::new(match class.conjugation() {
                        Conjugation::First => "1st-conjugation present ending",
                        Conjugation::Second => "2nd-conjugation present ending",
                    });
                    if supplied {
                        trace = trace.then("present stem supplied as a principal part");
                    }
                    if tense == Tense::Future {
                        trace = trace.then("a perfective's present endings carry future sense");
                    }
                    Prediction::new(join(&stem, &ending), trace)
                }
                Tense::Aorist => Prediction::new(
                    join(&inf_stem()?, aorist_ending(pn)),
                    Trace::new("sigmatic aorist on the infinitive stem"),
                ),
                Tense::Imperfect => Prediction::new(
                    join(&inf_stem()?, imperfect_ending(pn)),
                    Trace::new("imperfect -jah- on the infinitive stem"),
                ),
                // The perfect and pluperfect are the l-participle plus a copula
                // — a phrase, not a cell. The caller composes them from
                // `LParticiple` and `byti`; fusing them here would be a second
                // way to build the same form, which law 1 forbids.
                Tense::Perfect | Tense::Pluperfect => {
                    return Err(unsupported(
                        "perfect and pluperfect are periphrastic: compose the l-participle with byti",
                    ));
                }
            }
        }

        VerbSlot::LParticiple { gender, number } => {
            let ending = match (number, gender) {
                (Number::Singular, Some(Gender::Feminine)) => "la",
                (Number::Singular, Some(Gender::Neuter)) => "lo",
                (Number::Singular, _) => "l",
                _ => "li",
            };
            Prediction::new(
                join(&inf_stem()?, ending),
                Trace::new("l-participle on the infinitive stem"),
            )
        }

        VerbSlot::Imperative { person, number } => {
            let stem = pres_stem()?;
            // Present stem + `-i`, or the bare stem after `j` (§7.10).
            let after_j = stem.ends_with('j');
            let ending = match (person, number) {
                (Person::Second, Number::Singular) => {
                    if after_j {
                        ""
                    } else {
                        "i"
                    }
                }
                (Person::Second, Number::Dual) => {
                    if after_j {
                        "ta"
                    } else {
                        "ita"
                    }
                }
                (Person::Second, Number::Plural) => {
                    if after_j {
                        "tje"
                    } else {
                        "itje"
                    }
                }
                (Person::First, Number::Dual) => {
                    if after_j {
                        "vje"
                    } else {
                        "ivje"
                    }
                }
                (Person::First, Number::Plural) => {
                    if after_j {
                        "m"
                    } else {
                        "im"
                    }
                }
                // Filtered by `slot_exists`; unreachable in practice, and an
                // explicit `Ok(None)` rather than a panic if that ever changes.
                _ => return Ok(None),
            };
            Prediction::new(
                join(&stem, ending),
                Trace::new(match person {
                    Person::First => "hortative on the present stem",
                    _ => "imperative on the present stem",
                }),
            )
        }

        VerbSlot::Participle { kind, voice, tense } => {
            let present = tense == Tense::Present;
            let suffix = match (kind, voice, present) {
                (ParticipleKind::Adjectival, Voice::Active, true) => match class.conjugation() {
                    Conjugation::First => "uszczij",
                    Conjugation::Second => "jaszczij",
                },
                (ParticipleKind::Adjectival, Voice::Active, false) => "vszij",
                (ParticipleKind::Adjectival, Voice::Passive, true) => match class.conjugation() {
                    Conjugation::First => "jemyj",
                    Conjugation::Second => "imyj",
                },
                (ParticipleKind::Adjectival, Voice::Passive, false) => "nyj",
                (ParticipleKind::Adverbial, _, true) => "a",
                (ParticipleKind::Adverbial, _, false) => "vszi",
            };
            let stem = if present { pres_stem()? } else { inf_stem()? };
            Prediction::new(
                join(&stem, suffix),
                Trace::new("participle suffix by kind, voice and tense"),
            )
        }
    };

    Ok(Some(p))
}

/// Join a stem and an ending, collapsing a doubled `j` at the seam.
///
/// A class-1 present stem already ends in `j` (`czitaj`), and the first-conjugation
/// endings begin with one (`-jet`); the seam is one `j`, not two.
fn join(stem: &str, ending: &str) -> String {
    if stem.ends_with('j') {
        if let Some(rest) = ending.strip_prefix('j') {
            return format!("{stem}{rest}");
        }
    }
    format!("{stem}{ending}")
}
