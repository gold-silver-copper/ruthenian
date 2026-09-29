# Conformance fixture provenance

`BRASSICA_LICENSE` is a verbatim copy of Brassica's BSD-3-Clause notice at
revision `f3a92ad0eb5a890fdf92208c48325371e5e770ff`. The complete
Latin-to-Portuguese rule chain and ten-form lexicon in
`../brassica_latin_portuguese.rs` are adapted from
`examples/latin2port.bsc` and `examples/latin2port.lex` at that revision.
Rust rule names and syntax are local adaptations.

The cases in `../conformance.rs` are small, independently authored semantic
witnesses. Their `source_id` fields point to the upstream test file or data
category that motivated the capability check; the declarations, inputs, and
outputs were written from scratch for `sound-law`. This distinction matters:

- Brassica is BSD-3-Clause. The one adapted example retains the full notice;
  cases in `conformance.rs` remain original.
- Lexurgy is GPL-3.0. Its code, declarations, test prose, inputs, and expected
  strings were not copied or lightly transliterated.
- MixteCaSo is CC BY-SA 4.0. No dataset rows are vendored.
- Index Diachronica text is CC BY-NC-SA 3.0, while the conversion code has no
  declared license. No text, data rows, or code are vendored.

Pinned revisions, access date, row counts, license decisions, and every audited
source category are maintained in `../../UPSTREAM_CONFORMANCE.md`.
