# Changelog

## Unreleased

### Added

- **`ruthenian-orthography` 0.1.0** — bijective Cyrillic↔Latin conversion, the
  workspace's first crate. Zero dependencies.
  - The round-trip contract holds by construction: a greedy longest-match reader
    defines how Ruthenian is read, and the writer inserts the separator `'`
    exactly where re-reading its own output would diverge.
  - The alphabet is declared, and includes four context rules — the environments
    of `ъ`, `ь` and `й`, and hard-sign case agreement — which let the reverse
    direction decide rather than guess. Each was validated against 41 462 lines
    of Russian prose before being declared.
  - Case is a separate layer: ALL-CAPS tokens get ALL-CAPS digraphs
    (`ЩУКА` → `SZCZUKA`), everything else Title-case ones, and mixed case
    round-trips without a special path.
  - Stress is carried as a combining acute on the Latin vowel (`pisátj`).
    Stressed and unstressed spellings are different strings.
  - `to_latin` is strict; `to_latin_mixed` transliterates declared runs only and
    reports skipped spans, returning `String` rather than `Ruthenian` because its
    output does not participate in the contract.
  - 11 guards, each verified to fail under its stated mutation.

Measured against the reference implementation
(gold-silver-copper/ruthenian@49d3af7) on the same corpus: **0 round-trip
failures against its 3** (lines 12695, 13444, 31725).
