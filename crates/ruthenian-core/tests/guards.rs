//! The guards from `docs/specs/ruthenian-core.md` §9, plus the three this
//! phase's measurements required.
//!
//! Every one names the minimal mutation that must make it fail. A guard that
//! survives its own witness is stale and must be fixed or deleted — phase 1
//! found two that way.

use ruthenian_core::class::ZaliznyakVerbClass;
use ruthenian_core::phono;
use ruthenian_core::policy::{self, Policy, Trace};
use ruthenian_core::types::*;
use ruthenian_core::verb::VerbInfo;
use ruthenian_core::{adjective, noun, verb};
use ruthenian_orthography::Ruthenian;

const CLASS_CODES: &str = include_str!("paradigms/class-codes.txt");

fn ipf() -> VerbInfo {
    VerbInfo {
        aspect: Aspect::Imperfective,
        transitive: Some(true),
        reflexive: false,
    }
}
fn pf() -> VerbInfo {
    VerbInfo {
        aspect: Aspect::Perfective,
        ..ipf()
    }
}

const CASES: [Case; 6] = [
    Case::Nom,
    Case::Gen,
    Case::Dat,
    Case::Acc,
    Case::Ins,
    Case::Loc,
];
const NUMBERS: [Number; 2] = [Number::Singular, Number::Plural];
const GENDERS: [Gender; 3] = [Gender::Masculine, Gender::Feminine, Gender::Neuter];
const STEMS: [StemClass; 7] = [
    StemClass::Hard,
    StemClass::Soft,
    StemClass::Velar,
    StemClass::Sibilant,
    StemClass::Ts,
    StemClass::I,
    StemClass::Vowel,
];
const ACCENTS: [AccentPattern; 6] = [
    AccentPattern::A,
    AccentPattern::B,
    AccentPattern::C,
    AccentPattern::D,
    AccentPattern::E,
    AccentPattern::F,
];

// --------------------------------------------------------------------------
// 1. slot_exhaustive — every class x slot resolves or declares a gap.
//    Witness: add a Slot variant without handling it.
// --------------------------------------------------------------------------
#[test]
fn slot_exhaustive() {
    let mut checked = 0;
    for stem in STEMS {
        for accent in ACCENTS {
            let class = NounClass {
                stem,
                accent,
                reducible: false,
            };
            for g in GENDERS {
                for c in CASES {
                    for n in NUMBERS {
                        // Must not panic, and must give a definite answer.
                        let _ = noun("stol", class, g, Animacy::Inanimate, c, n);
                        checked += 1;
                    }
                }
            }
        }
    }
    // Verbs: every implemented class against every slot.
    for code in ["1a", "2a", "3a", "4b", "5a", "6c"] {
        let class = ZaliznyakVerbClass::parse(code).expect("code parses");
        for slot in all_verb_slots() {
            for info in [ipf(), pf()] {
                let r = verb("citatj", &class, info, &PrincipalPartsRef::default(), slot);
                assert!(
                    r.is_ok(),
                    "{code} {slot:?}: implemented classes must not report Unsupported"
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 1500, "only {checked} cells exercised");
}

fn all_verb_slots() -> Vec<VerbSlot> {
    let mut v = vec![VerbSlot::Infinitive];
    for person in [Person::First, Person::Second, Person::Third] {
        for number in NUMBERS {
            for tense in [Tense::Present, Tense::Future] {
                v.push(VerbSlot::Finite {
                    person,
                    number,
                    tense,
                });
            }
        }
    }
    for number in NUMBERS {
        v.push(VerbSlot::Imperative { number });
        v.push(VerbSlot::Past {
            gender: if number == Number::Plural {
                None
            } else {
                Some(Gender::Masculine)
            },
            number,
        });
    }
    for kind in [ParticipleKind::Adjectival, ParticipleKind::Adverbial] {
        for voice in [Voice::Active, Voice::Passive] {
            for tense in [Tense::Present, Tense::Past] {
                v.push(VerbSlot::Participle { kind, voice, tense });
            }
        }
    }
    v
}

// --------------------------------------------------------------------------
// 2. regular_rules_golden — the predictor's output is stable.
//    Witness: change any ending; the golden diff shows exactly what moved.
// --------------------------------------------------------------------------
#[test]
fn regular_rules_golden() {
    let hard = NounClass {
        stem: StemClass::Hard,
        accent: AccentPattern::A,
        reducible: false,
    };
    let velar = NounClass {
        stem: StemClass::Velar,
        accent: AccentPattern::A,
        reducible: false,
    };
    let cases = [
        (
            noun(
                "stol",
                hard,
                Gender::Masculine,
                Animacy::Inanimate,
                Case::Gen,
                Number::Singular,
            ),
            "stola",
        ),
        (
            noun(
                "stol",
                hard,
                Gender::Masculine,
                Animacy::Inanimate,
                Case::Ins,
                Number::Singular,
            ),
            "stolom",
        ),
        (
            noun(
                "stol",
                hard,
                Gender::Masculine,
                Animacy::Inanimate,
                Case::Gen,
                Number::Plural,
            ),
            "stolov",
        ),
        (
            noun(
                "knig",
                velar,
                Gender::Feminine,
                Animacy::Inanimate,
                Case::Gen,
                Number::Singular,
            ),
            "knigi",
        ),
        (
            noun(
                "okn",
                hard,
                Gender::Neuter,
                Animacy::Inanimate,
                Case::Nom,
                Number::Plural,
            ),
            "okna",
        ),
    ];
    for (got, want) in cases {
        assert_eq!(got.expect("cell exists").text, want);
    }

    let c1 = ZaliznyakVerbClass::parse("1a").unwrap();
    let parts = PrincipalPartsRef::default();
    let fin = |p, n, t| VerbSlot::Finite {
        person: p,
        number: n,
        tense: t,
    };
    for (slot, want) in [
        (
            fin(Person::First, Number::Singular, Tense::Present),
            "citaju",
        ),
        (
            fin(Person::Second, Number::Singular, Tense::Present),
            "citajeszj",
        ),
        (
            fin(Person::Third, Number::Plural, Tense::Present),
            "citajut",
        ),
    ] {
        let got = verb("citatj", &c1, ipf(), &parts, slot)
            .expect("supported")
            .expect("cell exists");
        assert_eq!(got.text, want);
    }
}

// --------------------------------------------------------------------------
// 3. trace_non_empty — every prediction explains itself.
//    Witness: return a Prediction with Trace::default().
// --------------------------------------------------------------------------
#[test]
fn trace_non_empty() {
    let class = NounClass {
        stem: StemClass::Hard,
        accent: AccentPattern::A,
        reducible: false,
    };
    for c in CASES {
        for n in NUMBERS {
            if let Some(p) = noun("stol", class, Gender::Masculine, Animacy::Inanimate, c, n) {
                assert!(!p.trace.is_empty(), "{c:?} {n:?} has an empty trace");
            }
        }
    }
    let vc = ZaliznyakVerbClass::parse("1a").unwrap();
    for slot in all_verb_slots() {
        if let Ok(Some(p)) = verb("citatj", &vc, ipf(), &PrincipalPartsRef::default(), slot) {
            assert!(!p.trace.is_empty(), "{slot:?} has an empty trace");
        }
    }
    assert!(Trace::new("x").steps().len() == 1);
}

// --------------------------------------------------------------------------
// 4. policy_isolation — a rule only changes what it claims to.
//    Witness: make gap.fill-defective-1sg also alter the 3pl.
// --------------------------------------------------------------------------
#[test]
fn policy_isolation() {
    let base = Policy::attested();
    let with = Policy::attested().with(policy::GAP_FILL_DEFECTIVE_1SG);
    assert!(!base.has(policy::GAP_FILL_DEFECTIVE_1SG));
    assert!(with.has(policy::GAP_FILL_DEFECTIVE_1SG));
    assert_eq!(with.active().len(), 1);
    assert_eq!(
        with.clone().without(policy::GAP_FILL_DEFECTIVE_1SG),
        base,
        "removing a rule must restore the baseline exactly"
    );
    // Every registry rule is off in both presets until phase 6 prices it.
    for r in policy::RULES {
        assert!(!Policy::attested().has(r.id), "{} on in attested", r.id);
        assert!(
            !Policy::regularized().has(r.id),
            "{} on in regularized",
            r.id
        );
    }
}

// --------------------------------------------------------------------------
// 5. attested_is_pure — output depends only on the arguments.
//    Witness: read an environment variable inside a rule.
// --------------------------------------------------------------------------
#[test]
fn attested_is_pure() {
    let class = NounClass {
        stem: StemClass::Hard,
        accent: AccentPattern::A,
        reducible: false,
    };
    let once = noun(
        "stol",
        class,
        Gender::Masculine,
        Animacy::Inanimate,
        Case::Gen,
        Number::Singular,
    );
    for _ in 0..100 {
        let again = noun(
            "stol",
            class,
            Gender::Masculine,
            Animacy::Inanimate,
            Case::Gen,
            Number::Singular,
        );
        assert_eq!(
            once.as_ref().map(|p| &p.text),
            again.as_ref().map(|p| &p.text)
        );
    }
}

// --------------------------------------------------------------------------
// 6. output_is_valid_ruthenian — every emitted string parses.
//    Witness: emit a raw ъ or a stray uppercase mid-word.
// --------------------------------------------------------------------------
#[test]
fn output_is_valid_ruthenian() {
    let mut checked = 0;
    for stem in STEMS {
        let class = NounClass {
            stem,
            accent: AccentPattern::A,
            reducible: false,
        };
        for g in GENDERS {
            for c in CASES {
                for n in NUMBERS {
                    if let Some(p) = noun("stolj", class, g, Animacy::Inanimate, c, n) {
                        assert!(
                            Ruthenian::parse(&p.text).is_ok(),
                            "{:?} is not valid Ruthenian",
                            p.text
                        );
                        checked += 1;
                    }
                }
            }
        }
    }
    for code in ["1a", "2a", "4b", "6c"] {
        let vc = ZaliznyakVerbClass::parse(code).unwrap();
        for slot in all_verb_slots() {
            if let Ok(Some(p)) = verb("citatj", &vc, ipf(), &PrincipalPartsRef::default(), slot) {
                for word in p.text.split(' ') {
                    assert!(
                        Ruthenian::parse(word).is_ok(),
                        "{code} {slot:?}: {:?} is not valid Ruthenian",
                        p.text
                    );
                }
                checked += 1;
            }
        }
    }
    for c in CASES {
        for n in NUMBERS {
            for g in GENDERS {
                if let Some(p) = adjective("nov", c, n, g, Animacy::Inanimate, AdjForm::Long) {
                    assert!(Ruthenian::parse(&p.text).is_ok(), "{:?}", p.text);
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 200, "only {checked} forms checked");
}

// --------------------------------------------------------------------------
// 7. stress_placed — a class that determines stress produces exactly one mark.
//    Witness: emit a form with no stress mark, or with two.
// --------------------------------------------------------------------------
#[test]
fn stress_placed() {
    let class = NounClass {
        stem: StemClass::Hard,
        accent: AccentPattern::A,
        reducible: false,
    };
    for c in CASES {
        for n in NUMBERS {
            let p = noun(
                "sto\u{301}l",
                class,
                Gender::Masculine,
                Animacy::Inanimate,
                c,
                n,
            );
            if let Some(p) = p {
                let marks = p.text.matches(phono::STRESS).count();
                assert_eq!(
                    marks, 1,
                    "{c:?} {n:?}: {:?} has {marks} stress marks",
                    p.text
                );
            }
        }
    }
    // A stressed infinitive keeps its stress through the non-past.
    let vc = ZaliznyakVerbClass::parse("1a").unwrap();
    let got = verb(
        "cita\u{301}tj",
        &vc,
        ipf(),
        &PrincipalPartsRef::default(),
        VerbSlot::Finite {
            person: Person::First,
            number: Number::Singular,
            tense: Tense::Present,
        },
    )
    .unwrap()
    .unwrap();
    assert_eq!(got.text.matches(phono::STRESS).count(), 1, "{:?}", got.text);
}

// --------------------------------------------------------------------------
// 8. morphophonology_single_owner — one seam module.
//    Witness: copy `mutate_present_stem` into verb.rs; the source check finds a
//    second definition and the behavioural check finds them disagreeing.
// --------------------------------------------------------------------------
#[test]
fn morphophonology_single_owner() {
    let sources = [
        ("noun.rs", include_str!("../src/noun.rs")),
        ("verb.rs", include_str!("../src/verb.rs")),
        ("adjective.rs", include_str!("../src/adjective.rs")),
        ("pronoun.rs", include_str!("../src/pronoun.rs")),
        ("numeral.rs", include_str!("../src/numeral.rs")),
    ];
    for (name, src) in sources {
        assert!(
            !src.contains("fn mutate_present_stem"),
            "{name} defines a second mutation implementation"
        );
        assert!(
            !src.contains("const MUTATIONS"),
            "{name} defines a second mutation table"
        );
    }
    // Behavioural half: the one implementation is the one being used.
    assert_eq!(phono::mutate_present_stem("pis"), "pisz");
    assert_eq!(phono::mutate_present_stem("ljub"), "ljublj");
}

// --------------------------------------------------------------------------
// 9. no_lexical_data — no word lists anywhere.
//    Witness: add a `const IRREGULARS: &[(&str, &str)]`.
// --------------------------------------------------------------------------
#[test]
fn no_lexical_data() {
    let sources = [
        include_str!("../src/noun.rs"),
        include_str!("../src/verb.rs"),
        include_str!("../src/adjective.rs"),
        include_str!("../src/phono.rs"),
        include_str!("../src/class.rs"),
    ];
    for src in sources {
        for needle in ["IRREGULARS", "EXCEPTIONS", "LEMMA_TABLE", "KNOWN_"] {
            assert!(!src.contains(needle), "lexical data table {needle} in core");
        }
    }
}

// --------------------------------------------------------------------------
// 10. no_dependencies — orthography only.
//     Witness: add any third-party dependency.
// --------------------------------------------------------------------------
#[test]
fn no_dependencies() {
    let manifest = include_str!("../Cargo.toml");
    let deps = manifest
        .split("[dependencies]")
        .nth(1)
        .expect("a [dependencies] section must exist");
    let entries: Vec<&str> = deps
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('['))
        .collect();
    assert_eq!(
        entries.len(),
        1,
        "only ruthenian-orthography may be depended on, found: {entries:?}"
    );
    assert!(entries[0].starts_with("ruthenian-orthography"));
    assert!(!manifest.contains("[dev-dependencies]"));
}

// --------------------------------------------------------------------------
// 11. structural_gaps_are_derived (new this phase)
//     Witness: make a present-tense slot of a perfective return a form.
// --------------------------------------------------------------------------
#[test]
fn structural_gaps_are_derived() {
    let vc = ZaliznyakVerbClass::parse("1a").unwrap();
    let parts = PrincipalPartsRef::default();

    // A perfective has no present tense, and no present participles or gerund.
    for person in [Person::First, Person::Second, Person::Third] {
        for number in NUMBERS {
            let slot = VerbSlot::Finite {
                person,
                number,
                tense: Tense::Present,
            };
            assert_eq!(
                verb("citatj", &vc, pf(), &parts, slot).unwrap(),
                None,
                "a perfective must have no present tense: {slot:?}"
            );
            // …while the same slot exists for an imperfective.
            assert!(verb("citatj", &vc, ipf(), &parts, slot).unwrap().is_some());
        }
    }
    for kind in [ParticipleKind::Adjectival, ParticipleKind::Adverbial] {
        for voice in [Voice::Active, Voice::Passive] {
            let slot = VerbSlot::Participle {
                kind,
                voice,
                tense: Tense::Present,
            };
            assert_eq!(verb("citatj", &vc, pf(), &parts, slot).unwrap(), None);
        }
    }
    // An intransitive verb has no passive participle.
    let intr = VerbInfo {
        transitive: Some(false),
        ..ipf()
    };
    for tense in [Tense::Present, Tense::Past] {
        let slot = VerbSlot::Participle {
            kind: ParticipleKind::Adjectival,
            voice: Voice::Passive,
            tense,
        };
        assert_eq!(verb("citatj", &vc, intr, &parts, slot).unwrap(), None);
    }
    // And the derivation consults no data: it is a pure function of the grammar.
    assert!(!ruthenian_core::slot_exists(
        VerbSlot::Finite {
            person: Person::First,
            number: Number::Singular,
            tense: Tense::Present
        },
        pf()
    ));
}

// --------------------------------------------------------------------------
// 12. class_codes_parse (new this phase)
//     Witness: add a code the parser silently defaults on.
// --------------------------------------------------------------------------
#[test]
fn class_codes_parse() {
    let codes: Vec<&str> = CLASS_CODES
        .lines()
        .last()
        .unwrap_or("")
        .split_whitespace()
        .collect();
    assert!(
        codes.len() > 100,
        "only {} codes in the corpus",
        codes.len()
    );
    let mut failed = Vec::new();
    for code in &codes {
        if ZaliznyakVerbClass::parse(code).is_err() {
            failed.push(*code);
        }
    }
    assert!(failed.is_empty(), "unparsed class codes: {failed:?}");

    // irreg and `-` are valid codes with their own meaning, not parse failures.
    assert!(ZaliznyakVerbClass::parse("irreg").unwrap().irregular);
    assert!(ZaliznyakVerbClass::parse("-").unwrap().unclassified);
    assert!(
        ZaliznyakVerbClass::parse("irreg")
            .unwrap()
            .needs_principal_parts()
    );

    // An unrecognized code is an error, never a silent default.
    for bad in ["99z", "zz!", "4a??"] {
        assert!(
            ZaliznyakVerbClass::parse(bad).is_err(),
            "{bad:?} must not parse"
        );
    }
}

// --------------------------------------------------------------------------
// 13. mutation_is_class_conditioned (new this phase)
//     Witness: key the mutation on the stem's final consonant instead of the
//     class; hundreds of -ivatj/-yvatj verbs break.
// --------------------------------------------------------------------------
#[test]
fn mutation_is_class_conditioned() {
    let parts = PrincipalPartsRef::default();
    let slot = VerbSlot::Finite {
        person: Person::First,
        number: Number::Singular,
        tense: Tense::Present,
    };
    // Class 1 with a labial-final stem takes no epenthesis: probivatj -> probivaju,
    // never *probivlju. 670 sampled verbs have this shape.
    let c1 = ZaliznyakVerbClass::parse("1a").unwrap();
    for inf in ["probivatj", "nalazzivatj", "podumyvatj", "unyvatj"] {
        let got = verb(inf, &c1, ipf(), &parts, slot).unwrap().unwrap();
        assert!(
            !got.text.contains("lj") || inf.contains("lj"),
            "{inf}: class 1 must not take labial epenthesis, got {}",
            got.text
        );
        assert!(
            got.text.starts_with(&inf[..inf.len() - 2]),
            "{inf}: class 1 stem must survive intact, got {}",
            got.text
        );
    }
    // Class 4 with the same shape does mutate: ljubitj -> ljublju.
    let c4 = ZaliznyakVerbClass::parse("4c").unwrap();
    let got = verb("ljubitj", &c4, ipf(), &parts, slot).unwrap().unwrap();
    assert!(
        phono::unstress(&got.text).starts_with("ljublj"),
        "class 4 must take labial epenthesis, got {}",
        got.text
    );
}
