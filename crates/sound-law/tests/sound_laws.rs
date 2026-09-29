use sound_law::{Class, IterationError, Rule, RuleSet, sound_laws};

const RUTHENIAN_SEAM: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e", "i", "o", "u", "y"];
        HUSHING = ["zz", "sz", "cz", "szcz"];
        VELAR_OR_HUSHING = ["k", "g", "h", "zz", "sz", "cz", "szcz"];
    }
    rules {
        // `|` marks the stem/ending seam that the old specialized interpreter
        // received as two separate function arguments.
        drop_glide: "|j" => "|" / "{HUSHING}" _ "{V}";
        y_after_velar_or_hushing: "|y" => "|i" / "{VELAR_OR_HUSHING}" _ "";
        collapse_jj: "|j" => "|" / "j" _ "";
        remove_seam: "|" => "";
    }
};

#[test]
fn subsumes_the_existing_contextual_rewrite_dsl() {
    let laws = RUTHENIAN_SEAM.compile().unwrap();
    assert_eq!(laws.apply("nozz|jy"), "nozzi");
    assert_eq!(laws.apply("czitaj|jesz"), "czitajesz");
    assert_eq!(laws.apply("noczj|ju"), "noczju");
}

#[test]
fn simple_rules_subsume_letter_maps() {
    const PALATALIZATION: RuleSet<'static> = sound_laws! {
        classes {}
        rules {
            k: "k" => "cz";
            g: "g" => "zz";
            h: "h" => "sz";
        }
    };

    assert_eq!(PALATALIZATION.apply("drug").unwrap(), "druzz");
    assert_eq!(PALATALIZATION.apply("ruka").unwrap(), "rucza");
}

#[test]
fn captures_can_be_reordered_duplicated_and_deleted() {
    const METATHESIS: RuleSet<'static> = sound_laws! {
        classes {
            V = ["a", "e", "i", "o", "u"];
            C = ["p", "t", "k", "s", "r"];
        }
        rules {
            swap: "{c:C}{v:V}" => "{v}{c}";
        }
    };
    assert_eq!(METATHESIS.apply("pat ku").unwrap(), "apt uk");

    const ASSIMILATION: RuleSet<'static> = sound_laws! {
        classes { C = ["p", "t", "k"]; }
        rules { regress: "{left:C}{right:C}" => "{right}{right}"; }
    };
    assert_eq!(ASSIMILATION.apply("apta").unwrap(), "atta");
}

#[test]
fn correspondence_maps_preserve_class_pairing() {
    const VOICING: RuleSet<'static> = sound_laws! {
        classes {
            VOICELESS = ["p", "t", "k"];
            V = ["a", "e"];
        }
        maps {
            VOICE = ["p" => "b", "t" => "d", "k" => "g"];
        }
        rules {
            intervocalic_voicing: "{stop:VOICELESS}" => "{stop|VOICE}" / "{V}" _ "{V}";
        }
    };

    assert_eq!(VOICING.apply("apata eke").unwrap(), "abada ege");
}

#[test]
fn correspondence_maps_must_cover_every_possible_capture() {
    const INCOMPLETE: RuleSet<'static> = sound_laws! {
        classes { STOPS = ["p", "t", "k"]; }
        maps { VOICE = ["p" => "b", "t" => "d"]; }
        rules { voice: "{stop:STOPS}" => "{stop|VOICE}"; }
    };

    let error = INCOMPLETE.compile().unwrap_err();
    assert!(error.message().contains("does not cover"));
    assert!(error.message().contains('k'));
}

#[test]
fn excluded_environments_override_positive_environments() {
    const LOSS: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "e", "i"]; }
        rules {
            intervocalic_s_loss: "s" => "" / "{V}" _ "{V}" unless "i" _ "";
        }
    };

    assert_eq!(LOSS.apply("asa isi ese").unwrap(), "aa isi ee");

    const CAPTURED_EXCLUSION: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "e"]; }
        rules {
            no_loss_between_identicals: "s" => "" / "{v:V}" _ "{V}"
                unless "{=v}" _ "{=v}";
        }
    };
    assert_eq!(
        CAPTURED_EXCLUSION.apply("asa ase ese esa").unwrap(),
        "asa ae ese ea"
    );
}

#[test]
fn excluded_environments_cannot_introduce_captures() {
    const INVALID: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "e"]; }
        rules { bad: "s" => "" unless "{v:V}" _ ""; }
    };

    assert!(
        INVALID
            .compile()
            .unwrap_err()
            .message()
            .contains("cannot declare captures")
    );
}

#[test]
fn rule_direction_controls_overlap_selection() {
    const LEFT: RuleSet<'static> = sound_laws! {
        classes {}
        rules { collapse: "aa" => "X"; }
    };
    const RIGHT: RuleSet<'static> = sound_laws! {
        classes {}
        rules { collapse[rtl]: "aa" => "X"; }
    };

    assert_eq!(LEFT.apply("aaa").unwrap(), "Xa");
    assert_eq!(RIGHT.apply("aaa").unwrap(), "aX");
}

#[test]
fn once_applies_at_the_first_match_in_the_selected_direction() {
    const FIRST: RuleSet<'static> = sound_laws! {
        classes {}
        rules { mark[once]: "a" => "X"; }
    };
    const LAST: RuleSet<'static> = sound_laws! {
        classes {}
        rules {
            mark[rtl, once]: "a" => "X";
            insert_last[rtl, once]: "" => "!";
        }
    };

    assert_eq!(FIRST.apply("aaa").unwrap(), "Xaa");
    assert_eq!(LAST.apply("aaa").unwrap(), "aaX!");
}

#[test]
fn quantified_captures_are_greedy_but_backtrack() {
    const CLUSTERS: RuleSet<'static> = sound_laws! {
        classes {
            V = ["a", "e"];
            C = ["s", "t", "r", "p"];
        }
        rules {
            move_onset: "{onset:C+}{v:V}" => "{v}{onset}";
        }
    };
    assert_eq!(CLUSTERS.apply("stra").unwrap(), "astr");

    const OPTIONAL: RuleSet<'static> = sound_laws! {
        classes { V = ["a"]; C = ["p"]; }
        rules { copy: "{onset:C?}{v:V}" => "{v}{onset}"; }
    };
    assert_eq!(OPTIONAL.apply("apa").unwrap(), "aap");

    const ZERO_OR_MORE: RuleSet<'static> = sound_laws! {
        classes { V = ["a"]; C = ["p"]; }
        rules { move: "{onset:C*}{v:V}" => "{v}{onset}"; }
    };
    assert_eq!(ZERO_OR_MORE.apply("ppa").unwrap(), "app");
}

#[test]
fn backreferences_enforce_identity() {
    const SHORTEN: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "e", "i", "o", "u"]; }
        rules { identical_vowels: "{v:V}{=v}" => "{v}"; }
    };
    assert_eq!(SHORTEN.apply("aardvark see").unwrap(), "ardvark se");
    assert_eq!(SHORTEN.apply("ae").unwrap(), "ae");

    const BETWEEN_IDENTICAL_VOWELS: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "e"]; }
        rules { intervocalic: "x" => "j" / "{v:V}" _ "{=v}"; }
    };
    assert_eq!(
        BETWEEN_IDENTICAL_VOWELS.apply("axa exe").unwrap(),
        "aja eje"
    );
    assert_eq!(BETWEEN_IDENTICAL_VOWELS.apply("axe").unwrap(), "axe");
}

#[test]
fn anchors_and_context_captures_work_on_unicode_boundaries() {
    const CONTEXT: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "æ"]; }
        rules {
            initial_h_loss: "h" => "" / "^" _ "";
            final_devoice: "b" => "p" / "" _ "$";
            echo_left_vowel: "t" => "{v}t" / "{v:V}" _ "";
        }
    };
    assert_eq!(CONTEXT.apply("hatab").unwrap(), "aatap");
    assert_eq!(CONTEXT.apply("hæt").unwrap(), "ææt");
}

#[test]
fn word_boundaries_are_distinct_from_complete_form_anchors() {
    const WORDS: RuleSet<'static> = sound_laws! {
        classes {}
        rules {
            initial_h_loss: "h" => "" / "#" _ "";
            final_devoice: "b" => "p" / "" _ "#";
            literal_hash: r"\#" => "number";
        }
    };

    assert_eq!(
        WORDS.apply("hab ahab\thæb #").unwrap(),
        "ap ahap\tæp number"
    );
}

#[test]
fn multi_scalar_class_members_are_tried_longest_first() {
    const AFFRICATES: RuleSet<'static> = sound_laws! {
        classes {
            C = ["t", "ts"];
            V = ["a"];
        }
        rules { swap: "{c:C}{v:V}" => "{v}{c}"; }
    };
    assert_eq!(AFFRICATES.apply("tsa").unwrap(), "ats");
}

#[test]
fn inline_literal_alternatives_support_sequences_captures_and_quantifiers() {
    const ALTERNATIVES: RuleSet<'static> = sound_laws! {
        classes {}
        maps { RAISE = ["a" => "e", "ai" => "ei"]; }
        rules {
            raise: "{v:[a|ai]}" => "{v|RAISE}";
            move_cluster: "{cluster:[st|p]+}u" => "u{cluster}";
        }
    };

    assert_eq!(ALTERNATIVES.apply("a ai stpu").unwrap(), "e ei ustp");

    const ESCAPED_SEPARATOR: RuleSet<'static> = sound_laws! {
        classes {}
        rules { mark: r"{x:[a\|b|c]}" => "[{x}]"; }
    };
    assert_eq!(ESCAPED_SEPARATOR.apply("a|bc").unwrap(), "[a|b][c]");
}

#[test]
fn finite_class_expressions_support_union_intersection_and_difference() {
    const SETS: RuleSet<'static> = sound_laws! {
        classes {
            C = ["p", "t", "k", "s", "m"];
            STOP = ["p", "t", "k"];
            CORONAL = ["t", "s"];
            V = ["a", "e"];
        }
        rules {
            mark_coronal_stop: "{x:STOP&CORONAL}" => "<{x}>";
            mark_non_labial_consonant: "{x:C-[p|m]}" => "[{x}]";
            mark_remaining_segment: "{x:C|V}" => "({x})";
        }
    };

    assert_eq!(
        SETS.apply("ptksmae").unwrap(),
        "(p)<[(t)]>[(k)][(s)](m)(a)(e)"
    );
}

#[test]
fn negated_classes_and_wildcards_capture_unicode_scalars() {
    const GENERIC: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "e"]; }
        rules {
            mark_non_vowels: "{x:!V}" => "[{x}]";
        }
    };
    assert_eq!(GENERIC.apply("aże").unwrap(), "a[ż]e");

    const REVERSE_TWO: RuleSet<'static> = sound_laws! {
        classes {}
        rules { reverse: "{a:.}{b:.}" => "{b}{a}"; }
    };
    assert_eq!(REVERSE_TWO.apply("żæ").unwrap(), "æż");
}

#[test]
fn one_rule_is_simultaneous_and_rules_are_ordered() {
    const ORDERED: RuleSet<'static> = sound_laws! {
        classes {}
        rules {
            double_a: "a" => "aa";
            raise: "a" => "e";
        }
    };
    assert_eq!(ORDERED.apply("aa").unwrap(), "eeee");
}

#[test]
fn empty_targets_insert_once_at_each_eligible_boundary() {
    const EPENTHESIS: RuleSet<'static> = sound_laws! {
        classes { C = ["k", "t"]; }
        rules { schwa: "" => "ə" / "{C}" _ "{C}"; }
    };
    assert_eq!(EPENTHESIS.apply("akta").unwrap(), "akəta");

    const EVERY_BOUNDARY: RuleSet<'static> = sound_laws! {
        classes {}
        rules { mark: "" => "."; }
    };
    assert_eq!(EVERY_BOUNDARY.apply("żæ").unwrap(), ".ż.æ.");

    const CONTEXT_CAPTURE: RuleSet<'static> = sound_laws! {
        classes { V = ["a", "e"]; }
        rules { break_hiatus: "" => "{v}j" / "{v:V}" _ "{=v}"; }
    };
    assert_eq!(CONTEXT_CAPTURE.apply("aa ae").unwrap(), "aaja ae");
}

#[test]
fn boundary_insertions_are_ordered_but_not_self_recursive() {
    const ORDERED: RuleSet<'static> = sound_laws! {
        classes {}
        rules {
            initial_a: "^" => "a";
            raise: "a" => "e";
        }
    };
    assert_eq!(ORDERED.apply("pa").unwrap(), "epe");
}

#[test]
fn traces_only_include_rules_that_changed_the_form() {
    let program = RUTHENIAN_SEAM.compile().unwrap();
    let (form, trace) = program.apply_with_trace("nozz|jy");
    assert_eq!(form, "nozzi");
    assert_eq!(trace.len(), 3);
    assert_eq!(trace[0].rule, "drop_glide");
    assert_eq!(trace[1].rule, "y_after_velar_or_hushing");
    assert_eq!(trace[2].rule, "remove_seam");
}

#[test]
fn named_and_indexed_partial_runs_are_checked() {
    const STAGES: RuleSet<'static> = sound_laws! {
        classes {}
        rules {
            a_to_b: "a" => "b";
            b_to_c: "b" => "c";
            c_to_d: "c" => "d";
        }
    };
    let program = STAGES.compile().unwrap();

    assert_eq!(
        program.rule_names().collect::<Vec<_>>(),
        ["a_to_b", "b_to_c", "c_to_d"]
    );
    assert_eq!(program.apply_range("a", 0..2).unwrap(), "c");
    assert_eq!(
        program.apply_named_range("b", "b_to_c", "c_to_d").unwrap(),
        "d"
    );
    assert_eq!(program.apply_through("a", "b_to_c").unwrap(), "c");
    assert_eq!(program.apply_from("a", "b_to_c").unwrap(), "a");
    assert!(program.apply_range("a", 2..4).is_err());
    assert!(program.apply_named_range("a", "c_to_d", "a_to_b").is_err());
    assert!(program.apply_through("a", "missing").is_err());
}

#[test]
fn compiled_programs_compose_in_order_without_ambiguous_names() {
    const FIRST: RuleSet<'static> = sound_laws! {
        classes {}
        rules { lower: "A" => "a"; }
    };
    const SECOND: RuleSet<'static> = sound_laws! {
        classes {}
        rules { raise: "a" => "e"; }
    };
    let first = FIRST.compile().unwrap();
    let second = SECOND.compile().unwrap();
    let combined = first.then(&second).unwrap();
    assert_eq!(combined.apply("A"), "e");
    assert_eq!(
        combined.rule_names().collect::<Vec<_>>(),
        ["lower", "raise"]
    );

    let error = first.then(&first).unwrap_err();
    assert_eq!(error.duplicate_rule(), "lower");
}

#[test]
fn fixed_point_application_detects_cycles_and_limits() {
    const CYCLIC: RuleSet<'static> = sound_laws! {
        classes {}
        rules {
            b_to_c: "^b$" => "c";
            a_to_b: "^a$" => "b";
            c_to_a: "^c$" => "a";
        }
    };
    let error = CYCLIC
        .compile()
        .unwrap()
        .apply_until_stable("a", 10)
        .unwrap_err();
    assert!(matches!(error, IterationError::Cycle { .. }));

    const GROWING: RuleSet<'static> = sound_laws! {
        classes {}
        rules { grow: "a" => "aa"; }
    };
    let error = GROWING
        .compile()
        .unwrap()
        .apply_until_stable("a", 2)
        .unwrap_err();
    assert!(matches!(error, IterationError::Limit { max_passes: 2, .. }));
}

#[test]
fn matcher_work_limits_are_deterministic_and_identify_the_rule() {
    const AMBIGUOUS: RuleSet<'static> = sound_laws! {
        classes { A = ["a"]; }
        rules { search: "{A*}{A*}{A*}{A*}{A*}b" => "x"; }
    };
    let program = AMBIGUOUS.compile().unwrap();
    let error = program
        .apply_with_limit("aaaaaaaaaaaaaaaa", 25)
        .unwrap_err();
    assert_eq!(error.max_steps(), 25);
    assert_eq!(error.rule(), "search");
    assert_eq!(
        error,
        program
            .apply_with_limit("aaaaaaaaaaaaaaaa", 25)
            .unwrap_err()
    );
    assert_eq!(program.apply_with_limit("aaab", 10_000).unwrap(), "x");
}

#[test]
fn compiler_bounds_recursive_pattern_depth() {
    let target = "{X}".repeat(257);
    let classes = [Class::new("X", &["x"])];
    let rules = [Rule::new("too_deep", &target, "y")];
    let error = RuleSet::new(&classes, &rules).compile().unwrap_err();
    assert!(error.message().contains("maximum is 256"));
}

#[test]
fn invalid_programs_report_the_rule_and_cause() {
    const UNKNOWN_CLASS: RuleSet<'static> = sound_laws! {
        classes {}
        rules { bad: "{x:MISSING}" => "{x}"; }
    };
    let error = UNKNOWN_CLASS.compile().unwrap_err();
    assert_eq!(error.rule(), Some("bad"));
    assert!(error.message().contains("unknown class `MISSING`"));

    const UNKNOWN_CAPTURE: RuleSet<'static> = sound_laws! {
        classes {}
        rules { bad: "x" => "{missing}"; }
    };
    assert!(UNKNOWN_CAPTURE.compile().is_err());

    let classes = [Class::new("X", &["x", "x"])];
    let rules = [Rule::new("duplicate_member", "{X}", "y")];
    assert!(RuleSet::new(&classes, &rules).compile().is_err());
}

#[test]
fn metacharacters_can_be_escaped() {
    const ESCAPED: RuleSet<'static> = sound_laws! {
        classes {}
        rules { braces: r"\{x\}\$" => r"\^"; }
    };
    assert_eq!(ESCAPED.apply("{x}$").unwrap(), "^");
}
