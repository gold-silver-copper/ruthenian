//! The **random held-out samples** — the accuracy measurement.
//!
//! Drawn with a fixed seed from the complete set of Russian records, with no
//! hand-picking and no class targeting, so they measure the language rather than
//! the hard tail. This is the number to publish; `fixture.rs` is the regression
//! net. See `INVARIANTS.md` I1 and I3.

mod common;

const NOUNS: &str = include_str!("paradigms/random_nouns.tsv");
const NOUNS_META: &str = include_str!("paradigms/random_nouns_meta.tsv");
const VERBS: &str = include_str!("paradigms/random_verbs.tsv");
const VERBS_META: &str = include_str!("paradigms/random_verbs_meta.tsv");
const ADJS: &str = include_str!("paradigms/random_adjs.tsv");
const ADJS_META: &str = include_str!("paradigms/random_adjs_meta.tsv");

#[test]
fn random_sample_accuracy() {
    let mut all = common::Scored::default();
    for (rows, meta, label) in [
        (NOUNS, NOUNS_META, "random-noun"),
        (VERBS, VERBS_META, "random-verb"),
        (ADJS, ADJS_META, "random-adj"),
    ] {
        all.merge(common::score(rows, meta, label));
    }
    common::report("random held-out sample (ACCURACY)", &all.by_pos);
    common::dump_misses(&all.misses);

    println!("\nweakest slots (>=20 comparable):");
    for (slot, ok, n) in all.weakest(20, 10) {
        println!("   {ok:>5.1}%  {slot}  (n={n})");
    }

    let total: usize = all.by_pos.values().map(|s| s.comparable).sum();
    assert!(
        total > 3000,
        "only {total} comparable cells in the random sample"
    );
}
