# Prompt: write the Ruthenian direction document and the per-crate specs

> **Status: executed 2026-07-25.** Produced `DIRECTION.md` and the eight specs in
> `docs/specs/`. `PROMPT.md`, referenced below, was absorbed and deleted as
> instructed — its verified content now lives in
> `docs/specs/ruthenian-orthography.md` (the mapping table and the audited
> defects) and `docs/specs/ruthenian-extract.md` (the dump counts and record
> schemas). This file is kept as the record of what was asked for.

Your task is **documentation, not code**. Produce the two-level plan that every
later phase will be executed against:

1. `DIRECTION.md` — one general direction document for the whole project: what
   Ruthenian is, the crate map, and the laws that hold across all crates.
2. `docs/specs/<crate>.md` — one detailed spec per crate, all of them written
   now, before any implementation.

Write no Rust in this task beyond illustrative type sketches inside the specs. Do
not create crates, `Cargo.toml`s, or a git repo. Do not implement Phase 1 — its
execution prompt is a separate, later artifact.

## Read first

- `LESSONS.md` (this directory) — the cross-project research pass. It is the
  evidence base; the direction document must be consistent with it, and where you
  disagree with a lesson, say so explicitly and give your reason.
- `PROMPT.md` (this directory) — the current scoped prompt. **It contains
  verified facts that must survive into the new documents**: the measured dump
  counts, the noun/verb record schemas, the orthography table, and the seven
  audited defects of the reference implementation. Re-verify anything you carry
  over, then retire `PROMPT.md` (delete it; `DIRECTION.md` plus the specs
  replace it).
- The source repos, for shape rather than content: `~/Desktop/code/english`
  (crate layering, accuracy discipline), `~/Desktop/code/interslavic-rs`
  (`INTEGRATION.md`, `CHANGELOG.md` policy header), `~/Desktop/code/slovowiki`
  (`docs/GUARD-REGISTRY.md`, `docs/ARCHITECTURE-ACCURACY-REVIEW.md`),
  `~/Desktop/code/rogue-rs/mrzavec` (`GLOSSARY.md` trust legend).

## What Ruthenian is

Restate this in `DIRECTION.md` in your own words, and treat it as fixed scope:

- A **Latin-script, lightly regularized Russian**. Russian grammar and phonology,
  written in the Ruthenian alphabet, with a declared set of departures from
  standard Russian.
- **CLI only.** No website, no server, no static-site generation.
- **No Interslavic data**, ever. Interslavic is a stylistic reference point (a
  language may be more regular than its source material) and an architectural
  one. Its dictionary does not enter this repository.
- **One vocabulary source**: the English Wiktionary dump at
  `~/Desktop/code/wikidata/raw-wiktextract-data.jsonl`.

## The crate map

Each phase is one crate. Below is the proposed decomposition — adopt it, or
change it and justify the change in `DIRECTION.md` against the lessons.

| Phase | Crate | Goal |
|---|---|---|
| 1 | `ruthenian-orthography` | Bijective Cyrillic↔Latin. Dependency-free. The only place a script conversion exists. |
| 2 | `ruthenian-core` | Productive Russian morphology as pure rules. Dependency-free, no data files. Doubles as the OOV fallback and as the extractor's predictor. |
| 3 | `ruthenian-lexicon` | The lexical data model: entry types, Zaliznyak class, principal parts, provenance, policy identifiers. Dependency-free schema shared by extractor, facade, and eval — the single canonical data owner. |
| 4 | `ruthenian-extract` | Streaming dump → lexicon. Pure build plan separated from filesystem writing. Emits the reject histogram and the dump fingerprint. |
| 5 | `ruthenian` | The facade: generated tables + rules behind one query path, every answer carrying provenance. |
| 6 | `ruthenian-eval` | Measurement: sealed test set, slot metric, paired diffs, and the one canonical summary every reported number is generated from. |
| 7 | `ruthenian-cli` | The `ruth` binary. A thin adapter over the facade — no morphology of its own. |
| 8 | `xtask` | `refresh-data`, `check-registry`, `accuracy`. Orchestration only. |

Two decisions to make explicitly in `DIRECTION.md`, with reasoning:

- **Where the regularization policy lives.** The recommended answer is *not* a
  separate crate: a `ruthenian-standard` that re-generates forms would recreate
  slovowiki's Finding 1 (two divergent generation pipelines producing the same
  data). Instead, `Policy` is a typed parameter on one generation path, each
  departure carries a `RuleId`, and `docs/REGULARIZATION.md` is generated from
  the rule table rather than hand-maintained. If you disagree, argue it.
- **Whether `ruthenian-lexicon` earns its place** as a crate rather than a module
  of the facade. The case for it: the extractor and the eval both need the types
  without depending on the generated tables.

## Required structure: `DIRECTION.md`

1. **What Ruthenian is** — the scope above, plus what it is explicitly not.
2. **The shape of the system** — one diagram, in the style of slovowiki's target
   architecture: sources → extract → lexicon → one engine → {CLI, eval}. Show
   that the CLI and the eval consume the *same* result type.
3. **The crate map** — the table above, expanded: for each crate, its goal in one
   sentence, what it owns, what it may depend on, and its one-line done-criterion.
4. **The laws** — cross-cutting rules every crate obeys, each traceable to a
   lesson. At minimum:
   - one generation path; the CLI and eval are adapters (A1);
   - rules predict, tables store only the residue, and changing a rule requires
     regeneration (A2);
   - typed until the last moment; one stringification point (A3);
   - no droppable side channels — losing data is a type error (A4);
   - one builder per concept (A5);
   - derived state over hand-maintained state (A6);
   - pure build logic separated from IO (A7);
   - provenance and policy on every form; `None` means "no such form exists" (B1, B2);
   - one canonical owner per artifact; docs and metrics are generated from it (B3, C3);
   - sourced vs policy claims labelled (B4);
   - every guard has a failure witness and an owner (D1);
   - structure, not strings, at every boundary (E1).
5. **The phase order and why it is a dependency order** — what each phase unlocks,
   and what "green" means at each boundary.
6. **The stability contract** — what is deterministic, what is immutable, what may
   change between regenerations, and how a change is announced (E3, E6).
7. **Open decisions** — the things deliberately not yet decided, each with the
   options and the point in the schedule where it must be closed. Stress marking,
   homograph keying, and which regularizations ship on by default all belong here
   unless you close them now.

Keep it a direction document: no implementation detail that belongs in a spec, no
task lists, no code.

## Required structure: `docs/specs/<crate>.md`

Every spec uses the same sections, in this order:

1. **Purpose** — one paragraph. What this crate is for, and what would be
   *wrong* to put in it.
2. **Public API sketch** — the types and function signatures that define the
   contract. Sketches, not implementations. Name the invariants each type
   enforces (what does its existence make impossible?).
3. **Inputs and outputs** — precisely what it consumes and produces, including
   file artifacts and their formats.
4. **Data owned** — what this crate is the single canonical owner of. If the
   answer is "nothing", say so.
5. **Dependencies allowed** — the explicit list. `ruthenian-orthography`,
   `ruthenian-core`, and `ruthenian-lexicon` allow zero non-dev dependencies;
   state that as a gate, not an aspiration.
6. **Invariants** — the properties that must always hold, phrased so a test can
   check them.
7. **Guards** — a table with the registry fields from slovowiki's methodology:
   name, invariant, **failure witness** (the minimal mutation that must make it
   fail), required-vs-diagnostic, cost, owner. A guard you cannot write a failure
   witness for does not go in the table.
8. **Out of scope** — what this crate must never grow into, and which crate owns
   that instead.
9. **Done criteria** — the checkable list that closes the phase, with the numbers
   that must be reported.
10. **Open questions** — decisions this spec defers, and who/what closes them.

Specs are allowed to be uneven in length: `ruthenian-orthography` and
`ruthenian-core` carry the most detail, `xtask` the least.

### Facts each spec must carry forward

- **`ruthenian-orthography`**: the full mapping table from `PROMPT.md`; the seven
  audited defects with their failing inputs (Ийон → `Ijon` → Иён; the reference
  ships 3 failures in 41 462 lines); the by-construction round-trip design
  (greedy longest-match reader as the single source of truth for how Ruthenian is
  *read*, writer inserting the separator only where re-reading would diverge);
  the case layer; the stress-mark decision.
- **`ruthenian-core`**: the two-stem problem and **principal parts** as the
  organizing concept — a verb's present stem is often not derivable from its
  infinitive (iotation `писа́ть → пишу́`, epenthetic `-л-` `люби́ть → люблю́`,
  stress shift, suppletion, defective paradigms), which is exactly what the
  Zaliznyak class encodes; the rule engine's dual role as OOV fallback and
  extractor predictor.
- **`ruthenian-lexicon`**: gender, animacy, stem class, accent pattern, reducible
  stem, aspect and aspect partner, transitivity, Zaliznyak class, principal
  parts, paradigm gaps, provenance, policy `RuleId`.
- **`ruthenian-extract`**: the measured dump counts (22 GB, 10 667 129 lines;
  `ru-noun+` 28 261, `ru-conj` 13 473, `ru-adj` 10 011, `ru-verb` 13 232,
  `ru-noun` 1 530; the `"lang_code": "ru"` selector's 605 446 hits as an upper
  bound only); the record schemas for nouns and verbs; bounded-memory streaming;
  the reject histogram; **the trap that Wiktionary's `roman` field is not
  Ruthenian**.
- **`ruthenian-eval`**: per-slot rather than per-form scoring and why; the sealed
  test set; separating mismatches explained by a declared regularization rule
  from unexplained ones; the canonical summary that README numbers are generated
  from.
- **`ruthenian-cli`**: the subcommand surface, `--json` everywhere, `--policy`,
  `--show-deviations`, labelled guesses for unknown lemmas.

## Deliverables

```text
ruthenian/
  DIRECTION.md
  LESSONS.md            # exists; leave it, cite it
  docs/specs/ruthenian-orthography.md
  docs/specs/ruthenian-core.md
  docs/specs/ruthenian-lexicon.md
  docs/specs/ruthenian-extract.md
  docs/specs/ruthenian.md
  docs/specs/ruthenian-eval.md
  docs/specs/ruthenian-cli.md
  docs/specs/xtask.md
  (PROMPT.md deleted, its verified content absorbed)
```

## Acceptance

- Every crate in the map has a spec; every spec has all ten sections; no section
  is a placeholder.
- Every guard in every spec has a failure witness. Count them and report the
  total.
- Every claim carried over from `PROMPT.md` has been re-verified against the dump
  or the reference repo — say which ones you checked and how.
- The direction document and the specs do not contradict each other. Where a
  spec makes a choice the direction document lists as open, the direction
  document is updated to say it is closed and where.
- No implementation, no crates, no `Cargo.toml`, no `git init`.

When you are done, report: the crate map as finally decided, every departure you
made from the proposed decomposition with its reason, the open decisions still
outstanding, and which spec you consider the riskiest to implement.
