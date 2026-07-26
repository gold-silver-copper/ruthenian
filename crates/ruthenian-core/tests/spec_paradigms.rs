//! Conformance against the specification's noun paradigms.
//!
//! Two tests with deliberately different jobs:
//!
//! * [`noun_paradigms_conform`] asserts the engine against the **committed
//!   corpus** in `paradigms/spec_nouns.tsv`. That file is data: readable,
//!   diffable, and stable under any reformatting of the specification's prose.
//! * [`spec_corpus_is_current`] re-extracts the corpus from `docs/RUTHENIAN.md`
//!   and fails if it has drifted, so amending the spec shows up as a corpus diff
//!   in review rather than as a conformance test that quietly checks less.
//!
//! Splitting them is the point. An earlier version parsed the markdown inside
//! the conformance assertion, and a heading match silently found the wrong
//! table — checking `dom`'s forms against `noczj` while reporting a clean parse.
//! Extraction is now a separate step whose output a human can read.

mod support;

use ruthenian_core::{Animacy, Case, Declension, Gender, NounClass, Number, noun};
use support::Row;

const SPEC: &str = include_str!("../../../docs/RUTHENIAN.md");
const CORPUS_PATH: &str = "tests/paradigms/spec_nouns.tsv";
const CORPUS: &str = include_str!("paradigms/spec_nouns.tsv");

/// How to build each paradigm the corpus names.
fn engine(paradigm: &str) -> (&'static str, NounClass, Gender, Animacy) {
    match paradigm {
        "dom" => (
            "dom",
            NounClass::hard(Declension::II),
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        "konj" => (
            "konj",
            NounClass::soft(Declension::II),
            Gender::Masculine,
            Animacy::Animate,
        ),
        "drug" => (
            "drug",
            NounClass::hard(Declension::II),
            Gender::Masculine,
            Animacy::Animate,
        ),
        "okno" => (
            "okn",
            NounClass::hard(Declension::II),
            Gender::Neuter,
            Animacy::Inanimate,
        ),
        "zzena" => (
            "zzen",
            NounClass::hard(Declension::I),
            Gender::Feminine,
            Animacy::Animate,
        ),
        "noczj" => (
            "noczj",
            NounClass::hard(Declension::III),
            Gender::Feminine,
            Animacy::Inanimate,
        ),
        other => panic!("corpus names an unknown paradigm: {other}"),
    }
}

fn case_of(label: &str) -> Case {
    match label {
        "nominative" => Case::Nom,
        "vocative" => Case::Voc,
        "accusative" => Case::Acc,
        "genitive" => Case::Gen,
        "ablative" => Case::Abl,
        "dative" => Case::Dat,
        "instrumental" => Case::Ins,
        "locative" => Case::Loc,
        other => panic!("unknown case {other}"),
    }
}

fn number_of(label: &str) -> Number {
    match label {
        "singular" => Number::Singular,
        "dual" => Number::Dual,
        "plural" => Number::Plural,
        other => panic!("unknown number {other}"),
    }
}

/// The engine reproduces every cell the specification states.
#[test]
fn noun_paradigms_conform() {
    let rows: Vec<Row> = support::from_tsv(CORPUS);
    assert_eq!(
        rows.len(),
        support::HEADINGS.len() * support::CASES.len() * support::NUMBERS.len(),
        "the corpus is not the expected size; regenerate it"
    );

    let mut failures = Vec::new();
    for row in &rows {
        let (stem, class, gender, animacy) = engine(&row.paradigm);
        let got = noun(
            stem,
            class,
            gender,
            animacy,
            case_of(&row.case),
            number_of(&row.number),
        )
        .map(|p| p.text);

        match got {
            Some(text) if row.forms.contains(&text) => {}
            Some(text) => failures.push(format!(
                "{:<6} {:<13} {:<9} spec {:?}, engine {:?}",
                row.paradigm, row.case, row.number, row.forms, text
            )),
            None => failures.push(format!(
                "{:<6} {:<13} {:<9} spec {:?}, engine says the cell does not exist",
                row.paradigm, row.case, row.number, row.forms
            )),
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} cells disagree with the specification:\n{}",
        failures.len(),
        rows.len(),
        failures.join("\n")
    );
    println!("{} cells conform to docs/RUTHENIAN.md", rows.len());
}

/// The committed corpus still matches the specification.
///
/// Set `RUTHENIAN_REGEN_CORPUS=1` to rewrite it after amending the spec.
#[test]
fn spec_corpus_is_current() {
    let extracted = support::extract(SPEC);
    let rendered = support::to_tsv(&extracted);

    if std::env::var_os("RUTHENIAN_REGEN_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &rendered).expect("write corpus");
        println!("regenerated {CORPUS_PATH} ({} rows)", extracted.len());
        return;
    }

    if rendered != CORPUS {
        let committed = support::from_tsv(CORPUS);
        let mut diff = Vec::new();
        for row in &extracted {
            let found = committed.iter().find(|c| {
                c.paradigm == row.paradigm && c.case == row.case && c.number == row.number
            });
            match found {
                Some(c) if c.forms == row.forms => {}
                Some(c) => diff.push(format!(
                    "  {} {}/{}: corpus {:?}, spec now {:?}",
                    row.paradigm, row.case, row.number, c.forms, row.forms
                )),
                None => diff.push(format!(
                    "  {} {}/{}: missing from the corpus",
                    row.paradigm, row.case, row.number
                )),
            }
        }
        for c in &committed {
            if !extracted
                .iter()
                .any(|r| r.paradigm == c.paradigm && r.case == c.case && r.number == c.number)
            {
                diff.push(format!(
                    "  {} {}/{}: in the corpus but no longer in the spec",
                    c.paradigm, c.case, c.number
                ));
            }
        }
        panic!(
            "{CORPUS_PATH} has drifted from docs/RUTHENIAN.md.\n{}\n\n\
             Regenerate and review the diff:\n  \
             RUTHENIAN_REGEN_CORPUS=1 cargo test -p ruthenian-core --test spec_paradigms",
            if diff.is_empty() {
                "  (cells agree; only formatting or ordering changed)".to_string()
            } else {
                diff.join("\n")
            }
        );
    }
}

/// §11 tabulates how many distinct surface forms each paradigm has after
/// syncretism — a second, independent claim about the same tables. A paradigm
/// can reproduce every cell and still have the wrong shape if a syncretism is
/// missing or spurious.
#[test]
fn paradigm_sizes_match_section_11() {
    use ruthenian_core::noun_forms;

    let expected = parse_size_table();
    assert!(
        !expected.is_empty(),
        "§11's size table was not parsed; this guard would pass vacuously"
    );

    let built = [
        ("dom", Declension::II, Gender::Masculine, Animacy::Inanimate),
        ("okno", Declension::II, Gender::Neuter, Animacy::Inanimate),
        ("zzena", Declension::I, Gender::Feminine, Animacy::Animate),
        (
            "noczj",
            Declension::III,
            Gender::Feminine,
            Animacy::Inanimate,
        ),
    ];

    let mut failures = Vec::new();
    for (lemma, declension, gender, animacy) in built {
        let p = noun_forms(lemma, NounClass::hard(declension), gender, animacy);
        let Some(&(sg, du, pl, total)) = expected.get(lemma) else {
            failures.push(format!("§11 has no row for {lemma}"));
            continue;
        };
        let per: Vec<usize> = Number::ALL
            .into_iter()
            .map(|number| {
                let mut seen: Vec<&str> = Case::ALL
                    .into_iter()
                    .filter_map(|case| p.get(case, number).map(|f| f.text.as_str()))
                    .collect();
                seen.sort_unstable();
                seen.dedup();
                seen.len()
            })
            .collect();
        if per != vec![sg, du, pl] || p.distinct_forms() != total {
            failures.push(format!(
                "{lemma}: §11 says {sg}/{du}/{pl} = {total}, engine gives {}/{}/{} = {}",
                per[0],
                per[1],
                per[2],
                p.distinct_forms()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn parse_size_table() -> std::collections::BTreeMap<String, (usize, usize, usize, usize)> {
    let mut out = std::collections::BTreeMap::new();
    let start = SPEC
        .find("# 11. Summary of paradigm sizes")
        .expect("§11 heading");
    for line in SPEC[start..].lines() {
        if line.starts_with("# 12.") {
            break;
        }
        if !line.starts_with("| noun") {
            continue;
        }
        let cols: Vec<&str> = line.trim_matches('|').split('|').collect();
        if cols.len() < 5 {
            continue;
        }
        let Some(lemma) = cols[0].split('`').nth(1) else {
            continue;
        };
        let n = |s: &str| -> Option<usize> { s.trim().trim_matches('*').trim().parse().ok() };
        if let (Some(sg), Some(du), Some(pl), Some(total)) =
            (n(cols[1]), n(cols[2]), n(cols[3]), n(cols[4]))
        {
            out.insert(lemma.to_string(), (sg, du, pl, total));
        }
    }
    out
}
