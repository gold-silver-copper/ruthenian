//! Print paradigms for eyeballing.
//!
//! The corpus proves the engine agrees with the specification. It cannot prove
//! the specification is what the language should be — only a speaker's judgement
//! does that, so this exists to put forms in front of one.
//!
//! ```text
//! cargo run -p ruthenian-core --example samples              # a default set
//! cargo run -p ruthenian-core --example samples -- dom Drug  # named lemmas
//! ```

use ruthenian_core::{Adjective, Animacy, Case, Gender, Noun, Number, comparative, superlative};

/// Lemmas that exercise something distinct, one per line of interest.
const DEFAULT: &[(&str, &str)] = &[
    ("dom", "II masc hard — the reference paradigm"),
    ("Drug", "velar + animate: druzzje against druzi"),
    ("otjec", "the -jec class: c palatalizes to cz"),
    ("Konj", "II masc soft"),
    ("okno", "II neuter hard"),
    ("polje", "II neuter soft"),
    ("zzena", "I hard"),
    ("kniga", "I velar: knigi against knizi"),
    ("zjemlja", "I soft"),
    ("nacija", "I soft, vowel-final stem: nacii"),
    ("Sluga'", "masc in -a, animate"),
    ("noczj'", "III feminine"),
    ("universitet", "a Latin loan, masc II by its ending"),
    ("museum", "a Latin loan keeping its -um"),
    ("nozz", "a hushing stem: rule 2 gives -jem"),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let lemmas: Vec<(&str, &str)> = if args.is_empty() {
        DEFAULT.to_vec()
    } else {
        args.iter().map(|a| (a.as_str(), "")).collect()
    };

    for (lemma, note) in lemmas {
        if note.is_empty() {
            println!("\n{lemma}");
        } else {
            println!("\n{lemma}  — {note}");
        }
        println!("{}", "-".repeat(58));
        let n = Noun::new(lemma);
        println!("               singular       dual           plural");
        for case in Case::ALL {
            println!(
                "{:<14} {:<14} {:<14} {}",
                format!("{case:?}").to_lowercase(),
                n.form(case, Number::Singular),
                n.form(case, Number::Dual),
                n.form(case, Number::Plural),
            );
        }
    }
    adjectives();
    println!();
}

/// §4's two declensions side by side, and §4.3's degrees.
fn adjectives() {
    for stem in ["dobr", "dorog"] {
        let a = Adjective::new(stem);
        println!("\n{stem}  — long / short, masculine · neuter · feminine");
        println!("{}", "-".repeat(74));
        println!("               long                        short");
        for number in Number::ALL {
            println!("  {number:?}");
            for case in Case::ALL {
                let g = |f: &dyn Fn(Case, Number, Gender) -> String| {
                    Gender::ALL.map(|gender| f(case, number, gender)).join(" ")
                };
                println!(
                    "  {:<12} {:<27} {}",
                    format!("{case:?}").to_lowercase(),
                    g(&|c, n, gd| a.long(c, n, gd, Animacy::Inanimate)),
                    g(&|c, n, gd| a.short(c, n, gd, Animacy::Inanimate)),
                );
            }
        }
        println!(
            "  degrees      {} / {}  ->  {}",
            comparative(stem),
            superlative(stem),
            a.long(
                Case::Nominative,
                Number::Singular,
                Gender::Masculine,
                Animacy::Inanimate
            ),
        );
    }
}
