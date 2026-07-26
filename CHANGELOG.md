# Changelog

## Unreleased

### Added

- **The language specification.** [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md) is
  now in the repository and is **normative**: eight cases, three numbers, three
  declensions, six conjugation classes, three past tenses, with the etymology of
  each restored category. `docs/COMPARATIVE_GRAMMAR.md` carries the measured
  evidence behind it, and `docs/README.md` states which document outranks which.
- **`ruthenian-core` 0.1.0** — the productive morphology of Ruthenian as pure
  rules. Zero third-party dependencies.
  - Eight cases including the restored **ablative** (`doma` "from the house"
    against `domogo` "of the house" — PIE `*-ōd` and `*-osyo` returned to their
    inherited functions) and a productive **vocative**.
  - Three numbers: the **dual** throughout, in nouns, adjectives, pronouns, verb
    agreement and numeral government. `dva` governs it, which is what removes
    Russian's 2–4 genitive singular rather than merely simplifying it.
  - Three declensions with a hard/soft alternation, replacing eight; six
    conjugation classes, replacing sixteen.
  - Three past tenses — **aorist** and **imperfect** are synthetic and
    independent of aspect, as in OCS.
  - **All three palatalizations.** The second, which Russian levelled to 0 %,
    distinguishes locative `druzi` from vocative `druzzje` in the consonant and
    makes genitive `knigi` differ from dative `knizi` by the consonant alone.
  - **Aspect is derived, never stored** — `aspect_of` computes it from surface
    shape and its trace names which of §7.2's three rules fired.
  - Whole-paradigm accessors `noun_forms` / `adj_forms` / `verb_forms`, and
    `verb_with` for the suppletive cases no class derives.
  - **`spec_paradigms_match`**: 144 cells across six paradigms, plus §11's
    paradigm-size table, parsed out of `docs/RUTHENIAN.md` at test time. No
    expected form is transcribed, so the corpus cannot drift from the document.
  - 15 guards, four verified against their failure witnesses; 24 doc tests.
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
