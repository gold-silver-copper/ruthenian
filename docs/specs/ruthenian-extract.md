# Spec: `ruthenian-extract`

Phase 4. Depends on `ruthenian-lexicon`, `ruthenian-core`, `ruthenian-orthography`.

> **Scope change (2026-07-25): extraction is multi-language.** The dump is
> scanned in full for Russian (419 283 lemmas), Polish (152 325), Ukrainian
> (52 223), Belarusian (6 899) and Old Church Slavonic (4 311), plus borrowing
> etymologies. `INVARIANTS.md` I1 applies per language: a full scan each, never a
> sample. See `DIRECTION.md` and `docs/RUTHENIAN.md` §9.

## 1. Purpose

Read the English Wiktionary dump once and turn it into the lexicon artifact, the
attested-forms artifact, and the generated PHF tables. Deterministically, with
bounded memory, and with an honest account of everything it threw away.

This is the only crate that knows Wiktionary exists. Every fact about
`head_templates`, `ru-conj`, tag vocabularies and template arguments is confined
here; nothing downstream should be able to tell where the data came from except
by reading `Provenance`.

Wrong to put here: morphology (it is in `core`, and this crate *calls* it as the
predictor), the entry schema (it is in `lexicon`), or any runtime behaviour. This
crate runs offline, on a developer's machine, on demand.

## 2. The source

`~/Desktop/code/wikidata/raw-wiktextract-data.jsonl`, verified 2026-07-25:

- **23 622 298 877 bytes (22.0 GiB)**, **10 667 129 lines**, one JSON object per
  line (`wc -l`, independently matching slovowiki's recorded `lines_scanned`).
- Record keys observed: `word`, `pos`, `lang`, `lang_code`, `senses`, `forms`,
  `head_templates`, `inflection_templates`, `etymology_text`,
  `etymology_templates`, `sounds`.
- The field is emitted **with a space after the colon**: `"lang_code": "ru"`. A
  fixed-string scan for that literal is the fast path.

Single-pass `grep -o -F` counts over the whole file:

| Selector | Count | Meaning |
|---|---:|---|
| `"lang_code": "ru"` | 605 446 | Russian records — an **upper bound**. The key also occurs in nested structures, and most Russian records are inflected-form pages rather than lemmas. Re-verify per record; do not report this as a lemma count. |
| `"name": "ru-noun+"` | 28 261 | Noun lemmas with a declension table |
| `"name": "ru-noun"` | 1 530 | Legacy/manual noun headwords |
| `"name": "ru-conj"` | 13 473 | Verb lemmas with a conjugation table |
| `"name": "ru-verb"` | 13 232 | Verb headwords |
| `"name": "ru-adj"` | 10 011 | Adjective lemmas |

Expected extractable inventory: **~30 k nouns, ~13.5 k verbs, ~10 k adjectives**,
plus adverbs, pronouns and numerals. These are the numbers Phase 4 must either
reproduce or explain.

### Verified record shapes

**Noun** (`корюшка`, `вафельница` — real records):

```json
"head_templates": [{"name": "ru-noun+", "args": {"1": "ко́рюшка", "2": "*", "a": "an", "adj": "ко́рюшковый"}}]
"forms": [
  {"form": "ко́рюшка", "tags": ["animate", "canonical", "feminine"]},
  {"form": "velar-stem", "source": "declension", "tags": ["class"]},
  {"form": "accent-a",   "source": "declension", "tags": ["class"]},
  {"form": "ко́рюшки",   "tags": ["nominative", "plural"], "source": "declension", "roman": "kórjuški"}
]
```

Yields: gender and animacy from the canonical form's tags; stem class and accent
pattern from the `class` forms (`velar-stem`, `ц-stem`, `accent-a`); the
reducible-stem marker from `ru-noun+` arg `2` (`*`); the relational adjective from
arg `adj`; the full case × number paradigm with stress.

**Verb** (`недоплатить`, `недоплачивать`, `ебашить` — real records):

```json
"head_templates":       [{"name": "ru-verb", "args": {"1": "недоплати́ть", "2": "pf", "impf": "недопла́чивать"}}]
"inflection_templates": [{"name": "ru-conj", "args": {"1": "pf", "2": "4c+p", "3": "недоплати́ть"}}]
"forms": [
  {"form": "4c perfective transitive", "source": "conjugation", "tags": ["class"]},
  {"form": "-",              "tags": ["first-person", "present", "singular"], "source": "conjugation"},
  {"form": "недоплачу́",     "tags": ["first-person", "future", "singular"],  "source": "conjugation"},
  {"form": "недопла́ченный", "tags": ["participle", "passive", "past"],       "source": "conjugation"}
]
```

Yields: the **Zaliznyak class** from `ru-conj` arg `2` (`4c+p`, `4a+p`, `1a`) —
the single field that determines the present stem; aspect and aspect partner from
the `ru-verb` args; transitivity from the `class` form; the attested 1sg/2sg
present, which are the **principal parts**; participles and gerunds; and the
**paradigm gaps, marked `"-"`**, which are the exact input to
`ruthenian-core`'s gap handling — but see the correction below.

### Correction (phase 2): most `"-"` slots are structural, not defects

Measured over 2 941 verbs: perfectives carry 13 922 gap slots against
imperfectives' 2 509, and the six present-tense slots each appear ~1 519 times,
matching the perfective count. A perfective verb has no present tense, so those
`"-"` entries are grammar and `ruthenian-core` derives them from
`(aspect, transitivity, slot)`. The extractor must **not** record them as
lexical facts.

What the extractor *must* capture is the small set the source marks explicitly:
`победить` carries `futr_1sg: "-"` as a `ru-conj` **argument**, not merely as an
absent form. Those overrides are the lexical gaps, they belong in the lexicon,
and they are what `gap.fill-defective-1sg` targets.

### Two traps, both verified

1. **Wiktionary's `roman` field is not Ruthenian.** It is Wiktionary's own
   romanization scheme (`kórjuška`, `nedopláčivatʹ`, `jebášitʹ`). Passing it
   through as Ruthenian would silently corrupt the lexicon with a second,
   incompatible orthography. All romanization goes through
   `ruthenian-orthography` and nowhere else. A guard enforces this.
2. **Stress marks are everywhere** — combining U+0301 on every form
   (`недоплати́ть`). Verified that the reference orthography carries them through
   cleanly (`писа́ть` → `pisátj`), so they are free information. Normalize
   Unicode once, at the boundary, and record which normal form.

## 3. Public API sketch

```rust
/// Pure: dump path in, plan out. Writes nothing.
pub fn plan(dump: &Path, opts: &ExtractOpts) -> Result<BuildPlan, ExtractError>;

/// The only function in the crate that touches the filesystem.
pub fn write(plan: &BuildPlan, out: &Path) -> Result<Written, ExtractError>;

pub struct BuildPlan {
    pub entries: Vec<Entry>,
    pub attested: Vec<(EntryKey, Slot, Ruthenian)>,
    pub senses: SenseBlob,         // the `senses.rdb` payload, built not written
    pub tables: TableSet,          // only forms the predictor does NOT produce
    pub rejects: RejectHistogram,
    pub fingerprint: DumpFingerprint,   // size, line count, content hash
}

pub struct RejectHistogram { /* counted by reason, never a bare total */ }
```

The `plan`/`write` split is law 6. Everything testable is in `plan`; `write` is
thin enough to read in one sitting. It is also what makes a dry run free.

## 3a. Inputs and outputs

**In:** the dump path (§2), and nothing else — no network, no config file, no
environment. The vendored CI fixture (§10) is the same format, read the same way.

**Out:** four artifacts, written only by `write`:

| Artifact | Contents | Consumed by |
|---|---|---|
| `lexicon.jsonl` | one `Entry` per line, sorted by key — inflection data only | `ruthenian`, humans |
| `attested.tsv` | `(key, slot, form)` ground truth | `ruthenian-eval` |
| `senses.rdb` | the binary sense blob with its sorted key index | `ruthenian-cli` via `include_bytes!` |
| `crates/ruthenian/generated/*.rs` | PHF tables — the residue the rules cannot predict | `ruthenian` |

Plus the reject histogram and the dump fingerprint, emitted to stdout and
embedded in each artifact's header.

## 4. Data owned

- All knowledge of the dump's schema and template vocabulary.
- The reject histogram and its reason taxonomy.
- The dump fingerprint.

## 5. Dependencies allowed

`ruthenian-lexicon`, `ruthenian-core`, `ruthenian-orthography`, plus `serde`,
`serde_json`, and a streaming reader. No async runtime. No parallelism in v1
unless a measurement shows the single-threaded pass is too slow — and then the
measurement goes in the commit message.

## 6. Invariants

1. **Bounded memory.** The dump is never loaded. Anything accumulated across the
   pass is explicitly capped, and the cap is logged when hit.
2. **One pass produces everything** — entries, attested forms, tables, rejects.
3. **Deterministic**: same dump → byte-identical artifacts, verified by
   re-running and diffing.
4. **Lossless on the unpredictable.** Every metadata field in §2 that the rules
   cannot recompute survives into the lexicon.
5. **Tables hold only the residue**: for every table row, the predictor's output
   differs from the attested form. This is the mechanical enforcement of law 2.
6. **Every rejected record is counted by reason.** A kept-count without a
   reject-histogram is not a result.
7. Lemma records and inflected-form pages are distinguished; the `form-of` /
   `inflection-of` sense tags are the signal.
8. No Wiktionary romanization reaches an artifact.

## 7. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `bounded_memory` | Inv. 1 | Collect all records into a `Vec` before processing; the RSS ceiling test fails | required | minutes | crate |
| `determinism_rerun` | Inv. 3 | Iterate a `HashMap` when emitting entries | required | 2× full run, **scheduled** not per-PR | crate |
| `tables_are_residue` | Inv. 5 | Emit a table row whose value equals the predictor's output | required | seconds (on artifacts) | crate |
| `reject_histogram_totals` | Inv. 6 | Drop a record on a path that does not increment a counter; kept + rejected ≠ scanned | required | ms | crate |
| `no_wiktionary_roman` | Inv. 8 | Write `form.roman` into an entry; the check finds `ʹ`/`š`/`č` — characters our alphabet cannot produce | required | seconds | crate |
| `lemma_vs_form_page` | Inv. 7 | Accept a `form-of` sense as a lemma; the noun count jumps past 28 261 | required | seconds | crate |
| `fingerprint_recorded` | §3 | Write artifacts without the dump fingerprint | required | ms | crate |
| `yo_stress_transferred` | §10 — ё→е carries the implicit stress | Replace ё with bare е; `клёв` yields a stress-free `клев` and the check fails | required | ms | crate |
| `keys_assigned_globally` | §10 — `Disambiguator::None` only for genuinely unique (lemma, pos) | Assign keys per record; a homograph pair both claim the bare key | required | seconds | crate |
| `schema_drift_canary` | The template vocabulary still matches reality | Run against the dump and assert the six §2 counts within a stated tolerance; a Wiktionary schema change breaks it loudly instead of silently halving the lexicon | **diagnostic** (needs the dump) | minutes | crate |
| `fixture_extracts_correctly` | The vendored real-record fixture produces the expected entries | Break any parsing path; the fixture's pinned entries diff. Runs per-PR with no dump present | required | ms | crate |
| `senses_verbatim` | Gloss text reaches the artifact untouched | Transliterate or trim a gloss; the fixture's pinned English text diffs | required | ms | crate |
| `sense_index_sorted` | `senses.rdb`'s key index uses the lexicon's key ordering | Emit the index in insertion order; a `SenseIndex::get` round-trip over every fixture key fails to find some of them | required | ms | crate |
| `plan_writes_nothing` | Law 6 | Open a file inside `plan` | required | ms | crate |

Nine guards. Two are deliberately not per-PR: `determinism_rerun` costs two full
passes, and `schema_drift_canary` needs the 22 GiB file, which CI will not have.
Both are marked and scheduled — a guard that pretends to run everywhere and
quietly does not is worse than an honestly scheduled one.

## 8. Out of scope

- Runtime lookup — this crate is never in the binary's hot path, and ideally not
  in the binary at all.
- Morphological rules → `ruthenian-core`.
- Accuracy measurement → `ruthenian-eval`. This crate reports *coverage* (how
  many entries, how many rejects); it never reports correctness.
- Other languages, other dumps, incremental updates.

## 9. Done criteria

- Full run completes with memory bounded and wall-clock recorded.
- Reported: entries by part of speech, forms extracted, reject histogram by
  reason, and reconciliation against the §2 counts — every gap between "28 261
  `ru-noun+` templates" and "N noun entries" explained by a named reject reason.
- Table size reported both raw and as residue, with the compression ratio: the
  headline for this phase is *how much the rules predicted*.
- `ATTRIBUTION.md` written **in the same commit** that first vendors extracted
  data: Wiktionary content is CC BY-SA 4.0 + GFDL, attribution and ShareAlike
  required, generated forms labelled machine-generated.
- `docs/CORPUS.md` records the dump revision, the counts, and the normalization
  decisions (Unicode form, `ё` handling).
- Nine guards present, each demonstrated to fail under its witness.

## 10. Closed decisions

### ё → е, with the stress transferred

**Ruthenian normalizes ё to е at extraction.** These words are therefore spelled
`je`, not `jo`, and no lexicon entry contains the `jo` digraph.

This has one consequence that must not be missed, and it is a data-loss bug if it
is: **the dump almost never marks stress on ё.** Measured over the whole dump,
79 803 ё-bearing forms carry 80 064 occurrences of ё, of which only **179
(0.22 %)** carry a U+0301 — and every one of those is a reduplicated intensive
(`чё́рный-пречё́рный`, `жёлтый-прежёлтый`), where the mark disambiguates which
half of the compound is primary. Everywhere else ё is inherently stressed and
Wiktionary does not mark it.

A naive `ё → е` replacement therefore produces `клев` from `клёв`, with no stress
information at all and no way to recover it. So the normalization is not a
character substitution:

```text
ё  →  е + U+0301        (transfer the implicit stress to an explicit mark)
ё́  →  е + U+0301        (already marked: keep the one mark, never add a second)
```

giving `клёв` → `кле́в` → `kljév`. Since Ruthenian stores stress (Phase 1
decision), this keeps the lexicon strictly more informative than the source.

The 179 pre-marked cases are the exception the rule must handle rather than trip
over; they were invisible to an earlier window sample, which is why
`INVARIANTS.md` I1 forbids sampling. Where a form carries more than one stress
mark after normalization, count it in a `yo_multiple_stress` reject-histogram
bucket for inspection rather than guessing.

### Multi-sense lemmas

Resolved by the Phase 3 composite-key decision: one page with several paradigms
becomes several entries, distinguished by the class facts in their keys. Senses
that share a paradigm collapse to one entry. Key assignment is a **whole-lexicon
pass**, not per-record, because `Disambiguator::None` is only valid when nothing
else shares the (lemma, pos) — build for that from the start.

### A real-record fixture is vendored for CI

**Yes — a few hundred genuine dump records, committed under `tests/fixture/`.**
This is what makes most of §7's guards runnable per-PR instead of only on a
machine holding the 22 GiB file.

Non-negotiable properties, because a fixture that drifts is worse than none:

- **Real records, copied byte-for-byte from the dump.** Never hand-written, never
  "simplified for the test". A hand-made fixture encodes what we *think* the
  schema is, which is precisely the thing the guard is supposed to check.
- **Chosen to cover the hard cases, not the easy ones**: a defective verb with
  `"-"` slots, a reducible-stem noun (`ru-noun+` arg `2` = `*`), an aspect pair,
  a ё-bearing lemma, a homograph pair, a `ц-stem` and a `velar-stem` noun, an
  irregular Zaliznyak class (`6°b`), and an inflected-form page that must be
  rejected.
- **Provenance recorded** in `tests/fixture/README.md`: dump fingerprint, the
  page title and line offset of each record, and the reason it was chosen.
- **Refreshed with the dump.** When `refresh-data` runs against a newer dump, the
  fixture is re-pulled and any change to it is reviewed as a schema change.

### Senses are captured in full

Following the Phase 3 decision, extraction keeps the whole structured sense list
per lemma — gloss, tags, topics — in source order, and emits it as **`senses.rdb`**,
the third artifact: a binary blob with a sorted key index, embeddable with
`include_bytes!` or readable at runtime (format in `ruthenian-lexicon.md` §2a).

Three extraction-side consequences:

- `form-of` / `inflection-of` senses remain the signal for rejecting an
  inflected-form page, and are never stored as senses of a lemma;
- sense text is English prose that must reach the artifact **verbatim** — never
  transliterated, trimmed, or normalized;
- the blob's key index must be sorted by the same key ordering
  `ruthenian-lexicon` defines, or `SenseIndex::get`'s binary search silently
  returns wrong answers. This is worth its own guard rather than a comment.

## 11. Open questions

None. Every question this spec opened is closed above.
