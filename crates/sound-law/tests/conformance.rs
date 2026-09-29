//! Offline shared-subset conformance cases.
//!
//! All stimuli and expected outputs in this file are independently authored.
//! `source_id` identifies the upstream behavioral category that motivated a
//! case; it does not mean the upstream test text or data was copied.

use sound_law::{RuleSet, sound_laws};

const BASIC_REWRITES: RuleSet<'static> = sound_laws! {
    classes {}
    rules {
        labialize: "p" => "f";
        raise: "a" => "e";
    }
};

const PAIRED_MAP: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e"];
        STOP = ["p", "t", "k"];
    }
    maps {
        VOICE = ["p" => "b", "t" => "d", "k" => "g"];
    }
    rules {
        voice: "{stop:STOP}" => "{stop|VOICE}" / "{V}" _ "{V}";
    }
};

const ENVIRONMENTS: RuleSet<'static> = sound_laws! {
    classes { V = ["a", "e", "i"]; }
    rules {
        lenite: "s" => "h" / "{V}" _ "{V}" unless "i" _ "";
    }
};

const REORDER_AND_IDENTITY: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e"];
        C = ["p", "t", "k", "r"];
    }
    rules {
        shorten: "{v:V}{=v}" => "{v}";
        metathesize: "{onset:C+}{v:V}" => "{v}{onset}";
    }
};

const BOUNDARIES: RuleSet<'static> = sound_laws! {
    classes {}
    rules {
        lose_initial_h: "h" => "" / "^" _ "";
        devoice_final_b: "b" => "p" / "" _ "$";
    }
};

const WORD_BOUNDARIES: RuleSet<'static> = sound_laws! {
    classes {}
    rules {
        lose_word_initial_h: "h" => "" / "#" _ "";
        devoice_word_final_b: "b" => "p" / "" _ "#";
    }
};

const INSERTION: RuleSet<'static> = sound_laws! {
    classes { C = ["p", "t", "k"] ; }
    rules {
        break_cluster: "" => "ə" / "{C}" _ "{C}";
    }
};

const LEFT_OVERLAP: RuleSet<'static> = sound_laws! {
    classes {}
    rules { contract: "aa" => "X"; }
};

const RIGHT_OVERLAP: RuleSet<'static> = sound_laws! {
    classes {}
    rules { contract[rtl]: "aa" => "X"; }
};

const FIRST_ONLY: RuleSet<'static> = sound_laws! {
    classes {}
    rules { mark[once]: "a" => "!"; }
};

const LAST_ONLY: RuleSet<'static> = sound_laws! {
    classes {}
    rules { mark[rtl, once]: "a" => "!"; }
};

const ORDERED: RuleSet<'static> = sound_laws! {
    classes {}
    rules {
        fortify: "a" => "p";
        voice: "p" => "b";
    }
};

struct Case {
    id: &'static str,
    source: &'static str,
    source_id: &'static str,
    provenance: &'static str,
    tags: &'static [&'static str],
    laws: &'static RuleSet<'static>,
    input: &'static str,
    expected: &'static str,
}

const CASES: &[Case] = &[
    Case {
        id: "brassica-literal-ordered",
        source: "Brassica",
        source_id: "test/changes.bsc: literal changes and rule ordering",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["literal", "ordered", "simultaneous"],
        laws: &BASIC_REWRITES,
        input: "papa",
        expected: "fefe",
    },
    Case {
        id: "brassica-corresponding-categories",
        source: "Brassica",
        source_id: "test/changes.bsc: corresponding category alternatives",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["class", "map", "environment"],
        laws: &PAIRED_MAP,
        input: "apata aketa",
        expected: "abada ageda",
    },
    Case {
        id: "brassica-environment-exclusion",
        source: "Brassica",
        source_id: "test/changes.bsc: positive and excluded environments",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["environment", "negative-context"],
        laws: &ENVIRONMENTS,
        input: "asa isi ese",
        expected: "aha isi ehe",
    },
    Case {
        id: "brassica-metathesis",
        source: "Brassica",
        source_id: "test/changes.bsc: metathesis",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["capture", "reordering", "quantifier"],
        laws: &REORDER_AND_IDENTITY,
        input: "ptra",
        expected: "aptr",
    },
    Case {
        id: "brassica-degemination",
        source: "Brassica",
        source_id: "test/changes.bsc: identity-sensitive degemination",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["capture", "backreference", "deletion"],
        laws: &REORDER_AND_IDENTITY,
        input: "aapta",
        expected: "aapt",
    },
    Case {
        id: "brassica-edge-conditions",
        source: "Brassica",
        source_id: "test/changes.bsc: boundary conditions",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["form-boundary", "deletion"],
        laws: &BOUNDARIES,
        input: "hab",
        expected: "ap",
    },
    Case {
        id: "brassica-epenthesis",
        source: "Brassica",
        source_id: "test/changes.bsc: insertion",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["zero-width", "insertion", "unicode"],
        laws: &INSERTION,
        input: "apta",
        expected: "apəta",
    },
    Case {
        id: "brassica-left-to-right-overlap",
        source: "Brassica",
        source_id: "test/changes.bsc: default direction",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["overlap", "left-to-right"],
        laws: &LEFT_OVERLAP,
        input: "aaaaa",
        expected: "XXa",
    },
    Case {
        id: "brassica-right-to-left-overlap",
        source: "Brassica",
        source_id: "test/changes.bsc: -rtl",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["overlap", "right-to-left"],
        laws: &RIGHT_OVERLAP,
        input: "aaaaa",
        expected: "aXX",
    },
    Case {
        id: "brassica-once-left",
        source: "Brassica",
        source_id: "test/changes.bsc: -1",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["once", "left-to-right"],
        laws: &FIRST_ONLY,
        input: "ara",
        expected: "!ra",
    },
    Case {
        id: "brassica-once-right",
        source: "Brassica",
        source_id: "test/changes.bsc: -rtl combined with -1",
        provenance: "independently-re-authored; upstream BSD-3-Clause",
        tags: &["once", "right-to-left"],
        laws: &LAST_ONLY,
        input: "ara",
        expected: "ar!",
    },
    Case {
        id: "lexurgy-sequential-rules",
        source: "Lexurgy",
        source_id: "TestRules.kt: sequential rule application",
        provenance: "independently-re-authored behavioral witness; upstream GPL-3.0",
        tags: &["ordered", "sequential"],
        laws: &ORDERED,
        input: "tara",
        expected: "tbrb",
    },
    Case {
        id: "lexurgy-environment-observes-original-rule-input",
        source: "Lexurgy",
        source_id: "TestEnvironment.kt: simultaneous environment observation",
        provenance: "independently-re-authored behavioral witness; upstream GPL-3.0",
        tags: &["environment", "simultaneous"],
        laws: &ENVIRONMENTS,
        input: "asasa",
        expected: "ahaha",
    },
    Case {
        id: "lexurgy-negated-environment",
        source: "Lexurgy",
        source_id: "TestNegation.kt and TestEnvironment.kt: excluded environment",
        provenance: "independently-re-authored behavioral witness; upstream GPL-3.0",
        tags: &["negation", "environment"],
        laws: &ENVIRONMENTS,
        input: "asi ise",
        expected: "ahi ise",
    },
    Case {
        id: "lexurgy-symbol-longest-and-emitter",
        source: "Lexurgy",
        source_id: "TestSymbolMatcherAndEmitter.kt: paired finite emission",
        provenance: "independently-re-authored behavioral witness; upstream GPL-3.0",
        tags: &["class", "map", "emitter"],
        laws: &PAIRED_MAP,
        input: "epete",
        expected: "ebede",
    },
    Case {
        id: "lexurgy-repeaters",
        source: "Lexurgy",
        source_id: "TestRepeaters.kt: repeated category matcher",
        provenance: "independently-re-authored behavioral witness; upstream GPL-3.0",
        tags: &["quantifier", "capture", "greedy"],
        laws: &REORDER_AND_IDENTITY,
        input: "ktrape",
        expected: "aktrep",
    },
    Case {
        id: "lexurgy-boundary-subset",
        source: "Lexurgy",
        source_id: "TestBoundaries.kt: whole-form edge subset only",
        provenance: "independently-re-authored behavioral witness; upstream GPL-3.0",
        tags: &["form-boundary", "unicode"],
        laws: &BOUNDARIES,
        input: "hæb",
        expected: "æp",
    },
    Case {
        id: "lexurgy-multiword-boundary-subset",
        source: "Lexurgy",
        source_id: "TestMultiWord.kt and TestBoundaries.kt: whitespace-delimited word edges",
        provenance: "independently-re-authored behavioral witness; upstream GPL-3.0",
        tags: &["word-boundary", "multi-word", "unicode"],
        laws: &WORD_BOUNDARIES,
        input: "hab ahab\thæb",
        expected: "ap ahap\tæp",
    },
];

#[test]
fn shared_subset_cases() {
    assert_eq!(CASES.len(), 18, "update the documented conformance count");
    for case in CASES {
        let actual = case.laws.apply(case.input).unwrap_or_else(|error| {
            panic!(
                "{} [{} / {}] failed to compile ({}, tags: {}): {error}",
                case.id,
                case.source,
                case.source_id,
                case.provenance,
                case.tags.join(", ")
            )
        });
        assert_eq!(
            actual,
            case.expected,
            "{} [{} / {}] ({}, tags: {})",
            case.id,
            case.source,
            case.source_id,
            case.provenance,
            case.tags.join(", ")
        );
    }
}

#[test]
fn trace_case_records_stable_rule_identity_and_intermediates() {
    let program = ORDERED.compile().unwrap();
    let (form, trace) = program.apply_with_trace("a");
    assert_eq!(form, "b");
    assert_eq!(trace.len(), 2);
    assert_eq!(trace[0].rule, "fortify");
    assert_eq!(trace[0].before, "a");
    assert_eq!(trace[0].after, "p");
    assert_eq!(trace[1].rule, "voice");
    assert_eq!(trace[1].after, "b");
}
