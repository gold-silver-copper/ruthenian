# PR #2 fixes — research and execution prompt

Work on `agent/core`, the branch behind [PR #2]. Every fix below lands in that
PR; do not open a new one.

Part 1 is the research the fixes rest on — measured, with the commands. Part 2 is
what to do about it. Read Part 1 first: three of the fixes exist because the
numbers currently published in the PR are wrong, and the reasons matter more than
the corrections.

[PR #2]: https://github.com/gold-silver-copper/ruthenian/pull/2

---

# Part 1 — Research

All measurements taken 2026-07-25 against
`~/Desktop/code/wikidata/raw-wiktextract-data.jsonl` (23 622 298 877 bytes,
10 667 129 lines) and the phase-2 crate at `8464fc7`.

Two sampling methods are used and the difference matters:

- **full pass** — `LC_ALL=C grep -oE` over the whole 22 GB. Exact.
- **windowed sample** — three `dd` windows (`skip=6000/14000/21000`,
  `count=500`). Russian records are scattered rather than sorted, so windows are
  representative, but they are estimates and are labelled as such.

## Finding 1 — the fixture is selection-biased, and the PR's accuracy table understates the crate

The paradigm fixture was built to *cover* things: one lemma per class, one per
mutation, plus hand-picked hard cases (`имя`, `дитя`, `время`, `мать`, `дочь`,
`победить`). That is the right design for a regression net and the wrong design
for an accuracy figure — it is a sample of the hard tail, not of the language.

Measured on a **random 120-lemma noun sample** (seed 7, no hand-picking, no class
targeting, 1 390 comparable cells):

| | PR #2 reports | Random sample |
|---|---:|---:|
| noun | 58.0 % | **79.1 %** |
| adj | 80.2 % | **93.1 %** ¹ |
| verb | 84.7 % | 87.7 % ¹ |

¹ adj/verb figures are from splitting the existing fixture by selection reason;
the noun figure is from the independent random sample.

Splitting the fixture by why each lemma was chosen:

| pos | selection | lemmas | rows | misses | ok |
|---|---|---:|---:|---:|---:|
| noun | hand-picked hard cases | 15 | 157 | 75 | 52.2 % |
| noun | class-coverage | 40 | 460 | 163 | 64.6 % |
| adj | hand-picked hard cases | 2 | 52 | 12 | 76.9 % |
| adj | class-coverage | 18 | 450 | 31 | **93.1 %** |
| verb | hand-picked hard cases | 11 | 257 | 32 | 87.5 % |
| verb | class-coverage | 65 | 1489 | 183 | 87.7 % |

**Caveat on that table**: the split is itself contaminated. The fixture builder
fills class buckets before named ones, so a hard case that also filled a class
bucket (`имя`, `дитя`) is counted as "class-coverage". That is why the noun
class-coverage figure (64.6 %) still sits well below the clean random sample
(79.1 %) — and why the random sample, not a re-slice of the fixture, is the
number to publish.

Verbs show almost no bias (87.5 vs 87.7), which is a useful control: it says the
bias is real and specific, not an artefact of the splitting.

## Finding 2 — the class-code parser fails on 9 real codes, and its test corpus is 8 % of reality

A full pass found **183 distinct `ru-conj` class codes over 13 468 verbs**. The
committed parser corpus (`tests/paradigms/class-codes.txt`) holds **117**, taken
from an 8 % window sample — it never saw 66 of them.

```bash
LC_ALL=C grep -oE '"name": "ru-conj", "args": \{"1": "[^"]*", "2": "[^"]*"' \
  raw-wiktextract-data.jsonl | sed 's/.*"2": "//' | sort | uniq -c | sort -rn
```

Against all 183, the parser rejects **9**, in three groups:

| Codes | Feature | Why it fails |
|---|---|---|
| `14b/c'+p`, `14c/c'+p`, `16b/c'`, `irreg/c'+p` | ASCII apostrophe `'` marking a softness distinction | the parser handles U+02B9 `ʹ` but not U+0027 `'` |
| `4b/c-nd` | a `-nd` stem suffix (cf. the `-bd` in `14c/c-bd`) | `n` is neither a digit nor `a`–`f`, so it hits the error arm |
| `6a1as13`, `6a1as13+p`, `6a1as14`, `6a1as14+p` | an `sNN` suffix | same |

The parser erroring rather than silently defaulting is correct and is why these
are visible at all. But 9 codes covering real verbs will abort extraction in
phase 4.

Class distribution, full dump — the top of a very long tail:

```
5060 1a      953 2a+p     693 4a+p    639 4b+p    547 1a+p    539 2a
 491 4b      483 4c+p     480 4a      356 4c      224 5b      211 3b
```

**Classes 1–6 are 12 205 / 13 468 = 90.6 %** of verbs carrying a class code.

## Finding 3 — the same coverage figure was published three times, three different ways, all wrong

| Where | Figure | How it was got |
|---|---|---|
| `lib.rs` (original) | ~76 % | top-22 codes only |
| PR #2 description | 73.2 % | top-22 codes only, recounted |
| `lib.rs` (corrected, `a05f5e5`) | 87.6 % | windowed sample, all codes |
| **True** | **90.6 %** | **full dump, all codes** |

The first two were wrong because they counted only the twenty-two most frequent
codes, so every class-1–6 *variant* outside that list went uncounted. The third
is right in method and low only because it used an 8 % sample.

This is the metric-drift failure `LESSONS.md` records from slovowiki — README,
machine summary and reports carrying different numbers — reproduced inside the PR
that cites the lesson. The structural remedy (generate every published number
from one canonical result) belongs to phase 6, but the immediate remedy is to
have one number, computed one way, stated once.

## Finding 4 — stress placement is buggy, not unmodelled

The PR attributes the low strict score (4.1 % verb, 10.4 % noun) to accent
patterns `d`–`f` being unmodelled. The data does not support that.

Accent-pattern distribution (windowed sample):

| Nouns | count | | Verbs | count |
|---|---:|---|---|---:|
| `accent-a` | 2434 | | `a` | 576 |
| `accent-b` | 174 | | `b` | 163 |
| `accent-c` | 27 | | `c` | 73 |
| `accent-d` | 41 | | `e` | 17 |
| `accent-e` | 15 | | none/irreg | 15 |
| `accent-f` | 2 | | | |
| **a+b modelled** | **2608 / 2693 = 96.8 %** | | **a+b+c modelled** | **812 / 844 = 96.2 %** |

So ~96 % of both nouns and verbs fall in patterns the crate *claims* to model,
and strict accuracy is 4–10 %. The unmodelled tail cannot account for that. The
placement logic inside the implemented patterns is wrong, and that is a bug to
fix rather than a coverage limit to document.

## Finding 5 — the noun weakness is two cells, not a systematic fault

From the random sample, by slot (only slots with ≥ 8 observations):

| Slot | segmental |
|---|---:|
| genitive plural | 64.7 % (77/119) |
| instrumental singular | 65.8 % (73/111) |
| accusative plural | 78.8 % (93/118) |
| dative singular | 78.4 % (87/111) |
| nominative plural | 79.2 % (95/120) |
| prepositional singular | 79.3 % (88/111) |
| genitive singular | 80.2 % (89/111) |
| accusative singular | 81.7 % (89/109) |

Genitive plural and instrumental singular stand out, and both are known-hard for
principled reasons: the genitive plural is a zero ending that triggers fleeting-
vowel insertion (`okon`, `sestjor`), and the instrumental singular carries the
`-om`/`-em` alternation after sibilants and `c`. Everything else clusters around
79–82 %, which is the general level rather than a defect.

Categorising the fixture's noun misses by shape: 136 wrong ending, 14 with no
accent pattern in the source at all, 13 where the ending is a prefix or extension
of the attested form.

## Finding 6 — adjective soft stems are 1.6 % of adjectives and are derivable

`adjective.rs` hardcodes `soft = false` and documents the resulting forms as
wrong. Measured over 1 090 sampled adjectives:

| Type | count | share |
|---|---:|---:|
| hard `-yj` | 818 | 75.0 % |
| velar/sibilant (hard endings, `i`-spelling) | 203 | 18.6 % |
| stressed `-oj` (hard) | 52 | 4.8 % |
| **true soft (`-nij` type)** | **17** | **1.6 %** |

Two consequences. The exposure is 1.6 %, not the open-ended problem the code
comment implies. And softness *is* derivable from the citation form — ends `-ij`
**and** the stem-final consonant is not velar or sibilant — which is exactly the
rule the research script used to produce this table. The velar/sibilant group is
correctly handled already: those take hard endings with the `y`→`i` spelling
rule, not soft endings.

## Finding 7 — the repository redistributes 560 KB of third-party data with no attribution file

| Artefact | Size | Source | Licence |
|---|---:|---|---|
| `crates/ruthenian-orthography/tests/corpus/sample.tsv` | 328 KB | Russian Synodal Bible, via `gold-silver-copper/ruthenian@49d3af7` | public domain (1876); provenance still required |
| `crates/ruthenian-core/tests/paradigms/fixture.tsv` | 216 KB | English Wiktionary via Wiktextract | **CC BY-SA 4.0 + GFDL** |
| `crates/ruthenian-core/tests/paradigms/fixture_meta.tsv` | 16 KB | same | same |

`docs/specs/ruthenian-extract.md` requires `ATTRIBUTION.md` "in the same commit
that first vendors extracted data". It does not exist. The obligation began in
phase 1 and doubled in phase 2.

Note the comparison with slovowiki, whose `ATTRIBUTION.md` states that the raw
dump "is **not** redistributed in this repository; it is read locally at build
time". This repository *does* redistribute dump-derived data, so its obligation
is strictly larger than the model it is copying.

---

# Part 2 — The fixes

Land all of these on `agent/core`. Order is by risk, not by size: the licence
and the parser first, the measurement rewrite next, the rule work last.

## Fix 1 — `ATTRIBUTION.md` (licence condition)

Write it at the repository root, on slovowiki's structure (`~/Desktop/code/slovowiki/ATTRIBUTION.md`):
a table separating **source code** (MIT OR Apache-2.0) from **vendored data**
from **generated content**, then a section per source.

Must state, per Finding 7: the corpus sample's provenance and public-domain
status; that the paradigm fixture is English Wiktionary content extracted with
Wiktextract, carrying **CC BY-SA 4.0 + GFDL**, requiring attribution and
share-alike from anyone redistributing it; that Wiktextract itself (Tatu Ylonen)
is MIT while the data it produces keeps Wiktionary's licence; and that the raw
dump is *not* redistributed here.

Link it from the root `README.md` and from both `tests/*/README.md` files.

## Fix 2 — class-code parser: three notation features, and the real corpus

Handle all three groups from Finding 2:

- **ASCII `'`** as well as `ʹ` (U+02B9). Both mark the same softness
  distinction; treat them identically rather than adding a second concept.
- **`-xx` stem suffixes** (`-nd`, `-bd`): a hyphen followed by Cyrillic or Latin
  letters, captured like the existing `+p` mutation rather than skipped.
- **`sNN` suffixes** (`s13`, `s14`): captured as a numbered variant.

Then **replace `tests/paradigms/class-codes.txt` with the full 183-code list**
from the full-dump command in Finding 2, and make `class_codes_parse` assert the
count is ≥ 183 so a shrunken corpus fails loudly. Keep the existing negative
cases: an unrecognized code must still error, and `irreg`/`-` must still parse to
their own variants.

Acceptance: all 183 parse; the guard still fails under its witness.

## Fix 3 — measure honestly: two samples, one number each

The single most important fix, because the PR currently understates the crate.

1. **Add a random held-out sample** at `tests/paradigms/random.tsv` — ~120
   lemmas per part of speech drawn with a fixed seed from the dump, with **no
   hand-picking and no class targeting**. Record the seed, the windows and the
   selection code in `tests/paradigms/README.md` so it is reproducible.
2. **Report both, and say what each is for.** The targeted fixture is a
   regression net over hard cases; the random sample is the accuracy figure.
   Never average them, and never quote the targeted one as accuracy.
3. **Keep the `RUTHENIAN_DUMP_MISSES` diagnostic** already added to
   `tests/fixture.rs` (currently uncommitted) — it is what made Finding 5
   possible.
4. **Correct every published figure** to the full-dump numbers: classes 1–6 are
   **90.6 %**, not 87.6 %; noun accent `a+b` is **96.8 %**, not "~93 %"; and the
   accuracy table in the PR description gets both columns.

Acceptance: every number in `lib.rs`, the crate README and the PR description
traces to one stated command, and no two of them disagree.

## Fix 4 — derive adjective softness

Replace the `soft = false` hardcode with the derivation from Finding 6: soft iff
the citation form ends `-ij` and the stem-final consonant is not velar or
sibilant. The function currently takes a bare stem, which cannot carry the
distinction — so take the citation form, or add an explicit `soft: bool` the
caller supplies. Prefer whichever keeps `adjective()` a pure function of its
arguments.

Delete the code comment admitting the forms are wrong; it will no longer be true.

Acceptance: `sinij` declines soft (`sinjego`), `russkij` stays hard
(`russkogo`, not `*russkjego`), `novyj` unchanged. Add all three as golden cases.

## Fix 5 — fix stress placement inside the modelled patterns

Per Finding 4, ~96 % of nouns and verbs are in patterns the crate claims to
model, so the 4–10 % strict score is placement bugs. Work from the
`RUTHENIAN_DUMP_MISSES` dump filtered to rows that differ **only** in stress —
that set is the work list, and it is already computable.

Do not extend to patterns `d`–`f` in this PR. They are 3–4 % and the honest
statement about them ("segmental form correct, stress not modelled, recorded in
the trace") is already true.

Acceptance: strict scores rise substantially on both samples and are reported;
`stress_placed` still fails under its witness.

## Fix 6 — genitive plural and instrumental singular

The two weak cells from Finding 5.

- **Genitive plural**: the zero ending with fleeting-vowel insertion
  (`okno` → `okon`, `sestra` → `sestjor`). `NounClass::reducible` already carries
  the `*` marker from the source and is currently unused.
- **Instrumental singular**: the `-om`/`-em` alternation after sibilants and `c`.
  `phono::spell_after_stem` already implements the unstressed `o`→`e` rule; check
  whether it is being reached on this path, since the ending arrives pre-spelled.

Acceptance: both slots improve measurably on the random sample; report before and
after.

## Fix 7 — `Policy::regularized()`

Decide and apply one: drop the method until phase 6 gives it content, or keep it
and make the no-op unmissable at the call site. The risk is a consumer blessing
its output into a test and depending on behaviour that will change. Prefer
dropping — an alias that silently means something else is exactly what
`LESSONS.md` E3 warns about.

## Gates and house rules

Unchanged from phase 2, and all must still hold:

- `cargo test --workspace`, `cargo test --doc --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Zero third-party dependencies in `ruthenian-core`; `ruthenian-orthography` only.
- Phase 1's full-corpus guard still passes: 41 462 lines, 0 failures.
- **All 13 guards re-mutation-tested after the changes**, not just the ones you
  touched. Fix 4 and Fix 5 both alter code that guards watch.
- Assert nothing you have not executed. Every number in the report needs a
  command behind it — the whole reason this document exists is that three
  published figures did not.

## Report

State: the two sample tables side by side, before and after; the 183-code parse
result; which stress bugs you found and what they cost; the gen-pl/ins-sg
movement; the 13 guards with each mutation outcome; and anything in Part 1 that
turned out to be wrong when you re-measured it.
