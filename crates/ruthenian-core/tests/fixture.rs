//! The **targeted** paradigm fixture: one lemma per class, per mutation, plus
//! hand-picked hard cases.
//!
//! This is a regression net over the hard tail. Its pass rate is **not** an
//! accuracy figure — quoting it as one understated the crate by 20 points for
//! nouns. The accuracy measurement is `random.rs`. See `INVARIANTS.md` I3.

mod common;

const FIXTURE: &str = include_str!("paradigms/fixture.tsv");
const META: &str = include_str!("paradigms/fixture_meta.tsv");

#[test]
fn targeted_fixture() {
    let scored = common::score(FIXTURE, META, "targeted");
    common::report(
        "targeted fixture (regression net, NOT accuracy)",
        &scored.by_pos,
    );
    common::dump_misses(&scored.misses);

    let lemmas: std::collections::BTreeSet<&str> = scored
        .misses
        .iter()
        .filter_map(|m| m.split('\t').nth(2))
        .collect();
    println!("\nlemmas needing lexicon support: {}", lemmas.len());
    for l in lemmas.iter().take(20) {
        println!("  {l}");
    }

    let total: usize = scored.by_pos.values().map(|s| s.comparable).sum();
    assert!(
        total > 2000,
        "only {total} comparable cells; fixture broken?"
    );
}
