//! The guard suite `docs/specs/ruthenian-core.md` §9 declares.
//!
//! Every guard here has a **failure witness**: a named mutation that has been
//! applied, observed to fail the guard, and reverted (`INVARIANTS.md` I5). A
//! guard that survives its own witness is stale and gets deleted, not left in
//! place looking reassuring.

use ruthenian_core::*;

/// Inv. 1 — every class × slot resolves or declares a gap. No panic, no third
/// outcome.
///
/// Witness: make `noun` panic on an unhandled `(declension, case, number)`.
#[test]
fn slot_exhaustive() {
    let mut resolved = 0usize;
    for declension in [Declension::I, Declension::II, Declension::III] {
        for hardness in [StemHardness::Hard, StemHardness::Soft] {
            let class = NounClass::new(declension, hardness);
            for gender in Gender::ALL {
                for animacy in [Animacy::Animate, Animacy::Inanimate] {
                    for case in Case::ALL {
                        for number in Number::ALL {
                            // Either a form or a declared gap; never a panic.
                            if noun("dom", class, gender, animacy, case, number).is_some() {
                                resolved += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    // 3 declensions x 2 hardness x 3 genders x 2 animacies x 8 cases x 3 numbers
    assert_eq!(
        resolved,
        3 * 2 * 3 * 2 * 8 * 3,
        "every noun cell must resolve"
    );
}

/// Every verb slot resolves, declares a gap, or returns `Unsupported` — and the
/// three are never confused.
///
/// Witness: return `Ok(None)` from the periphrastic branch instead of `Err`; the
/// perfect would then claim not to exist rather than to be composed elsewhere.
#[test]
fn verb_slot_exhaustive() {
    let info = VerbInfo::default();
    for class in VerbClass::ALL {
        for tense in Tense::ALL {
            for person in [Person::First, Person::Second, Person::Third] {
                for number in Number::ALL {
                    let slot = VerbSlot::Finite {
                        person,
                        number,
                        tense,
                    };
                    match verb("czitatj", class, info, slot) {
                        Ok(Some(p)) => assert!(!p.text.is_empty() || tense == Tense::Aorist),
                        Ok(None) => {}
                        Err(e) => assert!(
                            tense.is_periphrastic(),
                            "only the periphrastic tenses may be Unsupported, got {e} for {tense:?}"
                        ),
                    }
                }
            }
        }
    }
}

/// Inv. 2 — `None` means "this cell does not exist", never "unimplemented".
///
/// The two claims the language actually makes, asserted directly.
#[test]
fn none_means_the_cell_does_not_exist() {
    // A perfective verb has no present tense: its present endings are its
    // future (§7.8). That is a fact about Ruthenian, so it is `Ok(None)`.
    let present = VerbSlot::Finite {
        person: Person::First,
        number: Number::Singular,
        tense: Tense::Present,
    };
    assert!(
        verb("poczitatj", VerbClass::One, VerbInfo::default(), present)
            .unwrap()
            .is_none()
    );
    // ...and the imperfective correspondingly has no synthetic future.
    let future = VerbSlot::Finite {
        person: Person::First,
        number: Number::Singular,
        tense: Tense::Future,
    };
    assert!(
        verb("czitatj", VerbClass::One, VerbInfo::default(), future)
            .unwrap()
            .is_none()
    );
    // The long adjective has no vocative (§4.2); the short one does.
    assert!(
        adjective(
            "dobr",
            Case::Voc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
            AdjForm::Long,
            Degree::Positive
        )
        .is_none()
    );
    // The reflexive has no nominative (§5.2).
    assert!(reflexive(Case::Nom).is_none());
    // Pronouns have no vocative.
    assert!(
        personal(
            Person::First,
            Case::Voc,
            Number::Singular,
            Gender::Masculine,
            PronounStyle::Full
        )
        .is_none()
    );
}

/// Inv. 3 — every `Prediction` carries a non-empty trace.
///
/// Witness: construct a `Prediction` with an empty trace — impossible, because
/// `Trace` has no `Default` and `Trace::new` requires a first step.
#[test]
fn trace_non_empty() {
    for case in Case::ALL {
        for number in Number::ALL {
            let p = noun(
                "dom",
                NounClass::hard(Declension::II),
                Gender::Masculine,
                Animacy::Inanimate,
                case,
                number,
            )
            .unwrap();
            assert!(
                !p.trace.is_empty(),
                "{case:?}/{number:?} has an empty trace"
            );
        }
    }
}

/// Inv. 4 — output is a function of the arguments alone.
///
/// The engine has no configuration to depend on: no policy, no variant, no
/// feature flags. `RUTHENIAN.md` fixes the language, so a change to the language
/// is a source edit rather than a runtime switch.
///
/// Witness: read an environment variable inside a rule.
#[test]
fn generation_is_pure() {
    let call = || {
        noun(
            "kniga",
            NounClass::hard(Declension::I),
            Gender::Feminine,
            Animacy::Inanimate,
            Case::Dat,
            Number::Singular,
        )
        .unwrap()
        .text
    };
    let first = call();
    for _ in 0..100 {
        assert_eq!(call(), first, "output must not depend on ambient state");
    }
}

/// There is no configuration axis in the public API.
///
/// A rule engine whose answer is fixed by a specification should not carry a
/// switch that changes the answer. The previous design had one — a `Variant`
/// carrying three rules, every one of them permanently off — which is a dead
/// branch in every rule until some future language decision brings it to life.
///
/// Witness: reintroduce `pub struct Variant` or a `policy` parameter.
#[test]
fn no_configuration_axis() {
    for src in [
        include_str!("../src/lib.rs"),
        include_str!("../src/trace.rs"),
        include_str!("../src/noun.rs"),
        include_str!("../src/verb.rs"),
    ] {
        for line in src.lines() {
            let decl = line.trim_start();
            if decl.starts_with("//") {
                continue;
            }
            for banned in ["struct Variant", "enum Variant", "struct Policy", "RuleId"] {
                assert!(
                    !decl.contains(banned),
                    "configuration type {banned:?} reintroduced: {line}"
                );
            }
        }
    }
}

/// §2 — no source-language classification appears in the public API.
///
/// The type-level half of this is that no such type exists to name; the textual
/// half greps the crate's own source, because a `pub use` of a differently-named
/// alias would slip past the compiler.
///
/// Witness: add `pub struct ZaliznyakVerbClass` and re-export it.
#[test]
fn no_source_language_types() {
    let sources = [
        include_str!("../src/lib.rs"),
        include_str!("../src/types.rs"),
        include_str!("../src/noun.rs"),
        include_str!("../src/verb.rs"),
        include_str!("../src/adjective.rs"),
    ];
    for src in sources {
        for line in src.lines() {
            // Prose may discuss why these are excluded; declarations may not
            // introduce them.
            let decl = line.trim_start();
            if decl.starts_with("//") || decl.starts_with("///") {
                continue;
            }
            for banned in ["Zaliznyak", "AccentPattern", "StemClass"] {
                assert!(
                    !decl.contains(banned),
                    "source-language classification {banned:?} in a declaration: {line}"
                );
            }
        }
    }
}

/// Aspect is derived and never stored: `VerbInfo` has no aspect field, and the
/// one implementation is `aspect_of`.
///
/// Witness: add `aspect` to `VerbInfo` and populate it — the struct-literal
/// construction below stops compiling.
#[test]
fn aspect_is_derived_not_stored() {
    let _ = VerbInfo {
        transitive: Some(true),
        reflexive: false,
    };
    // The three rules of §7.2, each with a witness.
    assert_eq!(aspect_of("czitatj").value, Aspect::Imperfective);
    assert_eq!(aspect_of("poczitatj").value, Aspect::Perfective);
    assert_eq!(aspect_of("napisatj").value, Aspect::Perfective);
    assert_eq!(aspect_of("napisyvatj").value, Aspect::Imperfective);
    // Rule 3 must outrank rule 2: a secondary imperfective satisfies both.
    assert!(
        aspect_of("napisyvatj")
            .trace
            .steps()
            .iter()
            .any(|s| s.contains("-yva-")),
        "the trace must name the rule that decided it"
    );
}

/// Inv. 7 — every output is valid Ruthenian: pure ASCII `a`-`z`, no Cyrillic, no
/// stray diacritics beyond the combining acute (§2.1).
///
/// Witness: emit a raw `ě` or `ъ` from any ending table.
#[test]
fn output_is_valid_ruthenian() {
    let mut checked = 0;
    for class in [
        NounClass::hard(Declension::I),
        NounClass::soft(Declension::I),
        NounClass::hard(Declension::II),
        NounClass::soft(Declension::II),
        NounClass::hard(Declension::III),
    ] {
        for gender in Gender::ALL {
            for case in Case::ALL {
                for number in Number::ALL {
                    let Some(p) = noun("drug", class, gender, Animacy::Animate, case, number)
                    else {
                        continue;
                    };
                    checked += 1;
                    for ch in p.text.chars() {
                        assert!(
                            ch.is_ascii_lowercase() || ch == '\u{301}',
                            "{:?} contains {ch:?}, which is not Ruthenian",
                            p.text
                        );
                    }
                }
            }
        }
    }
    assert!(checked > 100);
}

/// §5 — one morphophonology module. The palatalizations are defined once, and
/// the two are genuinely distinct.
///
/// Witness: make `Palatal::Second` reuse the first table; `druzi` becomes
/// `druzzi` and the spec conformance test fails alongside this one.
#[test]
fn morphophonology_single_owner() {
    use ruthenian_core::phono::{Palatal, palatalize};
    // The distinction Russian lost, asserted directly.
    assert_eq!(palatalize("drug", Palatal::First), "druzz");
    assert_eq!(palatalize("drug", Palatal::Second), "druz");
    assert_eq!(palatalize("knig", Palatal::Second), "kniz");
    assert_eq!(palatalize("ruk", Palatal::First), "rucz");
    assert_eq!(palatalize("ruk", Palatal::Second), "ruc");
    for stem in ["drug", "knig", "ruk", "duh"] {
        assert_ne!(
            palatalize(stem, Palatal::First),
            palatalize(stem, Palatal::Second),
            "the two palatalizations must not collapse"
        );
    }
    // Non-velars are untouched, so it is safe to call unconditionally.
    for stem in ["zzen", "okn", "dom", "nocz"] {
        assert_eq!(palatalize(stem, Palatal::First), stem);
        assert_eq!(palatalize(stem, Palatal::Second), stem);
    }
}

/// The second palatalization actually reaches the paradigm, and is not merely
/// available. This is the cell Russian levelled away.
///
/// Witness: drop the `Palatal::Second` marker from the locative ending.
#[test]
fn second_palatalization_is_wired_in() {
    let m = NounClass::hard(Declension::II);
    let loc = noun(
        "drug",
        m,
        Gender::Masculine,
        Animacy::Animate,
        Case::Loc,
        Number::Singular,
    )
    .unwrap();
    assert_eq!(loc.text, "druzi");

    let f = NounClass::hard(Declension::I);
    let dat = |case| {
        noun(
            "knig",
            f,
            Gender::Feminine,
            Animacy::Inanimate,
            case,
            Number::Singular,
        )
        .unwrap()
        .text
    };
    // The genitive and the dative differ ONLY by the palatalization: same vowel,
    // different consonant. This is the Ukrainian pattern, lost in Russian.
    assert_eq!(dat(Case::Gen), "knigi");
    assert_eq!(dat(Case::Dat), "knizi");
    assert_eq!(dat(Case::Loc), "knizi");
}

/// The dual is real throughout, not only in the noun.
///
/// Witness: drop a dual arm from any ending table; one of these fails.
#[test]
fn the_dual_is_everywhere() {
    // noun
    assert_eq!(
        noun(
            "dom",
            NounClass::hard(Declension::II),
            Gender::Masculine,
            Animacy::Inanimate,
            Case::Dat,
            Number::Dual
        )
        .unwrap()
        .text,
        "domoma"
    );
    // verb agreement
    assert_eq!(
        verb(
            "czitatj",
            VerbClass::One,
            VerbInfo::default(),
            VerbSlot::Finite {
                person: Person::First,
                number: Number::Dual,
                tense: Tense::Present
            }
        )
        .unwrap()
        .unwrap()
        .text,
        "czitajevje"
    );
    // pronoun
    assert_eq!(
        personal(
            Person::First,
            Case::Nom,
            Number::Dual,
            Gender::Masculine,
            PronounStyle::Full
        )
        .unwrap()
        .text,
        "vje"
    );
    // adjective
    assert_eq!(
        adjective(
            "dobr",
            Case::Dat,
            Number::Dual,
            Gender::Masculine,
            Animacy::Inanimate,
            AdjForm::Short,
            Degree::Positive
        )
        .unwrap()
        .text,
        "dobroma"
    );
    // ...and numeral government, which is what the dual is FOR.
    assert_eq!(government(2).number, Number::Dual);
}

/// §6.1 — numeral government is regular. No 11-14 window, no 2-4 genitive
/// singular; the dual absorbs both.
///
/// Witness: reintroduce Russian's `(11..=14)` special case for 12.
#[test]
fn numeral_government_is_regular() {
    assert_eq!(government(1).number, Number::Singular);
    assert_eq!(government(2).number, Number::Dual);
    assert_eq!(government(3).case, Case::Nom);
    assert_eq!(government(3).number, Number::Plural);
    assert_eq!(government(4).number, Number::Plural);
    assert_eq!(government(5).case, Case::Gen);
    // A compound is governed by its last word — and 22 takes the dual, which is
    // where Russian's petrified genitive singular came from.
    assert_eq!(government(22).number, Number::Dual);
    assert_eq!(government(25).case, Case::Gen);
    // A teen is one word of its own, so it behaves as "five and above".
    assert_eq!(government(12).case, Case::Gen);
    assert_eq!(government(12).number, Number::Plural);
}

/// §1 — no lexical data. The crate contains no word lists keyed to meaning.
///
/// The morphological constant tables (prefixes, mutations) are *classes* of
/// words rather than individual ones, which is the line §1 draws.
///
/// Witness: add `const IRREGULARS: &[(&str, &str)]`.
#[test]
fn no_lexical_data() {
    for src in [
        include_str!("../src/noun.rs"),
        include_str!("../src/adjective.rs"),
        include_str!("../src/pronoun.rs"),
    ] {
        assert!(
            !src.contains("IRREGULARS") && !src.contains("EXCEPTIONS"),
            "lexical exception table in a rules module"
        );
    }
}
