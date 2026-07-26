//! Extraction of the specification's paradigm tables into a reviewable corpus.
//!
//! This code runs when the corpus is **regenerated**, not when conformance is
//! checked. Conformance asserts against the committed TSV, so a change to
//! `docs/RUTHENIAN.md`'s markdown formatting shows up as a corpus diff in review
//! rather than as a test that quietly checks fewer cells.
//!
//! That distinction is the whole point. An earlier version parsed the markdown
//! inside the conformance test, and its heading match found `noczj` in §3.2's
//! declension summary instead of §3.6's paradigm — silently comparing `dom`'s
//! forms against `noczj`. A committed artifact makes that a visible diff.

use std::fmt::Write as _;

/// One cell of one paradigm, as the specification states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub paradigm: String,
    pub case: String,
    pub number: String,
    /// One or more forms; several where the spec gives alternatives.
    pub forms: Vec<String>,
}

pub const CASES: [&str; 8] = [
    "nominative",
    "vocative",
    "accusative",
    "genitive",
    "ablative",
    "dative",
    "instrumental",
    "locative",
];
pub const NUMBERS: [&str; 3] = ["singular", "dual", "plural"];

/// The paradigm tables to extract, by the heading each sits under.
pub const HEADINGS: [(&str, &str); 6] = [
    ("dom", "Hard: `dom`"),
    ("konj", "Soft: `konj`"),
    ("drug", "Velar: `drug`"),
    ("okno", "Hard: `okno`"),
    ("zzena", "Hard: `zzena`"),
    ("noczj", "`noczj` \"night\""),
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Forms(Vec<String>),
    SameAs(String),
}

fn parse_cell(raw: &str) -> Option<Cell> {
    let text = raw.trim();
    if let Some(rest) = text.strip_prefix('=') {
        let target = rest.trim();
        return match target {
            "nom" => Some(Cell::SameAs("nominative".into())),
            "dat" => Some(Cell::SameAs("dative".into())),
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
    (!forms.is_empty()).then_some(Cell::Forms(forms))
}

fn case_label(raw: &str) -> Option<String> {
    let l = raw.trim().trim_matches('*').trim();
    CASES.contains(&l).then(|| l.to_string())
}

/// Find a heading LINE containing `needle`, and read the table beneath it.
///
/// Matching must be anchored to a heading; matching anywhere in the document
/// finds `noczj` in §3.2's prose table instead.
fn table_under(spec: &str, needle: &str) -> Vec<(String, [Cell; 3])> {
    let mut offset = 0usize;
    let mut start = None;
    for line in spec.lines() {
        if line.starts_with('#') && line.contains(needle) {
            start = Some(offset);
            break;
        }
        offset += line.len() + 1;
    }
    let start = start.unwrap_or_else(|| panic!("no heading line contains {needle:?}"));

    let mut rows = Vec::new();
    let mut seen_header = false;
    for line in spec[start..].lines().skip(1) {
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
        let Some(case) = case_label(cols[0]) else {
            continue;
        };
        if let (Some(s), Some(d), Some(p)) = (
            parse_cell(cols[1]),
            parse_cell(cols[2]),
            parse_cell(cols[3]),
        ) {
            rows.push((case, [s, d, p]));
        }
    }
    rows
}

/// Extract every configured paradigm from the specification text.
pub fn extract(spec: &str) -> Vec<Row> {
    let mut out = Vec::new();
    for (name, heading) in HEADINGS {
        let table = table_under(spec, heading);
        assert_eq!(
            table.len(),
            CASES.len(),
            "{name}: expected {} case rows under {heading:?}, found {}",
            CASES.len(),
            table.len()
        );
        for (case, cells) in &table {
            for (i, number) in NUMBERS.iter().enumerate() {
                // Resolve `= nom` / `= dat` through the spec's own table, so the
                // corpus records the forms rather than the cross-reference.
                let forms = match &cells[i] {
                    Cell::Forms(f) => f.clone(),
                    Cell::SameAs(source) => {
                        let src = table
                            .iter()
                            .find(|(c, _)| c == source)
                            .unwrap_or_else(|| panic!("{name}: {case} refers to missing {source}"));
                        match &src.1[i] {
                            Cell::Forms(f) => f.clone(),
                            Cell::SameAs(_) => {
                                panic!("{name}: {case}/{number} is a chained cross-reference")
                            }
                        }
                    }
                };
                out.push(Row {
                    paradigm: name.to_string(),
                    case: case.clone(),
                    number: (*number).to_string(),
                    forms,
                });
            }
        }
    }
    out
}

pub fn to_tsv(rows: &[Row]) -> String {
    let mut s = String::new();
    s.push_str(
        "# Noun paradigm cells, extracted from docs/RUTHENIAN.md §§3.3-3.6.\n\
         #\n\
         # GENERATED. Do not edit by hand. Regenerate with:\n\
         #   RUTHENIAN_REGEN_CORPUS=1 cargo test -p ruthenian-core --test spec_paradigms\n\
         #\n\
         # This file is the conformance corpus: `noun_paradigms_conform` asserts the\n\
         # engine against THESE rows, not against the markdown. The extraction is a\n\
         # separate, reviewable step, so a change to the specification's formatting\n\
         # appears here as a diff instead of silently shrinking what is checked.\n\
         #\n\
         # paradigm\tcase\tnumber\tforms (comma-separated alternatives)\n",
    );
    for r in rows {
        let _ = writeln!(
            s,
            "{}\t{}\t{}\t{}",
            r.paradigm,
            r.case,
            r.number,
            r.forms.join(",")
        );
    }
    s
}

pub fn from_tsv(text: &str) -> Vec<Row> {
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            assert!(c.len() >= 4, "malformed corpus row: {l:?}");
            Row {
                paradigm: c[0].to_string(),
                case: c[1].to_string(),
                number: c[2].to_string(),
                forms: c[3].split(',').map(str::to_string).collect(),
            }
        })
        .collect()
}
