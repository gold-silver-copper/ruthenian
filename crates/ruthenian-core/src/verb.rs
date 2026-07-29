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
use crate::grammar::{Gender, Number, Person};
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

/// Inflect a verb in the **non-past**, the language's one synthetic tense (§7.1).
///
/// It is the present for an imperfective and the future for a perfective: the
/// morphology is the same and only the sense differs (§7.8), which is why there
/// is no `Present` and no `Future`, and no tense parameter either.
///
/// **Every past is periphrastic.** The perfect is [`bytj`] plus
/// [`l_participle`], the pluperfect the same with `byl` between them, and the
/// imperfective future [`future_auxiliary`] plus [`infinitive`] — all composed
/// by the caller, since composing them here would mean doing agreement and word
/// order, which is syntax.
///
/// ```
/// use ruthenian_core::{verb, bytj, l_participle, Gender, Number::*, Person::*};
///
/// // §7.4, class 1: the theme vowel stays and `-j-` is added.
/// assert_eq!(verb("czitatj", First, Singular), "czitaju");
/// assert_eq!(verb("czitatj", Second, Singular), "czitajesz");
/// assert_eq!(verb("czitatj", Third, Singular), "czitajet");
/// assert_eq!(verb("czitatj", First, Dual), "czitajevje");
/// assert_eq!(verb("czitatj", Third, Plural), "czitajut");
///
/// // Class 6, which the word-final mark selects.
/// assert_eq!(verb("pisatj'", First, Singular), "piszu");
/// assert_eq!(verb("pisatj'", Second, Singular), "piszesz");
/// // Without the mark it is class 1 and a different verb.
/// assert_eq!(verb("pisatj", First, Singular), "pisaju");
///
/// // The past is two words, and the caller joins them.
/// let perfect = format!(
///     "{} {}",
///     bytj(First, Singular),
///     l_participle("czitatj", Gender::Masculine, Singular)
/// );
/// assert_eq!(perfect, "jesm czital");
/// ```
pub fn verb(word: &str, person: Person, number: Number) -> String {
    match Verb::read(word) {
        Some(v) => present(&v, person, number),
        None => UNREADABLE.to_string(),
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

    let col = usize::from(v.class.second_conjugation());
    let Some((ending, _)) = crate::dsl::lookup(NON_PAST, (person, number), col) else {
        // The 1sg row is deliberately absent — handled above — so this is
        // unreachable for every real query; UNREADABLE is the declared
        // fallback, and the corpus walks all nine cells of both conjugations.
        return UNREADABLE.to_string();
    };
    join(&stem, ending)
}

crate::dsl::table! {
    /// §7.3/§7.4 — the non-past endings, one column per conjugation.
    ///
    /// The theme vowel is the whole difference: `-je-` against `-i-`, with the
    /// third plural `-ut` against `-jat`. The 1sg row is **absent by design**:
    /// it is `-u`/`-ju` decided by the stem's own mutation (§7.4), which is
    /// logic about the stem, not a cell of this table.
    pub const NON_PAST: [(Person, Number); 2] = [
        //                                        1st        2nd
        (Person::Second, Number::Singular) =>  "jesz",   "isz";
        (Person::Third,  Number::Singular) =>  "jet",     "it";
        (Person::First,  Number::Dual)     =>  "jevje",   "ivje";
        (Person::Second, Number::Dual)     =>  "jeta",    "ita";
        (Person::Third,  Number::Dual)     =>  "jetje",   "itje";
        (Person::First,  Number::Plural)   =>  "jemy",    "imy";
        (Person::Second, Number::Plural)   =>  "jetje",   "itje";
        (Person::Third,  Number::Plural)   =>  "ut",      "jat";
    ];
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
/// use ruthenian_core::{imperative, verb, Number::*, Person::*};
///
/// assert_eq!(imperative("czitatj", Second, Singular), "czitaj");
/// assert_eq!(imperative("czitatj", Second, Dual), "czitajta");
/// assert_eq!(imperative("czitatj", Second, Plural), "czitajtje");
/// assert_eq!(imperative("czitatj", First, Dual), "czitajvje");
/// assert_eq!(imperative("czitatj", First, Plural), "czitajmy");
///
/// // A stem that does not end in `j` takes the `-i`.
/// assert_eq!(imperative("govoritj", Second, Singular), "govori");
/// assert_eq!(imperative("pisatj'", Second, Singular), "piszi");
///
/// // The two cells the language builds with a particle instead.
/// assert_eq!(imperative("czitatj", Third, Singular), verb("czitatj", Third, Singular));
/// ```
pub fn imperative(word: &str, person: Person, number: Number) -> String {
    use Number::*;
    use Person::*;
    let Some(v) = Verb::read(word) else {
        return UNREADABLE.to_string();
    };
    // §7.9's imperative is `bud-`, a root the present stem `byj-` cannot reach:
    // `bytj` is suppletive, and this is the one cell of it the general path
    // would otherwise get wrong (`byj` for `budj`).
    if v.infinitive == "by" {
        return match (person, number) {
            (Second, Singular) => "budj".to_string(),
            (Second, Dual) => "budjta".to_string(),
            (Second, Plural) => "budjtje".to_string(),
            (First, Dual) => "budjvje".to_string(),
            (First, Plural) => "budjmy".to_string(),
            _ => bytj(person, number),
        };
    }
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
    let Some((ending, _)) = crate::dsl::lookup(IMPERATIVE, (person, number), 0) else {
        // Third persons and the 1sg returned above as the present indicative.
        return UNREADABLE.to_string();
    };
    format!("{base}{ending}")
}

crate::dsl::table! {
    /// §7.10 — the five synthetic imperative cells, on the imperative base.
    ///
    /// The base is the present stem plus `-i`, or the bare stem after `j`. The
    /// third persons and the 1sg have no row: no Slavic language builds them,
    /// and the declared fallback is the present indicative the particle
    /// attaches to (`da idjet`).
    pub const IMPERATIVE: [(Person, Number); 1] = [
        (Person::Second, Number::Singular) =>  "";
        (Person::Second, Number::Dual)     =>  "ta";
        (Person::Second, Number::Plural)   =>  "tje";
        (Person::First,  Number::Dual)     =>  "vje";
        (Person::First,  Number::Plural)   =>  "my";
    ];
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
/// composes: `jesm czital`, and `jesm byl czital`. Unlike Russian, the
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

/// `bytj`, the copula (§7.9) — the language's **one** suppletive verb, in the
/// present.
///
/// It gets functions of its own rather than an escape hatch in the general path,
/// because its irregularity would otherwise be spread across every stage of that
/// path and nothing would tell a reader there were five sites to find.
///
/// **Two roots, two functions.** `jes-` is the present and `bud-` the future
/// ([`future_auxiliary`]). They are suppletively unified rather than one stem
/// inflected two ways, so naming them separately reflects what they are — and
/// with no synthetic past left in the language (§7.1) there is no tense
/// parameter for them to share.
///
/// There is no past form. `bjah` went with the rest of the synthetic past, and
/// the pluperfect is composed instead from this verb's own `l`-participle:
///
/// Russian's zero copula (`он врач`) is an East Slavic innovation. Ruthenian
/// follows OCS, Polish and Ukrainian: `on jest vracz`.
///
/// ```
/// use ruthenian_core::{bytj, l_participle, Gender::Masculine, Number::*, Person::*};
///
/// assert_eq!(bytj(First, Singular), "jesm");
/// assert_eq!(bytj(Second, Singular), "jesesz");
/// assert_eq!(bytj(Third, Singular), "jest");
/// assert_eq!(bytj(First, Dual), "jesvje");
/// assert_eq!(bytj(Third, Plural), "jesut");
///
/// // perfect: the copula and the verb's participle
/// let perfect = format!("{} {}", bytj(First, Singular), l_participle("czitatj", Masculine, Singular));
/// assert_eq!(perfect, "jesm czital");
///
/// // pluperfect: this verb's own participle stands between them
/// let pluperfect = format!(
///     "{} {} {}",
///     bytj(First, Singular),
///     l_participle("bytj", Masculine, Singular),
///     l_participle("czitatj", Masculine, Singular),
/// );
/// assert_eq!(pluperfect, "jesm byl czital");
/// ```
pub fn bytj(person: Person, number: Number) -> String {
    use Number::*;
    use Person::*;
    match (person, number) {
        (First, Singular) => "jesm",
        (Second, Singular) => "jesesz",
        (Third, Singular) => "jest",
        (First, Dual) => "jesvje",
        (Second, Dual) => "jesta",
        (Third, Dual) => "jestje",
        (First, Plural) => "jesmy",
        (Second, Plural) => "jestje",
        (Third, Plural) => "jesut",
    }
    .to_string()
}

/// `budu` (§7.8) — the auxiliary that builds the imperfective future.
///
/// Its own function rather than a second tense of [`bytj`], because `bud-` is a
/// different root from `jes-`: the two are suppletively unified, not one stem
/// inflected two ways. There is no `Future` tense because a regular
/// verb's future is either identical to its `NonPast` (perfective) or two words
/// (imperfective), and neither needs a slot.
///
/// ```
/// use ruthenian_core::{future_auxiliary, infinitive, Number::*, Person::*};
///
/// assert_eq!(future_auxiliary(First, Singular), "budu");
/// assert_eq!(future_auxiliary(Second, Singular), "budjesz");
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
        (Second, Singular) => "budjesz",
        (Third, Singular) => "budjet",
        (First, Dual) => "budjevje",
        (Second, Dual) => "budjeta",
        (Third, Dual) => "budjetje",
        (First, Plural) => "budjemy",
        (Second, Plural) => "budjetje",
        (Third, Plural) => "budut",
    }
    .to_string()
}

/// Every synthetic cell of one verb: 3 persons × 3 numbers.
///
/// Law 2 — this calls [`verb`] rather than computing anything.
///
/// ```
/// use ruthenian_core::verb_paradigm;
/// assert_eq!(verb_paradigm("czitatj").len(), 9);
/// ```
pub fn verb_paradigm(word: &str) -> Vec<(Person, Number, String)> {
    let mut out = Vec::with_capacity(9);
    for number in Number::ALL {
        for person in Person::ALL {
            out.push((person, number, verb(word, person, number)));
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
