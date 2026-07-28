//! Emit every form of a representative sample, for a speaker to review.
//!
//! Writes a flat table to stdout rather than formatting it, so the forms reach a
//! reviewer without passing through anyone's hands:
//!
//! ```text
//! T <section> <title> <note>      a table begins
//! H <col> <col> …                 its column labels
//! R <row> <cell> <cell> …         a row; a cell marked `*` is diagnostic
//! ```
//!
//! ```text
//! cargo run -p ruthenian-core --example review > sample.tsv
//! ```

use ruthenian_core::*;

/// Mark a cell if it is one of the forms this word is in the sample *for*.
fn cell(form: &str, diagnostic: &[&str]) -> String {
    match diagnostic.contains(&form) {
        true => format!("*{form}"),
        false => form.to_string(),
    }
}

fn table(section: &str, title: &str, note: &str) {
    println!("T\t{section}\t{title}\t{note}");
}

fn header(cols: &[&str]) {
    println!("H\t{}", cols.join("\t"));
}

fn row(label: &str, cells: &[String]) {
    println!("R\t{label}\t{}", cells.join("\t"));
}

const CASES: [(&str, Case); 8] = [
    ("nominative", Case::Nominative),
    ("vocative", Case::Vocative),
    ("accusative", Case::Accusative),
    ("genitive", Case::Genitive),
    ("ablative", Case::Ablative),
    ("dative", Case::Dative),
    ("instrumental", Case::Instrumental),
    ("locative", Case::Locative),
];

fn nouns() {
    let sample: &[(&str, &str, &[&str])] = &[
        (
            "dom",
            "declension II masculine, hard — the reference paradigm",
            &[],
        ),
        (
            "Drug",
            "velar stem, animate. The two palatalizations part the vocative from the locative in the consonant as well as the vowel",
            &["druzze", "druzi"],
        ),
        (
            "otjec",
            "the -jec class. The first palatalization reaches c, and the vocative ending then loses its glide",
            &["otjecze"],
        ),
        (
            "Konj",
            "declension II masculine, soft. Yat makes the locative -je where rule 1's respelt -y keeps the plural -ji",
            &["konje", "konji"],
        ),
        ("okno", "declension II neuter, hard", &[]),
        (
            "polje",
            "declension II neuter, soft. The locative and the nominative fall together, as Russian's поле does",
            &["polje"],
        ),
        ("zzena", "declension I, hard", &[]),
        (
            "kniga",
            "velar feminine. The genitive and the dative differ by the palatalization alone — the vowel is the same",
            &["knigi", "knizi"],
        ),
        (
            "zjemlja",
            "declension I, soft. Yat gives -je after a soft stem, so the dative and locative coincide as §3.5 requires",
            &["zjemlje", "zjemljej"],
        ),
        (
            "nacija",
            "a vowel-final stem. The doubled i is regular and is not contracted",
            &["nacii"],
        ),
        (
            "Sluga'",
            "masculine in -a, animate. Declension I in form, masculine in agreement — and the accusative is -u, not the ablative",
            &["slugu", "sluzi"],
        ),
        (
            "noczj'",
            "declension III. The instrumental keeps the soft sign and takes the ending; the plural drops the softness marker after cz",
            &["noczjju", "noczev", "noczam"],
        ),
        (
            "universitet",
            "a Latin loan: masculine II by its ending, whatever universitas was",
            &[],
        ),
        ("museum", "a Latin loan keeping its -um", &[]),
    ];
    for (lemma, note, diag) in sample {
        table("§3 Nouns", lemma, note);
        header(&["", "singular", "dual", "plural"]);
        for (name, case) in CASES {
            row(
                name,
                Number::ALL
                    .map(|n| cell(&noun(lemma, case, n), diag))
                    .as_ref(),
            );
        }
    }
}

fn adjectives() {
    for (long, title, note) in [
        (
            false,
            "dobr — short, indefinite",
            "the nominal declension: its endings are the noun's exactly, including the animate accusative",
        ),
        (
            true,
            "dobryj — long, definite",
            "the pronominal declension. No vocative — the nominative is used",
        ),
    ] {
        table("§4 Adjectives", title, note);
        header(&["", "masc sg", "neut sg", "fem sg", "dual", "plural"]);
        for (name, case) in CASES {
            let f = |n: Number, g: Gender| match long {
                true => adjective("dobr", case, n, g, Animacy::Inanimate),
                false => short_adjective("dobr", case, n, g, Animacy::Inanimate),
            };
            row(
                name,
                &[
                    f(Number::Singular, Gender::Masculine),
                    f(Number::Singular, Gender::Neuter),
                    f(Number::Singular, Gender::Feminine),
                    f(Number::Dual, Gender::Masculine),
                    f(Number::Plural, Gender::Masculine),
                ],
            );
        }
    }
    table(
        "§4 Adjectives",
        "degrees",
        "a derivation, so each returns a stem that declines like any other adjective. On a velar stem the suffix loses its own glide",
    );
    header(&["stem", "comparative", "superlative", "long comparative"]);
    for s in ["dobr", "dorog", "tih"] {
        let c = comparative(s);
        row(
            s,
            &[
                cell(&c, &["dorozzejsz", "tiszejsz"]),
                superlative(s),
                adjective(
                    &c,
                    Case::Nominative,
                    Number::Singular,
                    Gender::Masculine,
                    Animacy::Inanimate,
                ),
            ],
        );
    }
}

fn pronouns() {
    table(
        "§5 Pronouns",
        "the personal series",
        "person, number and gender select a pronoun; there is no name for one. A clitic follows the slash where §5.1a gives one",
    );
    header(CASES.map(|(n, _)| n).as_ref());
    for (label, person, number) in [
        ("1 sg", Person::First, Number::Singular),
        ("2 sg", Person::Second, Number::Singular),
        ("3 sg masc", Person::Third, Number::Singular),
        ("1 du", Person::First, Number::Dual),
        ("2 du", Person::Second, Number::Dual),
        ("3 du", Person::Third, Number::Dual),
        ("1 pl", Person::First, Number::Plural),
        ("2 pl", Person::Second, Number::Plural),
        ("3 pl", Person::Third, Number::Plural),
    ] {
        row(
            label,
            CASES
                .map(|(_, case)| {
                    let full = pronoun(person, number, Gender::Masculine, case);
                    let clitic = clitic_pronoun(person, number, Gender::Masculine, case);
                    match clitic == full {
                        true => full,
                        false => format!("*{full} / {clitic}"),
                    }
                })
                .as_ref(),
        );
    }
    row(
        "3 sg fem",
        CASES
            .map(|(_, c)| pronoun(Person::Third, Number::Singular, Gender::Feminine, c))
            .as_ref(),
    );
    row(
        "reflexive",
        CASES
            .map(|(_, c)| {
                let full = reflexive(c);
                let cl = clitic_reflexive(c);
                match cl == full {
                    true => full,
                    false => format!("*{full} / {cl}"),
                }
            })
            .as_ref(),
    );

    table(
        "§5 Pronouns",
        "tot and sjej",
        "the pronominal declension. tot's masculine nominative is reduplicated and nothing else is; sjej is wholly regular and keeps its stem whole",
    );
    header(&["", "masc sg", "neut sg", "fem sg", "dual", "plural"]);
    for (name, case) in CASES {
        let mut cells = Vec::new();
        for (n, g) in [
            (Number::Singular, Gender::Masculine),
            (Number::Singular, Gender::Neuter),
            (Number::Singular, Gender::Feminine),
            (Number::Dual, Gender::Masculine),
            (Number::Plural, Gender::Masculine),
        ] {
            let t = that(case, n, g, Animacy::Inanimate);
            let s = this(case, n, g, Animacy::Inanimate);
            cells.push(cell(&format!("{t} / {s}"), &["tot / sjej"]));
        }
        row(name, &cells);
    }

    table(
        "§5 Pronouns",
        "kto, czto, izze",
        "two interrogatives and the restrictive relative. kto's accusative is the ablative, by §3.7",
    );
    header(&["", "kto", "czto", "izze"]);
    for (name, case) in CASES {
        row(
            name,
            &[
                cell(&who(case), &["koga"]),
                what(case),
                relative(case, Number::Singular, Gender::Masculine),
            ],
        );
    }
}

fn numerals() {
    table(
        "§6 Numerals",
        "the cardinals",
        "one rule per rank: unit + nadjesjat, unit + djesjat, unit + sto. Above a thousand the rank word is a noun governed by its count",
    );
    header(&["value", "nominative", "genitive", "instrumental"]);
    for v in [
        0u64,
        1,
        2,
        3,
        4,
        5,
        10,
        11,
        15,
        19,
        20,
        40,
        90,
        100,
        132,
        200,
        500,
        999,
        1_000,
        2_000,
        5_000,
        21_000,
        1_000_000,
        1_000_000_000,
    ] {
        row(
            &v.to_string(),
            &[
                cell(
                    &numeral(v, Case::Nominative, Gender::Masculine, Animacy::Inanimate),
                    &["nolj", "dvjesto", "sto tridjesjat dva"],
                ),
                numeral(v, Case::Genitive, Gender::Masculine, Animacy::Inanimate),
                numeral(v, Case::Instrumental, Gender::Masculine, Animacy::Inanimate),
            ],
        );
    }
    table(
        "§6 Numerals",
        "odin and dva agree in gender",
        "and dva takes the plain nominal dual endings, so it declines exactly as dom does in the dual",
    );
    header(&["", "masculine", "feminine", "neuter"]);
    for (name, case) in CASES {
        for v in [1u64, 2] {
            row(
                &format!("{v} — {name}"),
                Gender::ALL
                    .map(|g| {
                        cell(
                            &numeral(v, case, g, Animacy::Inanimate),
                            &["dvoma", "odina"],
                        )
                    })
                    .as_ref(),
            );
        }
    }
    table(
        "§6 Numerals",
        "the ordinals",
        "a derivation like the participles. One to four are suppletive; from five up the stem is the cardinal less its final j",
    );
    header(&["value", "stem", "long", "short"]);
    for v in [1u64, 2, 3, 4, 5, 7, 10, 11, 20, 100, 1_000] {
        let s = ordinal(v);
        row(
            &v.to_string(),
            &[
                s.clone(),
                cell(
                    &adjective(
                        &s,
                        Case::Nominative,
                        Number::Singular,
                        Gender::Masculine,
                        Animacy::Inanimate,
                    ),
                    &["trjetyj"],
                ),
                short_adjective(
                    &s,
                    Case::Nominative,
                    Number::Singular,
                    Gender::Masculine,
                    Animacy::Inanimate,
                ),
            ],
        );
    }
}

/// §7.10 builds no synthetic imperative for the third person or the first
/// singular — a particle plus the present indicative does that work — so those
/// cells are left empty here rather than shown filled with a form that is really
/// the present.
fn imperative_row(lemma: &str) -> Vec<String> {
    let mut out = Vec::new();
    for number in Number::ALL {
        for person in Person::ALL {
            let synthetic = !matches!(
                (person, number),
                (Person::Third, _) | (Person::First, Number::Singular)
            );
            out.push(match synthetic {
                true => cell(&imperative(lemma, person, number), &["budj", "piszi"]),
                false => "?".to_string(),
            });
        }
    }
    out
}

fn verbs() {
    let sample: &[(&str, &str, &[&str])] = &[
        (
            "czitatj",
            "class 1 — the theme vowel stays and -j- is added",
            &[],
        ),
        (
            "mytj",
            "class 1, monosyllabic — the vowel is the root, so there is no theme to drop",
            &[],
        ),
        ("njegodovatj", "class 2 — ova becomes uj", &[]),
        ("dvinutj", "class 3 — the theme drops", &[]),
        (
            "govoritj",
            "class 4 — nothing in the stem mutates, so the first singular keeps its -ju",
            &["govorju", "govorjah"],
        ),
        (
            "ljubitj",
            "class 4 — b becomes blj, and the mutation supplies the palatal element, so the ending is the bare -u",
            &["ljublju"],
        ),
        (
            "vidjetj",
            "class 5 — d becomes dzz, keeping the root visible",
            &["vidzzu", "vidjah"],
        ),
        (
            "ljetjetj",
            "class 5 — t becomes tcz, which is what parts it from ljeczitj below",
            &["ljetczu"],
        ),
        (
            "ljeczitj",
            "class 4 — cz is already palatal, so nothing is added",
            &["ljeczu"],
        ),
        (
            "pisatj'",
            "class 6, which the word-final mark selects. The stem mutates throughout and rule 2 drops the ending's glide",
            &["piszu", "piszeszj", "piszah"],
        ),
    ];
    for (lemma, note, diag) in sample {
        table("§7 Verbs", lemma, note);
        header(&[
            "", "1 sg", "2 sg", "3 sg", "1 du", "2 du", "3 du", "1 pl", "2 pl", "3 pl",
        ]);
        let mut cells = Vec::new();
        for number in Number::ALL {
            for person in Person::ALL {
                cells.push(cell(&verb(lemma, person, number), diag));
            }
        }
        row("non-past", &cells);
        row("imperative", &imperative_row(lemma));
    }

    table(
        "§7 Verbs",
        "byti, the one suppletive verb",
        "three roots and three functions. Its past is the only synthetic past left in the language; the l-participle is §7.7's rule on by- and comes out of the general path",
    );
    header(&[
        "", "1 sg", "2 sg", "3 sg", "1 du", "2 du", "3 du", "1 pl", "2 pl", "3 pl",
    ]);
    let mut cells = Vec::new();
    for number in Number::ALL {
        for person in Person::ALL {
            cells.push(cell(&byti(person, number), &["jesmj", "sutj"]));
        }
    }
    row("present", &cells);
    let mut past = Vec::new();
    for number in Number::ALL {
        for person in Person::ALL {
            past.push(cell(&byti_past(person, number), &["bjah"]));
        }
    }
    row("past", &past);
    let mut fut = Vec::new();
    for number in Number::ALL {
        for person in Person::ALL {
            fut.push(future_auxiliary(person, number));
        }
    }
    row("future aux", &fut);
    row("imperative", &imperative_row("bytj"));

    table(
        "§7 Verbs",
        "participles and gerunds",
        "each participle returns an adjective stem that declines long or short; the gerunds are indeclinable and are finished forms",
    );
    header(&[
        "verb",
        "pres. active",
        "past active",
        "pres. passive",
        "past passive",
        "pres. gerund",
        "past gerund",
        "l-participle",
    ]);
    for lemma in [
        "czitatj",
        "poczitatj",
        "govoritj",
        "vidjetj",
        "rjeszitj",
        "bitj",
        "dvinutj",
        "pisatj'",
    ] {
        row(
            lemma,
            &[
                present_active_participle(lemma),
                past_active_participle(lemma),
                present_passive_participle(lemma),
                cell(
                    &past_passive_participle(lemma),
                    &["rjeszen", "bit", "dvinut", "poczitan"],
                ),
                present_gerund(lemma),
                past_gerund(lemma),
                l_participle(lemma, Gender::Masculine, Number::Singular),
            ],
        );
    }
}

fn main() {
    nouns();
    adjectives();
    pronouns();
    numerals();
    verbs();
}
