//! Conservative, offline metrics for user-supplied upstream corpus checkouts.

use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sound_law::{Class, Rule, RuleSet};

const MIXTECASO_REVISION: &str = "b38621d4ac309e23fde616b5424347586a8dbf2c";
const DIACHRONICA_REVISION: &str = "0d8a9790d102ea7ed753f37eeaabaf5bc034397b";

fn main() {
    if let Err(error) = run() {
        eprintln!("corpus_inventory: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args_os().skip(1);
    let source = arguments
        .next()
        .ok_or("usage: corpus_inventory <mixtecaso|diachronica> <checkout>")?;
    let checkout = arguments
        .next()
        .ok_or("usage: corpus_inventory <mixtecaso|diachronica> <checkout>")?;
    if arguments.next().is_some() {
        return Err("usage: corpus_inventory <mixtecaso|diachronica> <checkout>".into());
    }
    let checkout = PathBuf::from(checkout);

    match source.to_str() {
        Some("mixtecaso") => inventory_mixtecaso(&checkout),
        Some("diachronica") => inventory_diachronica(&checkout),
        _ => Err("source must be `mixtecaso` or `diachronica`".into()),
    }
}

fn verify_revision(checkout: &Path, expected: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(checkout)
        .args(["rev-parse", "HEAD"])
        .output()?;
    if !output.status.success() {
        return Err(format!("{} is not a readable Git checkout", checkout.display()).into());
    }
    let actual = std::str::from_utf8(&output.stdout)?.trim();
    if actual != expected {
        return Err(
            format!("checkout revision is {actual}; expected pinned revision {expected}").into(),
        );
    }
    Ok(())
}

#[derive(Debug)]
struct Table<'a> {
    header: Vec<&'a str>,
    rows: Vec<Vec<&'a str>>,
    malformed: usize,
    physical_lines: usize,
}

fn parse_tsv(contents: &str) -> Result<Table<'_>, Box<dyn Error>> {
    let physical_lines = contents.lines().count();
    let mut lines = contents.lines();
    let header: Vec<&str> = lines.next().ok_or("TSV is empty")?.split('\t').collect();
    let mut rows = Vec::new();
    let mut malformed = 0;
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() == header.len() {
            rows.push(fields);
        } else {
            malformed += 1;
        }
    }
    Ok(Table {
        header,
        rows,
        malformed,
        physical_lines,
    })
}

fn read_tsv(checkout: &Path, relative: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(checkout.join(relative))?)
}

fn column(table: &Table<'_>, name: &str) -> Result<usize, Box<dyn Error>> {
    table
        .header
        .iter()
        .position(|column| *column == name)
        .ok_or_else(|| format!("required TSV column `{name}` is missing").into())
}

fn inventory_mixtecaso(checkout: &Path) -> Result<(), Box<dyn Error>> {
    verify_revision(checkout, MIXTECASO_REVISION)?;
    let segment_contents = read_tsv(checkout, "definitions/changes_segments.tsv")?;
    let segments = parse_tsv(&segment_contents)?;
    let from = column(&segments, "SOUND_FROM")?;
    let to = column(&segments, "SOUND_TO")?;
    let left = column(&segments, "ENVIRONMENT_LEFT")?;
    let right = column(&segments, "ENVIRONMENT_RIGHT")?;

    let mut unconditioned = 0;
    let mut representable = 0;
    let mut witnesses = 0;
    for row in &segments.rows {
        if !row[left].is_empty() || !row[right].is_empty() {
            continue;
        }
        unconditioned += 1;
        let target = row[from].strip_prefix('*').unwrap_or(row[from]);
        let output = normalize_empty_output(row[to]);
        if !is_conservative_literal(target) || !is_conservative_output(output) {
            continue;
        }
        representable += 1;
        if literal_witness_executes(target, output)? {
            witnesses += 1;
        }
    }

    let tone_contents = read_tsv(checkout, "definitions/changes_tones.tsv")?;
    let tones = parse_tsv(&tone_contents)?;
    let cognate_contents = read_tsv(checkout, "data/cognates.tsv")?;
    let cognates = parse_tsv(&cognate_contents)?;
    let segment_variable_contents = read_tsv(checkout, "variables/variables_segments.tsv")?;
    let segment_variables = parse_tsv(&segment_variable_contents)?;
    let tone_variable_contents = read_tsv(checkout, "variables/variables_tones.tsv")?;
    let tone_variables = parse_tsv(&tone_variable_contents)?;

    println!("source=mixtecaso");
    println!("revision={MIXTECASO_REVISION}");
    println!("license=CC-BY-SA-4.0");
    print_table_metrics("segment_definitions", &segments);
    println!("segment_unconditioned_rows={unconditioned}");
    println!("segment_conservative_representable_rows={representable}");
    println!("segment_executable_literal_witnesses={witnesses}");
    print_table_metrics("tone_definitions", &tones);
    println!("tone_supported_rows=0");
    print_table_metrics("cognates", &cognates);
    print_table_metrics("segment_variables", &segment_variables);
    print_table_metrics("tone_variables", &tone_variables);
    println!("ordered_chain_claims=0");
    Ok(())
}

fn print_table_metrics(prefix: &str, table: &Table<'_>) {
    println!("{prefix}_physical_lines={}", table.physical_lines);
    println!("{prefix}_parsed_rows={}", table.rows.len());
    println!("{prefix}_malformed_rows={}", table.malformed);
}

fn inventory_diachronica(checkout: &Path) -> Result<(), Box<dyn Error>> {
    verify_revision(checkout, DIACHRONICA_REVISION)?;
    let contents = fs::read_to_string(checkout.join("html/diachronica-data"))?;
    let mut section_rows = 0;
    let mut change_rows = 0;
    let mut parsed_change_rows = 0;
    let mut malformed_change_rows = 0;
    let mut rows_with_context = 0;
    let mut rows_with_excluded_context = 0;
    let mut rows_with_parallel_changes = 0;
    let mut rows_with_alternatives = 0;
    let mut representable = 0;
    let mut witnesses = 0;

    for line in contents.lines() {
        if line.starts_with("s ") {
            section_rows += 1;
            continue;
        }
        if !line.starts_with("c ") {
            continue;
        }
        change_rows += 1;
        let fields: Vec<&str> = line.split('\u{1}').collect();
        if fields.len() != 5 {
            malformed_change_rows += 1;
            continue;
        }
        parsed_change_rows += 1;
        let from = fields[2];
        let to = fields[3];
        let context = fields[4];
        rows_with_context += usize::from(!context.is_empty());
        rows_with_excluded_context += usize::from(context.contains('\u{2}'));
        rows_with_parallel_changes += usize::from(from.contains('\u{2}') || to.contains('\u{2}'));
        rows_with_alternatives += usize::from(
            from.contains('\u{3}') || to.contains('\u{3}') || context.contains('\u{3}'),
        );

        if context.is_empty()
            && !from.contains(['\u{2}', '\u{3}'])
            && !to.contains(['\u{2}', '\u{3}'])
        {
            let output = normalize_empty_output(to);
            if is_conservative_literal(from) && is_conservative_output(output) {
                representable += 1;
                if literal_witness_executes(from, output)? {
                    witnesses += 1;
                }
            }
        }
    }

    println!("source=diachronica");
    println!("revision={DIACHRONICA_REVISION}");
    println!("text_license=CC-BY-NC-SA-3.0");
    println!("conversion_code_license=undeclared");
    println!("physical_lines={}", contents.lines().count());
    println!("section_rows={section_rows}");
    println!("change_rows={change_rows}");
    println!("parsed_change_rows={parsed_change_rows}");
    println!("malformed_change_rows={malformed_change_rows}");
    println!("rows_with_context={rows_with_context}");
    println!("rows_with_excluded_context={rows_with_excluded_context}");
    println!("rows_with_parallel_changes={rows_with_parallel_changes}");
    println!("rows_with_alternatives={rows_with_alternatives}");
    println!("conservative_unconditioned_representable_rows={representable}");
    println!("locally_executable_literal_witnesses={witnesses}");
    println!("linguistic_correctness_claims=0");
    Ok(())
}

fn normalize_empty_output(output: &str) -> &str {
    if matches!(output, "∅" | "Ø" | "0") {
        ""
    } else {
        output
    }
}

fn is_conservative_literal(value: &str) -> bool {
    !value.is_empty()
        && !matches!(value, "∅" | "Ø" | "0")
        && !value.chars().any(|ch| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    ',' | '{'
                        | '}'
                        | '['
                        | ']'
                        | '('
                        | ')'
                        | '<'
                        | '>'
                        | '/'
                        | '!'
                        | '#'
                        | '_'
                        | ';'
                        | '^'
                        | '$'
                        | '\\'
                        | '\u{1}'
                        | '\u{2}'
                        | '\u{3}'
                )
        })
}

fn is_conservative_output(value: &str) -> bool {
    value.is_empty() || is_conservative_literal(value)
}

fn literal_witness_executes(target: &str, output: &str) -> Result<bool, Box<dyn Error>> {
    let classes: [Class<'_>; 0] = [];
    let rules = [Rule::new("corpus_witness", target, output)];
    let program = RuleSet::new(&classes, &rules).compile()?;
    Ok(program.apply(target) == output)
}
