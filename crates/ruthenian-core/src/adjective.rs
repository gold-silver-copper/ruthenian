//! Adjectives: §4's two declensions, and §4.3's degrees.
//!
//! **Two paradigms, so two functions** (`DIRECTION.md`). The long form is
//! definite and declines pronominally; the short form is indefinite and declines
//! as a noun. They are different tables, not two cells of one.
//!
//! The opposition is **definiteness**, the only one Ruthenian has — there is no
//! article. Unlike Russian, the short form is not restricted to the predicate:
//! `on jestj dobr` "he is good" against `on jestj dobryj` "he is the good one".
//!
//! The word passed in is the **stem**, which is also the short masculine
//! nominative — `dobr`. That is the same convention as nouns, where §2.5 makes
//! the citation form the stem.

use crate::fallback::UNREADABLE;
use crate::grammar::{Animacy, Case, Gender, Number};
use crate::spelling::{Palatal, join, palatalize};

/// Is this a usable adjective stem?
///
/// Adjectives take no word-final mark and no animacy capital of their own — they
/// agree with a head noun, so both facts come from elsewhere — but the stem must
/// still be a word (§2.1's alphabet, and a vowel).
fn stem(word: &str) -> Option<String> {
    let parsed = ruthenian_orthography::Ruthenian::parse(word).ok()?;
    let bare = parsed.word().to_lowercase();
    let ok = !bare.is_empty()
        && bare.chars().all(|c| c.is_ascii_alphabetic() || c == '\'')
        && bare.chars().any(|c| "aeiouy".contains(c));
    ok.then_some(bare)
}

/// The **short**, indefinite adjective — the nominal declension (§4.1).
///
/// Its endings are the noun's exactly, so this restates nothing: it calls the
/// same table `noun` does, including the animate accusative, which belongs to
/// the nominal declension rather than to nouns as a word class.
///
/// ```
/// use ruthenian_core::{short_adjective as short, Case, Number, Gender, Animacy};
/// use Animacy::Inanimate as In;
///
/// // §4.1, the masculine column
/// assert_eq!(short("dobr", Case::Nominative, Number::Singular, Gender::Masculine, In), "dobr");
/// assert_eq!(short("dobr", Case::Vocative, Number::Singular, Gender::Masculine, In), "dobrje");
/// assert_eq!(short("dobr", Case::Genitive, Number::Singular, Gender::Masculine, In), "dobrogo");
/// assert_eq!(short("dobr", Case::Locative, Number::Singular, Gender::Masculine, In), "dobri");
///
/// // the neuter and feminine columns
/// assert_eq!(short("dobr", Case::Nominative, Number::Singular, Gender::Neuter, In), "dobro");
/// assert_eq!(short("dobr", Case::Nominative, Number::Singular, Gender::Feminine, In), "dobra");
/// assert_eq!(short("dobr", Case::Instrumental, Number::Singular, Gender::Feminine, In), "dobroj");
///
/// // The short form marks animacy too (§4.2's note): acc = ablative in the
/// // singular, genitive in the plural.
/// let anim = Animacy::Animate;
/// assert_eq!(short("dobr", Case::Accusative, Number::Singular, Gender::Masculine, anim), "dobra");
/// assert_eq!(short("dobr", Case::Accusative, Number::Plural, Gender::Masculine, anim), "dobrov");
/// ```
pub fn short_adjective(
    word: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    let Some(s) = stem(word) else {
        return UNREADABLE.to_string();
    };
    let (ending, palatal) = crate::noun::nominal(gender, case, number, animacy);
    join(&palatalize(&s, palatal), ending)
}

/// The **long**, definite adjective — the pronominal declension (§4.2).
///
/// ```
/// use ruthenian_core::{adjective as long, Case, Number, Gender, Animacy};
/// use Animacy::Inanimate as In;
///
/// assert_eq!(long("dobr", Case::Nominative, Number::Singular, Gender::Masculine, In), "dobryj");
/// assert_eq!(long("dobr", Case::Nominative, Number::Singular, Gender::Neuter, In), "dobroje");
/// assert_eq!(long("dobr", Case::Nominative, Number::Singular, Gender::Feminine, In), "dobraja");
/// assert_eq!(long("dobr", Case::Dative, Number::Singular, Gender::Masculine, In), "dobromu");
/// assert_eq!(long("dobr", Case::Instrumental, Number::Singular, Gender::Masculine, In), "dobrym");
/// assert_eq!(long("dobr", Case::Nominative, Number::Plural, Gender::Masculine, In), "dobryje");
/// assert_eq!(long("dobr", Case::Genitive, Number::Plural, Gender::Masculine, In), "dobryh");
///
/// // The feminine singular is one form in five cells.
/// for case in [Case::Genitive, Case::Ablative, Case::Dative, Case::Instrumental, Case::Locative] {
///     assert_eq!(long("dobr", case, Number::Singular, Gender::Feminine, In), "dobroj");
/// }
///
/// // Long adjectives have no vocative: the nominative is used (§4.2).
/// assert_eq!(
///     long("dobr", Case::Vocative, Number::Singular, Gender::Masculine, In),
///     long("dobr", Case::Nominative, Number::Singular, Gender::Masculine, In)
/// );
/// ```
pub fn adjective(
    word: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    let Some(s) = stem(word) else {
        return UNREADABLE.to_string();
    };
    join(&s, pronominal(case, number, gender, animacy))
}

/// §4.2's endings. The pronominal declension takes **no** palatalization: every
/// ending begins with a vowel that is not the yat-derived `-i`, so §3.8's rules 4
/// and 5 have no environment here.
///
/// Gender is a distinction in the singular only — §4.2 and §5.4 both give one
/// dual column and one plural column.
fn pronominal(case: Case, number: Number, gender: Gender, animacy: Animacy) -> &'static str {
    use Case::*;
    use Number::*;

    // §4.2: long adjectives have no vocative, the nominative is used.
    let case = if case == Vocative { Nominative } else { case };
    // §3.1: the ablative is distinct only in the singular.
    let case = if case == Ablative && number != Singular {
        Dative
    } else {
        case
    };

    match number {
        Dual => match case {
            Nominative | Accusative => "aja",
            // `-oju`, not `-u`: every o-initial ending is shared with the
            // pronominal declension, and this was the one cell where the two
            // disagreed — the nominal dual `domu`, borrowed by mistake.
            Genitive | Locative => "oju",
            _ => "yma",
        },
        Plural => match case {
            // §3.7: animate accusative plural = genitive.
            Accusative if animacy == Animacy::Animate => "yh",
            Nominative | Accusative => "yje",
            Genitive | Locative => "yh",
            Instrumental => "ymi",
            _ => "ym",
        },
        Singular => match gender {
            Gender::Feminine => match case {
                Nominative => "aja",
                Accusative => "uju",
                // One form for the genitive, ablative, dative, instrumental and
                // locative — the pronominal feminine's whole oblique singular.
                _ => "oj",
            },
            g => match case {
                // §3.7: animate accusative singular = ablative, which for this
                // declension is `-a`. §4.2's note: the long and short forms
                // coincide in exactly this cell, both giving `dobra`.
                Accusative if animacy == Animacy::Animate => "a",
                Nominative | Accusative => {
                    if g == Gender::Neuter {
                        "oje"
                    } else {
                        "yj"
                    }
                }
                Genitive => "ogo",
                Ablative => "a",
                Dative => "omu",
                Instrumental => "ym",
                _ => "om",
            },
        },
    }
}

/// The comparative stem (§4.3): `-jejsz-`, triggering the **first**
/// palatalization.
///
/// A derivation rather than a cell, so the result is a stem that declines through
/// [`adjective`] and [`short_adjective`] like any other. The long form comes out
/// `dobrjejszij` rather than `*dobrjejszyj` because §3.8's first rule writes `y`
/// as `i` after `sz`.
///
/// **On a velar stem the suffix loses its own glide.** `dorog` palatalizes to
/// `dorozz`, and `zz` is hard (§2.2), so §3.8's rule 2 drops the `j` of
/// `-jejsz-`: `dorozzejsz`, not `*dorozzjejsz`. §4.3 gives no velar example, so
/// this is the rules deciding rather than the specification stating.
///
/// ```
/// use ruthenian_core::{comparative, adjective, short_adjective};
/// use ruthenian_core::{Case, Number, Gender, Animacy::Inanimate};
///
/// assert_eq!(comparative("dobr"), "dobrjejsz");
/// // The first palatalization: a velar stem changes before the front vowel, and
/// // the suffix then loses its glide, because `zz` is hard (§3.8 rule 2).
/// assert_eq!(comparative("dorog"), "dorozzejsz");
/// assert_eq!(comparative("tih"), "tiszejsz");
///
/// // And it declines as an ordinary adjective.
/// let c = comparative("dobr");
/// assert_eq!(
///     adjective(&c, Case::Nominative, Number::Singular, Gender::Masculine, Inanimate),
///     "dobrjejszij"
/// );
/// assert_eq!(
///     short_adjective(&c, Case::Nominative, Number::Singular, Gender::Masculine, Inanimate),
///     "dobrjejsz"
/// );
/// ```
pub fn comparative(word: &str) -> String {
    let Some(s) = stem(word) else {
        return UNREADABLE.to_string();
    };
    join(&palatalize(&s, Palatal::First), "jejsz")
}

/// The superlative stem (§4.3): `naj-` prefixed to the comparative.
///
/// `naj-` follows OCS, Ukrainian, Belarusian, Polish and Interslavic against
/// Russian's analytic `самый`.
///
/// ```
/// use ruthenian_core::superlative;
/// assert_eq!(superlative("dobr"), "najdobrjejsz");
/// assert_eq!(superlative("dorog"), "najdorozzejsz");
/// ```
pub fn superlative(word: &str) -> String {
    let c = comparative(word);
    if c == UNREADABLE {
        return c;
    }
    format!("naj{c}")
}

/// An adjective stem with its lexical facts bound.
///
/// Holds the stem and nothing else — law 3.
///
/// ```
/// use ruthenian_core::{Adjective, Case, Number, Gender, Animacy::Inanimate};
///
/// let dobr = Adjective::new("dobr");
/// assert_eq!(dobr.long(Case::Genitive, Number::Singular, Gender::Masculine, Inanimate), "dobrogo");
/// assert_eq!(dobr.short(Case::Nominative, Number::Singular, Gender::Feminine, Inanimate), "dobra");
/// assert_eq!(dobr.long_paradigm(Inanimate).len(), 72);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adjective {
    stem: String,
}

impl Adjective {
    /// Bind a stem — the short masculine nominative, `dobr`.
    ///
    /// ```
    /// use ruthenian_core::Adjective;
    /// assert_eq!(Adjective::new("dobr").stem(), "dobr");
    /// ```
    pub fn new(stem: &str) -> Self {
        Self {
            stem: stem.to_string(),
        }
    }

    /// The stem as given.
    ///
    /// ```
    /// use ruthenian_core::Adjective;
    /// assert_eq!(Adjective::new("dorog").stem(), "dorog");
    /// ```
    pub fn stem(&self) -> &str {
        &self.stem
    }

    /// One cell of the long, definite declension.
    ///
    /// ```
    /// use ruthenian_core::{Adjective, Case, Number, Gender, Animacy::Inanimate};
    /// let a = Adjective::new("dobr");
    /// assert_eq!(a.long(Case::Locative, Number::Singular, Gender::Masculine, Inanimate), "dobrom");
    /// ```
    pub fn long(&self, case: Case, number: Number, gender: Gender, animacy: Animacy) -> String {
        adjective(&self.stem, case, number, gender, animacy)
    }

    /// One cell of the short, indefinite declension.
    ///
    /// ```
    /// use ruthenian_core::{Adjective, Case, Number, Gender, Animacy::Inanimate};
    /// let a = Adjective::new("dobr");
    /// assert_eq!(a.short(Case::Locative, Number::Singular, Gender::Masculine, Inanimate), "dobri");
    /// ```
    pub fn short(&self, case: Case, number: Number, gender: Gender, animacy: Animacy) -> String {
        short_adjective(&self.stem, case, number, gender, animacy)
    }

    /// Every cell of the long declension: 8 cases × 3 numbers × 3 genders.
    ///
    /// Law 2 — this calls [`Adjective::long`] rather than computing anything.
    ///
    /// ```
    /// use ruthenian_core::{Adjective, Animacy::Inanimate};
    /// assert_eq!(Adjective::new("dobr").long_paradigm(Inanimate).len(), 72);
    /// ```
    pub fn long_paradigm(&self, animacy: Animacy) -> Vec<(Case, Number, Gender, String)> {
        self.walk(animacy, true)
    }

    /// Every cell of the short declension.
    ///
    /// ```
    /// use ruthenian_core::{Adjective, Animacy::Inanimate};
    /// assert_eq!(Adjective::new("dobr").short_paradigm(Inanimate).len(), 72);
    /// ```
    pub fn short_paradigm(&self, animacy: Animacy) -> Vec<(Case, Number, Gender, String)> {
        self.walk(animacy, false)
    }

    fn walk(&self, animacy: Animacy, long: bool) -> Vec<(Case, Number, Gender, String)> {
        let mut out = Vec::with_capacity(72);
        for gender in Gender::ALL {
            for number in Number::ALL {
                for case in Case::ALL {
                    let form = match long {
                        true => self.long(case, number, gender, animacy),
                        false => self.short(case, number, gender, animacy),
                    };
                    out.push((case, number, gender, form));
                }
            }
        }
        out
    }
}
