//! Compare §5's pronouns against their Russian equivalents.
//!
//! The Russian forms are transliterated by `ruthenian-orthography` rather than
//! by hand, so the comparison is of the same alphabet on both sides and no
//! typo of mine can create a spurious difference.
//!
//! This is an *example*, not a test: Russian is evidence about what a reader
//! will recognize, not an authority over the specification (§1 — grammar
//! follows OCS, phonology follows Russian).
//!
//! ```text
//! cargo run -p ruthenian-core --example against_russian
//! ```

use ruthenian_core::{Animacy::Inanimate, Case, Gender, Number, pronominal, relative, what, who};
use ruthenian_orthography::{Cyrillic, to_latin};

fn ru(cyrillic: &str) -> String {
    to_latin(&Cyrillic::parse(cyrillic).expect("well-formed Cyrillic"))
        .as_str()
        .to_string()
}

fn row(label: &str, ours: String, theirs_cyrillic: &str) {
    let theirs = ru(theirs_cyrillic);
    let mark = if ours == theirs { "" } else { "  <-- differs" };
    println!("  {label:<26} {ours:<12} {theirs:<12} {theirs_cyrillic:<10}{mark}");
}

fn main() {
    let m = Gender::Masculine;
    let sg = Number::Singular;
    let pl = Number::Plural;
    println!("  cell                       ruthenian    russian      cyrillic");

    println!("\ntoj  — against Russian тот");
    println!("{}", "-".repeat(70));
    row(
        "nom sg m",
        pronominal("t", Case::Nominative, sg, m, Inanimate),
        "тот",
    );
    row(
        "gen sg m",
        pronominal("t", Case::Genitive, sg, m, Inanimate),
        "того",
    );
    row(
        "dat sg m",
        pronominal("t", Case::Dative, sg, m, Inanimate),
        "тому",
    );
    row(
        "ins sg m",
        pronominal("t", Case::Instrumental, sg, m, Inanimate),
        "тем",
    );
    row(
        "loc sg m",
        pronominal("t", Case::Locative, sg, m, Inanimate),
        "том",
    );
    row(
        "nom sg n",
        pronominal("t", Case::Nominative, sg, Gender::Neuter, Inanimate),
        "то",
    );
    row(
        "nom sg f",
        pronominal("t", Case::Nominative, sg, Gender::Feminine, Inanimate),
        "та",
    );
    row(
        "acc sg f",
        pronominal("t", Case::Accusative, sg, Gender::Feminine, Inanimate),
        "ту",
    );
    row(
        "gen sg f",
        pronominal("t", Case::Genitive, sg, Gender::Feminine, Inanimate),
        "той",
    );
    row(
        "nom pl",
        pronominal("t", Case::Nominative, pl, m, Inanimate),
        "те",
    );
    row(
        "gen pl",
        pronominal("t", Case::Genitive, pl, m, Inanimate),
        "тех",
    );
    row(
        "dat pl",
        pronominal("t", Case::Dative, pl, m, Inanimate),
        "тем",
    );
    row(
        "ins pl",
        pronominal("t", Case::Instrumental, pl, m, Inanimate),
        "теми",
    );

    println!("\nsjej — against Russian сей (archaic; the everyday word is этот)");
    println!("{}", "-".repeat(70));
    row(
        "nom sg m",
        pronominal("sj", Case::Nominative, sg, m, Inanimate),
        "сей",
    );
    row(
        "gen sg m",
        pronominal("sj", Case::Genitive, sg, m, Inanimate),
        "сего",
    );
    row(
        "dat sg m",
        pronominal("sj", Case::Dative, sg, m, Inanimate),
        "сему",
    );
    row(
        "ins sg m",
        pronominal("sj", Case::Instrumental, sg, m, Inanimate),
        "сим",
    );
    row(
        "loc sg m",
        pronominal("sj", Case::Locative, sg, m, Inanimate),
        "сем",
    );
    row(
        "nom pl",
        pronominal("sj", Case::Nominative, pl, m, Inanimate),
        "сии",
    );

    println!("\nkto — against Russian кто");
    println!("{}", "-".repeat(70));
    row("nominative", who(Case::Nominative), "кто");
    row("accusative", who(Case::Accusative), "кого");
    row("genitive", who(Case::Genitive), "кого");
    row("dative", who(Case::Dative), "кому");
    row("instrumental", who(Case::Instrumental), "кем");
    row("locative", who(Case::Locative), "ком");

    println!("\nczto — against Russian что");
    println!("{}", "-".repeat(70));
    row("nominative", what(Case::Nominative), "что");
    row("accusative", what(Case::Accusative), "что");
    row("genitive", what(Case::Genitive), "чего");
    row("dative", what(Case::Dative), "чему");
    row("instrumental", what(Case::Instrumental), "чем");
    row("locative", what(Case::Locative), "чём");

    println!("\nizzje — against Russian иже (lost; the modern word is который)");
    println!("{}", "-".repeat(70));
    row("nominative", relative(Case::Nominative, sg, m), "иже");
    row("genitive", relative(Case::Genitive, sg, m), "егоже");
    row("dative", relative(Case::Dative, sg, m), "емуже");
    println!();
}
