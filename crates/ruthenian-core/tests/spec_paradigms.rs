//! `spec_paradigms_match` — the guard that makes `docs/RUTHENIAN.md` executable.
//!
//! Every expected form here is **parsed out of the specification at test time**,
//! never transcribed. Hand-copying the tables would create a second copy of the
//! language that drifts from the first, which is law 9 and the failure this
//! project has already watched happen elsewhere.
//!
//! The specification is normative: where the engine and the document disagree,
//! the engine is wrong. A failure names the table and cell it contradicts.

use ruthenian_core::{Animacy, Case, Declension, Gender, NounClass, Number, noun};

const SPEC: &str = include_str!("../../../docs/RUTHENIAN.md");

/// One paradigm table in the specification, and how to build it.
struct Paradigm {
    /// The `###` heading the table sits under, matched as a substring.
    heading: &'static str,
    stem: &'static str,
    class: NounClass,
    gender: Gender,
    animacy: Animacy,
}

const PARADIGMS: &[Paradigm] = &[
    Paradigm {
        heading: "Hard: `dom`",
        stem: "dom",
        class: NounClass::hard(Declension::II),
        gender: Gender::Masculine,
        animacy: Animacy::Inanimate,
    },
    Paradigm {
        heading: "Soft: `konj`",
        stem: "konj",
        class: NounClass::soft(Declension::II),
        gender: Gender::Masculine,
        animacy: Animacy::Animate,
    },
    Paradigm {
        heading: "Velar: `drug`",
        stem: "drug",
        class: NounClass::hard(Declension::II),
        gender: Gender::Masculine,
        animacy: Animacy::Animate,
    },
    Paradigm {
        heading: "Hard: `okno`",
        stem: "okn",
        class: NounClass::hard(Declension::II),
        gender: Gender::Neuter,
        animacy: Animacy::Inanimate,
    },
    Paradigm {
        heading: "Hard: `zzena`",
        stem: "zzen",
        class: NounClass::hard(Declension::I),
        gender: Gender::Feminine,
        animacy: Animacy::Animate,
    },
    Paradigm {
        heading: "`noczj`",
        stem: "noczj",
        class: NounClass::hard(Declension::III),
        gender: Gender::Feminine,
        animacy: Animacy::Inanimate,
    },
];

/// A cell as the specification writes it: one or more backticked forms, or a
/// `= nom` / `= dat` reference, or a footnote marker to be stripped.
#[derive(Debug, PartialEq, Eq)]
enum SpecCell {
    Forms(Vec<String>),
    SameAs(Case),
}

fn parse_cell(raw: &str) -> Option<SpecCell> {
    let text = raw.trim();
    // Footnote markers are superscripts; strip everything after the last
    // backtick-delimited run so `dom` / `doma` ¹ parses as two forms.
    if let Some(rest) = text.strip_prefix('=') {
        return match rest.trim() {
            "nom" => Some(SpecCell::SameAs(Case::Nom)),
            "dat" => Some(SpecCell::SameAs(Case::Dat)),
            _ => None,
        };
    }
    let forms: Vec<String> = text
        .split('`')
        .skip(1)
        .step_by(2)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    (!forms.is_empty()).then_some(SpecCell::Forms(forms))
}

fn case_of(label: &str) -> Option<Case> {
    // The ablative is bolded in every table, so strip emphasis first.
    let l = label.trim().trim_matches('*').trim();
    Some(match l {
        "nominative" => Case::Nom,
        "vocative" => Case::Voc,
        "accusative" => Case::Acc,
        "genitive" => Case::Gen,
        "ablative" => Case::Abl,
        "dative" => Case::Dat,
        "instrumental" => Case::Ins,
        "locative" => Case::Loc,
        _ => return None,
    })
}

/// Extract the `Case | Singular | Dual | Plural` table under a heading.
///
/// The heading must be matched on a **heading line**. Matching anywhere would
/// find `noczj` in §3.2's declension summary rather than §3.6's paradigm, and
/// then silently check the wrong table — which is exactly how a conformance
/// guard goes stale without failing.
fn table_for(heading: &str) -> Vec<(Case, [SpecCell; 3])> {
    let start = SPEC
        .lines()
        .scan(0usize, |offset, line| {
            let at = *offset;
            *offset += line.len() + 1;
            Some((at, line))
        })
        .find(|(_, line)| line.starts_with('#') && line.contains(heading))
        .map(|(at, _)| at)
        .unwrap_or_else(|| panic!("spec heading line not found: {heading}"));
    let mut rows = Vec::new();
    let mut seen_header = false;
    for line in SPEC[start..].lines().skip(1) {
        let line = line.trim();
        if line.starts_with("##") && !rows.is_empty() {
            break;
        }
        if !line.starts_with('|') {
            if seen_header && !rows.is_empty() {
                break;
            }
            continue;
        }
        let cols: Vec<&str> = line.trim_matches('|').split('|').collect();
        if cols.len() < 4 {
            continue;
        }
        if !seen_header {
            seen_header = true;
            continue;
        }
        if cols[0].contains("---") {
            continue;
        }
        let Some(case) = case_of(cols[0]) else {
            continue;
        };
        let cells = [
            parse_cell(cols[1]),
            parse_cell(cols[2]),
            parse_cell(cols[3]),
        ];
        if let [Some(s), Some(d), Some(p)] = cells {
            rows.push((case, [s, d, p]));
        }
    }
    assert!(
        rows.len() >= 6,
        "parsed only {} rows under {heading}; the table format changed",
        rows.len()
    );
    rows
}

#[test]
fn spec_paradigms_match() {
    let mut failures = Vec::new();
    let mut checked = 0usize;

    for p in PARADIGMS {
        let rows = table_for(p.heading);
        for (case, cells) in &rows {
            for (i, number) in [Number::Singular, Number::Dual, Number::Plural]
                .into_iter()
                .enumerate()
            {
                let expected = match &cells[i] {
                    SpecCell::Forms(f) => f.clone(),
                    // A `= nom` / `= dat` cell asserts a syncretism. Resolve it
                    // through the spec's own table, so the check still compares
                    // against the document rather than against our reading of it.
                    SpecCell::SameAs(source) => {
                        let Some((_, src)) = rows.iter().find(|(c, _)| c == source) else {
                            continue;
                        };
                        match &src[i] {
                            SpecCell::Forms(f) => f.clone(),
                            SpecCell::SameAs(_) => continue,
                        }
                    }
                };

                let got =
                    noun(p.stem, p.class, p.gender, p.animacy, *case, number).map(|pred| pred.text);
                checked += 1;

                match got {
                    Some(text) if expected.contains(&text) => {}
                    Some(text) => failures.push(format!(
                        "{:<16} {:?}/{:?}: spec says {:?}, engine gives {:?}",
                        p.heading, case, number, expected, text
                    )),
                    None => failures.push(format!(
                        "{:<16} {:?}/{:?}: spec says {:?}, engine says the cell does not exist",
                        p.heading, case, number, expected
                    )),
                }
            }
        }
    }

    assert!(
        checked >= 100,
        "only {checked} cells checked; the spec parser is not finding the tables"
    );
    assert!(
        failures.is_empty(),
        "{} of {checked} cells disagree with docs/RUTHENIAN.md:\n{}",
        failures.len(),
        failures.join("\n")
    );
    println!("{checked} cells checked against docs/RUTHENIAN.md, all matching");
}

/// The parser itself needs a failure witness: if it silently matched nothing,
/// `spec_paradigms_match` would pass vacuously and the guard would be stale.
#[test]
fn spec_tables_are_actually_parsed() {
    let rows = table_for("Hard: `dom`");
    assert_eq!(rows.len(), 8, "dom has eight cases");
    let (_, cells) = rows.iter().find(|(c, _)| *c == Case::Gen).unwrap();
    assert_eq!(
        cells[0],
        SpecCell::Forms(vec!["domogo".into()]),
        "the genitive singular must be read out of the spec, not assumed"
    );
    let (_, abl) = rows.iter().find(|(c, _)| *c == Case::Abl).unwrap();
    assert_eq!(
        abl[1],
        SpecCell::SameAs(Case::Dat),
        "dual ablative = dative"
    );
}
