# Paradigm fixture

`fixture.tsv` — 3 246 attested form rows for 151 lemmas, extracted from
`~/Desktop/code/wikidata/raw-wiktextract-data.jsonl` (22 GB, 10 667 129 lines) on
2026-07-25. `fixture_meta.tsv` carries one row per lemma: part of speech, class
code, metadata, and why it was selected.

Format: `lemma \t pos \t class \t slot-tags \t form`. Slot tags are the source's
own, sorted; qualifier-tagged variants (dated, obsolete, archaic, colloquial,
rare, regional …) are excluded so the comparison is against the primary form.

## Selection

Real records only — never hand-written paradigms, which encode what we believe
rather than what is attested. Chosen to cover:

- **38 verb classes**, led by the frequent ones (`1a`, `4a/b/c` with and without
  `+p`, `2a`, `3a`, `5b`, `6c`), plus `irreg` and `-`;
- **18 distinct present-stem mutations**, including `ov → u`, the commonest;
- every noun stem class and accent patterns a–f;
- adjectives with `*`, `①`, `②`;
- named hard cases: `победить` (the `futr_1sg: "-"` lexical gap), `идти`, `быть`,
  `есть`, `мать`, `дочь`, `время`, `путь`, `ножницы`, `кофе`, `метро`, `пальто`.

`class-codes.txt` is the parser's test corpus: every distinct `ru-conj` class
code in the sample (117 of them).

## Known fixture artefacts

`писать` resolves to the class `1a` homograph (*to piss*), not `писа́ть` (*to
write*, class `6c`). The builder takes the first record per lemma, so a homograph
pair collapses to one. This is a fixture limitation, not a rule failure, and it
is what the composite keys of phase 3 exist to fix.
