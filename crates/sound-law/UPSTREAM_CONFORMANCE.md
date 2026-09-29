# Upstream conformance inventory

This inventory is both the provenance record for the upstream audit and the
work queue for `sound-law`. Statuses describe only the explicitly named subset;
they are not claims of compatibility with an upstream language or application.

Audit date: 2026-08-04.

## Pinned sources and license decisions

| Source | Pinned revision | License at that revision | Local use |
|---|---|---|---|
| [Brassica](https://github.com/bradrn/brassica) | `f3a92ad0eb5a890fdf92208c48325371e5e770ff` (2025-10-06) | BSD-3-Clause; copyright Brad Neimann, 2020-2024 | Behavior inspected. The complete Latin-to-Portuguese example is adapted with the full notice; other shared-subset stimuli are original. |
| [Lexurgy](https://github.com/def-gthill/lexurgy) | `fa5027711cba3cd6a3f4b6defd0d38181fa98cd4` (2026-06-17) | GPL-3.0 | Behavioral taxonomy only. No code, declarations, test text, or fixtures are copied or lightly transliterated. |
| [MixteCaSo](https://github.com/SAuderset/MixteCaSo) | `b38621d4ac309e23fde616b5424347586a8dbf2c` (2024-10-08) | CC BY-SA 4.0 | Schema and aggregate row counts inspected. No dataset rows are vendored; corpus tooling must use a user-supplied checkout. |
| [Searchable Index Diachronica](https://bitbucket.org/chridd/diachronica) | `0d8a9790d102ea7ed753f37eeaabaf5bc034397b` (2022-01-29) | Index text: CC BY-NC-SA 3.0. Conversion code: no license declared (`README.md` says TODO). | No text, converted data, or conversion code is vendored. Only an original, optional analyzer may consume a user-supplied checkout. |

The complete Brassica BSD notice must accompany any future copied or adapted
fixture. MixteCaSo adaptations must be segregated and distributed under
compatible ShareAlike terms. Diachronica data and conversion code remain
license-blocked for vendoring.

Status vocabulary: `ported`, `equivalent-existing`,
`implemented-with-original-test`, `unsupported`, `not-applicable`,
`license-blocked`, and `data-insufficient`.

## Baseline and current coverage

The initial audit found 4 upstreams, 2 Brassica test programs, 3 Brassica
natural-language examples, all 30 Lexurgy sound-change test files, 8 MixteCaSo
data/schema categories, and 9 recurring Diachronica notation/data categories.
The checked-in conformance harness currently contains 18 independently authored
shared-subset cases; this count is asserted by the test and must be updated here
whenever cases are added.

Implemented engine subsets include ordered simultaneous rules, deletion,
zero-width insertion, capture/reordering, greedy quantified class captures,
backreferences, inline literal alternatives, finite class-set operations, form
and word boundaries, negated classes, positive and excluded environments,
correspondence maps, global/once selection, left-to-right and right-to-left
overlap selection, partial runs, program composition, traces, deterministic
matcher work limits, and bounded fixed-point execution.

## Brassica inventory

| Upstream path/category | Status | Local coverage or precise gap |
|---|---|---|
| `test/Spec.hs`: output golden driver | unsupported | A complete run requires translating `changes.bsc`, including syntax and modes below. The local table harness covers the shared engine subset only. |
| `test/Spec.hs`: log golden driver | implemented-with-original-test | `Program::apply_with_trace` covers changed rule/form traces; Brassica's complete log formatting and parser diagnostics are not applicable. |
| `test/DocTests.hs` | not-applicable | Exercises Brassica documentation and command/parser behavior, not the Rust API. Semantic examples are inventoried separately. |
| `test/changes.bsc`: literal/category replacement | implemented-with-original-test | Classes, literals, ordered rules, and mapped captures are covered in `conformance.rs` and `sound_laws.rs`. |
| `test/changes.bsc`: corresponding category alternatives | implemented-with-original-test | Named `maps` plus `{capture\|MAP}` preserve declared pairings and reject incomplete coverage. |
| `test/changes.bsc`: contexts and word/form edges | implemented-with-original-test | Positive/excluded environments, `^`/`$` form anchors, and `#` whitespace-delimited word edges are covered. Brassica-specific word-list parsing remains out of scope. |
| `test/changes.bsc`: insertion, deletion, metathesis, degemination | implemented-with-original-test | Empty targets, empty outputs, arbitrary capture emission order, and identity backreferences cover the shared semantics. |
| `test/changes.bsc`: greedy/optional/repeated category material | implemented-with-original-test | `?`, `*`, and `+` operate on class/wildcard atoms. General grouped sequence repetition is unsupported. |
| `test/changes.bsc`: `-rtl` and `-1` | implemented-with-original-test | Explicit `[rtl]` and `[once]` options, including their combination, have overlap and insertion regressions. |
| `test/changes.bsc`: sporadic `-?` | unsupported | Needs an explicit seeded RNG/application policy; nondeterminism will not be added implicitly. |
| `test/changes.bsc`: multiple result branches | unsupported | `Program::apply` currently produces one deterministic form. A result-set API and branch limits are required. |
| `test/changes.bsc`: stress/category automation | unsupported | No stress-bearing-unit or syllable model exists. Literal stress scalars are supported but have no structural semantics. |
| `test/words.in`, `words.golden`, `words-log.golden` | data-insufficient | Licensed for adaptation, but most rows depend on unsupported `changes.bsc` constructs. No partial golden result is presented as a full pass. |
| `examples/latin2port.bsc` and `examples/latin2port.lex` | ported | All 14 ordered rules and all 10 source forms are adapted under BSD-3-Clause; outputs are asserted end to end. An ignored differential test verifies and runs the pinned CLI when supplied. It was not run locally because no GHC/Cabal or Brassica binary is installed. |
| `examples/english.bsc` and `examples/english.lex` | unsupported | Requires stress/syllable behavior, optional/sporadic application, and a faithful 198-line rule translation. |
| `examples/thai.bsc` and `examples/thai.lex` | unsupported | Requires tone/stress-bearing units, branching, and a faithful 211-line rule translation. |

## Lexurgy inventory

Every local Lexurgy-referenced case is independently authored. GPL test prose,
inputs, expected forms, and declarations are not fixtures in this repository.

| Upstream test file | Status | Independently assessed subset or precise gap |
|---|---|---|
| `TestAlternatives.kt` | implemented-with-original-test | Inline alternatives cover finite literal symbols and fixed literal sequences, including captures and atom quantifiers. Alternatives containing nested matcher structures are missing. |
| `TestBlocks.kt` | unsupported | Named blocks/stages and block application modes are not modeled. |
| `TestBoundaries.kt` | implemented-with-original-test | Whole-form anchors and whitespace-delimited word edges are distinct. Syllable and user-defined structural boundaries still need explicit token types. |
| `TestDiacritics.kt` | unsupported | Unicode combining scalars can be literal/class members, but feature-mutating diacritics and canonical normalization are absent. |
| `TestEnvironment.kt` | implemented-with-original-test | Positive and excluded left/right environments, captures, backreferences, and simultaneous environment observation are covered. Multiple alternative environments still require grouped alternatives. |
| `TestFeatures.kt` | unsupported | No phonological feature schema, matrix matching, mutation, defaults, or feature variables. Named correspondence maps cover only finite explicit pairings. |
| `TestFilters.kt` | unsupported | No lexical metadata/filter predicate API. |
| `TestLscParse.kt` | not-applicable | Tests Lexurgy's textual grammar. `sound-law` uses Rust macro tokens plus validated pattern strings. Semantic parser failures have Rust-specific tests. |
| `TestMultiWord.kt` | implemented-with-original-test | Rules can target whitespace-delimited word edges while preserving separators. Rich phrase boundaries and configurable cross-word policies are absent. |
| `TestNegation.kt` | implemented-with-original-test | Negated classes and one excluded environment are covered; arbitrary logical negation/nested structures are missing. |
| `TestOperators.kt` | implemented-with-original-test | Finite class union, intersection, difference, and correspondence maps are covered. Nested/feature-aware and general transforming operators are absent. |
| `TestPartialRuns.kt` | implemented-with-original-test | Checked indexed and named inclusive ranges plus `apply_through`/`apply_from` support partial derivations. Lexurgy's block-selection syntax is not reproduced. |
| `TestPropagation.kt` | unsupported | Needs feature-bearing segments and an explicit directional propagation/harmony model. |
| `TestRealExamples.kt` | data-insufficient | End-to-end examples depend heavily on Lexurgy-only features and cannot be copied under the repository license. |
| `TestRepeaters.kt` | implemented-with-original-test | Atom `?/*/+` and whole-program bounded fixed-point repetition are covered; grouped repeaters and per-rule fixed points are missing. |
| `TestReusable.kt` | unsupported | Classes and maps are reusable declarations, but reusable pattern/rule fragments with parameters are absent. |
| `TestRobustness.kt` | implemented-with-original-test | Invalid identifiers/classes/captures/maps, escaping, Unicode, cycles, nullable quantified atoms, compile-time pattern-depth bounds, and deterministic matcher work limits are tested. |
| `TestRomanization.kt` | unsupported | Pre/post romanization stages and intermediate output sets are not modeled; ordinary ordered rules can only emulate simple fixed pipelines. |
| `TestRuleNames.kt` | equivalent-existing | Rule names are compile-time unique ASCII identifiers and stable trace identities. Arbitrary display names are intentionally not accepted. |
| `TestRules.kt` | implemented-with-original-test | Ordered sequential rules, simultaneous matches, insertion/deletion, identity, overlap selection, and finite paired maps are covered. Compound branching mappings are absent. |
| `TestSequencedRule.kt` | equivalent-existing | Declaration order is execution order and later rules see earlier output. Named subsequences remain unsupported. |
| `TestSoundChanger.kt` | implemented-with-original-test | Reusable compiled programs apply to arbitrary UTF-8 strings with traces and bounded iteration. Lexical metadata/result collections are absent. |
| `TestSoundChangerCombinations.kt` | implemented-with-original-test | `Program::then` composes independently compiled programs in order and rejects duplicate stable names. Filtered lexicon/result-set combinations are absent. |
| `TestSyllabifier.kt` | unsupported | No syllable parser or onset/nucleus/coda model. |
| `TestSyllables.kt` | unsupported | No syllable matcher, stress assignment, or syllable-local boundary semantics. |
| `TestSymbolMatcherAndEmitter.kt` | implemented-with-original-test | Declared multi-scalar symbols, longest-first matching, captures, reordering, and finite mapped emission are covered. Feature emitters are absent. |
| `TestTimeout.kt` | implemented-with-original-test | `apply_with_limit` uses deterministic matcher-state work units and identifies the responsible rule; fixed-point execution separately has pass/cycle guards. Ordinary `apply` remains intentionally unbounded. |
| `TestTracing.kt` | implemented-with-original-test | Changed intermediate forms and stable rule names are returned. Source spans and unchanged-rule events are not exposed. |
| `TestVariables.kt` | unsupported | Captures and correspondence maps cover local equality/pairing; general declaration variables and parameterized reusable expressions are absent. |
| `TestWhitespace.kt` | not-applicable | Rust macro whitespace is handled by Rust. Literal whitespace in patterns is significant; there is no separate textual program parser. |

## MixteCaSo inventory

Pinned-checkout baseline: `data/cognates.tsv` has 15,117 physical lines;
`definitions/changes_segments.tsv` has 251, `changes_tones.tsv` has 78,
`changes_josserand.tsv` has 90; `variables_segments.tsv` has 106 and
`variables_tones.tsv` has 44. Counts include headers and are revision-specific.
The optional pinned-checkout analyzer parsed all 250 segment rows, 77 tone rows,
15,116 cognate rows, 105 segment-variable rows, and 43 tone-variable rows with
zero malformed TSV rows. Only 2 segment rows were unconditioned conservative
literals; both compiled and executed as minimal witnesses. No ordered chain or
tone-support claim is made.

| Dataset category | Status | Use or precise gap |
|---|---|---|
| `definitions/changes_segments.tsv` target/output fields | implemented-with-original-test | The optional original analyzer identifies 2/250 conservative unconditioned literal rows and locally executes both minimal witnesses. The other 248 need environment/notation classification and remain correspondences, not an ordered program. |
| Segment conditioning environments | unsupported | Importer must classify prose/symbol conventions and generate both match and non-match witnesses; no rows are silently guessed. |
| `definitions/changes_tones.tsv` | unsupported | Tone melodies require tone-bearing units and suprasegmental alignment, not scalar substitution alone. |
| `definitions/changes_josserand.tsv` | data-insufficient | Comparative correspondence sets may have several outputs and do not establish ordered rules. |
| `data/cognates.tsv` | data-insufficient | Suitable for independently specified row properties and witness lookup after normalization; not a direct sound-law oracle. |
| `variables/variables_segments.tsv` | not-applicable | Presence/absence by doculect is analysis metadata, not an executable transformation. |
| `variables/variables_tones.tsv` | not-applicable | Presence/absence by doculect is analysis metadata, not an executable transformation. |
| Metadata, sources, PDF normalization guides, and R scripts | license-blocked | Useful to an optional external analyzer; CC BY-SA material is not vendored into the normal test suite. |

## Searchable Index Diachronica inventory

The pinned processed `html/diachronica-data` has 10,012 physical lines and the
source `data.txt` has 12,095. These are breadth metrics, not success counts.
The optional analyzer found 811 sections and parsed all 9,201 processed change
rows with zero malformed five-field records: 4,867 have contexts, 62 use the
documented excluded-context separator, 2,404 have parallel changes, and 3,053
contain alternatives. Its deliberately narrow classifier compiled and executed
1,807 unconditioned literal witnesses locally. These are syntax/engine metrics,
not redistributed fixtures or linguistic validation.

| Recurring category | Status | Use or precise gap |
|---|---|---|
| Simple `A > B` rows | implemented-with-original-test | The optional analyzer locally compiled 1,807 conservative unconditioned literal witnesses. Raw/adapted entries are not vendored under the current license posture. |
| Left/right conditioned changes | license-blocked | Many are expressible with environments; an original analyzer can count syntax conservatively from a user checkout. |
| Deletion and insertion | license-blocked | Core semantics are implemented, but individual Index rows are not fixtures. |
| Metathesis/reordering | license-blocked | Capture emission can represent explicit finite reorderings; raw examples are not fixtures. |
| Alternative targets/outputs | unsupported | Finite literal input alternatives are implemented, but nested matcher alternatives, parallel correspondence import, and branching output result sets are missing. |
| Word/morpheme/syllable boundaries | unsupported | Form anchors and whitespace word edges exist, but the Index's conventions need importer-specific interpretation; morpheme and syllable structures are absent. |
| Feature/natural-class notation | unsupported | Requires a declared feature system or a conservative external expansion into finite classes. |
| Prose, uncertain, or malformed conversion rows | data-insufficient | Must be reported separately; they are not safe to interpret automatically. |
| Conversion/parser code | license-blocked | The repository declares no code license, so no source is copied. Any local analyzer must be independently written. |

## Reproduction policy

Normal `cargo test` is offline and uses only original local cases. Optional
corpus tools must take an explicit checkout path and verify the pinned Git
revision before reporting comparable metrics. They must never download from a
mutable branch or make corpus claims from an unverified revision.
