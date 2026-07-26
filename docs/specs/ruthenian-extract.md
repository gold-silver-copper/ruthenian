# Spec: `ruthenian-extract`

Phase 4. Depends on `ruthenian-lexicon`, `ruthenian-core`, `ruthenian-orthography`.

> **Scope change (2026-07-25): extraction is multi-language.** The dump is
> scanned in full for Russian (419 283 lemmas), Polish (152 325), Ukrainian
> (52 223), Belarusian (6 899) and Old Church Slavonic (4 311), plus borrowing
> etymologies. `INVARIANTS.md` I1 applies per language: a full scan each, never a
> sample. See `DIRECTION.md` and `docs/RUTHENIAN.md` §9.

## 1. Purpose

Read the English Wiktionary dump once and turn it into the lexicon artifact and
the generated PHF tables. Deterministically, with bounded memory, and with an
honest account of everything it threw away.

This is the only crate that knows Wiktionary exists. Every fact about
`head_templates`, `ru-conj`, tag vocabularies and template arguments is confined
here; nothing downstream should be able to tell where the data came from except
by reading `Provenance`.

### This crate reconstructs; it does not transliterate

The dump holds **source languages**. Ruthenian is in none of them, and no record
in 10 667 129 lines is a Ruthenian form. What this crate produces is therefore a
*reconstruction*, by the procedure `../RUTHENIAN.md` §12.2 specifies:

1. **Group cognates by etymon** — the reflexes of one Proto-Slavic form across
   Russian, Ukrainian, Belarusian, Polish and OCS make one entry.
2. **Derive the Ruthenian form** by regular sound correspondence from that
   etymon, using the cognates to resolve what any single language lost.
3. **Record the evidence** — which languages attest it, and how confidently the
   derivation follows.

Taking the Russian cognate and transliterating it is not this procedure and does
not produce Ruthenian. Russian has merged yat, levelled the second
palatalization, and lost the dual, the ablative and the aorist; a lemma derived
from it alone cannot supply the endings the language needs. That is the whole
argument for the multi-source scan, and it is why step 3 is not optional
bookkeeping — a form resting on one reflex is a weaker claim than one attested
across four, and `Provenance` must say which it is.

**This is the hardest unsolved problem in the project** (`../RUTHENIAN.md` §12.2,
`DIRECTION.md` open decisions). Explicit Proto-Slavic links cover 5 517 etyma,
only 88 of them attested across all five source languages, and 2 700 in just one.
Cognate grouping therefore cannot rely on etymology templates alone; it needs
phonological matching and the English gloss as a pivot. It is scoped as its own
phase and must not be assumed away.

### Source-language classifications stop here

Zaliznyak classes, accent letters and Russian stem classes are how a cognate is
read out of the dump and mapped onto one of Ruthenian's three declensions or six
conjugation classes (`../RUTHENIAN.md` §3.2, §7.3). That mapping is this crate's
job, and its output is a Ruthenian class. **No Zaliznyak index, accent pattern or
stem class appears in `Entry`, in `ruthenian-core`, or in anything the facade
returns** (`DIRECTION.md`, "Three structural decisions"; guarded in
`ruthenian-core` by `no_source_language_types`).

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

Yields: the **Zaliznyak class** from `ru-conj` arg `2` (`4c+p`, `4a+p`, `1a`),
which determines the Russian present stem; transitivity from the `class` form;
the attested 1sg/2sg present; participles and gerunds; and the stress position.

**What survives the boundary, and what does not.** These fields are read to
identify the cognate and to locate its stem and stress. They are then mapped:

| Read from the source | Becomes | Or is discarded |
|---|---|---|
| Zaliznyak class digit (1–16) | one of Ruthenian's six classes (§7.3 — his 1–6 map directly; 7–16 are regularized onto them) | |
| Zaliznyak accent letter (`a`, `b`, `c`, `c″`) | the **stress position** in the lemma | the mobility pattern — Ruthenian stress is fixed (§2.1) |
| Russian stem class (velar, sibilant, `c`, vowel) | declension + hardness (§3.2) | the eight-way split — these are spelling adjustments, not declensions (§3.8) |
| aspect, aspect partner | *nothing* | discarded entirely — aspect is derived from surface shape (§7.2) and no entry stores it |
| transitivity | kept — it conditions the passive participle gap | |

The aspect row is the one most likely to be implemented wrongly out of habit.
`ru-verb` supplies both an aspect and an aspect partner, and both are Russian
lexical facts that Ruthenian has abolished. Storing either would reintroduce
exactly the lexical pairing §7.2 exists to remove, and would then quietly
disagree with `Rules::aspect_of`.

### Gaps are grammar, not data

Measured over 2 941 Russian verbs: perfectives carry 13 922 gap slots against
imperfectives' 2 509, and the six present-tense slots each appear ~1 519 times,
matching the perfective count. Those `"-"` entries are grammar, not lexical
facts, and `ruthenian-core` derives the corresponding Ruthenian gaps from
`(aspect, transitivity, slot)`.

The extractor therefore records **no gaps at all**. Russian's lexical
defectiveness — `победить` carrying `futr_1sg: "-"` as an explicit `ru-conj`
argument — is a fact about Russian and does not transfer: it does not make the
corresponding Ruthenian verb defective, and Ruthenian's paradigms are specified as
regular and complete. If a Ruthenian lemma should ever carry a genuine lexical
gap, `../RUTHENIAN.md` must say so; there is no rule for inferring one from a
cognate.

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
    /// The cognate evidence each entry was reconstructed from: which source
    /// languages attested it, and in what form. Kept so the reconstruction can
    /// be audited and so `ruthenian-eval` can report distance — NOT as a set of
    /// expected outputs. There is no attested Ruthenian.
    pub evidence: Vec<(EntryKey, SourceLang, SourceForm)>,
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
| `evidence.tsv` | `(key, source_lang, source_form)` — the cognates each entry was reconstructed from. **Evidence, not ground truth**: it records what the reconstruction rests on, and is what `ruthenian-eval` measures *distance* against. It is never an expected-output set, because no line of it is Ruthenian. | `ruthenian-eval` |
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
9. **No source-language classification reaches an artifact.** No Zaliznyak index,
   accent letter or Russian stem class appears in `Entry`. They are read, mapped
   to a Ruthenian class, and dropped.
10. **No entry stores aspect or an aspect partner.** Both are derived (§7.2);
    storing either would create a second, divergent answer.
11. **Every entry carries its reconstruction evidence** — which source languages
    attested it, and how confidently the derivation follows. An entry that cannot
    say what it rests on is not admissible.

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
| `no_source_classification_in_entry` | Inv. 9 | Store the Zaliznyak string on `Entry`; the check on the artifact schema fails | required | ms | crate |
| `no_stored_aspect` | Inv. 10 | Add an `aspect` field and populate it from `ru-verb`; the check fails on the schema **and** a differential test against `Rules::aspect_of` diverges | required | seconds | crate |
| `entry_carries_evidence` | Inv. 11 | Emit an entry with an empty evidence set | required | ms | crate |

Seventeen guards. Two are deliberately not per-PR: `determinism_rerun` costs two
full passes, and `schema_drift_canary` needs the 22 GiB file, which CI will not
have. Both are marked and scheduled — a guard that pretends to run everywhere and
quietly does not is worse than an honestly scheduled one.

`no_stored_aspect` is paired the same way `morphophonology_single_owner` is: a
schema check alone would not catch an aspect value smuggled in under another
name, so it is backed by a differential test against the deriving function.

## 8. Out of scope

- Runtime lookup — this crate is never in the binary's hot path, and ideally not
  in the binary at all.
- Morphological rules → `ruthenian-core`.
- Conformance measurement → `ruthenian-eval`. This crate reports *yield* (how
  many entries, how many rejects, how well-evidenced); it never reports
  correctness, and it has no expected outputs to compare against.
- **Deciding the sound correspondences.** Which reflex Ruthenian takes for each
  Common Slavic divergence — pleophony, `*tj`/`*dj`, the nasals, the jers — is a
  language design question, answered in `../RUTHENIAN.md` and currently open
  (`PROMPT_SPEC_COMPLETION.md` Part 1 A). This crate *applies* the correspondence
  table; it must not invent one, and until the spec states it, reconstruction
  cannot be implemented.
- Other dumps, incremental updates.

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
- **Reconstruction reported by confidence**, not just by count: how many entries
  rest on cognates in four or more source languages, how many on one. The
  headline for the lexicon's quality is that distribution, since an entry derived
  from a single Russian reflex is the weakest thing this crate produces and the
  count of those is the number that must come down.
- Seventeen guards present, each demonstrated to fail under its witness.

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

- **Cognate grouping** (`../RUTHENIAN.md` §12.2) — the unsolved problem this
  crate cannot be finished without. Explicit Proto-Slavic links cover 5 517
  etyma, only 88 of them attested across all five source languages and 2 700 in
  just one, so etymology templates alone will not group the lexicon. Needs
  phonological matching plus English-gloss pivoting, and is scoped as its own
  phase.
- **The sound-correspondence table** that reconstruction applies. It is a
  language-design output, owed by `../RUTHENIAN.md`
  (`PROMPT_SPEC_COMPLETION.md` Part 1 A and P1), and this crate is blocked on it:
  choosing between a Russian, Ukrainian, Polish and OCS cognate *is* choosing a
  reflex, so without the table there is no defined answer for the extractor to
  produce.
