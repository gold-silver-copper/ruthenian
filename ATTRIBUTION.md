# Attribution and licensing

This repository mixes **original source code** with **third-party data** and
**machine-generated content derived from that data**. They are licensed
differently. Read this before reusing anything.

| Part | What | Licence |
|---|---|---|
| Source code | everything under `crates/*/src`, `tools/`, `Cargo.toml` | **MIT OR Apache-2.0** |
| Reference implementation | `legacy/` | MIT OR Apache-2.0, © gold-silver-copper |
| Russian corpus | `biblija_ru.txt`, `biblija_ukr.txt`, `crates/ruthenian-orthography/tests/corpus/sample.tsv` | **public domain** (see below) |
| Paradigm fixtures | `crates/ruthenian-core/tests/paradigms/*.tsv` | **CC BY-SA 4.0 + GFDL** (see below) |
| Generated forms | anything the crates produce at runtime | derived from the above; treat as CC BY-SA + GFDL and **machine-generated, unverified** |

If you redistribute the data or generated content, keep the attribution below
and share adaptations under the same terms.

---

## English Wiktionary via Wiktextract — the paradigm fixtures

`crates/ruthenian-core/tests/paradigms/fixture.tsv`, `random_nouns.tsv`,
`random_verbs.tsv`, `random_adjs.tsv` and their `_meta` companions contain
attested Russian word forms, class codes and grammatical metadata extracted from
**English Wiktionary**. They are *source-language* data — Russian, not Ruthenian
— and their role is described in that directory's `README.md`.

- English Wiktionary content is dual-licensed **CC BY-SA (3.0/4.0) and GFDL**.
  <https://en.wiktionary.org/wiki/Wiktionary:Copyrights>
- Extracted with **Wiktextract** by **Tatu Ylonen**, which is MIT-licensed; the
  extracted *data* keeps Wiktionary's licence.
  <https://github.com/tatuylonen/wiktextract>
- **Attribution and share-alike are required** of anyone who redistributes these
  files or content derived from them.

The raw dump (`raw-wiktextract-data.jsonl`, 22 GB) is **not** redistributed here.
It is read locally, in full, by `tools/measure.py` and `tools/build_fixture.py` —
see [`DIRECTION.md`](DIRECTION.md) law 3, which forbids sampling it.

## Russian Synodal Bible — the orthography corpus

`biblija_ru.txt` and `biblija_ukr.txt` at the repository root, and the 1 621-line
excerpt at `crates/ruthenian-orthography/tests/corpus/sample.tsv`, are the
Russian and Ukrainian Synodal Bible translations. The Russian Synodal text (1876)
is **public domain** by age.

Provenance: both arrived via the reference implementation,
<https://github.com/gold-silver-copper/ruthenian> at commit `49d3af7`. The
excerpt keeps the original line numbers so any row can be traced back.

## The reference implementation

`legacy/` is the original `ruthenian` crate, unchanged, © gold-silver-copper,
MIT OR Apache-2.0. It is kept so the head-to-head in
[`crates/ruthenian-orthography/README.md`](crates/ruthenian-orthography/README.md)
reproduces from one checkout.

---

## How to attribute when you reuse this

- **Code**: keep the MIT/Apache-2.0 notices.
- **Fixtures or anything derived from them**: credit *English Wiktionary and its
  contributors*, note the CC BY-SA 4.0 + GFDL licence, and license your
  adaptation the same way.
- **Generated forms**: say they are machine-generated and unverified, and carry
  the Wiktionary attribution through.
