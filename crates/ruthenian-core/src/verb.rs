//! Verbs: §7's six classes and three synthetic tenses.
//!
//! **Nothing about a verb is supplied.** §7.3 derives the conjugation class from
//! the citation form — every ending decides its own, and `-atj`, the one
//! genuinely ambiguous ending, is disambiguated by the word-final `'` that marks
//! a class-6 lemma. `pisatj'` carries its class in its spelling.
//!
//! Aspect is not a parameter either. It changes what a form *means*, never what
//! it looks like: the endings are identical for both aspects, and all aspect
//! decides is whether `NonPast` is a present or a future (§7.8).

use crate::fallback::UNREADABLE;
use crate::grammar::{FiniteTense, Gender, Number, Person};
use crate::spelling::{join, mutate_present_stem};

/// §7.3's six classes, named by what they do to the stem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    /// Theme vowel stays, `-j-` added: `czitatj` → `czitaj-`, `pitj` → `pij-`.
    One,
    /// `ova` → `uj`: `njegodovatj` → `njegoduj-`.
    Two,
    /// Theme drops: `dvinutj` → `dvin-`.
    Three,
    /// Theme drops, 1sg mutates: `govoritj` → `govor-`.
    Four,
    /// Theme drops: `vidjetj` → `vid-`.
    Five,
    /// Theme drops, stem mutates throughout: `pisatj'` → `pisz-`.
    Six,
}

impl Class {
    /// 1st or 2nd conjugation (§7.3's fourth column).
    fn second_conjugation(self) -> bool {
        matches!(self, Class::Four | Class::Five)
    }

    /// Does the 1sg take a mutated stem? Classes 4 and 5 mutate there; class 6
    /// mutates throughout and so has already done it.
    fn mutates_in_1sg(self) -> bool {
        matches!(self, Class::Four | Class::Five)
    }
}

/// A verb's citation form, read.
struct Verb {
    /// The infinitive stem: the citation form less `-tj`. The aorist, the
    /// imperfect and the `l`-participle are all built on it (§7.5–§7.7).
    infinitive: String,
    class: Class,
}

/// The plain vowels (§2.3); the iotated series are `j` + one of these.
fn vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouy".contains(*c)).count()
}

impl Verb {
    fn read(word: &str) -> Option<Self> {
        let parsed = ruthenian_orthography::Ruthenian::parse(word).ok()?;
        let marked = parsed.is_marked();
        let lower = parsed.word().to_lowercase();
        if !lower.chars().all(|c| c.is_ascii_alphabetic() || c == '\'') {
            return None;
        }
        // Every infinitive ends in `-tj`; anything else is not a verb lemma.
        let infinitive = lower.strip_suffix("tj")?.to_string();
        if infinitive.is_empty() || vowels(&infinitive) == 0 {
            return None;
        }

        // §7.3's table, stated by ending. A stem is monosyllabic when it has one
        // vowel — the vowel *is* the root, so there is no theme to drop, and the
        // verb takes class 1's operation whichever vowel it is.
        let mono = vowels(&infinitive) == 1;
        let class = if infinitive.ends_with("ova") {
            Class::Two
        } else if infinitive.ends_with("nu") {
            Class::Three
        } else if infinitive.ends_with('i') {
            if mono { Class::One } else { Class::Four }
        } else if infinitive.ends_with("je") {
            if mono { Class::One } else { Class::Five }
        } else if infinitive.ends_with('y') {
            Class::One
        } else if infinitive.ends_with('a') {
            // The one ending that cannot decide for itself; the mark decides.
            if marked { Class::Six } else { Class::One }
        } else {
            // A citation form in `-tj` whose stem ends in a consonant. Nothing
            // in §7.3 covers it, and class 1's operation is the one that needs
            // no theme vowel to be present.
            Class::One
        };
        Some(Self { infinitive, class })
    }

    /// The present stem (§7.3's second column).
    fn present(&self) -> String {
        let s = &self.infinitive;
        match self.class {
            Class::One => format!("{s}j"),
            Class::Two => format!("{}uj", s.strip_suffix("ova").unwrap_or(s)),
            // The theme vowel drops. For class 5 it is the whole `je`.
            Class::Five => s.strip_suffix("je").unwrap_or(s).to_string(),
            Class::Three | Class::Four => s[..s.len() - 1].to_string(),
            Class::Six => mutate_present_stem(&s[..s.len() - 1]),
        }
    }
}

/// Inflect a verb in one of the three **synthetic** tenses (§7.1).
///
/// `NonPast` is the present for an imperfective and the future for a perfective;
/// the morphology is the same and only the sense differs (§7.8). The perfect,
/// pluperfect and imperfective future are periphrastic and are composed by the
/// caller from [`byti`], [`future_auxiliary`], [`l_participle`] and
/// [`infinitive`].
///
/// ```
/// use ruthenian_core::{verb, FiniteTense::*, Number::*, Person::*};
///
/// // §7.4, class 1: the theme vowel stays and `-j-` is added.
/// assert_eq!(verb("czitatj", First, Singular, NonPast), "czitaju");
/// assert_eq!(verb("czitatj", Second, Singular, NonPast), "czitajeszj");
/// assert_eq!(verb("czitatj", Third, Singular, NonPast), "czitajet");
/// assert_eq!(verb("czitatj", First, Dual, NonPast), "czitajevje");
/// assert_eq!(verb("czitatj", Third, Plural, NonPast), "czitajut");
///
/// // Class 6, which the word-final mark selects.
/// assert_eq!(verb("pisatj'", First, Singular, NonPast), "piszu");
/// assert_eq!(verb("pisatj'", Second, Singular, NonPast), "piszeszj");
/// // Without the mark it is class 1 and a different verb.
/// assert_eq!(verb("pisatj", First, Singular, NonPast), "pisaju");
///
/// // §7.5's aorist: the 2nd and 3rd singular are the bare stem, which is what
/// // makes an aorist recognizable at sight.
/// assert_eq!(verb("czitatj", First, Singular, Aorist), "czitah");
/// assert_eq!(verb("czitatj", Second, Singular, Aorist), "czita");
/// assert_eq!(verb("czitatj", Third, Plural, Aorist), "czitasza");
///
/// // §7.6's imperfect.
/// assert_eq!(verb("czitatj", First, Singular, Imperfect), "czitajah");
/// assert_eq!(verb("czitatj", Second, Singular, Imperfect), "czitajasze");
/// assert_eq!(verb("czitatj", Third, Plural, Imperfect), "czitajahu");
/// ```
pub fn verb(word: &str, person: Person, number: Number, tense: FiniteTense) -> String {
    let Some(v) = Verb::read(word) else {
        return UNREADABLE.to_string();
    };
    match tense {
        FiniteTense::NonPast => present(&v, person, number),
        FiniteTense::Aorist => join(&v.infinitive, aorist_ending(person, number)),
        FiniteTense::Imperfect => join(&v.infinitive, imperfect_ending(person, number)),
    }
}

fn present(v: &Verb, person: Person, number: Number) -> String {
    use Number::*;
    use Person::*;
    let stem = v.present();

    // §7.4's 1sg is `-u` in the 1st conjugation and `-ju` in the 2nd. Where the
    // stem mutates (§7.3, classes 4 and 5), the mutation already supplies the
    // palatal element, so the ending is the bare `-u`: `ljublju`, `vidzzu`, and
    // `govorju` only because `govor-` has nothing to mutate.
    if person == First && number == Singular {
        if !v.class.second_conjugation() {
            return join(&stem, "u");
        }
        if v.class.mutates_in_1sg() {
            let mutated = mutate_present_stem(&stem);
            if mutated != stem {
                return join(&mutated, "u");
            }
        }
        return join(&stem, "ju");
    }

    let ending = match (v.class.second_conjugation(), person, number) {
        (false, Second, Singular) => "jeszj",
        (false, Third, Singular) => "jet",
        (false, First, Dual) => "jevje",
        (false, Second, Dual) => "jeta",
        (false, Third, Dual) => "jetje",
        (false, First, Plural) => "jem",
        (false, Second, Plural) => "jetje",
        (false, Third, Plural) => "ut",
        (true, Second, Singular) => "iszj",
        (true, Third, Singular) => "it",
        (true, First, Dual) => "ivje",
        (true, Second, Dual) => "ita",
        (true, Third, Dual) => "itje",
        (true, First, Plural) => "im",
        (true, Second, Plural) => "itje",
        (true, Third, Plural) => "jat",
        (_, First, Singular) => unreachable!("handled above"),
    };
    join(&stem, ending)
}

/// §7.5, the OCS sigmatic aorist, on the infinitive stem.
fn aorist_ending(person: Person, number: Number) -> &'static str {
    use Number::*;
    use Person::*;
    match (person, number) {
        (First, Singular) => "h",
        // The bare stem — the inherited shape, and the reason an aorist is
        // recognizable at sight.
        (Second | Third, Singular) => "",
        (First, Dual) => "hovje",
        (Second, Dual) => "sta",
        (Third, Dual) => "stje",
        (First, Plural) => "hom",
        (Second, Plural) => "stje",
        (Third, Plural) => "sza",
    }
}

/// §7.6, the imperfect, on the infinitive stem.
fn imperfect_ending(person: Person, number: Number) -> &'static str {
    use Number::*;
    use Person::*;
    match (person, number) {
        (First, Singular) => "jah",
        (Second | Third, Singular) => "jasze",
        (First, Dual) => "jahovje",
        (Second, Dual) => "jaszeta",
        (Third, Dual) => "jaszetje",
        (First, Plural) => "jahom",
        (Second, Plural) => "jaszetje",
        (Third, Plural) => "jahu",
    }
}

/// The imperative (§7.10): the present stem plus `-i`, or the bare stem after
/// `j`.
///
/// **There is no synthetic third person or first singular.** No Slavic language
/// builds one, and §7.10 uses a particle plus the present indicative — `da
/// idjet`, `nehaj idjet`, `pustj idjet`. Asking for those cells returns exactly
/// that present indicative, which is the form the particle attaches to; the
/// caller supplies the particle. It is a declared fallback, listed in
/// [`crate::fallback`].
///
/// ```
/// use ruthenian_core::{imperative, verb, FiniteTense, Number::*, Person::*};
///
/// assert_eq!(imperative("czitatj", Second, Singular), "czitaj");
/// assert_eq!(imperative("czitatj", Second, Dual), "czitajta");
/// assert_eq!(imperative("czitatj", Second, Plural), "czitajtje");
/// assert_eq!(imperative("czitatj", First, Dual), "czitajvje");
/// assert_eq!(imperative("czitatj", First, Plural), "czitajm");
///
/// // A stem that does not end in `j` takes the `-i`.
/// assert_eq!(imperative("govoritj", Second, Singular), "govori");
/// assert_eq!(imperative("pisatj'", Second, Singular), "piszi");
///
/// // The two cells the language builds with a particle instead.
/// assert_eq!(
///     imperative("czitatj", Third, Singular),
///     verb("czitatj", Third, Singular, FiniteTense::NonPast)
/// );
/// ```
pub fn imperative(word: &str, person: Person, number: Number) -> String {
    use Number::*;
    use Person::*;
    let Some(v) = Verb::read(word) else {
        return UNREADABLE.to_string();
    };
    // §7.10 gives no synthetic form here; the present indicative is what the
    // particle attaches to.
    if matches!((person, number), (Third, _) | (First, Singular)) {
        return present(&v, person, number);
    }
    let stem = v.present();
    let base = match stem.ends_with('j') {
        true => stem,
        false => join(&stem, "i"),
    };
    let ending = match (person, number) {
        (Second, Singular) => "",
        (Second, Dual) => "ta",
        (Second, Plural) => "tje",
        (First, Dual) => "vje",
        _ => "m",
    };
    format!("{base}{ending}")
}

/// The infinitive: the citation form itself, less its class mark.
///
/// The mark is morphology rather than sound (§2.1), so it is not part of the
/// word — `pisatj'` is a lemma and `pisatj` is the infinitive of it.
///
/// ```
/// use ruthenian_core::infinitive;
/// assert_eq!(infinitive("czitatj"), "czitatj");
/// assert_eq!(infinitive("pisatj'"), "pisatj");
/// assert_eq!(infinitive("Govoritj"), "govoritj");
/// ```
pub fn infinitive(word: &str) -> String {
    match Verb::read(word) {
        Some(v) => format!("{}tj", v.infinitive),
        None => UNREADABLE.to_string(),
    }
}

/// The `l`-participle (§7.7), which agrees in gender and number.
///
/// It is half of the perfect and the pluperfect, both of which the caller
/// composes: `jesmj czital`, `byh czital`, `bjah czital`. Unlike Russian, the
/// copula is **not** dropped.
///
/// ```
/// use ruthenian_core::{l_participle, Gender::*, Number::*};
///
/// assert_eq!(l_participle("czitatj", Masculine, Singular), "czital");
/// assert_eq!(l_participle("czitatj", Feminine, Singular), "czitala");
/// assert_eq!(l_participle("czitatj", Neuter, Singular), "czitalo");
/// assert_eq!(l_participle("czitatj", Masculine, Dual), "czitala");
/// assert_eq!(l_participle("czitatj", Masculine, Plural), "czitali");
/// ```
pub fn l_participle(word: &str, gender: Gender, number: Number) -> String {
    let Some(v) = Verb::read(word) else {
        return UNREADABLE.to_string();
    };
    let ending = match number {
        // §7.7's dual column is one form for all three genders.
        Number::Dual => "la",
        Number::Plural => "li",
        Number::Singular => match gender {
            Gender::Masculine => "l",
            Gender::Feminine => "la",
            Gender::Neuter => "lo",
        },
    };
    join(&v.infinitive, ending)
}

/// `byti`, the copula (§7.9) — the language's **one** suppletive verb.
///
/// It gets a function of its own rather than an escape hatch in the general
/// path, because its irregularity would otherwise be spread across every stage
/// of that path and nothing would tell a reader there were five sites to find.
/// The cost is a second generation path, which law 2 otherwise forbids; it is
/// tolerable only because this is a closed nine-cell paradigm the specification
/// tabulates in full, checked against §7.9 by the same corpus as everything
/// else.
///
/// `NonPast` is the present `jesmj`, not a present/future blend: the future uses
/// a different root altogether (`bud-` against `jes-`), which is suppletion
/// rather than a tense of one stem, and it lives in [`future_auxiliary`].
///
/// Russian's zero copula (`он врач`) is an East Slavic innovation. Ruthenian
/// follows OCS, Polish and Ukrainian: `on jestj vracz`.
///
/// ```
/// use ruthenian_core::{byti, l_participle, verb, FiniteTense::*, Number::*, Person::*};
///
/// assert_eq!(byti(First, Singular, NonPast), "jesmj");
/// assert_eq!(byti(Second, Singular, NonPast), "jesi");
/// assert_eq!(byti(Third, Singular, NonPast), "jestj");
/// assert_eq!(byti(First, Dual, NonPast), "jesvje");
/// assert_eq!(byti(Third, Plural, NonPast), "sutj");
///
// The aorist is regular, so it is the general path on `bytj` rather than a
/// // second table.
/// assert_eq!(byti(First, Singular, Aorist), "byh");
/// assert_eq!(byti(Second, Singular, Aorist), "by");
/// assert_eq!(byti(Third, Plural, Aorist), "bysza");
/// assert_eq!(byti(First, Singular, Aorist), verb("bytj", First, Singular, Aorist));
/// assert_eq!(l_participle("bytj", ruthenian_core::Gender::Masculine, Singular), "byl");
///
/// assert_eq!(byti(First, Singular, Imperfect), "bjah");
/// assert_eq!(byti(Third, Plural, Imperfect), "bjahu");
/// ```
pub fn byti(person: Person, number: Number, tense: FiniteTense) -> String {
    use FiniteTense::*;
    use Number::*;
    use Person::*;
    // §7.9's own table marks the aorist and the `l`-participle **regular** —
    // §7.5's endings on the stem `by-` — so they are not tabulated here. They go
    // through the general path on the lemma `bytj`, and law 2's second-path
    // exemption is spent only on the present and the imperfect, which are the
    // two rows that table marks irregular.
    if tense == Aorist {
        return verb("bytj", person, number, tense);
    }
    match (tense, person, number) {
        (NonPast, First, Singular) => "jesmj",
        (NonPast, Second, Singular) => "jesi",
        (NonPast, Third, Singular) => "jestj",
        (NonPast, First, Dual) => "jesvje",
        (NonPast, Second, Dual) => "jesta",
        (NonPast, Third, Dual) => "jestje",
        (NonPast, First, Plural) => "jesm",
        (NonPast, Second, Plural) => "jestje",
        (NonPast, Third, Plural) => "sutj",
        (Aorist, ..) => unreachable!("delegated to the general path above"),
        (Imperfect, First, Singular) => "bjah",
        (Imperfect, Second | Third, Singular) => "bjasze",
        (Imperfect, First, Dual) => "bjahovje",
        (Imperfect, Second, Dual) => "bjaszeta",
        (Imperfect, Third, Dual) => "bjaszetje",
        (Imperfect, First, Plural) => "bjahom",
        (Imperfect, Second, Plural) => "bjaszetje",
        (Imperfect, Third, Plural) => "bjahu",
    }
    .to_string()
}

/// `budu` (§7.8) — the auxiliary that builds the imperfective future.
///
/// Its own function rather than a fourth tense of [`byti`], because `bud-` is a
/// different root from `jes-`: the two are suppletively unified, not one stem
/// inflected two ways. `FiniteTense` has no `Future` variant because a regular
/// verb's future is either identical to its `NonPast` (perfective) or two words
/// (imperfective), and neither needs a slot.
///
/// ```
/// use ruthenian_core::{future_auxiliary, infinitive, Number::*, Person::*};
///
/// assert_eq!(future_auxiliary(First, Singular), "budu");
/// assert_eq!(future_auxiliary(Second, Singular), "budjeszj");
/// assert_eq!(future_auxiliary(Third, Plural), "budut");
///
/// // The imperfective future is two words, and the caller joins them.
/// let f = format!("{} {}", future_auxiliary(First, Singular), infinitive("czitatj"));
/// assert_eq!(f, "budu czitatj");
/// ```
pub fn future_auxiliary(person: Person, number: Number) -> String {
    use Number::*;
    use Person::*;
    match (person, number) {
        (First, Singular) => "budu",
        (Second, Singular) => "budjeszj",
        (Third, Singular) => "budjet",
        (First, Dual) => "budjevje",
        (Second, Dual) => "budjeta",
        (Third, Dual) => "budjetje",
        (First, Plural) => "budjem",
        (Second, Plural) => "budjetje",
        (Third, Plural) => "budut",
    }
    .to_string()
}

/// Every synthetic cell of one verb: 3 persons × 3 numbers × 3 tenses.
///
/// Law 2 — this calls [`verb`] rather than computing anything.
///
/// ```
/// use ruthenian_core::verb_paradigm;
/// assert_eq!(verb_paradigm("czitatj").len(), 27);
/// ```
pub fn verb_paradigm(word: &str) -> Vec<(Person, Number, FiniteTense, String)> {
    let mut out = Vec::with_capacity(27);
    for tense in FiniteTense::ALL {
        for number in Number::ALL {
            for person in Person::ALL {
                out.push((person, number, tense, verb(word, person, number, tense)));
            }
        }
    }
    out
}

// --- §7.12 participles and gerunds ------------------------------------------

impl Verb {
    /// One syllable in the infinitive stem — the vowel *is* the root (§7.3).
    fn monosyllabic(&self) -> bool {
        vowels(&self.infinitive) == 1
    }
}

/// The **present active** participle stem (§7.12): `-uszcz-` in the 1st
/// conjugation, `-jaszcz-` in the 2nd, on the present stem.
///
/// A participle is a derivation, not a cell: it returns an adjective **stem**
/// that declines through [`crate::adjective`] and
/// [`crate::short_adjective`] like any other, in both the long and the short
/// form. That is what keeps the adjective API at two entry points.
///
/// ```
/// use ruthenian_core::{present_active_participle as pap, adjective};
/// use ruthenian_core::{Case, Number, Gender, Animacy::Inanimate};
///
/// assert_eq!(pap("czitatj"), "czitajuszcz");
/// assert_eq!(pap("govoritj"), "govorjaszcz");
///
/// // and it declines
/// let long = adjective(&pap("czitatj"), Case::Nominative, Number::Singular,
///                      Gender::Masculine, Inanimate);
/// assert_eq!(long, "czitajuszczij");
/// ```
pub fn present_active_participle(word: &str) -> String {
    let Some(v) = Verb::read(word) else {
        return UNREADABLE.to_string();
    };
    let suffix = match v.class.second_conjugation() {
        true => "jaszcz",
        false => "uszcz",
    };
    join(&v.present(), suffix)
}

/// The **past active** participle stem (§7.12): `-vsz-` on the infinitive stem.
///
/// ```
/// use ruthenian_core::{past_active_participle as pap, adjective};
/// use ruthenian_core::{Case, Number, Gender, Animacy::Inanimate};
///
/// assert_eq!(pap("czitatj"), "czitavsz");
/// let long = adjective(&pap("czitatj"), Case::Nominative, Number::Singular,
///                      Gender::Masculine, Inanimate);
/// assert_eq!(long, "czitavszij");
/// ```
pub fn past_active_participle(word: &str) -> String {
    match Verb::read(word) {
        Some(v) => join(&v.infinitive, "vsz"),
        None => UNREADABLE.to_string(),
    }
}

/// The **present passive** participle stem (§7.12): `-jem-` in the 1st
/// conjugation, `-im-` in the 2nd, on the present stem.
///
/// ```
/// use ruthenian_core::{present_passive_participle as ppp, adjective};
/// use ruthenian_core::{Case, Number, Gender, Animacy::Inanimate};
///
/// assert_eq!(ppp("czitatj"), "czitajem");
/// assert_eq!(ppp("ljubitj"), "ljubim");
///
/// let long = adjective(&ppp("czitatj"), Case::Nominative, Number::Singular,
///                      Gender::Masculine, Inanimate);
/// assert_eq!(long, "czitajemyj");
/// ```
pub fn present_passive_participle(word: &str) -> String {
    let Some(v) = Verb::read(word) else {
        return UNREADABLE.to_string();
    };
    let suffix = match v.class.second_conjugation() {
        true => "im",
        false => "jem",
    };
    join(&v.present(), suffix)
}

/// The **past passive** participle stem (§7.12): `-n-`, `-jen-` or `-t-`,
/// decided by the class and not stored.
///
/// §7.12 says the class decides but does not say how. The division is the one
/// Russian draws, and it falls out of what each class does to its stem:
///
/// | | suffix | on | because |
/// |---|---|---|---|
/// | classes 4 and 5 | `-jen` | the **present** stem | the theme vowel is gone, so the suffix meets the bare stem |
/// | class 3, and a monosyllabic class 1 | `-t` | the infinitive stem | there is no theme vowel to carry an `-n` |
/// | otherwise | `-n` | the infinitive stem | the theme vowel is there to carry it |
///
/// **The `n` is single, not doubled.** Russian writes `-nnyj` long against `-n`
/// short, which gives the two forms different stems; Ruthenian writes one `n`
/// throughout, so the participle is a plain adjective stem and nothing is told
/// apart by the difference.
///
/// ```
/// use ruthenian_core::{past_passive_participle as ppp, adjective, short_adjective};
/// use ruthenian_core::{Case, Number, Gender, Animacy::Inanimate};
///
/// // §7.12's own three examples.
/// assert_eq!(ppp("poczitatj"), "poczitan");
/// assert_eq!(ppp("rjeszitj"), "rjeszen");
/// assert_eq!(ppp("bitj"), "bit");
///
/// // -t also for the -nutj class, which has no theme vowel to carry an -n.
/// assert_eq!(ppp("dvinutj"), "dvinut");
/// assert_eq!(ppp("vidjetj"), "vidjen");
///
/// // One stem, so the long and short forms are the same adjective.
/// let s = ppp("poczitatj");
/// let long = adjective(&s, Case::Nominative, Number::Singular, Gender::Masculine, Inanimate);
/// let short = short_adjective(&s, Case::Nominative, Number::Singular, Gender::Masculine, Inanimate);
/// assert_eq!((long.as_str(), short.as_str()), ("poczitanyj", "poczitan"));
/// ```
pub fn past_passive_participle(word: &str) -> String {
    let Some(v) = Verb::read(word) else {
        return UNREADABLE.to_string();
    };
    match v.class {
        // The theme vowel is gone, so the suffix meets the bare present stem.
        // `rjesz` + `-jen` is `rjeszen` by §3.8's rule 2, with no special case.
        Class::Four | Class::Five => join(&v.present(), "jen"),
        // No theme vowel to carry an `-n`.
        Class::Three => join(&v.infinitive, "t"),
        Class::One if v.monosyllabic() => join(&v.infinitive, "t"),
        _ => join(&v.infinitive, "n"),
    }
}

/// The **present gerund** (§7.12): `-ja` on the present stem.
///
/// Gerunds are indeclinable, so this returns a finished form rather than a stem.
///
/// ```
/// use ruthenian_core::present_gerund;
/// assert_eq!(present_gerund("czitatj"), "czitaja");
/// assert_eq!(present_gerund("govoritj"), "govorja");
/// ```
pub fn present_gerund(word: &str) -> String {
    match Verb::read(word) {
        Some(v) => join(&v.present(), "ja"),
        None => UNREADABLE.to_string(),
    }
}

/// The **past gerund** (§7.12): `-v` on the infinitive stem.
///
/// ```
/// use ruthenian_core::past_gerund;
/// assert_eq!(past_gerund("czitatj"), "czitav");
/// assert_eq!(past_gerund("govoritj"), "govoriv");
/// ```
pub fn past_gerund(word: &str) -> String {
    match Verb::read(word) {
        Some(v) => join(&v.infinitive, "v"),
        None => UNREADABLE.to_string(),
    }
}
