//! Pronouns: §5.1's personal series, §5.1a's clitics, §5.2's reflexive.
//!
//! **A pronoun has no name.** It is exhausted by the features it agrees in, so
//! there is no `Pronoun` enum — see `DIRECTION.md`, "A pronoun has no name". The
//! mapping is exact: every pronoun §5.1 lists is one cell of person × number ×
//! gender, and every cell is a pronoun.
//!
//! | | Singular | Dual | Plural |
//! |---|---|---|---|
//! | **1st** | `ja` | `vje` | `my` |
//! | **2nd** | `ty` | `va` | `vy` |
//! | **3rd** | `on` / `ono` / `ona` by gender | `ona` | `oni` |
//!
//! **Gender is inert outside the third-person singular**, and that is the
//! language rather than a wart: Ruthenian's first and second persons do not
//! inflect for gender, and §5.1's third-person dual and plural do not either.
//!
//! There is **no post-prepositional `n-`** (§5.1): a pronoun after a preposition
//! is the plain form — `u jego`, not `*u njego`. Every Slavic language has that
//! prefix, and it is a Slavic-internal reanalysis rather than an inheritance, so
//! declining it is conservative in the same sense the ablative and the dual are.

use crate::grammar::{Animacy, Case, Gender, Number, Person};

/// A personal pronoun, full series (§5.1).
///
/// ```
/// use ruthenian_core::{pronoun, Case, Gender, Number, Person};
/// use Gender::Masculine as M;
///
/// assert_eq!(pronoun(Person::First, Number::Singular, M, Case::Nominative), "ja");
/// assert_eq!(pronoun(Person::First, Number::Singular, M, Case::Genitive), "mjenjego");
/// assert_eq!(pronoun(Person::First, Number::Singular, M, Case::Ablative), "mjenja");
/// assert_eq!(pronoun(Person::First, Number::Singular, M, Case::Instrumental), "mnoj");
///
/// // The restored OCS duals.
/// assert_eq!(pronoun(Person::First, Number::Dual, M, Case::Nominative), "vje");
/// assert_eq!(pronoun(Person::Second, Number::Dual, M, Case::Nominative), "va");
///
/// // The third person inflects for gender in the singular only.
/// assert_eq!(pronoun(Person::Third, Number::Singular, M, Case::Nominative), "on");
/// assert_eq!(pronoun(Person::Third, Number::Singular, Gender::Neuter, Case::Nominative), "ono");
/// assert_eq!(pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Nominative), "ona");
/// assert_eq!(pronoun(Person::Third, Number::Dual, M, Case::Nominative), "ona");
/// assert_eq!(pronoun(Person::Third, Number::Plural, M, Case::Nominative), "oni");
///
/// // Gender is inert for the first and second persons.
/// for g in Gender::ALL {
///     assert_eq!(pronoun(Person::First, Number::Singular, g, Case::Nominative), "ja");
/// }
///
/// // §5.1 has no vocative row; §3.1's convention is that the nominative is used.
/// assert_eq!(pronoun(Person::Second, Number::Singular, M, Case::Vocative), "ty");
/// ```
pub fn pronoun(person: Person, number: Number, gender: Gender, case: Case) -> String {
    use Case::*;
    use Number::*;
    use Person::*;

    // §5.1 tabulates no vocative: the nominative is used (see `fallback`).
    let case = if case == Vocative { Nominative } else { case };
    // §3.1: the ablative is distinct only in the singular.
    let case = if case == Ablative && number != Singular {
        Dative
    } else {
        case
    };

    let form = match (person, number) {
        (First, Singular) => match case {
            Nominative => "ja",
            Accusative | Ablative => "mjenja",
            Genitive => "mjenjego",
            Dative | Locative => "mnje",
            _ => "mnoj",
        },
        (Second, Singular) => match case {
            Nominative => "ty",
            Accusative | Ablative => "tjebja",
            Genitive => "tjebjego",
            Dative | Locative => "tjebje",
            _ => "toboj",
        },
        (First, Dual) => match case {
            Nominative => "vje",
            Accusative => "na",
            Genitive | Locative => "naju",
            _ => "nama",
        },
        (Second, Dual) => match case {
            Nominative | Accusative => "va",
            Genitive | Locative => "vaju",
            _ => "vama",
        },
        (First, Plural) => match case {
            Nominative => "my",
            Accusative | Genitive | Locative => "nas",
            Dative => "nam",
            _ => "nami",
        },
        (Second, Plural) => match case {
            Nominative => "vy",
            Accusative | Genitive | Locative => "vas",
            Dative => "vam",
            _ => "vami",
        },
        (Third, Singular) => match gender {
            Gender::Feminine => match case {
                Nominative => "ona",
                Accusative => "ju",
                Genitive | Ablative => "jeje",
                Instrumental => "jeju",
                _ => "jej",
            },
            g => match case {
                Nominative => {
                    if g == Gender::Neuter {
                        "ono"
                    } else {
                        "on"
                    }
                }
                Accusative | Genitive | Ablative => "jego",
                Dative => "jemu",
                Instrumental => "jim",
                _ => "jem",
            },
        },
        (Third, Dual) => match case {
            Nominative => "ona",
            Accusative => "ja",
            Genitive | Locative => "jeju",
            _ => "jima",
        },
        (Third, Plural) => match case {
            Nominative => "oni",
            Accusative | Genitive | Locative => "jih",
            Dative => "jim",
            _ => "jimi",
        },
    };
    form.to_string()
}

/// A personal pronoun, **clitic** series (§5.1a).
///
/// Every personal pronoun has a short, unstressed form beside the full one, as in
/// OCS and Sanskrit. Russian lost the opposition entirely; OCS, Czech, Polish and
/// Interslavic keep it.
///
/// §5.1a gives clitics for the **accusative and dative only**, and for the
/// singular and plural only. Every other cell returns the full form — a declared
/// fallback, listed in [`crate::fallback`] — because a clitic that does not
/// exist has to be *something* and the full form is what a speaker would use.
///
/// Clitics stand in second position and cannot follow a preposition, be stressed,
/// be focused, or stand alone as an answer. Each of those requires the full form,
/// and none of it is this crate's business: placement is syntax.
///
/// ```
/// use ruthenian_core::{clitic_pronoun, pronoun, Case, Gender, Number, Person};
/// use Gender::Masculine as M;
///
/// assert_eq!(clitic_pronoun(Person::First, Number::Singular, M, Case::Accusative), "mja");
/// assert_eq!(clitic_pronoun(Person::First, Number::Singular, M, Case::Dative), "mi");
/// assert_eq!(clitic_pronoun(Person::Second, Number::Singular, M, Case::Accusative), "tja");
/// assert_eq!(clitic_pronoun(Person::Third, Number::Singular, M, Case::Accusative), "go");
/// assert_eq!(clitic_pronoun(Person::Third, Number::Singular, M, Case::Dative), "mu");
/// assert_eq!(clitic_pronoun(Person::First, Number::Plural, M, Case::Accusative), "ny");
///
/// // The feminine accusative and the whole third plural are the same as the
/// // full form — §5.1a lists them that way.
/// assert_eq!(clitic_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Accusative), "ju");
/// assert_eq!(clitic_pronoun(Person::Third, Number::Plural, M, Case::Dative), "jim");
///
/// // Outside the accusative and dative there is no clitic, so the full form
/// // stands in.
/// let full = pronoun(Person::First, Number::Singular, M, Case::Instrumental);
/// assert_eq!(clitic_pronoun(Person::First, Number::Singular, M, Case::Instrumental), full);
/// // And §5.1a lists no dual clitics at all.
/// let du = pronoun(Person::First, Number::Dual, M, Case::Accusative);
/// assert_eq!(clitic_pronoun(Person::First, Number::Dual, M, Case::Accusative), du);
/// ```
pub fn clitic_pronoun(person: Person, number: Number, gender: Gender, case: Case) -> String {
    use Case::*;
    use Number::*;
    use Person::*;

    let clitic = match (person, number, case) {
        (First, Singular, Accusative) => Some("mja"),
        (First, Singular, Dative) => Some("mi"),
        (Second, Singular, Accusative) => Some("tja"),
        (Second, Singular, Dative) => Some("ti"),
        (Third, Singular, Accusative) => Some(match gender {
            Gender::Feminine => "ju",
            _ => "go",
        }),
        (Third, Singular, Dative) => Some(match gender {
            Gender::Feminine => "ji",
            _ => "mu",
        }),
        (First, Plural, Accusative) => Some("ny"),
        (First, Plural, Dative) => Some("ni"),
        (Second, Plural, Accusative) => Some("vy"),
        (Second, Plural, Dative) => Some("vi"),
        (Third, Plural, Accusative) => Some("jih"),
        (Third, Plural, Dative) => Some("jim"),
        _ => None,
    };
    match clitic {
        Some(c) => c.to_string(),
        None => pronoun(person, number, gender, case),
    }
}

/// The reflexive, full series (§5.2).
///
/// No gender and no number: a reflexive takes its reference from the subject, not
/// from its own shape.
///
/// **§5.2 gives it no nominative** — a reflexive cannot be a subject — so asking
/// for one returns `sjebja`, the form the pronoun is cited by. That is a declared
/// fallback, not a form of the language.
///
/// ```
/// use ruthenian_core::{reflexive, Case};
///
/// assert_eq!(reflexive(Case::Accusative), "sjebja");
/// assert_eq!(reflexive(Case::Ablative), "sjebja");
/// assert_eq!(reflexive(Case::Genitive), "sjebjego");
/// assert_eq!(reflexive(Case::Dative), "sjebje");
/// assert_eq!(reflexive(Case::Locative), "sjebje");
/// assert_eq!(reflexive(Case::Instrumental), "soboj");
///
/// // The cell the language does not have.
/// assert_eq!(reflexive(Case::Nominative), "sjebja");
/// ```
pub fn reflexive(case: Case) -> String {
    match case {
        Case::Genitive => "sjebjego",
        Case::Dative | Case::Locative => "sjebje",
        Case::Instrumental => "soboj",
        // Accusative, ablative, and the nominative the language lacks.
        _ => "sjebja",
    }
    .to_string()
}

/// The reflexive, **clitic** series (§5.2).
///
/// `sja` is a **free second-position clitic** like every other, not a bound
/// verbal suffix: `on sja myjet`, `myjet li sja on?`. This follows OCS `sę`,
/// Czech `se` and Polish `się`; Russian's written-attached `-sja` is a later
/// fusion, and carrying it would give one morph two grammars — a suffix in the
/// verb and a pronoun in the paradigm.
///
/// ```
/// use ruthenian_core::{clitic_reflexive, reflexive, Case};
///
/// assert_eq!(clitic_reflexive(Case::Accusative), "sja");
/// assert_eq!(clitic_reflexive(Case::Dative), "si");
///
/// // Elsewhere there is no clitic, so the full form stands in.
/// assert_eq!(clitic_reflexive(Case::Instrumental), reflexive(Case::Instrumental));
/// ```
pub fn clitic_reflexive(case: Case) -> String {
    match case {
        Case::Accusative => "sja".to_string(),
        Case::Dative => "si".to_string(),
        _ => reflexive(case),
    }
}

/// Every case of one personal pronoun, in §3.1's order.
///
/// Law 2 — this calls [`pronoun`] rather than computing anything.
///
/// ```
/// use ruthenian_core::{pronoun_paradigm, Gender, Number, Person};
///
/// let table = pronoun_paradigm(Person::First, Number::Singular, Gender::Masculine);
/// assert_eq!(table.len(), 8);
/// assert_eq!(table[0].1, "ja");
/// ```
pub fn pronoun_paradigm(person: Person, number: Number, gender: Gender) -> Vec<(Case, String)> {
    Case::ALL
        .iter()
        .map(|&case| (case, pronoun(person, number, gender, case)))
        .collect()
}

/// The **pronominal declension** itself (§5.4), for any stem that takes it.
///
/// `toj` "that" is `pronominal("t", ..)` and `sjej` "this" is
/// `pronominal("sj", ..)`. Two degrees of deixis, as OCS had — Russian lost the
/// near one and Ruthenian keeps it.
///
/// **Hardness comes from the stem**, by §3.2's rule: a stem is soft exactly when
/// it ends in `j`. The soft series is the hard one with an `o`-initial ending
/// written `e` and a `je`-initial ending written `i`, since the stem's own `j`
/// already carries the softness.
///
/// This is *not* the long adjective's table. §4.2 says its endings are "the
/// pronoun `toj`'s", but that names the declension type: thirteen of the
/// seventeen endings differ, because the long adjective is the contracted
/// `short + jь` form. What the two genuinely share is every `o`-initial ending.
///
/// ```
/// use ruthenian_core::{pronominal, Case, Gender, Number, Animacy};
/// use Gender::Masculine as M;
/// use Animacy::Inanimate as In;
///
/// // §5.4's table, on the hard stem `t-`.
/// assert_eq!(pronominal("t", Case::Nominative, Number::Singular, M, In), "toj");
/// assert_eq!(pronominal("t", Case::Genitive, Number::Singular, M, In), "togo");
/// assert_eq!(pronominal("t", Case::Ablative, Number::Singular, M, In), "toga");
/// assert_eq!(pronominal("t", Case::Dative, Number::Singular, M, In), "tomu");
/// assert_eq!(pronominal("t", Case::Instrumental, Number::Singular, M, In), "tjem");
/// assert_eq!(pronominal("t", Case::Nominative, Number::Singular, Gender::Neuter, In), "to");
/// assert_eq!(pronominal("t", Case::Nominative, Number::Singular, Gender::Feminine, In), "ta");
/// assert_eq!(pronominal("t", Case::Genitive, Number::Dual, M, In), "toju");
/// assert_eq!(pronominal("t", Case::Nominative, Number::Plural, M, In), "ti");
/// assert_eq!(pronominal("t", Case::Genitive, Number::Plural, M, In), "tjeh");
///
/// // §3.7: the animate accusative is the ablative in the singular and the
/// // genitive in the plural.
/// let anim = Animacy::Animate;
/// assert_eq!(pronominal("t", Case::Accusative, Number::Singular, M, anim), "toga");
/// assert_eq!(pronominal("t", Case::Accusative, Number::Plural, M, anim), "tjeh");
///
/// // The soft stem `sj-`, and the four forms §5.4 cites for it.
/// assert_eq!(pronominal("sj", Case::Nominative, Number::Singular, M, In), "sjej");
/// assert_eq!(pronominal("sj", Case::Genitive, Number::Singular, M, In), "sjego");
/// assert_eq!(pronominal("sj", Case::Dative, Number::Singular, M, In), "sjemu");
/// assert_eq!(pronominal("sj", Case::Instrumental, Number::Singular, M, In), "sjim");
/// assert_eq!(pronominal("sj", Case::Locative, Number::Singular, M, In), "sjem");
/// ```
pub fn pronominal(
    stem: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    use Case::*;
    use Number::*;

    let Some(s) = bound_stem(stem) else {
        return crate::fallback::UNREADABLE.to_string();
    };
    let soft = s.ends_with('j');

    let case = if case == Vocative { Nominative } else { case };
    let case = if case == Ablative && number != Singular {
        Dative
    } else {
        case
    };

    let hard = match number {
        Dual => match case {
            Nominative | Accusative => "a",
            Genitive | Locative => "oju",
            _ => "jema",
        },
        Plural => match case {
            Accusative if animacy == Animacy::Animate => "jeh",
            Nominative | Accusative => "i",
            Genitive | Locative => "jeh",
            Instrumental => "jemi",
            _ => "jem",
        },
        Singular => match gender {
            Gender::Feminine => match case {
                Nominative => "a",
                Accusative => "u",
                _ => "oj",
            },
            g => match case {
                Accusative if animacy == Animacy::Animate => "oga",
                Nominative | Accusative => {
                    if g == Gender::Neuter {
                        "o"
                    } else {
                        "oj"
                    }
                }
                Genitive => "ogo",
                Ablative => "oga",
                Dative => "omu",
                Instrumental => "jem",
                _ => "om",
            },
        },
    };

    // §3.2's soft alternation, applied to the ending: the stem's own `j` carries
    // the softness, so an `o` fronts to `e` and a `je` contracts to `i`.
    let ending = match soft {
        false => hard.to_string(),
        true => match hard.strip_prefix("je") {
            Some(rest) => format!("i{rest}"),
            None => match hard.strip_prefix('o') {
                Some(rest) => format!("e{rest}"),
                None => hard.to_string(),
            },
        },
    };
    format!("{s}{ending}")
}

/// A **bound** stem, as opposed to a lemma.
///
/// `t-` and `sj-` are the demonstratives' stems and neither contains a vowel, so
/// the vowel test a citation form must pass (see `lemma`) would reject both. A
/// bound morpheme is not a word and is not required to look like one; it must
/// only be spellable.
fn bound_stem(s: &str) -> Option<String> {
    let parsed = ruthenian_orthography::Ruthenian::parse(s).ok()?;
    let bare = parsed.word().to_lowercase();
    let ok = !bare.is_empty() && bare.chars().all(|c| c.is_ascii_alphabetic() || c == '\'');
    ok.then_some(bare)
}

/// `kto` "who" (§5.5) — animate, and so with an oblique accusative.
///
/// A closed word with an irregular nominative, so it is tabulated rather than
/// derived. Its accusative is the **ablative** `koga` and not the genitive
/// `kogo`, by §3.7 — the same correction §5.4's `toga` needed.
///
/// ```
/// use ruthenian_core::{who, Case};
/// assert_eq!(who(Case::Nominative), "kto");
/// assert_eq!(who(Case::Accusative), "koga");
/// assert_eq!(who(Case::Genitive), "kogo");
/// assert_eq!(who(Case::Ablative), "koga");
/// assert_eq!(who(Case::Dative), "komu");
/// assert_eq!(who(Case::Instrumental), "kjem");
/// assert_eq!(who(Case::Locative), "kom");
///
/// // §5.6 builds the negative and indefinite series by prefix, which is
/// // composition rather than inflection: `nikto`, `njekto`, `kto-libo`.
/// assert_eq!(format!("ni{}", who(Case::Genitive)), "nikogo");
/// ```
pub fn who(case: Case) -> String {
    match case {
        Case::Nominative | Case::Vocative => "kto",
        Case::Accusative | Case::Ablative => "koga",
        Case::Genitive => "kogo",
        Case::Dative => "komu",
        Case::Instrumental => "kjem",
        Case::Locative => "kom",
    }
    .to_string()
}

/// `czto` "what" (§5.5) — inanimate, so the accusative is the nominative.
///
/// ```
/// use ruthenian_core::{what, Case};
/// assert_eq!(what(Case::Nominative), "czto");
/// assert_eq!(what(Case::Accusative), "czto");
/// assert_eq!(what(Case::Genitive), "czjego");
/// assert_eq!(what(Case::Ablative), "czjega");
/// assert_eq!(what(Case::Dative), "czjemu");
/// // §5.5 gives one form for the instrumental and the locative alike.
/// assert_eq!(what(Case::Instrumental), "czjem");
/// assert_eq!(what(Case::Locative), "czjem");
/// ```
pub fn what(case: Case) -> String {
    match case {
        Case::Nominative | Case::Vocative | Case::Accusative => "czto",
        Case::Genitive => "czjego",
        Case::Ablative => "czjega",
        Case::Dative => "czjemu",
        Case::Instrumental | Case::Locative => "czjem",
    }
    .to_string()
}

/// `izzje` (§5.5), the **restrictive** relative — OCS `иже`, which Russian lost
/// in favour of `который`. Ruthenian keeps both, `izzje` for restrictive clauses
/// and `kotoryj` (an ordinary adjective) for non-restrictive.
///
/// It is the third-person pronoun plus the invariant `-zzje`, so it agrees with
/// its antecedent in gender and number while taking its case from its own
/// clause: `czjelovjek, jegozzje vizzu` "the man whom I see".
///
/// ```
/// use ruthenian_core::{relative, Case, Gender, Number};
/// use Gender::Masculine as M;
///
/// assert_eq!(relative(Case::Nominative, Number::Singular, M), "izzje");
/// assert_eq!(relative(Case::Accusative, Number::Singular, M), "jegozzje");
/// assert_eq!(relative(Case::Genitive, Number::Singular, M), "jegozzje");
/// assert_eq!(relative(Case::Dative, Number::Singular, M), "jemuzzje");
/// ```
pub fn relative(case: Case, number: Number, gender: Gender) -> String {
    // §5.5 gives only the nominative `izzje`, without a gender or number series
    // for it; the obliques are the pronoun's own forms plus the particle.
    match case {
        Case::Nominative | Case::Vocative => "izzje".to_string(),
        _ => format!("{}zzje", pronoun(Person::Third, number, gender, case)),
    }
}
