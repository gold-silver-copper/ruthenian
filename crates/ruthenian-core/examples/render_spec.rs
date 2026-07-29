//! Regenerate the specification's marked tables in place.
//!
//! ```text
//! cargo run -p ruthenian-core --example render_spec
//! ```
//!
//! Every table between `<!-- render:ID -->` markers in `docs/RUTHENIAN.md` is
//! replaced with a fresh rendering from the engine (see `render.rs` for why the
//! engine is normative for tables). After an update, rerun
//! `python3 tools/extract_paradigms.py` so the corpus records the new spec
//! checksum — the `spec_currency` guard insists on it.

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/RUTHENIAN.md");
    let spec = std::fs::read_to_string(path).expect("docs/RUTHENIAN.md is in the repository");
    match ruthenian_core::render::apply(&spec) {
        Ok(fresh) if fresh == spec => println!("all rendered tables already current"),
        Ok(fresh) => {
            std::fs::write(path, fresh).expect("the specification is writable");
            println!("updated docs/RUTHENIAN.md — rerun tools/extract_paradigms.py");
        }
        Err(broken) => {
            eprintln!("marker errors: {broken}");
            std::process::exit(1);
        }
    }
}
