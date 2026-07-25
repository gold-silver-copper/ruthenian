//! The paradigm fixture: real attested forms from the Wiktionary dump, compared
//! against what the rules produce.
//!
//! Scored **two ways**, and both are reported: *segmental* (letters only) and
//! *strict* (including stress placement). Endings come right long before accent
//! patterns do, and a single all-or-nothing number would hide both the progress
//! and the stress bugs at once.
//!
//! Failures here are the point of this phase, not a problem with it: each lemma
//! the rules cannot reproduce names something the lexicon must store.

use ruthenian_core::class::ZaliznyakVerbClass;
use ruthenian_core::phono;
use ruthenian_core::types::*;
use ruthenian_core::verb::VerbInfo;
use ruthenian_core::{adjective, noun, verb};
use ruthenian_orthography::{Cyrillic, to_latin};
use std::collections::{BTreeMap, BTreeSet};

const FIXTURE: &str = include_str!("paradigms/fixture.tsv");
const META: &str = include_str!("paradigms/fixture_meta.tsv");

#[derive(Debug, Clone)]
struct Meta {
    class: String,
    extra: String,
}

fn meta() -> BTreeMap<String, Meta> {
    let mut m = BTreeMap::new();
    for line in META.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 4 {
            continue;
        }
        m.insert(
            f[0].to_string(),
            Meta {
                class: f[2].to_string(),
                extra: f[3].to_string(),
            },
        );
    }
    m
}

fn latin(cyr: &str) -> Option<String> {
    Cyrillic::parse(cyr)
        .ok()
        .map(|c| to_latin(&c).as_str().to_string())
}

fn tags(slot: &str) -> BTreeSet<&str> {
    slot.split_whitespace().collect()
}

fn case_of(t: &BTreeSet<&str>) -> Option<Case> {
    if t.contains("nominative") {
        Some(Case::Nom)
    } else if t.contains("genitive") {
        Some(Case::Gen)
    } else if t.contains("dative") {
        Some(Case::Dat)
    } else if t.contains("accusative") {
        Some(Case::Acc)
    } else if t.contains("instrumental") {
        Some(Case::Ins)
    } else if t.contains("prepositional") || t.contains("locative") {
        Some(Case::Loc)
    } else {
        None
    }
}

fn number_of(t: &BTreeSet<&str>) -> Option<Number> {
    if t.contains("plural") {
        Some(Number::Plural)
    } else if t.contains("singular") {
        Some(Number::Singular)
    } else {
        None
    }
}

fn gender_of(t: &BTreeSet<&str>) -> Option<Gender> {
    // A cell listing several genders is a merged cell; take the first, which is
    // the one the form is canonically cited under.
    if t.contains("masculine") {
        Some(Gender::Masculine)
    } else if t.contains("feminine") {
        Some(Gender::Feminine)
    } else if t.contains("neuter") {
        Some(Gender::Neuter)
    } else {
        None
    }
}

fn verb_slot(t: &BTreeSet<&str>) -> Option<VerbSlot> {
    if t.contains("infinitive") {
        return Some(VerbSlot::Infinitive);
    }
    if t.contains("participle") {
        let kind = if t.contains("adverbial") {
            ParticipleKind::Adverbial
        } else {
            ParticipleKind::Adjectival
        };
        let voice = if t.contains("passive") {
            Voice::Passive
        } else {
            Voice::Active
        };
        let tense = if t.contains("present") {
            Tense::Present
        } else {
            Tense::Past
        };
        return Some(VerbSlot::Participle { kind, voice, tense });
    }
    if t.contains("imperative") {
        return Some(VerbSlot::Imperative {
            number: number_of(t).unwrap_or(Number::Singular),
        });
    }
    if t.contains("past") {
        let number = if t.contains("plural") {
            Number::Plural
        } else {
            Number::Singular
        };
        return Some(VerbSlot::Past {
            gender: if number == Number::Plural {
                None
            } else {
                gender_of(t)
            },
            number,
        });
    }
    let person = if t.contains("first-person") {
        Person::First
    } else if t.contains("second-person") {
        Person::Second
    } else if t.contains("third-person") {
        Person::Third
    } else {
        return None;
    };
    let tense = if t.contains("future") {
        Tense::Future
    } else if t.contains("present") {
        Tense::Present
    } else {
        return None;
    };
    Some(VerbSlot::Finite {
        person,
        number: number_of(t)?,
        tense,
    })
}

/// An adjective's stem: the masculine nominative minus `-yj`/`-ij`/`-oj`,
/// stripped segmentally so a stressed ending does not defeat it.
fn adj_stem(lemma: &str) -> String {
    let idx = phono::stressed_index(lemma);
    let bare = phono::unstress(lemma);
    let stem = ["yj", "ij", "oj"]
        .iter()
        .find_map(|s| bare.strip_suffix(s))
        .unwrap_or(&bare)
        .to_string();
    match idx {
        Some(i) if i < phono::vowel_count(&stem) => phono::apply_stress_at(&stem, i),
        _ => stem,
    }
}

fn stem_class(extra: &str) -> StemClass {
    if extra.contains("velar-stem") {
        StemClass::Velar
    } else if extra.contains("sibilant-stem") {
        StemClass::Sibilant
    } else if extra.contains("ц-stem") {
        StemClass::Ts
    } else if extra.contains("i-stem") {
        StemClass::I
    } else if extra.contains("vowel-stem") {
        StemClass::Vowel
    } else if extra.contains("soft-stem") {
        StemClass::Soft
    } else {
        StemClass::Hard
    }
}

/// The source does not always record a stem class. It is derivable from the
/// lemma, and deriving it is what the lexicon will do too.
fn stem_class_for(extra: &str, lemma_lat: &str, gender: Gender) -> StemClass {
    if extra.contains("-stem") {
        return stem_class(extra);
    }
    let bare = phono::unstress(lemma_lat);
    if gender == Gender::Feminine && bare.ends_with('j') {
        StemClass::I
    } else if bare.ends_with("ij") || bare.ends_with("je") || bare.ends_with("ja") {
        StemClass::Vowel
    } else if bare.ends_with('j') {
        StemClass::Soft
    } else if phono::ends_velar(&bare) {
        StemClass::Velar
    } else if phono::ends_sibilant(&bare) {
        StemClass::Sibilant
    } else {
        StemClass::Hard
    }
}

fn accent(extra: &str) -> AccentPattern {
    for (needle, a) in [
        ("accent-a", AccentPattern::A),
        ("accent-b", AccentPattern::B),
        ("accent-c", AccentPattern::C),
        ("accent-d", AccentPattern::D),
        ("accent-e", AccentPattern::E),
        ("accent-f", AccentPattern::F),
    ] {
        if extra.contains(needle) {
            return a;
        }
    }
    AccentPattern::A
}

#[derive(Default, Debug)]
struct Score {
    comparable: usize,
    segmental: usize,
    strict: usize,
    absent_agreed: usize,
    absent_disagreed: usize,
    unsupported: usize,
    unconvertible: usize,
}

#[test]
fn paradigm_fixture() {
    let meta = meta();
    let mut by_pos: BTreeMap<&str, Score> = BTreeMap::new();
    let mut failing: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in FIXTURE.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 5 {
            continue;
        }
        let (lemma, pos, _cls, slot, form) = (f[0], f[1], f[2], f[3], f[4]);
        let Some(m) = meta.get(lemma) else { continue };
        let t = tags(slot);

        let Some(lemma_lat) = latin(lemma) else {
            by_pos.entry(pos).or_default().unconvertible += 1;
            continue;
        };
        let expected = if form == "-" {
            None
        } else {
            match latin(form) {
                Some(l) => Some(l),
                None => {
                    by_pos.entry(pos).or_default().unconvertible += 1;
                    continue;
                }
            }
        };

        let got: Result<Option<String>, String> = match pos {
            "noun" => {
                let (Some(case), Some(number)) = (case_of(&t), number_of(&t)) else {
                    continue;
                };
                let gender = if m.extra.contains("feminine") {
                    Gender::Feminine
                } else if m.extra.contains("neuter") {
                    Gender::Neuter
                } else {
                    Gender::Masculine
                };
                let animacy = if m.extra.contains("animate") && !m.extra.contains("inanimate") {
                    Animacy::Animate
                } else {
                    Animacy::Inanimate
                };
                let class = NounClass {
                    stem: stem_class_for(&m.extra, &lemma_lat, gender),
                    accent: accent(&m.extra),
                    reducible: m.extra.contains("reducible"),
                };
                let stem = ruthenian_core::noun::stem_of(&lemma_lat);
                Ok(noun(&stem, class, gender, animacy, case, number).map(|p| p.text))
            }
            "adj" if t.contains("short-form") => {
                let gender = gender_of(&t).unwrap_or(Gender::Masculine);
                let number = number_of(&t).unwrap_or(Number::Singular);
                let stem = adj_stem(&lemma_lat);
                Ok(adjective(
                    &stem,
                    Case::Nom,
                    number,
                    gender,
                    Animacy::Inanimate,
                    AdjForm::Short,
                )
                .map(|p| p.text))
            }
            "adj" => {
                let (Some(case), Some(number)) = (case_of(&t), number_of(&t)) else {
                    continue;
                };
                let gender = gender_of(&t).unwrap_or(Gender::Masculine);
                let animacy = if t.contains("animate") {
                    Animacy::Animate
                } else {
                    Animacy::Inanimate
                };
                let stem = adj_stem(&lemma_lat);
                Ok(adjective(&stem, case, number, gender, animacy, AdjForm::Long).map(|p| p.text))
            }
            "verb" => {
                let Some(vs) = verb_slot(&t) else { continue };
                let Ok(class) = ZaliznyakVerbClass::parse(&m.class) else {
                    continue;
                };
                let aspect = if m.extra.contains("aspect=pf") {
                    Aspect::Perfective
                } else {
                    Aspect::Imperfective
                };
                let info = VerbInfo {
                    aspect,
                    transitive: Some(!m.extra.contains("intr")),
                    reflexive: lemma_lat.ends_with("sja"),
                };
                let parts = PrincipalPartsRef::default();
                match verb(&lemma_lat, &class, info, &parts, vs) {
                    Ok(o) => Ok(o.map(|p| p.text)),
                    Err(e) => Err(e.reason.to_string()),
                }
            }
            _ => continue,
        };

        let s = by_pos.entry(pos).or_default();
        match (got, expected) {
            (Err(_), _) => s.unsupported += 1,
            (Ok(None), None) => s.absent_agreed += 1,
            (Ok(Some(_)), None) | (Ok(None), Some(_)) => {
                s.absent_disagreed += 1;
                failing
                    .entry(lemma.to_string())
                    .or_default()
                    .push(format!("{slot}: existence disagreement"));
            }
            (Ok(Some(g)), Some(e)) => {
                s.comparable += 1;
                if phono::unstress(&g) == phono::unstress(&e) {
                    s.segmental += 1;
                    if g == e {
                        s.strict += 1;
                    }
                } else {
                    failing
                        .entry(lemma.to_string())
                        .or_default()
                        .push(format!("{slot}: got {g}, attested {e}"));
                }
            }
        }
    }

    println!("\n=== paradigm fixture ===");
    for (pos, s) in &by_pos {
        let pct = |n: usize| {
            if s.comparable == 0 {
                0.0
            } else {
                100.0 * n as f64 / s.comparable as f64
            }
        };
        println!(
            "{pos:>5}: comparable {:>4}  segmental {:>4} ({:5.1}%)  strict {:>4} ({:5.1}%)  \
             gap-agree {:>4}  gap-disagree {:>3}  unsupported {:>4}  unconvertible {:>3}",
            s.comparable,
            s.segmental,
            pct(s.segmental),
            s.strict,
            pct(s.strict),
            s.absent_agreed,
            s.absent_disagreed,
            s.unsupported,
            s.unconvertible
        );
    }
    let total: usize = by_pos.values().map(|s| s.comparable).sum();
    let seg: usize = by_pos.values().map(|s| s.segmental).sum();
    println!(
        "total: {seg}/{total} segmental ({:.1}%)",
        100.0 * seg as f64 / total.max(1) as f64
    );
    println!("\nlemmas needing lexicon support: {}", failing.len());
    for (lemma, why) in failing.iter().take(25) {
        println!("  {lemma}: {} ({} slots)", why[0], why.len());
    }

    assert!(
        total > 500,
        "only {total} comparable cells; fixture broken?"
    );
}
