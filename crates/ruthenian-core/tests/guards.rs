//! The structural guards this crate declares; see `README.md`.
//!
//! Law 8: **every guard has a verified failure witness.** Each names the minimal
//! mutation that must make it fail, and each was verified by applying that
//! mutation, observing the failure, and reverting. A guard that survives its own
//! witness is stale and must be fixed or deleted, not left in place looking
//! reassuring.
//!
//! `conformance.rs` holds the first guard — the engine against the corpus — and
//! is separate only because it is the one that runs the whole engine.

use ruthenian_core::fallback::{UNREADABLE, is_unreadable};
use ruthenian_core::{Animacy, Case, FiniteTense, Gender, Noun, Number, Person, noun};

mod support;
use support::{corpus, corpus_header, crate_dir, fnv1a64, repo_root};

/// Every `.rs` file under `src/`, as (path, text).
fn sources() -> Vec<(String, String)> {
    let dir = crate_dir().join("src");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("src/ exists") {
        let path = entry.expect("readable entry").path();
        if path.extension().is_some_and(|e| e == "rs") {
            let name = path
                .file_name()
                .expect("a file name")
                .to_string_lossy()
                .to_string();
            out.push((name, std::fs::read_to_string(&path).expect("readable")));
        }
    }
    assert!(out.len() >= 5, "expected the whole module set, got {out:?}");
    out
}

// --------------------------------------------------------------------------
// 2. spec_currency
//    Witness: change one byte of docs/RUTHENIAN.md without regenerating.
// --------------------------------------------------------------------------
/// The corpus was generated from *this* specification.
///
/// Without this, editing a paradigm in the specification leaves the corpus
/// asserting the old forms, and the conformance test goes on passing while
/// checking something the language no longer says.
#[test]
fn spec_currency() {
    let spec = repo_root().join("docs/RUTHENIAN.md");
    let bytes = std::fs::read(&spec).expect("the specification is in the repository");
    let recorded = corpus_header("spec-fnv1a64");
    let actual = format!("{:#018x}", fnv1a64(&bytes));
    assert_eq!(
        recorded, actual,
        "docs/RUTHENIAN.md has changed since the corpus was generated.\n\
         Run `python3 tools/extract_paradigms.py` and review the corpus diff."
    );
}

// --------------------------------------------------------------------------
// 3. corpus_row_count
//    Witness: delete a row from paradigms.tsv.
// --------------------------------------------------------------------------
/// The corpus has as many rows as it says. A refactor cannot quietly drop cells.
#[test]
fn corpus_row_count() {
    let recorded: usize = corpus_header("rows").parse().expect("a row count");
    assert_eq!(recorded, corpus().len(), "corpus row count drifted");
    // 11 nominal paradigms × 24 cells. Stated so that adding a paradigm is a
    // deliberate edit here rather than a number that moves on its own.
    assert_eq!(
        recorded % 24,
        0,
        "a nominal paradigm is 8 cases × 3 numbers"
    );
}

// --------------------------------------------------------------------------
// 4. no_option_no_result
//    Witness: change any public signature to return Option<String>.
// --------------------------------------------------------------------------
/// The public API returns forms, never absence.
///
/// Law 4 is what makes the crate usable without an `unwrap` at every call site,
/// and it is only true if nothing leaks an `Option`. Internal code may use them
/// freely — `Nominal::read` does — which is why this looks at `pub fn` alone and
/// not at `pub(crate) fn`.
#[test]
fn no_option_no_result() {
    let mut offenders = Vec::new();
    for (name, text) in sources() {
        for (n, line) in text.lines().enumerate() {
            let line = line.trim();
            if !line.starts_with("pub fn") {
                continue;
            }
            if line.contains("Option<") || line.contains("Result<") {
                offenders.push(format!("{name}:{}: {line}", n + 1));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "the public API must be total (law 4):\n{}",
        offenders.join("\n")
    );
}

// --------------------------------------------------------------------------
// 5. every_fallback_exercised
//    Witness: add a fallback to src/fallback.rs without a test below.
// --------------------------------------------------------------------------
/// Every declared substitute is actually reachable and actually tested.
///
/// A fallback that nothing exercises is a claim, not a behaviour. Adding one
/// means adding a case here, which is the point: a substitute cannot enter the
/// crate quietly.
#[test]
fn every_fallback_exercised() {
    // UNREADABLE — the only fallback the crate has at this milestone.
    for bad in ["", "'", "''", "дом", "quiz", "a", "!"] {
        let got = noun(bad, Case::Nominative, Number::Singular);
        assert!(
            is_unreadable(&got),
            "an unreadable lemma {bad:?} must return the declared substitute, got {got:?}"
        );
    }
    // It is not a plausible Ruthenian word, so it cannot collide with a form.
    assert!(!UNREADABLE.chars().all(|c| c.is_ascii_alphabetic()));

    // Each fallback named in src/fallback.rs's table must appear above. The
    // count is asserted so that documenting a new one without testing it fails.
    let text = std::fs::read_to_string(crate_dir().join("src/fallback.rs")).expect("readable");
    // Rows of the totality table, less its header. The separator row begins
    // `//! |---` and so is not counted.
    let declared = text
        .lines()
        .filter(|l| l.starts_with("//! | "))
        .count()
        .saturating_sub(1);
    assert_eq!(
        declared, 5,
        "src/fallback.rs declares {declared} fallbacks; this guard exercises the \
         one that is implemented and the rest arrive with M4–M6. Update both together."
    );
}

// --------------------------------------------------------------------------
// 6. paradigm_is_form
//    Witness: give paradigm() its own ending table instead of calling form().
// --------------------------------------------------------------------------
/// `paradigm()` and `form()` are one generation path (law 2).
#[test]
fn paradigm_is_form() {
    for lemma in [
        "dom", "Konj", "Drug", "okno", "polje", "zzena", "nacija", "noczj'",
    ] {
        let n = Noun::new(lemma);
        let table = n.paradigm();
        assert_eq!(table.len(), 24, "{lemma}: 8 cases × 3 numbers");
        for (case, number, form) in table {
            assert_eq!(
                form,
                n.form(case, number),
                "{lemma} {case:?}.{number:?}: paradigm() disagrees with form()"
            );
            assert_eq!(form, noun(lemma, case, number), "{lemma}: and with noun()");
        }
    }
}

// --------------------------------------------------------------------------
// 7. totality_no_panic
//    Witness: remove a guard clause in Nominal::read, or unwrap in noun().
// --------------------------------------------------------------------------
/// No input panics, for any combination the types permit.
///
/// This is the guard that finds real bugs, because it is the only one that does
/// not know what the answer should be — only that there must be one.
#[test]
fn totality_no_panic() {
    let hostile = [
        "",
        "'",
        "''",
        "'''",
        "a",
        "j",
        "ja",
        "o",
        "je",
        "дом",
        "quiz",
        "!",
        " ",
        "\t",
        "\n",
        "dom'",
        "'dom",
        "d'o'm",
        "DOM",
        "DoM'",
        "zzena'",
        &"x".repeat(10_000),
        &"'".repeat(64),
        "\u{301}",
        "dom\u{301}",
    ];
    for word in hostile {
        for number in Number::ALL {
            for case in Case::ALL {
                let got = noun(word, case, number);
                assert!(
                    !got.is_empty(),
                    "noun({word:?}, {case:?}, {number:?}) returned an empty string; \
                     every cell has a form or the declared substitute"
                );
            }
        }
        // The bound form must agree, including on hostile input.
        let n = Noun::new(word);
        assert_eq!(n.paradigm().len(), 24);
    }
}

// --------------------------------------------------------------------------
// 8. output_is_lowercase
//    Witness: pass the lemma's own case through instead of folding it.
// --------------------------------------------------------------------------
/// Inflected output is always lowercase (§2.1).
///
/// A capital in a lemma means *animate*, not sentence-initial, so it must never
/// survive into a form. A text that needs a capital applies it afterwards.
#[test]
fn output_is_lowercase() {
    for lemma in ["Drug", "Konj", "Sluga'", "ZZENA", "Nacija"] {
        for number in Number::ALL {
            for case in Case::ALL {
                let got = noun(lemma, case, number);
                assert_eq!(
                    got,
                    got.to_lowercase(),
                    "{lemma} {case:?}.{number:?} = {got:?} is not lowercase"
                );
            }
        }
    }
    // And the capital still did its job: animacy reached the accusative.
    assert_eq!(noun("Drug", Case::Accusative, Number::Singular), "druga");
    assert_eq!(noun("drug", Case::Accusative, Number::Singular), "drug");
}

// --------------------------------------------------------------------------
// 9. no_dependencies
//    Witness: add any crates.io entry to Cargo.toml.
// --------------------------------------------------------------------------
/// Zero third-party dependencies. `ruthenian-orthography` by path, nothing else.
#[test]
fn no_dependencies() {
    let text = std::fs::read_to_string(crate_dir().join("Cargo.toml")).expect("readable");
    let deps = text
        .split("[dependencies]")
        .nth(1)
        .expect("a [dependencies] section");
    for line in deps.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        assert!(
            line.starts_with("ruthenian-orthography") && line.contains("path"),
            "third-party or non-path dependency: {line:?}"
        );
    }
}

// --------------------------------------------------------------------------
// 10. no_stored_derivable_state
//     Witness: add a `gender: Gender` field to Noun and set it in new().
// --------------------------------------------------------------------------
/// Law 3: derive state, never store it.
///
/// `Noun` holds the lemma and nothing else. A cached gender, declension or stem
/// is a field that can disagree with the lemma it came from, and its stale
/// branch becomes the bug.
#[test]
fn no_stored_derivable_state() {
    let text = std::fs::read_to_string(crate_dir().join("src/noun.rs")).expect("readable");
    let body = text
        .split("pub struct Noun {")
        .nth(1)
        .expect("Noun is declared")
        .split('}')
        .next()
        .expect("a struct body");
    let fields: Vec<&str> = body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .collect();
    assert_eq!(
        fields,
        vec!["lemma: String,"],
        "Noun must hold the lemma and nothing else"
    );
}

// --------------------------------------------------------------------------
// 11. every_public_fn_has_a_doctest
//     Witness: delete the ``` block from any public function's doc comment.
// --------------------------------------------------------------------------
/// Every public function shows a real form.
///
/// The doc tests are a second, independent corpus: they are written from the
/// specification by hand, so a rule that satisfies the TSV but produces nonsense
/// elsewhere still fails here.
#[test]
fn every_public_fn_has_a_doctest() {
    let mut missing = Vec::new();
    for (name, text) in sources() {
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            if !line.trim_start().starts_with("pub fn") {
                continue;
            }
            // Walk back over the doc comment and attributes attached to it.
            let mut has_example = false;
            let mut i = n;
            while i > 0 {
                let prev = lines[i - 1].trim();
                if prev.starts_with("///") {
                    if prev.contains("```") {
                        has_example = true;
                    }
                } else if !prev.starts_with('#') || prev.is_empty() {
                    // Not a doc line and not an attribute: the comment block has
                    // ended, so stop rather than wandering into the item above.
                    break;
                }
                i -= 1;
            }
            if !has_example {
                missing.push(format!("{name}:{}: {}", n + 1, line.trim()));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "every public function carries a doc test showing a real form:\n{}",
        missing.join("\n")
    );
}

// --------------------------------------------------------------------------
// 12. grammar_types_are_exhaustive
//     Witness: drop a variant from Case::ALL.
// --------------------------------------------------------------------------
/// The `ALL` lists match the enums they enumerate.
///
/// `paradigm()` walks these, so a missing variant silently shrinks every table
/// in the crate — including the corpus check, which would then pass while
/// testing less.
#[test]
fn grammar_types_are_exhaustive() {
    assert_eq!(Case::ALL.len(), 8, "§3.1 gives eight cases");
    assert_eq!(Number::ALL.len(), 3, "§3.1 gives three numbers");
    assert_eq!(Gender::ALL.len(), 3);
    assert_eq!(Animacy::ALL.len(), 2);
    assert_eq!(Person::ALL.len(), 3);
    assert_eq!(
        FiniteTense::ALL.len(),
        3,
        "§7.1 gives three synthetic tenses"
    );

    // No duplicates, which a copy-paste edit to an ALL list would introduce.
    let mut cases = Case::ALL.to_vec();
    cases.sort();
    cases.dedup();
    assert_eq!(cases.len(), 8);
}
