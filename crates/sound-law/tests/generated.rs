use sound_law::{RuleSet, sound_laws};

const SIMULTANEOUS: RuleSet<'static> = sound_laws! {
    classes {}
    rules { expand: "a" => "aa"; }
};

const INSERT_EVERYWHERE: RuleSet<'static> = sound_laws! {
    classes {}
    rules { insert: "" => "·"; }
};

const ORDERED: RuleSet<'static> = sound_laws! {
    classes {}
    rules {
        expand: "a" => "aa";
        raise: "a" => "e";
    }
};

fn words(alphabet: &[char], max_scalars: usize) -> Vec<String> {
    let mut all = vec![String::new()];
    let mut frontier = vec![String::new()];
    for _ in 0..max_scalars {
        let mut next = Vec::new();
        for prefix in &frontier {
            for symbol in alphabet {
                let mut word = prefix.clone();
                word.push(*symbol);
                next.push(word);
            }
        }
        all.extend(next.iter().cloned());
        frontier = next;
    }
    all
}

#[test]
fn generated_simultaneous_rules_preserve_unmatched_material() {
    let program = SIMULTANEOUS.compile().unwrap();
    for input in words(&['a', 'b', 'ż'], 6) {
        let expected = input.replace('a', "aa");
        assert_eq!(program.apply(&input), expected, "input {input:?}");
    }
}

#[test]
fn generated_insertion_visits_each_original_scalar_boundary_once() {
    let program = INSERT_EVERYWHERE.compile().unwrap();
    for input in words(&['a', 'æ', '界'], 5) {
        let output = program.apply(&input);
        assert!(output.is_char_boundary(output.len()));
        assert_eq!(
            output.chars().filter(|ch| *ch == '·').count(),
            input.chars().count() + 1,
            "input {input:?}, output {output:?}"
        );
        assert_eq!(output.replace('·', ""), input);
    }
}

#[test]
fn generated_later_rules_observe_earlier_output() {
    let program = ORDERED.compile().unwrap();
    for input in words(&['a', 'b', 'λ'], 6) {
        let expected = input.replace('a', "ee");
        assert_eq!(program.apply(&input), expected, "input {input:?}");
    }
}
