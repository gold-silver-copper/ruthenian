//! Numerals: §6, regularized to one rule per rank.
//!
//! **This is the one module that holds word data, and the laws allow it here.**
//! "Not a dictionary" is about open classes — a fact about one noun does not
//! belong in a crate that inflects every noun. The numerals are a *closed* class
//! the specification enumerates in full, so the units below are the same kind of
//! thing as the ending tables: a finite list the language itself fixes.
//!
//! Everything above nine is built, not listed:
//!
//! | rank | rule | |
//! |---|---|---|
//! | teens | unit stem + `nadjesjat` | `pjatnadjesjat` |
//! | tens | unit + `djesjat` | `pjatjdjesjat` |
//! | hundreds | unit + `sto` | `pjatjsto` |
//! | thousands up | a **noun**, governed by the count | `pjatj tysjaczov` |
//!
//! Russian's `сорок` and `девяносто` are lexical oddities with no transparent
//! structure, and `-дцать` is a second tens formation beside `-десят`; Ruthenian
//! keeps neither.

use crate::adjective::adjective;
use crate::fallback::UNREADABLE;
use crate::grammar::{Animacy, Case, Gender, Number};
use crate::noun::noun;

/// 0–10. Everything else is composed from these.
const UNITS: [&str; 11] = [
    "nolj", "odin", "dva", "tri", "czetyrje", "pjatj", "szestj", "sjedmj", "osmj", "djevjatj",
    "djesjatj",
];

/// The scale nouns, largest first, with `u64`'s ceiling in view.
///
/// **Short scale**, as English uses: each step is a thousand times the last, so
/// `biljon` is 10⁹ and not 10¹². The long-scale `miljard` that §6.3 had is gone
/// with it. `q` is not in the alphabet (§2.1), so *quadrillion* is `kvadriljon`.
const SCALES: [(u64, &str); 6] = [
    (1_000_000_000_000_000_000, "kvintiljon"),
    (1_000_000_000_000_000, "kvadriljon"),
    (1_000_000_000_000, "triljon"),
    (1_000_000_000, "biljon"),
    (1_000_000, "miljon"),
    (1_000, "tysjacza"),
];

/// The combining stem of a unit: the cardinal less a final `j` or `je`.
///
/// Only the teens use it — `pjatj` + `nadjesjat` is `pjatnadjesjat` — because
/// the tens and hundreds take the unit whole (`pjatjdjesjat`, `pjatjsto`).
fn unit_stem(n: u64) -> String {
    let u = UNITS[n as usize];
    u.strip_suffix("je")
        .or_else(|| u.strip_suffix('j'))
        .unwrap_or(u)
        .to_string()
}

/// The nominative of a number below 1 000, as one or more words.
fn under_thousand(n: u64, gender: Gender) -> Vec<String> {
    let mut out = Vec::new();
    let (hundreds, rest) = (n / 100, n % 100);
    if hundreds > 0 {
        out.push(match hundreds {
            1 => "sto".to_string(),
            // `dvje` rather than `dva`: `sto` was historically a dual here, and
            // Russian's `двести` and Interslavic's `dvěsto` both keep it.
            2 => "dvjesto".to_string(),
            h => format!("{}sto", UNITS[h as usize]),
        });
    }
    let (tens, units) = (rest / 10, rest % 10);
    match tens {
        0 => {}
        1 if units == 0 => out.push(UNITS[10].to_string()),
        // Teens: the unit's stem carries `nadjesjat`.
        1 => out.push(format!("{}nadjesjat", unit_stem(units))),
        t => out.push(format!("{}djesjat", UNITS[t as usize])),
    }
    if units > 0 && tens != 1 {
        out.push(gendered(units, gender));
    }
    out
}

/// `odin` and `dva` are the only cardinals that agree in gender.
fn gendered(n: u64, gender: Gender) -> String {
    match (n, gender) {
        (2, Gender::Feminine) => "dvje".to_string(),
        _ => UNITS[n as usize].to_string(),
    }
}

/// Which case a count word puts its noun in (§6.1).
fn governs(count: u64) -> (Case, Number) {
    match (count % 100, count % 10) {
        // The teens are not counted by their last digit — there is no 11–14
        // exception in §6.1 because five and above always take the genitive
        // plural, and a teen is above five.
        (11..=19, _) => (Case::Genitive, Number::Plural),
        (_, 1) => (Case::Nominative, Number::Singular),
        (_, 2) => (Case::Nominative, Number::Dual),
        (_, 3 | 4) => (Case::Nominative, Number::Plural),
        _ => (Case::Genitive, Number::Plural),
    }
}

/// A cardinal numeral (§6).
///
/// **Only the last word of a compound declines.** §6.1 makes the *government* of
/// a compound its last word's, and the declension follows it: `dvadjesjat pjati`
/// rather than Russian's `двадцати пяти`, where every part inflects. That is the
/// same trade §6.3 makes with `сорок` — a rule instead of a table.
///
/// ```
/// use ruthenian_core::{numeral, Case, Gender::Masculine as M, Animacy::Inanimate as In};
/// use Case::*;
///
/// // 0–10
/// assert_eq!(numeral(0, Nominative, M, In), "nolj");
/// assert_eq!(numeral(4, Nominative, M, In), "czetyrje");
/// assert_eq!(numeral(10, Nominative, M, In), "djesjatj");
///
/// // one rule per rank
/// assert_eq!(numeral(15, Nominative, M, In), "pjatnadjesjat");
/// assert_eq!(numeral(20, Nominative, M, In), "dvadjesjat");
/// assert_eq!(numeral(40, Nominative, M, In), "czetyrjedjesjat");
/// assert_eq!(numeral(90, Nominative, M, In), "djevjatjdjesjat");
/// assert_eq!(numeral(200, Nominative, M, In), "dvjesto");
/// assert_eq!(numeral(500, Nominative, M, In), "pjatjsto");
///
/// // compounds are spaced, and the last word alone declines
/// assert_eq!(numeral(132, Nominative, M, In), "sto tridjesjat dva");
/// assert_eq!(numeral(25, Nominative, M, In), "dvadjesjat pjatj");
/// assert_eq!(numeral(25, Genitive, M, In), "dvadjesjat pjati");
///
/// // `dva` alone agrees in gender
/// assert_eq!(numeral(2, Nominative, ruthenian_core::Gender::Feminine, In), "dvje");
///
/// // the scale words are nouns, governed by their count
/// assert_eq!(numeral(1_000, Nominative, M, In), "tysjacza");
/// assert_eq!(numeral(2_000, Nominative, M, In), "dvje tysjaczi"); // tysjacza is feminine
/// assert_eq!(numeral(5_000, Nominative, M, In), "pjatj tysjaczov");
/// assert_eq!(numeral(1_000_000, Nominative, M, In), "miljon");
/// assert_eq!(numeral(3_000_000, Nominative, M, In), "tri miljony");
/// ```
pub fn numeral(value: u64, case: Case, gender: Gender, animacy: Animacy) -> String {
    // Zero is a word and not an absence of words: `nolj`, which §6 did not have
    // and which declines as the soft masculine it looks like.
    if value == 0 {
        return noun("nolj", case, Number::Singular);
    }
    let mut words: Vec<String> = Vec::new();
    let mut rest = value;

    for (unit, name) in SCALES {
        let count = rest / unit;
        if count == 0 {
            continue;
        }
        rest %= unit;
        // `tysjacza` and `miljon` are nouns: the count governs them (§6.1), and
        // a count of exactly one is left implicit, as `сто` and `тысяча` are.
        if count > 1 {
            words.extend(under_thousand(count, scale_gender(name)));
        }
        let (c, n) = governs(count);
        words.push(noun(name, c, n));
    }
    if rest > 0 {
        words.extend(under_thousand(rest, gender));
    }

    // Only the last word inflects.
    if let Some(last) = words.pop() {
        words.push(decline(&last, value, case, gender, animacy));
    }
    words.join(" ")
}

/// `tysjacza` is feminine, so a count of two before it is `dvje`.
fn scale_gender(name: &str) -> Gender {
    match name {
        "tysjacza" => Gender::Feminine,
        _ => Gender::Masculine,
    }
}

/// Decline one numeral word.
fn decline(word: &str, value: u64, case: Case, gender: Gender, animacy: Animacy) -> String {
    use Case::*;
    // The scale nouns decline as the nouns they are.
    if SCALES.iter().any(|(_, n)| word.starts_with(n)) {
        let (_, n) = governs(value / scale_of(word));
        return noun(name_of(word), case, n);
    }
    match word {
        // §6.4: `odin` declines as a long adjective and agrees throughout. Its
        // masculine nominative is the bare stem, the way `tot`'s is — see the
        // note in `README.md` on the invariant stem.
        "odin" => match (case, gender) {
            (Nominative, Gender::Masculine) => "odin".to_string(),
            (Accusative, Gender::Masculine) if animacy == Animacy::Inanimate => "odin".to_string(),
            _ => adjective("odin", case, Number::Singular, gender, animacy),
        },
        // §6.4: `dva` is a dual form and takes the dual endings — the plain
        // nominal ones, so `dva` / `dvu` / `dvoma` exactly as `dom` has
        // `doma` / `domu` / `domoma`.
        "dva" | "dvje" => match case {
            Nominative | Vocative => word.to_string(),
            Accusative if animacy == Animacy::Inanimate => word.to_string(),
            Genitive | Locative | Accusative => "dvu".to_string(),
            _ => "dvoma".to_string(),
        },
        "tri" => plural_numeral("tri", "trj", case, animacy),
        "czetyrje" => plural_numeral("czetyrje", "czetyrj", case, animacy),
        // Everything else — five and up, and every built rank — is a
        // declension III noun (§6.4), which is what the higher numerals were in
        // OCS and still behave like.
        other => third_declension(other, case),
    }
}

/// `tri` and `czetyrje` decline as plurals (§6.4).
fn plural_numeral(nom: &str, stem: &str, case: Case, animacy: Animacy) -> String {
    use Case::*;
    match case {
        Nominative | Vocative => nom.to_string(),
        // §3.7: an animate accusative plural is the genitive.
        Accusative if animacy == Animacy::Inanimate => nom.to_string(),
        Genitive | Locative | Accusative => format!("{stem}eh"),
        Dative | Ablative => format!("{stem}em"),
        Instrumental => match stem {
            // `czetyrjmi` against `trjemi`: the stems differ, as Russian's
            // `четырьмя` and `тремя` do.
            "czetyrj" => "czetyrjmi".to_string(),
            _ => format!("{stem}emi"),
        },
    }
}

/// Declension III on a numeral's own stem (§6.4, §3.6).
fn third_declension(nom: &str, case: Case) -> String {
    use Case::*;
    let stem = nom.strip_suffix('j').unwrap_or(nom);
    match case {
        Nominative | Accusative => nom.to_string(),
        Instrumental => format!("{stem}jju"),
        _ => format!("{stem}i"),
    }
}

fn scale_of(word: &str) -> u64 {
    SCALES
        .iter()
        .find(|(_, n)| word.starts_with(n))
        .map(|(u, _)| *u)
        .unwrap_or(1)
}

fn name_of(word: &str) -> &'static str {
    SCALES
        .iter()
        .find(|(_, n)| word.starts_with(n))
        .map(|(_, n)| *n)
        .unwrap_or("nolj")
}

/// An ordinal's **adjective stem** (§6.5).
///
/// A derivation, like the participles: the result declines through
/// [`crate::adjective`] and [`crate::short_adjective`], long or short as §6.5
/// says. From five up it is the cardinal less its final `j`; one to four are
/// suppletive and are listed, which is what a closed class allows.
///
/// ```
/// use ruthenian_core::{ordinal, adjective, Case, Number, Gender, Animacy::Inanimate};
///
/// assert_eq!(ordinal(1), "pjerv");
/// assert_eq!(ordinal(2), "vtor");
/// assert_eq!(ordinal(5), "pjat");
/// assert_eq!(ordinal(100), "sot");
///
/// let long = adjective(&ordinal(5), Case::Nominative, Number::Singular,
///                      Gender::Masculine, Inanimate);
/// assert_eq!(long, "pjatyj");
/// ```
pub fn ordinal(value: u64) -> String {
    match value {
        1 => "pjerv".to_string(),
        2 => "vtor".to_string(),
        3 => "trjet".to_string(),
        4 => "czetvjert".to_string(),
        100 => "sot".to_string(),
        1_000 => "tysjaczn".to_string(),
        // From five up the cardinal supplies the stem directly.
        _ => {
            let card = numeral(
                value,
                Case::Nominative,
                Gender::Masculine,
                Animacy::Inanimate,
            );
            if card == UNREADABLE {
                return UNREADABLE.to_string();
            }
            let last = card.rsplit(' ').next().unwrap_or(&card);
            let stem = last.strip_suffix('j').unwrap_or(last).to_string();
            match card.rsplit_once(' ') {
                Some((head, _)) => format!("{head} {stem}"),
                None => stem,
            }
        }
    }
}
