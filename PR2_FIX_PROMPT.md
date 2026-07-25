# PR #2 fixes — research and execution prompt

Work on `agent/core`, the branch behind [PR #2]. Every fix below lands in that
PR; do not open a new one.

Part 1 is the research the fixes rest on. Part 2 is what to do about it. Read
Part 1 first: several fixes exist because numbers published in the PR were wrong,
and the reasons matter more than the corrections.

[PR #2]: https://github.com/gold-silver-copper/ruthenian/pull/2

---

# Part 1 — Research

Every figure below is measured over the **entire** dump —
`~/Desktop/code/wikidata/raw-wiktextract-data.jsonl`, 23 622 298 877 bytes,
10 667 129 lines, 441 629 Russian records — on 2026-07-25. Sampling is forbidden
(`INVARIANTS.md` I1). Regenerate everything with:

```bash
cd ~/Desktop/code/wikidata
LC_ALL=C grep -F '"lang_code": "ru"' raw-wiktextract-data.jsonl > /tmp/ru_all.jsonl
cd ~/Desktop/code/ruthenian && python3 tools/measure.py /tmp/ru_all.jsonl
```

## Finding 0 — sampling was wrong three times, which is why I1 exists

This research was originally done on `dd` windows covering ~8 % of the dump. The
full scan changed the answer materially in three places, and in one of them the
*shape* of the conclusion changed, not just the precision:

| From a sample | From the full scan |
|---|---|
| 117 distinct verb class codes | **226** — the sample missed 109 |
| classes 1–6 cover 87.6 % of verbs | **90.7 %** |
| 670 class-1 labial stems take no epenthesis | **1 977, and not one takes it** — exceptionless |
| `ov → u` occurs 146 times | **675** |
| noun accents are a–f | there are also primed patterns (`accent-dʹ`, `accent-fʺ`) the sample never saw |

The third row is the one that matters. A sample shows a rule holds *often*; only
a full scan shows it holds *always*, and "always" is what lets the engine key on
a rule instead of hedging around it.

A full-file `grep` is also not automatically a full scan. An intermediate attempt
used `grep -oE` with a pattern assuming JSON key order and found 183 of the 226
codes. Parse the records; do not pattern-match them.

## Finding 1 — the fixture is selection-biased, and the PR's accuracy table understates the crate

The paradigm fixture is built to *cover* things: one lemma per class, one per
mutation, plus hand-picked hard cases (`имя`, `дитя`, `время`, `мать`, `дочь`,
`победить`). That is right for a regression net and wrong for an accuracy figure
— it samples the hard tail, not the language.

Measured against a **random held-out sample** (fixed seed, no hand-picking, no
class targeting) instead:

| | PR #2 reports | Random sample |
|---|---:|---:|
| noun | 58.0 % | **79.1 %** |
| adj | 80.2 % | **93.1 %** |
| verb | 84.7 % | 87.7 % |

Splitting the old fixture by why each lemma was chosen shows the same thing, and
shows verbs as a control with almost no bias (87.5 vs 87.7) — so the bias is real
and specific, not an artefact of the splitting:

| pos | selection | lemmas | rows | misses | ok |
|---|---|---:|---:|---:|---:|
| noun | hand-picked hard cases | 15 | 157 | 75 | 52.2 % |
| noun | class-coverage | 40 | 460 | 163 | 64.6 % |
| adj | hand-picked hard cases | 2 | 52 | 12 | 76.9 % |
| adj | class-coverage | 18 | 450 | 31 | **93.1 %** |
| verb | hand-picked hard cases | 11 | 257 | 32 | 87.5 % |
| verb | class-coverage | 65 | 1489 | 183 | 87.7 % |

That split is itself contaminated — the builder fills class buckets before named
ones, so a hard case that also filled a class bucket (`имя`, `дитя`) counts as
"class-coverage". Which is why the noun class-coverage figure (64.6 %) still sits
below the clean random sample (79.1 %), and why the random sample — not a
re-slice of the fixture — is the number to publish.

The random samples are now committed:
`tests/paradigms/random_nouns.tsv` (150 lemmas, 1 932 rows),
`random_verbs.tsv` (150 / 3 358), `random_adjs.tsv` (100 / 2 384), seed 20260725.

## Finding 2 — the class-code parser failed on 9 of the 226 real codes

The committed corpus held 117 codes taken from a window sample. Against the full
226, the parser rejected 9, in three groups:

| Codes | Feature | Why it failed |
|---|---|---|
| `14b/c'+p`, `14c/c'+p`, `16b/c'`, `irreg/c'+p` | ASCII `'` marking a softness distinction | the parser handled U+02B9 `ʹ` but not U+0027 `'` |
| `4b/c-nd` | a `-nd` stem suffix (cf. `-bd` in `14c/c-bd`) | `n` is neither a digit nor `a`–`f`, so it hit the error arm |
| `6a1as13`, `6a1as13+p`, `6a1as14`, `6a1as14+p` | an `sNN` variant marker | same |

**Already fixed on the branch.** The parser now handles all three features
(`ZaliznyakVerbClass` gained `stem_suffix` and `variant`), the corpus at
`tests/paradigms/class-codes.txt` is the full 226, and `class_codes_parse`
asserts a floor of 226 so a shrunken corpus fails loudly. The parser erroring
rather than silently defaulting is what made these visible at all.

Class distribution, full dump:

```
5060 1a     953 2a+p    693 4a+p   639 4b+p   547 1a+p   539 2a
 491 4b     483 4c+p    480 4a     356 4c     224 5b     211 3b
```

**Classes 1–6 are 11 584 / 12 773 = 90.7 %** of verbs carrying a class code.

## Finding 3 — the same coverage figure was published three times, three ways, all wrong

| Where | Figure | Method |
|---|---|---|
| `lib.rs` (original) | ~76 % | top-22 codes only |
| PR #2 description | 73.2 % | top-22 codes only, recounted |
| `lib.rs` (corrected) | 87.6 % | window sample, all codes |
| **True** | **90.7 %** | **full scan, all codes** |

The first two counted only the twenty-two most frequent codes, so class-1–6
*variants* outside that list went uncounted. The third was right in method and
low only because it sampled. This is the metric-drift failure `LESSONS.md`
records from slovowiki, reproduced inside the PR that cites the lesson — and the
reason `INVARIANTS.md` I2 exists.

## Finding 4 — stress placement is buggy, not unmodelled

The PR attributes the low strict score (4.1 % verb, 10.4 % noun) to accent
patterns `d`–`f` being unmodelled. The full-dump distribution does not support
that:

| Nouns | count | | Verbs | count |
|---|---:|---|---|---:|
| `accent-a` | 25 442 | | `a` | 8 667 |
| `accent-b` | 2 382 | | `b` | 2 547 |
| `accent-c` | 536 | | `c` | 1 307 |
| `accent-d` | 495 | | `e` | 252 |
| `accent-e` | 287 | | | |
| `accent-f` | 75 | | | |
| primed (`dʹ fʹ bʹ fʺ`) | 72 | | | |
| **a+b modelled** | **27 824 / 29 381 = 94.7 %** | | **a+b+c modelled** | **12 521 / 12 773 = 98.0 %** |

So 95–98 % of nouns and verbs fall in patterns the crate *claims* to model while
strict accuracy is 4–10 %. The unmodelled tail cannot account for that. The
placement logic inside the implemented patterns is wrong — a bug to fix, not a
coverage limit to document.

## Finding 5 — the noun weakness is two cells, not a systematic fault

From the random sample, by slot:

| Slot | segmental |
|---|---:|
| genitive plural | 64.7 % |
| instrumental singular | 65.8 % |
| dative singular | 78.4 % |
| accusative plural | 78.8 % |
| nominative plural | 79.2 % |
| prepositional singular | 79.3 % |
| genitive singular | 80.2 % |
| accusative singular | 81.7 % |

Genitive plural and instrumental singular stand out, and both are hard for
principled reasons: the genitive plural is a zero ending triggering fleeting-
vowel insertion (`okon`, `sestjor`), and the instrumental singular carries the
`-om`/`-em` alternation after sibilants and `c`. Everything else clusters at
79–82 %, the general level rather than a defect.

## Finding 6 — adjective soft stems are 1.6 % of adjectives and are derivable

`adjective.rs` hardcodes `soft = false` and documents the resulting forms as
wrong. Over every Russian adjective in the dump:

| Type | count | share |
|---|---:|---:|
| hard `-yj` | 6 669 | 66.7 % |
| velar/sibilant (hard endings, `i`-spelling) | 2 356 | 23.6 % |
| stressed `-oj` (hard) | 540 | 5.4 % |
| other | 279 | 2.8 % |
| **true soft (`-nij`)** | **155** | **1.6 %** |

The exposure is 1.6 %, not the open-ended problem the comment implies. And
softness *is* derivable from the citation form — ends `-ij` **and** the
stem-final consonant is not velar or sibilant — which is exactly the rule
`tools/measure.py` uses to produce this table. The velar/sibilant group is
already handled correctly: hard endings with the `y`→`i` spelling rule.

## Finding 7 — 560 KB of third-party data ships with no attribution file

| Artefact | Size | Source | Licence |
|---|---:|---|---|
| `crates/ruthenian-orthography/tests/corpus/sample.tsv` | 328 KB | Russian Synodal Bible, via `gold-silver-copper/ruthenian@49d3af7` | public domain (1876); provenance still required |
| `crates/ruthenian-core/tests/paradigms/*.tsv` | 232 KB → now ~600 KB | English Wiktionary via Wiktextract | **CC BY-SA 4.0 + GFDL** |

`docs/specs/ruthenian-extract.md` requires `ATTRIBUTION.md` "in the same commit
that first vendors extracted data". It does not exist. The obligation began in
phase 1 and has grown with every fixture regeneration since.

Note the contrast with slovowiki, whose `ATTRIBUTION.md` states the raw dump "is
**not** redistributed in this repository". This repository *does* redistribute
dump-derived data, so its obligation is strictly larger than the model it copies.

---

# Part 2 — The fixes

Land all of these on `agent/core`. Order is by risk, not size.

Fix 2 and the sampling removal are **already done** on the branch — the parser
handles all 226 codes, `tools/measure.py` and `tools/build_fixture.py` do full
scans, the fixture and random samples are regenerated from the complete record
set, and `INVARIANTS.md` records the rule. What follows is the remainder.

## Fix 1 — `ATTRIBUTION.md` (licence condition)

Write it at the repository root on slovowiki's structure
(`~/Desktop/code/slovowiki/ATTRIBUTION.md`): a table separating **source code**
(MIT OR Apache-2.0) from **vendored data** from **generated content**, then a
section per source.

Must state, per Finding 7: the corpus sample's provenance and public-domain
status; that the paradigm fixture and the random samples are English Wiktionary
content extracted with Wiktextract, carrying **CC BY-SA 4.0 + GFDL** and
requiring attribution and share-alike from anyone redistributing them; that
Wiktextract itself (Tatu Ylonen) is MIT while the data keeps Wiktionary's
licence; and that the raw dump is not redistributed here.

Link it from the root `README.md`, from `tools/README.md`, and from both
`tests/*/README.md` files.

## Fix 3 — measure honestly: two samples, one number each

1. **Score the random held-out samples** — they are committed but nothing reads
   them yet. Add a harness beside `tests/fixture.rs` that scores
   `random_{nouns,verbs,adjs}.tsv` segmentally and strictly.
2. **Report both, and say what each is for.** The targeted fixture is a
   regression net; the random samples are the accuracy figure. Never average
   them, never quote the targeted one as accuracy (`INVARIANTS.md` I3).
3. **Keep the `RUTHENIAN_DUMP_MISSES` diagnostic** in `tests/fixture.rs` — it is
   what made Finding 5 possible — and extend it to the random harness.
4. **Correct every published figure** to full-dump values: classes 1–6 are
   **90.7 %**; noun accent `a+b` is **94.7 %**; verb `a+b+c` is **98.0 %**; and
   the PR description's accuracy table gets both columns.

Acceptance: every number in `lib.rs`, the crate README and the PR description
traces to `tools/measure.py`, and no two disagree (`INVARIANTS.md` I2).

## Fix 4 — derive adjective softness

Replace the `soft = false` hardcode with the derivation from Finding 6: soft iff
the citation form ends `-ij` **and** the stem-final consonant is not velar or
sibilant. `adjective()` currently takes a bare stem, which cannot carry the
distinction — take the citation form, or add an explicit `soft: bool` the caller
supplies, whichever keeps the function pure in its arguments.

Delete the comment admitting the forms are wrong; it will no longer be true.

Acceptance: `sinij` declines soft (`sinjego`), `russkij` stays hard (`russkogo`,
not `*russkjego`), `novyj` unchanged. All three as golden cases.

## Fix 5 — fix stress placement inside the modelled patterns

Per Finding 4, 95–98 % of nouns and verbs are in patterns the crate claims to
model, so the 4–10 % strict score is placement bugs. Work from the
`RUTHENIAN_DUMP_MISSES` dump filtered to rows differing **only** in stress —
that set is the work list and is already computable.

Do not extend to patterns `d`–`f` here. They are 2–5 % and the honest statement
about them ("segmental form correct, stress not modelled, recorded in the trace")
is already true.

Acceptance: strict scores rise substantially on both fixtures and are reported;
`stress_placed` still fails under its witness.

## Fix 6 — genitive plural and instrumental singular

The two weak cells from Finding 5.

- **Genitive plural**: the zero ending with fleeting-vowel insertion
  (`okno` → `okon`, `sestra` → `sestjor`). `NounClass::reducible` already carries
  the `*` marker and is currently unused.
- **Instrumental singular**: the `-om`/`-em` alternation after sibilants and `c`.
  `phono::spell_after_stem` implements the unstressed `o`→`e` rule; check whether
  it is reached on this path, since the ending arrives pre-spelled.

Acceptance: both slots improve measurably on the random noun sample; report
before and after.

## Fix 7 — `Policy::regularized()`

Decide and apply one: drop the method until phase 6 gives it content, or keep it
and make the no-op unmissable at the call site. The risk is a consumer blessing
its output into a test and depending on behaviour that will change. Prefer
dropping — an alias that silently means something else is what `LESSONS.md` E3
warns about.

## Fix 8 — extend the mutation table

The full scan surfaced six mutations the crate does not implement, listed in
`phono.rs` as known misses: `в → ∅` (давать/даю, 41), `ев → у` (бичевать/бичую,
19), `ев → ю` (блевать/блюю, 11), `им → емл` (внимать/внемлю, 5), `ер → р`
(тереть/тру, 3), `р → ер` (брать/беру, 2). Together they are ~81 verbs.

These interact with class conditioning — `ев → у` and `ев → ю` are both `-евать`
verbs distinguished by the lemma, not the class — so implement only what the
class determines and leave the rest to the lexicon, recording which is which.

## Gates and house rules

Unchanged, and all must still hold:

- `cargo test --workspace`, `cargo test --doc --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Zero third-party dependencies in `ruthenian-core`; `ruthenian-orthography` only.
- Phase 1's full-corpus guard still passes: 41 462 lines, 0 failures.
- **All 13 guards re-mutation-tested** after the changes, not only the ones you
  touched. Fixes 4, 5, 6 and 8 all alter guarded code.
- **`INVARIANTS.md` I1**: no sampling anywhere, in code, tests, tooling or docs.
  The check is in I1 and it is one grep.
- Assert nothing you have not executed. Every number in the report needs a
  command behind it — the reason this document exists is that several published
  figures did not.

## Report

State: the targeted and random tables side by side, before and after; the
226-code parse result; which stress bugs you found and what they cost; the
gen-pl/ins-sg movement; the 13 guards with each mutation outcome; and anything in
Part 1 that turned out to be wrong when you re-measured it.
