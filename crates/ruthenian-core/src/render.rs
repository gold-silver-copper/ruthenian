//! The specification's paradigm tables, rendered from the engine.
//!
//! Every markdown table in `docs/RUTHENIAN.md` that cites inflected forms sits
//! between a `<!-- render:ID -->` / `<!-- /render:ID -->` pair and is generated
//! by [`blocks`], through the same public API every caller uses. The
//! `spec_tables_current` guard fails whenever the file's blocks differ from a
//! fresh rendering, and `cargo run -p ruthenian-core --example render_spec`
//! rewrites them.
//!
//! This is the tabular half of law 1 turned around. For **prose**, the
//! specification decides and the code conforms, checked by the corpus. For
//! **tables**, the engine decides and the specification's copy is output —
//! because a table of forms is precisely the thing the engine exists to
//! produce, and two hand-maintained copies of it drifted three separate times
//! (`nacijoj`, the `-i`/`-ji` split, the review artifact's cell count) while
//! prose never held a table still.
//!
//! Conventions, applied uniformly (the hand tables varied):
//!
//! - a vocative identical to its nominative prints `= nom`; the dual and
//!   plural ablative print `= dat` (§3.1 makes both structural);
//! - an accusative is printed as its inanimate form, with ` / anim ¹` where an
//!   animate lemma would differ; a lemma glossed as animate (`konj`, `drug`)
//!   prints the animate form alone;
//! - everything else prints the form the engine produces, in backticks.

use crate::grammar::{Animacy, Case, Gender, Number, Person};
use crate::{
    Adjective, bytj, future_auxiliary, imperative, l_participle, noun, pronoun, verb, verb_paradigm,
};

/// Every generated block, as `(id, markdown)`.
///
/// ```
/// use ruthenian_core::render::blocks;
/// let all = blocks();
/// let dom = &all.iter().find(|(id, _)| *id == "noun-dom").unwrap().1;
/// assert!(dom.contains("`domogo`"));
/// // Rendered through the same API as any caller: the table cannot say
/// // anything the engine does not.
/// assert!(blocks().iter().all(|(_, md)| md.starts_with("|")));
/// ```
pub fn blocks() -> Vec<(&'static str, String)> {
    vec![
        ("noun-dom", noun_table("dom", AccStyle::Split)),
        ("noun-konj", noun_table("Konj", AccStyle::Animate)),
        ("noun-drug", noun_table("Drug", AccStyle::Animate)),
        ("noun-okno", noun_table("okno", AccStyle::Split)),
        ("noun-zzena", noun_table("zzena", AccStyle::Split)),
        ("noun-noczj", noun_table("noczj'", AccStyle::Split)),
        ("decl-i-parallel", parallel_table()),
        ("adj-short", adjective_table(false)),
        ("adj-long", adjective_table(true)),
        ("pron-personal", personal_table()),
        ("pron-clitic", clitic_table()),
        ("pron-third", third_person_table()),
        ("pron-tot", tot_table()),
        ("pron-kto-czto", kto_czto_table()),
        ("verb-nonpast", nonpast_table()),
        ("verb-l-participle", l_participle_table()),
        ("verb-bytj", bytj_table()),
        ("verb-imperative", imperative_table()),
        ("num-cardinals", cardinals_table()),
        ("num-tens", tens_table()),
        ("num-scales", scales_table()),
        ("num-dva", dva_table()),
        ("num-tri-czetyrje", tri_czetyrje_table()),
    ]
}

/// The nominative citation form of a number.
fn cite(value: u64) -> String {
    crate::numeral(
        value,
        Case::Nominative,
        Gender::Masculine,
        Animacy::Inanimate,
    )
}

/// Lay labelled forms out as §6's four-column grids.
fn grid(cells: Vec<String>) -> String {
    let mut rows = vec!["| | | | |".to_string(), "|---|---|---|---|".to_string()];
    for chunk in cells.chunks(4) {
        let mut padded: Vec<&str> = chunk.iter().map(String::as_str).collect();
        padded.resize(4, "");
        rows.push(format!("| {} |", padded.join(" | ")));
    }
    rows.join("\n")
}

/// §6.2 — the cardinals 0–10.
fn cardinals_table() -> String {
    grid((0..=10).map(|n| format!("{n} `{}`", cite(n))).collect())
}

/// §6.3 — the tens, "N tens" on the unit whole.
fn tens_table() -> String {
    grid(
        (2..=9)
            .map(|n| format!("{} `{}`", n * 10, cite(n * 10)))
            .collect(),
    )
}

/// §6.3 — the scale nouns, short scale: each step a thousand times the last.
fn scales_table() -> String {
    grid(
        [
            ("10³", 1_000, " (fem. I)"),
            ("10⁶", 1_000_000, ""),
            ("10⁹", 1_000_000_000, ""),
            ("10¹²", 1_000_000_000_000, ""),
            ("10¹⁵", 1_000_000_000_000_000, ""),
            ("10¹⁸", 1_000_000_000_000_000_000, ""),
        ]
        .map(|(rank, value, note)| format!("{rank} `{}`{note}", cite(value)))
        .to_vec(),
    )
}

/// §6.4 — `dva`, the one word with only dual endings.
fn dva_table() -> String {
    let d = |case, gender| code(&crate::numeral(2, case, gender, Animacy::Inanimate));
    [
        "| | Masc/neut | Fem |".to_string(),
        "|---|---|---|".to_string(),
        format!(
            "| nom / acc | {} | {} |",
            d(Case::Nominative, Gender::Masculine),
            d(Case::Nominative, Gender::Feminine)
        ),
        format!(
            "| gen / loc | {} | {} |",
            d(Case::Genitive, Gender::Masculine),
            d(Case::Genitive, Gender::Feminine)
        ),
        format!(
            "| dat / ins / abl | {} | {} |",
            d(Case::Dative, Gender::Masculine),
            d(Case::Dative, Gender::Feminine)
        ),
    ]
    .join("\n")
}

/// §6.4 — `tri` and `czetyrje`, declining as plurals.
fn tri_czetyrje_table() -> String {
    let f = |n, case| {
        code(&crate::numeral(
            n,
            case,
            Gender::Masculine,
            Animacy::Inanimate,
        ))
    };
    [
        "| | `tri` | `czetyrje` |".to_string(),
        "|---|---|---|".to_string(),
        format!(
            "| nominative | {} | {} |",
            f(3, Case::Nominative),
            f(4, Case::Nominative)
        ),
        format!(
            "| genitive / locative | {} | {} |",
            f(3, Case::Genitive),
            f(4, Case::Genitive)
        ),
        format!(
            "| dative | {} | {} |",
            f(3, Case::Dative),
            f(4, Case::Dative)
        ),
        format!(
            "| instrumental | {} | {} |",
            f(3, Case::Instrumental),
            f(4, Case::Instrumental)
        ),
    ]
    .join("\n")
}

/// Replace every marked block in `spec` with its fresh rendering.
///
/// A marker in the text that no renderer owns is an error naming the id — a
/// typo there would silently leave a table hand-maintained. Markers *absent*
/// from the text are not `apply`'s concern (it renders fragments too); the
/// `spec_tables_current` guard is what insists the spec carries every id.
///
/// ```
/// use ruthenian_core::render::apply;
/// let page = "before\n<!-- render:noun-dom -->\nstale\n<!-- /render:noun-dom -->\nafter";
/// let fresh = apply(page).unwrap();
/// assert!(fresh.contains("`domogo`"));
/// assert!(!fresh.contains("stale"));
/// // An unowned marker is an error, not a shrug.
/// assert!(apply("<!-- render:no-such-table -->\n<!-- /render:no-such-table -->").is_err());
/// ```
pub fn apply(spec: &str) -> Result<String, String> {
    let mut out = spec.to_string();
    let mut errors = Vec::new();
    let mut owned = Vec::new();
    for (id, body) in blocks() {
        let open = format!("<!-- render:{id} -->");
        let close = format!("<!-- /render:{id} -->");
        owned.push(id);
        let Some(a) = out.find(&open) else {
            continue;
        };
        let Some(b) = out[a..].find(&close) else {
            errors.push(format!("`{id}` is opened but never closed"));
            continue;
        };
        out.replace_range(a + open.len()..a + b, &format!("\n{body}\n"));
    }
    // A marker the registry does not own is a table nothing regenerates.
    let mut rest = out.as_str();
    while let Some(i) = rest.find("<!-- render:") {
        let tail = &rest[i + "<!-- render:".len()..];
        let id = tail.split(" -->").next().unwrap_or("");
        if !owned.contains(&id) {
            errors.push(format!("marker `{id}` has no renderer"));
        }
        rest = tail;
    }
    if errors.is_empty() {
        Ok(out)
    } else {
        Err(errors.join("; "))
    }
}

/// How a table prints the accusative.
enum AccStyle {
    /// The inanimate form, `/ animate ¹` where the two differ.
    Split,
    /// The animate form alone — for a lemma the spec glosses as animate.
    Animate,
}

fn code(form: &str) -> String {
    format!("`{form}`")
}

/// Flip a lemma's animacy mark (§2.1: capital = animate).
fn toggle_animacy(lemma: &str, animate: bool) -> String {
    let mut c = lemma.chars();
    match c.next() {
        Some(first) if animate => first.to_uppercase().chain(c).collect(),
        Some(first) => first.to_lowercase().chain(c).collect(),
        None => String::new(),
    }
}

fn noun_table(lemma: &str, style: AccStyle) -> String {
    let inan = toggle_animacy(lemma, false);
    let anim = toggle_animacy(lemma, true);
    let mut rows = vec![
        "| Case | Singular | Dual | Plural |".to_string(),
        "|---|---|---|---|".to_string(),
    ];
    let mut footnote = false;
    for case in Case::ALL {
        let mut cells = Vec::new();
        for number in Number::ALL {
            let nom = noun(lemma, Case::Nominative, number);
            let form = noun(lemma, case, number);
            cells.push(match case {
                Case::Vocative if form == nom => "= nom".to_string(),
                Case::Ablative if number != Number::Singular => "= dat".to_string(),
                Case::Accusative => {
                    let i = noun(&inan, Case::Accusative, number);
                    let a = noun(&anim, Case::Accusative, number);
                    match style {
                        _ if i == a && i == nom => "= nom".to_string(),
                        AccStyle::Animate if a == nom => "= nom".to_string(),
                        AccStyle::Animate => code(&a),
                        AccStyle::Split if i == a => code(&i),
                        AccStyle::Split => {
                            footnote = true;
                            format!("{} / {} ¹", code(&i), code(&a))
                        }
                    }
                }
                _ => code(&form),
            });
        }
        rows.push(format!(
            "| {} | {} | {} | {} |",
            label(case),
            cells[0],
            cells[1],
            cells[2]
        ));
    }
    if footnote {
        rows.push(String::new());
        rows.push("¹ animate nouns take this form in the accusative (§3.7).".to_string());
    }
    rows.join("\n")
}

fn label(case: Case) -> &'static str {
    match case {
        Case::Nominative => "nominative",
        Case::Vocative => "vocative",
        Case::Accusative => "accusative",
        Case::Genitive => "genitive",
        Case::Ablative => "**ablative**",
        Case::Dative => "dative",
        Case::Instrumental => "instrumental",
        Case::Locative => "locative",
    }
}

/// §3.5's four-cell comparison: the soft series against the hard, cell for cell.
fn parallel_table() -> String {
    let mut rows = vec![
        "| | genitive sg | dative/locative sg | dual | nominative pl |".to_string(),
        "|---|---|---|---|---|".to_string(),
    ];
    for (head, lemma) in [
        ("hard `zzena`", "zzena"),
        ("soft `zjemlja`", "zjemlja"),
        ("soft `nacija`", "nacija"),
    ] {
        rows.push(format!(
            "| {head} | {} | {} | {} | {} |",
            code(&noun(lemma, Case::Genitive, Number::Singular)),
            code(&noun(lemma, Case::Dative, Number::Singular)),
            code(&noun(lemma, Case::Nominative, Number::Dual)),
            code(&noun(lemma, Case::Nominative, Number::Plural)),
        ));
    }
    rows.join("\n")
}

/// §4.1 / §4.2 — `dobr` in five agreement columns. The long form has no
/// vocative row: the nominative is used (§4.2).
fn adjective_table(long: bool) -> String {
    let a = Adjective::new("dobr");
    let form = |case, number, gender, animacy| match long {
        true => a.long(case, number, gender, animacy),
        false => a.short(case, number, gender, animacy),
    };
    let mut rows = vec![
        "| Case | Masc sg | Neut sg | Fem sg | Dual | Plural |".to_string(),
        "|---|---|---|---|---|---|".to_string(),
    ];
    let mut footnote = false;
    let columns = [
        (Number::Singular, Gender::Masculine),
        (Number::Singular, Gender::Neuter),
        (Number::Singular, Gender::Feminine),
        (Number::Dual, Gender::Masculine),
        (Number::Plural, Gender::Masculine),
    ];
    for case in Case::ALL {
        if long && case == Case::Vocative {
            continue;
        }
        let mut cells = Vec::new();
        for (number, gender) in columns {
            let nom = form(Case::Nominative, number, gender, Animacy::Inanimate);
            let f = form(case, number, gender, Animacy::Inanimate);
            cells.push(match case {
                Case::Vocative if f == nom => "= nom".to_string(),
                Case::Ablative if number != Number::Singular => "= dat".to_string(),
                Case::Accusative => {
                    let an = form(case, number, gender, Animacy::Animate);
                    if an == f {
                        if f == nom {
                            "= nom".to_string()
                        } else {
                            code(&f)
                        }
                    } else {
                        footnote = true;
                        format!("{} / {} ¹", code(&f), code(&an))
                    }
                }
                _ => code(&f),
            });
        }
        rows.push(format!("| {} | {} |", label(case), cells.join(" | ")));
    }
    if footnote {
        rows.push(String::new());
        rows.push("¹ animate (§3.7).".to_string());
    }
    rows.join("\n")
}

/// §5.1 — first and second persons; six columns, no third (it has its own
/// table and gender).
fn personal_table() -> String {
    let columns = [
        ("1sg", Person::First, Number::Singular),
        ("2sg", Person::Second, Number::Singular),
        ("**1du**", Person::First, Number::Dual),
        ("**2du**", Person::Second, Number::Dual),
        ("1pl", Person::First, Number::Plural),
        ("2pl", Person::Second, Number::Plural),
    ];
    let mut rows = vec![
        format!("| | {} |", columns.map(|(h, _, _)| h).join(" | ")),
        "|---|---|---|---|---|---|---|".to_string(),
    ];
    for case in Case::ALL {
        if case == Case::Vocative {
            continue; // §5.1 gives the personal pronouns no vocative row.
        }
        let cells: Vec<String> = columns
            .iter()
            .map(|&(_, person, number)| {
                let f = pronoun(person, number, Gender::Masculine, case);
                let dat = pronoun(person, number, Gender::Masculine, Case::Dative);
                match case {
                    Case::Ablative if f == dat => "= dat".to_string(),
                    _ => code(&f),
                }
            })
            .collect();
        rows.push(format!("| {} | {} |", label(case), cells.join(" | ")));
    }
    rows.join("\n")
}

/// §5.1a — the clitics beside their full forms.
fn clitic_table() -> String {
    use crate::{clitic_pronoun, clitic_reflexive, reflexive};
    let mut rows = vec![
        "| | full acc | **clitic acc** | full dat | **clitic dat** |".to_string(),
        "|---|---|---|---|---|".to_string(),
    ];
    let mut push = |head: &str, full: [String; 2], clitic: [String; 2]| {
        let bold = |s: &str| {
            if head == "reflexive" && s == "sja" {
                format!("**`{s}`**")
            } else {
                code(s)
            }
        };
        rows.push(format!(
            "| {head} | {} | {} | {} | {} |",
            code(&full[0]),
            bold(&clitic[0]),
            code(&full[1]),
            bold(&clitic[1]),
        ));
    };
    let p = |person, number, case| pronoun(person, number, Gender::Masculine, case);
    let c = |person, number, case| clitic_pronoun(person, number, Gender::Masculine, case);
    for (head, person, number) in [
        ("1sg", Person::First, Number::Singular),
        ("2sg", Person::Second, Number::Singular),
    ] {
        push(
            head,
            [
                p(person, number, Case::Accusative),
                p(person, number, Case::Dative),
            ],
            [
                c(person, number, Case::Accusative),
                c(person, number, Case::Dative),
            ],
        );
    }
    push(
        "reflexive",
        [reflexive(Case::Accusative), reflexive(Case::Dative)],
        [
            clitic_reflexive(Case::Accusative),
            clitic_reflexive(Case::Dative),
        ],
    );
    let third = |number, gender, case| pronoun(Person::Third, number, gender, case);
    let third_c = |number, gender, case| clitic_pronoun(Person::Third, number, gender, case);
    push(
        "3sg masc/neut",
        [
            third(Number::Singular, Gender::Masculine, Case::Accusative),
            third(Number::Singular, Gender::Masculine, Case::Dative),
        ],
        [
            third_c(Number::Singular, Gender::Masculine, Case::Accusative),
            third_c(Number::Singular, Gender::Masculine, Case::Dative),
        ],
    );
    push(
        "3sg fem",
        [
            third(Number::Singular, Gender::Feminine, Case::Accusative),
            third(Number::Singular, Gender::Feminine, Case::Dative),
        ],
        [
            third_c(Number::Singular, Gender::Feminine, Case::Accusative),
            third_c(Number::Singular, Gender::Feminine, Case::Dative),
        ],
    );
    for (head, person, number) in [
        ("1pl", Person::First, Number::Plural),
        ("2pl", Person::Second, Number::Plural),
    ] {
        push(
            head,
            [
                p(person, number, Case::Accusative),
                p(person, number, Case::Dative),
            ],
            [
                c(person, number, Case::Accusative),
                c(person, number, Case::Dative),
            ],
        );
    }
    push(
        "3pl",
        [
            third(Number::Plural, Gender::Masculine, Case::Accusative),
            third(Number::Plural, Gender::Masculine, Case::Dative),
        ],
        [
            third_c(Number::Plural, Gender::Masculine, Case::Accusative),
            third_c(Number::Plural, Gender::Masculine, Case::Dative),
        ],
    );
    rows.join("\n")
}

/// The five-column pronoun shape shared by the third person, `toj` and the
/// adjectives: masc/neut/fem singular, dual, plural.
fn five_columns(
    form: &dyn Fn(Case, Number, Gender) -> String,
    with_vocative: bool,
    acc_animate: &dyn Fn(Number, Gender) -> Option<String>,
) -> String {
    let mut rows = vec![
        "| | Masc sg | Neut sg | Fem sg | Dual | Plural |".to_string(),
        "|---|---|---|---|---|---|".to_string(),
    ];
    let columns = [
        (Number::Singular, Gender::Masculine),
        (Number::Singular, Gender::Neuter),
        (Number::Singular, Gender::Feminine),
        (Number::Dual, Gender::Masculine),
        (Number::Plural, Gender::Masculine),
    ];
    let mut footnote = false;
    for case in Case::ALL {
        if case == Case::Vocative && !with_vocative {
            continue;
        }
        let cells: Vec<String> = columns
            .iter()
            .map(|&(number, gender)| {
                let f = form(case, number, gender);
                match case {
                    Case::Ablative if number != Number::Singular => "= dat".to_string(),
                    Case::Accusative => match acc_animate(number, gender) {
                        Some(a) if a != f => {
                            footnote = true;
                            format!("{} / {} ¹", code(&f), code(&a))
                        }
                        _ => code(&f),
                    },
                    _ => code(&f),
                }
            })
            .collect();
        rows.push(format!("| {} | {} |", label(case), cells.join(" | ")));
    }
    if footnote {
        rows.push(String::new());
        rows.push("¹ animate (§3.7).".to_string());
    }
    rows.join("\n")
}

fn third_person_table() -> String {
    five_columns(
        &|case, number, gender| pronoun(Person::Third, number, gender, case),
        false,
        &|_, _| None,
    )
}

fn tot_table() -> String {
    use crate::that;
    five_columns(
        &|case, number, gender| that(case, number, gender, Animacy::Inanimate),
        false,
        &|number, gender| Some(that(Case::Accusative, number, gender, Animacy::Animate)),
    )
}

/// §5.5 — `kto` and `czto`, singular-only.
fn kto_czto_table() -> String {
    use crate::{what, who};
    let mut rows = vec![
        "| | \"who\" | \"what\" |".to_string(),
        "|---|---|---|".to_string(),
    ];
    for case in Case::ALL {
        if case == Case::Vocative {
            continue;
        }
        rows.push(format!(
            "| {} | {} | {} |",
            label(case),
            code(&who(case)),
            code(&what(case))
        ));
    }
    rows.join("\n")
}

/// §7.4 — the non-past endings, both conjugations, read off real paradigms so
/// the table cannot disagree with [`verb`].
///
/// The endings are recovered by subtracting the shared stem from each form of
/// a mutation-free exemplar (`czitatj`, `govoritj`), then restoring the seam
/// spelling the subtraction loses: `czitajeszj` minus `czitaj-` is `-eszj` on
/// the surface, but the ending is `-jeszj` with rule 3b writing `jj` once.
fn nonpast_table() -> String {
    let mut rows = vec![
        "| | Singular | **Dual** | Plural |".to_string(),
        "|---|---|---|---|".to_string(),
    ];
    for (head, lemma, stem, glide) in [
        ("**1st conjugation**", "czitatj", "czitaj", true),
        ("**2nd conjugation**", "govoritj", "govor", false),
    ] {
        rows.push(format!("| {head} | | | |"));
        for person in Person::ALL {
            let cells: Vec<String> = Number::ALL
                .map(|number| {
                    let form = verb(lemma, person, number);
                    let mut e = form.strip_prefix(stem).unwrap_or(&form).to_string();
                    // Undo rule 3b for display: after the j-final stem the
                    // written form has one `j`, but the ending owns one too.
                    if glide && !e.starts_with('j') && !e.starts_with('u') {
                        e.insert(0, 'j');
                    }
                    format!("`-{e}`")
                })
                .to_vec();
            rows.push(format!(
                "| {} | {} | {} | {} |",
                person as u8 + 1,
                cells[0],
                cells[1],
                cells[2]
            ));
        }
    }
    let all: Vec<String> = verb_paradigm("czitatj")
        .into_iter()
        .map(|(_, _, f)| f)
        .collect();
    rows.push(String::new());
    rows.push(format!(
        "`czitatj`: `{}` · `{}` · `{}`.",
        all[0..3].join(", "),
        all[3..6].join(", "),
        all[6..9].join(", ")
    ));
    rows.join("\n")
}

fn l_participle_table() -> String {
    let f = |gender, number| code(&l_participle("czitatj", gender, number));
    [
        "| | Masculine | Feminine | Neuter | Dual | Plural |".to_string(),
        "|---|---|---|---|---|---|".to_string(),
        format!(
            "| | {} | {} | {} | {} | {} |",
            f(Gender::Masculine, Number::Singular),
            f(Gender::Feminine, Number::Singular),
            f(Gender::Neuter, Number::Singular),
            f(Gender::Masculine, Number::Dual),
            f(Gender::Masculine, Number::Plural),
        ),
    ]
    .join("\n")
}

/// §7.9 — the two suppletive rows of `bytj`.
fn bytj_table() -> String {
    let row = |name: &str, f: &dyn Fn(Person, Number) -> String| {
        let by_number: Vec<String> = Number::ALL
            .map(|n| Person::ALL.map(|p| code(&f(p, n))).join(", "))
            .to_vec();
        format!("| **{name}** | {} |", by_number.join(" | "))
    };
    [
        "| | Singular | Dual | Plural |".to_string(),
        "|---|---|---|---|".to_string(),
        row("present", &bytj),
        row("future", &future_auxiliary),
    ]
    .join("\n")
}

/// §7.10 — the five synthetic cells; the third person and 1sg are a particle
/// plus the present indicative, so they are not rows here.
fn imperative_table() -> String {
    let f = |p, n| code(&imperative("czitatj", p, n));
    [
        "| | Singular | Dual | Plural |".to_string(),
        "|---|---|---|---|".to_string(),
        format!(
            "| 2 | {} | {} | {} |",
            f(Person::Second, Number::Singular),
            f(Person::Second, Number::Dual),
            f(Person::Second, Number::Plural),
        ),
        format!(
            "| 1 (hortative) | — | {} | {} |",
            f(Person::First, Number::Dual),
            f(Person::First, Number::Plural),
        ),
    ]
    .join("\n")
}
