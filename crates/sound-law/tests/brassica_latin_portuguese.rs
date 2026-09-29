//! Complete shared-semantics translation of Brassica's Latin-to-Portuguese
//! example at revision f3a92ad0eb5a890fdf92208c48325371e5e770ff.
//!
//! The source rule/word files are BSD-3-Clause. See
//! `fixtures/BRASSICA_LICENSE`. Rule names and DSL spelling are local
//! adaptations; the ten source forms and ordered transformations come from
//! `examples/latin2port.bsc` and `examples/latin2port.lex`.

use sound_law::{RuleSet, sound_laws};
use std::path::PathBuf;
use std::process::Command;

const LATIN_TO_PORTUGUESE: RuleSet<'static> = sound_laws! {
    classes {
        V = ["a", "e", "i", "o", "u"];
        LONG_V = ["ā", "ē", "ī", "ō", "ū"];
        C = ["p", "t", "c", "q", "b", "d", "g", "m", "n", "l", "r", "j", "h", "f", "v", "s"];
        STOP = ["p", "t", "c"];
    }
    maps {
        SHORTEN = ["ā" => "a", "ē" => "e", "ī" => "i", "ō" => "o", "ū" => "u"];
        VOICE = ["p" => "b", "t" => "d", "c" => "g"];
    }
    rules {
        drop_final_s_or_m: "{[s|m]}" => "" / "" _ "#";
        glide_before_vowel: "i" => "j" / "" _ "{V}";
        shorten_long_vowels: "{v:LONG_V}" => "{v|SHORTEN}";
        drop_final_e_after_vowel_r: "e" => "" / "{V}r" _ "#";
        drop_intervocalic_v: "v" => "" / "{V}" _ "{V}";
        lower_final_u: "u" => "o" / "" _ "#";
        palatalize_gn: "gn" => "nh";
        voice_intervocalic_stops: "{stop:STOP}" => "{stop|VOICE}" / "{V}" _ "{V}";
        front_c_before_t: "c" => "i" / "{[i|e]}" _ "t";
        back_c_before_t: "c" => "u" / "{[o|u]}" _ "t";
        drop_pre_t_p: "p" => "" / "{V}" _ "t";
        contract_ii: "ii" => "i";
        syncopate_pretonic_e: "e" => "" / "{C}" _ "r{V}";
        palatalize_lj: "lj" => "lh";
    }
};

const EXAMPLE_FORMS: &[(&str, &str)] = &[
    ("lector", "leitor"),
    ("doctor", "doutor"),
    ("focus", "fogo"),
    ("jocus", "jogo"),
    ("districtus", "distrito"),
    ("cīvitatem", "cidade"),
    ("adoptare", "adotar"),
    ("opera", "obra"),
    ("secundus", "segundo"),
    ("fīliam", "filha"),
];

#[test]
fn complete_latin_to_portuguese_example() {
    let program = LATIN_TO_PORTUGUESE.compile().unwrap();
    assert_eq!(program.rule_count(), 14);
    for (latin, portuguese) in EXAMPLE_FORMS {
        assert_eq!(program.apply(latin), *portuguese, "source form {latin}");
    }
}

#[test]
#[ignore = "requires BRASSICA_BIN and BRASSICA_CHECKOUT at the pinned revision"]
fn differential_against_pinned_brassica_cli() {
    let executable = std::env::var_os("BRASSICA_BIN").expect("set BRASSICA_BIN");
    let checkout =
        PathBuf::from(std::env::var_os("BRASSICA_CHECKOUT").expect("set BRASSICA_CHECKOUT"));
    let revision = Command::new("git")
        .arg("-C")
        .arg(&checkout)
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("run git");
    assert!(revision.status.success());
    assert_eq!(
        String::from_utf8(revision.stdout).unwrap().trim(),
        "f3a92ad0eb5a890fdf92208c48325371e5e770ff"
    );

    let upstream = Command::new(executable)
        .arg(checkout.join("examples/latin2port.bsc"))
        .arg("--in")
        .arg(checkout.join("examples/latin2port.lex"))
        .arg("--wordlist")
        .output()
        .expect("run pinned Brassica CLI");
    assert!(
        upstream.status.success(),
        "Brassica stderr: {}",
        String::from_utf8_lossy(&upstream.stderr)
    );
    let upstream: Vec<&str> = std::str::from_utf8(&upstream.stdout)
        .unwrap()
        .lines()
        .collect();
    let program = LATIN_TO_PORTUGUESE.compile().unwrap();
    let local: Vec<String> = EXAMPLE_FORMS
        .iter()
        .map(|(input, _)| program.apply(input))
        .collect();
    assert_eq!(upstream, local);
}
