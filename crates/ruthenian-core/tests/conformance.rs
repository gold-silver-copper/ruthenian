//! The engine against the committed corpus.
//!
//! Law 1: **the specification decides; the code conforms.** Where
//! `docs/RUTHENIAN.md` states a form, that form is correct by definition and a
//! disagreeing engine is wrong.
//!
//! The corpus is a committed artifact, not a parse of the specification at test
//! time — see `tools/extract_paradigms.py` for why that distinction is
//! load-bearing.

use ruthenian_core::{
    Animacy, Case, Gender, Number, Person, adjective, clitic_pronoun, clitic_reflexive, noun,
    pronoun, reflexive, short_adjective,
};

mod support;
use support::{Row, corpus};

fn case_of(name: &str) -> Case {
    match name {
        "Nominative" => Case::Nominative,
        "Vocative" => Case::Vocative,
        "Accusative" => Case::Accusative,
        "Genitive" => Case::Genitive,
        "Ablative" => Case::Ablative,
        "Dative" => Case::Dative,
        "Instrumental" => Case::Instrumental,
        "Locative" => Case::Locative,
        other => panic!("unknown case in corpus: {other}"),
    }
}

fn person_of(name: &str) -> Person {
    match name {
        "First" => Person::First,
        "Second" => Person::Second,
        "Third" => Person::Third,
        other => panic!("unknown person in corpus: {other}"),
    }
}

fn gender_of(name: &str) -> Gender {
    match name {
        "Masculine" => Gender::Masculine,
        "Feminine" => Gender::Feminine,
        "Neuter" => Gender::Neuter,
        other => panic!("unknown gender in corpus: {other}"),
    }
}

fn number_of(name: &str) -> Number {
    match name {
        "Singular" => Number::Singular,
        "Dual" => Number::Dual,
        "Plural" => Number::Plural,
        other => panic!("unknown number in corpus: {other}"),
    }
}

/// Every cell of every paradigm the specification tabulates.
///
/// Reports **all** mismatches rather than stopping at the first, because one
/// wrong ending typically shows up in several cells and the shape of the set is
/// what identifies the rule at fault.
#[test]
fn conformance() {
    let rows = corpus();
    assert!(!rows.is_empty(), "the corpus is empty");

    let mut failures = Vec::new();
    let mut checked = 0usize;
    for Row {
        pos,
        lemma,
        features,
        form,
        section,
    } in &rows
    {
        let got = match pos.as_str() {
            "noun" => {
                let (case, number) = features
                    .split_once('.')
                    .expect("features are Case.Number for a noun");
                noun(lemma, case_of(case), number_of(number))
            }
            // Case.Number.Gender, optionally .Animate. Anything else is
            // inanimate, which is the unmarked value.
            "adjective" | "short_adjective" => {
                let f: Vec<&str> = features.split('.').collect();
                assert!(
                    (3..=4).contains(&f.len()),
                    "adjective features are Case.Number.Gender[.Animate], got {features:?}"
                );
                let animacy = match f.get(3) {
                    Some(&"Animate") => Animacy::Animate,
                    None => Animacy::Inanimate,
                    Some(other) => panic!("unknown animacy in corpus: {other}"),
                };
                let f = |w: fn(&str, Case, Number, Gender, Animacy) -> String| {
                    w(
                        lemma,
                        case_of(f[0]),
                        number_of(f[1]),
                        gender_of(f[2]),
                        animacy,
                    )
                };
                match pos.as_str() {
                    "adjective" => f(adjective),
                    _ => f(short_adjective),
                }
            }
            // Person.Number.Gender.Case
            "pronoun" | "clitic_pronoun" => {
                let f: Vec<&str> = features.split('.').collect();
                assert_eq!(f.len(), 4, "pronoun features are Person.Number.Gender.Case");
                let (p, n, g, c) = (
                    person_of(f[0]),
                    number_of(f[1]),
                    gender_of(f[2]),
                    case_of(f[3]),
                );
                match pos.as_str() {
                    "pronoun" => pronoun(p, n, g, c),
                    _ => clitic_pronoun(p, n, g, c),
                }
            }
            // The reflexive has no gender and no number (§5.2), so its only
            // feature is the case.
            "reflexive" => reflexive(case_of(features)),
            "clitic_reflexive" => clitic_reflexive(case_of(features)),
            // Milestones M5–M7 add their parts of speech here. An unknown `pos`
            // is a hard error rather than a skip, so a corpus row can never go
            // silently unchecked.
            other => panic!("no engine entry point for pos {other:?}"),
        };
        checked += 1;
        if &got != form {
            failures.push(format!(
                "  §{section} {lemma} {features}: expected {form:?}, got {got:?}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {checked} cells disagree with the specification:\n{}",
        failures.len(),
        failures.join("\n")
    );
    println!("{checked} cells conform");
}
