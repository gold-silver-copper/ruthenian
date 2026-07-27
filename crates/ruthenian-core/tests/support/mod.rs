#![allow(dead_code)]

//! Reading the committed corpus. Shared by `conformance.rs` and `guards.rs`.

use std::path::PathBuf;

/// One cell of one paradigm.
pub struct Row {
    pub pos: String,
    pub lemma: String,
    pub features: String,
    pub form: String,
    pub section: String,
}

/// The crate root, so tests do not depend on the working directory.
pub fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The repository root — two levels up from `crates/ruthenian-core`.
pub fn repo_root() -> PathBuf {
    crate_dir()
        .parent()
        .and_then(|p| p.parent())
        .expect("the crate sits at crates/<name> inside the repository")
        .to_path_buf()
}

fn corpus_path() -> PathBuf {
    crate_dir().join("tests/corpus/paradigms.tsv")
}

/// The corpus text, including its header comments.
pub fn corpus_text() -> String {
    let path = corpus_path();
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// A `# key\tvalue` header field.
pub fn corpus_header(key: &str) -> String {
    let want = format!("# {key}\t");
    corpus_text()
        .lines()
        .find_map(|l| l.strip_prefix(&want))
        .unwrap_or_else(|| panic!("the corpus has no `# {key}` header"))
        .trim()
        .to_string()
}

/// Every data row, header and comments dropped.
pub fn corpus() -> Vec<Row> {
    corpus_text()
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .skip(1) // the column-name line
        .map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            assert_eq!(f.len(), 5, "malformed corpus row: {line:?}");
            Row {
                pos: f[0].to_string(),
                lemma: f[1].to_string(),
                features: f[2].to_string(),
                form: f[3].to_string(),
                section: f[4].to_string(),
            }
        })
        .collect()
}

/// FNV-1a over the bytes, matching `tools/extract_paradigms.py`.
///
/// Hand-rolled because the crate has no dependencies and this is the whole of
/// what a currency check needs: a value that changes when the file does.
pub fn fnv1a64(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in data {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}
