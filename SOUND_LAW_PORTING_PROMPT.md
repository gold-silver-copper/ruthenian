# Prompt: Expand `sound-law` from real sound-change suites and corpora

Work in the Ruthenian repository and substantially harden the new generic
`crates/sound-law` crate. Port as many applicable tests and engine features as
possible from Brassica, Lexurgy, the Mixtec Sound Change Database (MixteCaSo),
and the Searchable Index Diachronica. When a ported case exposes a bug or an
underspecified semantic in `sound-law`, first add a focused regression test,
then fix the engine and documentation. Continue systematically until every
upstream test/feature/data category has been classified and all feasible,
in-scope items have either been implemented or have a concrete documented
reason they were not.

This is an implementation task, not merely a research report. Make the changes,
run the checks, review the full diff, and leave the working tree in a verified
state. Do not commit, push, publish, open a PR, or mutate upstream repositories
unless the user separately asks for that.

## Repository context and constraints

- Read and follow the repository's `AGENTS.md` instructions before acting.
- The working tree is intentionally dirty: the new `crates/sound-law` crate and
  workspace membership may still be untracked/uncommitted. Preserve all current
  work and do not overwrite unrelated user changes.
- Backward compatibility and breaking semver are not concerns. Prefer a clean,
  coherent model over preserving a weak API, but update every in-repository
  caller, test, and document affected by a redesign.
- Keep `sound-law` generic: do not bake Ruthenian-specific segments or grammar
  into the engine.
- The crate is currently dependency-free. Preserve that property if reasonably
  possible. A dependency is acceptable only if it materially improves
  correctness or maintainability and the rationale, license, and alternatives
  are documented.
- Preserve `#![forbid(unsafe_code)]`.
- Treat Unicode deliberately. Never index strings at non-character boundaries,
  and explicitly distinguish Unicode scalar, grapheme, declared segment, word,
  morpheme, and syllable boundaries wherever their semantics differ.
- Current rule semantics are ordered rules with simultaneous, non-overlapping
  matches within each rule. Empty-target insertion examines each original
  Unicode-scalar boundary once. If imported suites reveal that more application
  modes are necessary, model them explicitly rather than silently changing the
  default.

## Upstream sources

Inspect and pin an exact commit or release for every source used. Record source
URL, version/commit, access date, license, imported paths, and whether each local
fixture is copied, adapted, independently re-authored, generated, or downloaded
at test time.

### 1. Brassica — primary portable conformance source

- Repository: https://github.com/bradrn/brassica
- Tests: https://github.com/bradrn/brassica/tree/master/test
- Examples: https://github.com/bradrn/brassica/tree/master/examples
- Package/license reference:
  https://hackage.haskell.org/package/brassica

Brassica is the first priority because it has a permissive BSD-3-Clause license,
golden tests, and natural-language examples. Inspect all tests, not only the
README examples. Its examples currently include Latin to Portuguese, Early
Middle English to Modern English, and Proto-Tai to Thai; confirm the pinned
version rather than assuming those paths remain unchanged.

Where license terms permit, adapt fixtures into compact Rust integration tests
with attribution and the required copyright/license notice. Prefer test data
that exercises the semantic engine independently of Brassica's parser or UI.
For behavior that `sound-law` intentionally models differently, document and
test the divergence instead of forcing false equivalence.

### 2. Lexurgy — broad feature taxonomy and differential oracle

- Repository: https://github.com/def-gthill/lexurgy
- Sound-change tests:
  https://github.com/def-gthill/lexurgy/tree/master/core/src/test/kotlin/com/meamoria/lexurgy/sc
- License: GPL-3.0; verify at the pinned revision.

Systematically inspect its sound-change test categories, including alternatives,
blocks, boundaries, diacritics, environments, features, filters, multi-word
behavior, negation, operators, partial runs, propagation, repeaters, reusable
definitions, robustness, romanization, rule names, sequenced rules,
syllabification, tracing, variables, whitespace, and timeouts.

Do **not** paste or lightly transliterate GPL source code or test text into this
MIT/Apache repository. Use Lexurgy as a behavioral reference and feature
inventory. Independently author minimal stimuli and expected outputs, or run a
pinned upstream Lexurgy build as an external differential oracle and record only
the independently generated cases/results needed to describe behavior. If legal
or provenance uncertainty remains, classify the item and do not vendor it.

### 3. MixteCaSo — scholarly natural-language validation

- Working repository: https://github.com/SAuderset/MixteCaSo
- Published dataset/article:
  https://openhumanitiesdata.metajnl.com/articles/10.5334/johd.184/
- Archived releases are on Zenodo; locate and pin the most appropriate release.

Use this as linguistic validation rather than assuming it is an executable,
ordered sound-change program. Inspect its TSV schema, IPA normalization,
segmental correspondences, conditioning environments, cognate evidence, and
tone module. Determine which entries can become:

- direct rule compilation/representability tests;
- minimal before/after witness tests;
- property tests over cognate or correspondence rows;
- end-to-end chains, only where the dataset actually supplies ordering and
  enough conditioning information.

Verify the exact release license. Published versions have used Creative Commons
terms, which may require attribution and ShareAlike handling. Prefer an optional
download/cache integration test or a separately licensed fixture directory over
silently vendoring the full dataset. Do not claim that a correspondence proves
an ordered rule chain when it does not.

### 4. Searchable Index Diachronica — breadth and expressiveness corpus

- Searchable site: https://chridd.nfshost.com/diachronica/
- Source/data repository: https://bitbucket.org/chridd/diachronica
- Unofficial CSV discussed by the maintainer:
  https://chridd.nfshost.com/files/diachronica-data-1.csv

Use this primarily to measure DSL expressiveness and parser robustness. The data
originated in a human-oriented document, has non-standard/intermediate formats,
and includes acknowledged conversion imperfections. Its provenance and license
must be confirmed before committing any raw or adapted rows. If permission is
unclear, keep downloads outside version control and commit only original test
logic, aggregate metrics, unsupported-category descriptions, and a reproducible
fetch/import procedure that does not run during normal offline tests.

For representable entries, generate minimal witness strings from the stated
target and environment and verify match/non-match behavior. Do not treat the
Index as a gold-standard linguistic correctness oracle.

## Required working method

### Phase 1: establish provenance and a feature matrix

1. Inspect the complete current `sound-law` implementation, tests, README, and
   workspace configuration.
2. Clone or download upstream sources into a temporary directory outside the
   repository, unless a small licensed fixture is intentionally being vendored.
3. Pin revisions and verify licenses before copying or adapting anything.
4. Create a maintained feature/test matrix under `crates/sound-law/` that lists
   every relevant upstream test file, example, and corpus category with one of:
   `ported`, `equivalent-existing`, `implemented-with-original-test`,
   `unsupported`, `not-applicable`, `license-blocked`, or `data-insufficient`.
5. For every non-ported item, record the precise missing engine capability or
   reason. Avoid vague buckets such as "too complex".

The matrix is a work queue, not a substitute for implementation. Start with a
baseline count and update it as work lands.

### Phase 2: build reusable conformance infrastructure

Create a data-driven test harness instead of hundreds of unrelated handwritten
test functions. A useful fixture record should capture at least:

- stable local case ID;
- source project and upstream test/data identifier;
- provenance/adaptation kind and license note;
- classes/segments and ordered rules;
- input and expected output;
- expected intermediate trace when relevant;
- application mode and boundary model;
- supported/unsupported feature tags.

Keep normal `cargo test` deterministic, offline, and fast. Large downloads,
upstream executable comparisons, or full-corpus runs should be separate tools or
ignored tests with explicit environment variables and helpful skip messages.
Pin every external oracle and make normalization steps reproducible.

Where practical, add a differential runner that executes the same shared-subset
case in upstream Brassica or Lexurgy and `sound-law`, then compares normalized
outputs. Do not make the standard workspace tests depend on Haskell, Java, the
network, or mutable upstream branches.

### Phase 3: port tests and implement missing engine features

Work in small vertical slices:

1. Select one upstream semantic category.
2. Add the smallest failing regression or conformance fixture.
3. Validate that the failure reflects current code, not a mistranslation of the
   upstream notation or a different documented semantic.
4. Fix the parser/compiler/matcher/API.
5. Add positive, negative, boundary, Unicode, interaction, and no-panic cases.
6. Update public documentation and the feature matrix.
7. Run targeted formatting, Clippy, and tests before moving on.

Prioritize core sound-change behavior roughly in this order, adjusting based on
what the upstream inventory actually contains:

1. Matching correctness: literal and multi-scalar segments, longest match,
   overlap selection, rule simultaneity, ordering, deletion, insertion,
   metathesis, captures, backreferences, anchors, and environments.
2. Pattern expressiveness: alternatives, groups, optional/repeated sequences,
   nested constructs, negative contexts, reusable definitions, class/category
   operations, and equality/agreement constraints.
3. Explicit application modes: once/global, iterative with cycle/limit safety,
   right-to-left, first/last occurrence, overlapping versus non-overlapping,
   sporadic rules with deterministic seeded testing, and rule/stage ranges.
4. Structural boundaries: word and multi-word input, morpheme seams, declared
   multi-character segments, syllables, stress/tone-bearing units, and
   cross-boundary controls.
5. Feature systems: phonological feature bundles, feature-based classes,
   feature mutation/copying, diacritics, suprasegmentals, propagation/harmony,
   and autosegmental behavior where a clean generic model is possible.
6. Program organization: named stages/blocks, filters, variables, reusable rule
   fragments, romanization/preprocessing stages, partial execution, tracing,
   diagnostics, and stable rule identities.
7. Robustness: malformed declarations, unknown names, duplicate captures,
   nullable/repeating patterns, empty classes/members, pathological
   backtracking, timeouts or explicit complexity limits, large Unicode input,
   and guaranteed absence of panics for validated programs.

Do not implement unrelated GUI, MDF dictionary, or paradigm-builder features
inside the core engine merely because an upstream application contains them.
File-format adapters may live in tools or test support when they materially help
corpus validation.

### Phase 4: natural-language and generated validation

For Brassica's licensed natural-language examples, establish pinned baseline
outputs using the upstream program, translate the shared semantics carefully,
and compare complete word lists where feasible. Separate these outcomes:

- exact match;
- expected divergence caused by a documented semantic difference;
- unrepresentable rule with a named missing feature;
- uncertain linguistic/data interpretation.

For MixteCaSo and Index Diachronica, report import metrics such as total rows,
parsed rows, representable rows, executable witness cases, unsupported rows by
feature, malformed/ambiguous rows, and rows excluded for provenance reasons.
Keep counts tied to a pinned dataset revision.

Supplement external cases with deterministic generated tests:

- enumerate small alphabets and short words for each rule form;
- verify unaffected material is preserved;
- verify output is valid UTF-8;
- verify no successful consuming match has a zero-width traversal bug;
- verify empty-target insertion visits each intended original boundary once;
- verify applying one simultaneous rule does not feed its own output back into
  that same rule;
- verify ordered later rules do see earlier output;
- verify iterative execution detects cycles and respects limits;
- compare optimized and debug behavior where overflow or indexing is relevant.

Avoid asserting linguistically false transformations merely to increase a test
count. Every natural-language expectation needs a source identifier and every
synthetic expectation needs a clear semantic purpose.

## Bug-fixing requirements

- Treat every confirmed mismatch in the intended shared semantic subset as a
  `sound-law` bug unless evidence shows the upstream oracle is wrong or the
  semantics are intentionally different.
- Never accept upstream behavior blindly. Reduce mismatches to minimal witnesses
  and inspect both implementations or specifications.
- Add a regression test before or with every bug fix.
- Prefer fixing the underlying matcher/compiler model over adding case-specific
  branches.
- Maintain compile-time validation where possible and return structured errors
  for invalid declarations. Validated programs must not panic on user input.
- If matching can become exponentially pathological, add a principled algorithm
  or explicit deterministic resource limit with a tested diagnostic; do not
  hide hangs with flaky wall-clock tests.

## Documentation and provenance deliverables

By completion, the crate should include:

- updated user-facing DSL/API documentation with examples for every implemented
  major feature;
- an upstream feature/conformance matrix with pinned revisions and disposition;
- fixture provenance and license notices sufficient to identify every imported
  or adapted case;
- instructions for optional differential/full-corpus runs;
- a concise limitations section naming genuinely unsupported semantics;
- coverage/import metrics for each source.

Do not use claims such as "supports Lexurgy" or "passes Index Diachronica" when
only a subset was tested. State exact shared subsets and counts.

## Verification and review gate

Run targeted checks continuously, then at minimum run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
RUSTDOCFLAGS='-D warnings' cargo doc -p sound-law --no-deps
cargo test -p sound-law --release
git diff --check
```

Run any new corpus importer, differential suite, generated/property suite, and
large ignored tests that are available in the environment. Report skipped tests
and exact prerequisites instead of implying they ran.

After implementation, perform a fresh, explicit review of the full intended
diff—including untracked files—for correctness, regressions, security/resource
risks, unsafe edge cases, licensing/provenance problems, and missing tests.
Validate findings against current code, fix all confirmed high-severity issues,
rerun affected checks, and repeat the review if a fix changes semantics.

## Completion criteria

Do not stop after a small representative sample. Completion requires:

- every relevant upstream test file/example/corpus category is inventoried;
- all feasible shared-semantic tests are ported or independently re-authored;
- all feasible high-value engine features exposed by those sources are
  implemented and documented;
- every remaining item has a specific disposition and rationale;
- every confirmed engine bug found during the work has a regression and fix;
- standard tests remain offline and deterministic;
- licensing and provenance are explicit and compatible with how fixtures are
  stored and distributed;
- all required checks pass, apart from clearly reported external or unrelated
  blockers.

In the final handoff, summarize implemented features, tests/cases by source,
bugs fixed, intentionally different semantics, unsupported/blocked categories,
license handling, verification commands/results, full-corpus or differential
results, and remaining risks. Link directly to the main local implementation,
matrix, fixture manifest, and documentation files.
