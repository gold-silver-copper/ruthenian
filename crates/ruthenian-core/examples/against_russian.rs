//! Compare Ruthenian forms against their Russian equivalents.
//!
//! The Russian forms are transliterated by `ruthenian-orthography` rather than
//! by hand, so both sides are in the same alphabet and no typo of mine can
//! invent a difference.
//!
//! Only the cells Russian **has** are compared: six cases in two numbers. The
//! vocative, the ablative and the whole dual are Ruthenian's own restorations
//! and there is nothing to compare them against.
//!
//! This is an *example*, not a test: Russian is evidence about what a reader
//! will recognize, not an authority over the specification (§1 — grammar
//! follows OCS, phonology follows Russian).
//!
//! ```text
//! cargo run -p ruthenian-core --example against_russian
//! ```

use ruthenian_core::{
    Animacy, Case, FiniteTense, Gender, Number, Person, noun, relative, that, this, verb, what, who,
};
use ruthenian_orthography::{Cyrillic, to_latin};

/// The six cases Russian kept, in a fixed order.
const CASES: [(&str, Case); 6] = [
    ("nom", Case::Nominative),
    ("acc", Case::Accusative),
    ("gen", Case::Genitive),
    ("dat", Case::Dative),
    ("ins", Case::Instrumental),
    ("loc", Case::Locative),
];

fn ru(cyrillic: &str) -> String {
    // `ё` is not in the declared alphabet — it is stressed `е` — so a Russian
    // text is normalized first, which is what Russian orthography mostly does
    // anyway (`Unmapped::Yo`).
    let normalized = cyrillic.replace('ё', "е").replace('Ё', "Е");
    to_latin(&Cyrillic::parse(&normalized).expect("well-formed Cyrillic"))
        .as_str()
        .to_string()
}

struct Tally {
    same: usize,
    diff: usize,
}

impl Tally {
    fn row(&mut self, label: &str, ours: &str, theirs_cyrillic: &str) {
        let theirs = ru(theirs_cyrillic);
        let mark = if ours == theirs {
            self.same += 1;
            ""
        } else {
            self.diff += 1;
            "  <-- differs"
        };
        println!("  {label:<18} {ours:<12} {theirs:<12} {theirs_cyrillic:<9}{mark}");
    }

    /// Six cases of one gender/number block, against six Cyrillic forms.
    fn block(&mut self, title: &str, f: &dyn Fn(Case) -> String, russian: [&str; 6]) {
        for (i, (name, case)) in CASES.iter().enumerate() {
            self.row(&format!("{title} {name}"), &f(*case), russian[i]);
        }
    }
}

fn main() {
    let mut t = Tally { same: 0, diff: 0 };
    let (m, n, f) = (Gender::Masculine, Gender::Neuter, Gender::Feminine);
    let (sg, pl) = (Number::Singular, Number::Plural);
    let inan = Animacy::Inanimate;
    println!("  cell               ruthenian    russian      cyrillic\n");

    println!("tot — Russian тот");
    t.block(
        "sg m",
        &|c| that(c, sg, m, inan),
        ["тот", "тот", "того", "тому", "тем", "том"],
    );
    t.block(
        "sg n",
        &|c| that(c, sg, n, inan),
        ["то", "то", "того", "тому", "тем", "том"],
    );
    t.block(
        "sg f",
        &|c| that(c, sg, f, inan),
        ["та", "ту", "той", "той", "той", "той"],
    );
    t.block(
        "pl",
        &|c| that(c, pl, m, inan),
        ["те", "те", "тех", "тем", "теми", "тех"],
    );
    t.row(
        "sg m acc anim",
        &that(Case::Accusative, sg, m, Animacy::Animate),
        "того",
    );

    println!("\nsjej — Russian сей (archaic; the everyday word is этот)");
    t.block(
        "sg m",
        &|c| this(c, sg, m, inan),
        ["сей", "сей", "сего", "сему", "сим", "сем"],
    );
    t.block(
        "sg n",
        &|c| this(c, sg, n, inan),
        ["сие", "сие", "сего", "сему", "сим", "сем"],
    );
    t.block(
        "sg f",
        &|c| this(c, sg, f, inan),
        ["сия", "сию", "сей", "сей", "сею", "сей"],
    );
    t.block(
        "pl",
        &|c| this(c, pl, m, inan),
        ["сии", "сии", "сих", "сим", "сими", "сих"],
    );

    println!("\nkto / czto — Russian кто / что");
    t.block("kto", &who, ["кто", "кого", "кого", "кому", "кем", "ком"]);
    t.block("czto", &what, ["что", "что", "чего", "чему", "чем", "чём"]);

    println!("\nizzje — Russian иже (a relic; the modern word is который)");
    t.row("nom", &relative(Case::Nominative, sg, m), "иже");
    t.row("gen", &relative(Case::Genitive, sg, m), "егоже");
    t.row("dat", &relative(Case::Dative, sg, m), "емуже");

    println!("\nthe soft nouns, which meet the same seam");
    t.block(
        "konj sg",
        &|c| noun("Konj", c, sg),
        ["конь", "коня", "коня", "коню", "конём", "коне"],
    );
    t.row("konj nom pl", &noun("Konj", Case::Nominative, pl), "кони");
    t.block(
        "zjemlja sg",
        &|c| noun("zjemlja", c, sg),
        ["земля", "землю", "земли", "земле", "землёй", "земле"],
    );
    t.row(
        "zjemlja nom pl",
        &noun("zjemlja", Case::Nominative, pl),
        "земли",
    );
    t.block(
        "polje sg",
        &|c| noun("polje", c, sg),
        ["поле", "поле", "поля", "полю", "полем", "поле"],
    );

    println!("\nthe present, where Russian has one (§7.4)");
    let pns = [
        ("1sg", Person::First, Number::Singular),
        ("2sg", Person::Second, Number::Singular),
        ("3sg", Person::Third, Number::Singular),
        ("1pl", Person::First, Number::Plural),
        ("3pl", Person::Third, Number::Plural),
    ];
    for (lemma, ru) in [
        (
            "czitatj",
            ["читаю", "читаешь", "читает", "читаем", "читают"],
        ),
        (
            "njegodovatj",
            ["негодую", "негодуешь", "негодует", "негодуем", "негодуют"],
        ),
        (
            "dvinutj",
            ["двину", "двинешь", "двинет", "двинем", "двинут"],
        ),
        (
            "govoritj",
            ["говорю", "говоришь", "говорит", "говорим", "говорят"],
        ),
        ("ljubitj", ["люблю", "любишь", "любит", "любим", "любят"]),
        ("vidjetj", ["вижу", "видишь", "видит", "видим", "видят"]),
        ("ljetjetj", ["лечу", "летишь", "летит", "летим", "летят"]),
        ("ljeczitj", ["лечу", "лечишь", "лечит", "лечим", "лечат"]),
        ("pisatj'", ["пишу", "пишешь", "пишет", "пишем", "пишут"]),
    ] {
        for (i, (name, p, n)) in pns.iter().enumerate() {
            t.row(
                &format!("{lemma} {name}"),
                &verb(lemma, *p, *n, FiniteTense::NonPast),
                ru[i],
            );
        }
    }

    let total = t.same + t.diff;
    println!(
        "\n{} of {total} identical to Russian; {} differ",
        t.same, t.diff
    );
}
