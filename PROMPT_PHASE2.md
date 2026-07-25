# Phase 2: `ruthenian-core` — execution prompt

Build the second crate: the productive morphology of Russian as pure rules over
Ruthenian strings, plus the grammatical vocabulary the rest of the workspace
shares.

Work in `~/Desktop/code/ruthenian`, on a branch off `master`. Phase 1 is merged
(PR #1, `c0a3aa5`): `crates/ruthenian-orthography` is in the tree, the reference
implementation is in `legacy/`, and the corpora are at the repository root.

## Read first

- `docs/specs/ruthenian-core.md` — the authority for this crate. Where it and
  this prompt disagree, the spec wins **except** where this prompt corrects it
  from measured data; those corrections are marked "Correction" and must be
  written back into the spec.
- `DIRECTION.md` §"The laws", especially laws 2 (rules predict, tables store the
  residue), 5 (derive state, never hand-maintain), 8 (`None` means "no such form
  exists") and 12 (structure, not strings).
- `docs/ORTHOGRAPHY.md` — what Phase 1 guarantees and what it constrains.

## What Phase 1 gives you, and what it demands

`ruthenian-orthography` is your only dependency. It provides `Cyrillic`,
`Ruthenian`, `to_latin`, `to_cyrillic`, `Alphabet`, and a reader you can call.

Three consequences to design around rather than discover:

- **Every string you emit must be valid Ruthenian**, and there is a guard for it.
  Build forms from the alphabet's spellings, never by concatenating characters
  you assume are legal.
- **Stress is load-bearing.** It is part of the alphabet, carried as a combining
  acute on the Latin vowel, and stressed and unstressed spellings are *different
  strings*. The accent letter of the Zaliznyak class therefore drives real stress
  placement — it is not inert metadata. A form emitted without stress where the
  class determines one is a bug.
- **The alphabet constrains well-formedness**, not just characters: `ъ` only
  before `е ё ю я и`, `ь` only after a consonant, `й` never after one, hard-sign
  case agreement. If your morphophonology produces a stem that violates these,
  the rule is wrong, not the alphabet.

## The measured class inventory

Measured over the **entire** dump on 2026-07-25 — 441 629 Russian records,
12 773 verb lemmas carrying `ru-conj`. Sampling is forbidden (`INVARIANTS.md`
I1); regenerate with `python3 tools/measure.py`.

**Verbs — 226 distinct class codes, very heavily skewed:**

| Class | Count |
|---|---:|
| `1a` | 5 060 |
| `2a+p` | 953 |
| `4a+p` | 693 |
| `4b+p` | 639 |
| `1a+p` | 547 |
| `2a` | 539 |
| `4b` | 491 |
| `4c+p` | 483 |
| `4a` | 480 |
| `4c` | 356 |
| `5b` | 224 |
| `3b` | 211 |

**Classes 1–6 are 11 584 of 12 773 = 90.7 %** of verbs carrying a class code.
**Implement in frequency order** and report coverage as you go — that number is
this phase's headline.

**Nouns — a small closed inventory, which is why they are the easiest part:**

- stem classes: `hard-stem` (12 314), `velar-stem` (7 297), `i-stem` (3 583),
  `soft-stem` (1 452), `ц-stem` (1 340), `sibilant-stem` (899),
  `vowel-stem` (641);
- accent patterns: `a` (25 442), `b` (2 382), `c` (536), `d` (495), `e` (287),
  `f` (75), plus primed variants (`dʹ` 28, `fʹ` 28, `bʹ` 11, `fʺ` 5) that a
  sample never surfaced at all. `a`+`b` alone are 94.7 %.

**Adjectives:** hard `-yj` (6 669), velar/sibilant with `i`-spelling (2 356),
stressed `-oj` (540), **true soft `-nij` (155 = 1.6 %)**. `*` marks a fleeting
vowel in the short form, `①`/`②` short-form irregularities, `ʹ` a softness
distinction.

### Parsing the class code is part of this crate

The notation is not a simple enum. Observed shapes include `1a`, `4b+pжд`,
`7b/b(9)+p`, `a(2)`, `6°b`, `irreg`, `-`. Write a real parser, with **all 226** distinct codes as its test corpus. They are
committed at `crates/ruthenian-core/tests/paradigms/class-codes.txt`; regenerate
with `tools/measure.py`, never from a slice of the file.

**`irreg` and `-` are not parse failures.** They are valid codes meaning "the
rules cannot derive this verb" — a signal that `ruthenian-lexicon` must supply
the forms. Parse them into an explicit variant. An *unrecognized* code is a
different thing and must be an error, never a silent default to some class.

**Verified: `+p` means the verb forms a past passive participle.** Over the whole
dump: of codes carrying `+p`, 4 190 have an attested PPP and 6 do not; of codes
without it, 173 have one and 8 404 do not. So `+p` predicts the PPP with
**99.86 % precision and 96.0 % recall** — reliable enough to drive generation,
and the 173 exceptions are exactly the kind of residue the lexicon exists to
hold. In `4b+pжд` the trailing
`жд` is the participle's stem mutation (`победить` → `побеждённый`).

## The present-stem mutations, measured

This is the heart of the crate, and it is directly expressible in Ruthenian.
Counts and examples are from the full-dump scan; the Ruthenian column is the same
rule written in the alphabet you actually emit.

| Cyrillic | Ruthenian | Count | Example |
|---|---|---:|---|
| ов → у | `ov` → `u` | 675 | мульчировать/мульчирую |
| д → ж | `d` → `zz` | 112 | щадить/щажу |
| т → ч | `t` → `cz` | 60 | лететь/лечу |
| с → ш | `s` → `sz` | 56 | писать/пишу |
| в → ∅ | `v` → ∅ | 41 | давать/даю |
| з → ж | `z` → `zz` | 40 | возить/вожу |
| п → пл | `p` → `plj` | 38 | спать/сплю |
| в → вл | `v` → `vlj` | 27 | готовить/готовлю |
| б → бл | `b` → `blj` | 25 | любить/люблю |
| ст → щ | `st` → `szcz` | 24 | крестить/крещу |
| ев → у | `ev` → `u` | 19 | бичевать/бичую |
| м → мл | `m` → `mlj` | 19 | кормить/кормлю |
| ев → ю | `ev` → `ju` | 11 | блевать/блюю |
| к → ч | `k` → `cz` | 9 | плакать/плачу |
| ск → щ | `sk` → `szcz` | 6 | искать/ищу |
| х → ш | `h` → `sz` | 6 | махать/машу |
| т → щ | `t` → `szcz` | 5 | трепетать/трепещу |
| им → емл | `im` → `jemlj` | 5 | внимать/внемлю |
| ер → р | `er` → `r` | 3 | тереть/тру |
| р → ер | `r` → `er` | 2 | брать/беру |

Two things this table teaches that a grammar book states less sharply:

- **`ов` → `u` is the single most common mutation**, six times the next. It is
  the `-овать`/`-ировать` class, not an exotic case — do not leave it for later.
- **Mutation is conditioned on the class, not on the stem's final consonant.**
  Of the 1 977 class-1 verbs whose stem ends in a labial, **not one** takes
  epenthesis — they are `-ивать`/`-ывать` verbs where the theme vowel intervenes.
  The rule is exceptionless across the whole dump, which is a fact only a full
  scan can establish; a rule keyed on "stem ends in a labial" would corrupt all
  1 977. Key on the class.
- `д` → `zz` and `з` → `zz` collide, exactly as they do in Russian
  (водить/возить both → вожу). That is a real homograph, not a bug.

## Correction to the spec: two kinds of gap, and only one is fillable

`docs/specs/ruthenian-core.md` §7 says the dump marks defective slots `"-"` so
the affected set is "enumerated exactly by Phase 4". **That is wrong as written,
and acting on it would break the aspect system.** Measured over every Russian
verb in the dump:

| Aspect | Verbs | Gap slots | Per verb |
|---|---:|---:|---:|
| perfective (incl. `pf-intr`) | 5 881 | 55 646 | 9.5 |
| imperfective (incl. `impf-intr`) | 6 856 | 9 517 | 1.4 |

The commonest gap slots are `participle passive present` (9 036),
`participle passive past` (8 405), `adverbial participle present` (6 035), and
each of the six present-tense person/number slots at ~5 950 — tracking the
perfective verb count.

So the overwhelming majority of `"-"` slots are **structural**: a perfective verb
has no present tense (its non-past morphology realizes the future), no present
participles and no present gerund; an intransitive verb has no passive
participle. These are grammar. Filling them would invent a present tense for
perfective verbs and destroy the aspect distinction.

**Structural gaps must be derived by rule** from `(aspect, transitivity, slot)` —
law 5 — and returned as `None`, meaning "no such form exists" — law 8. They must
never be read from data.

**Lexical gaps are a separate, tiny category, and the dump marks them
explicitly.** Verified on the canonical case:

```json
"inflection_templates": [{"name": "ru-conj",
  "args": {"1": "pf", "2": "4b+pжд", "3": "победи́ть", "futr_1sg": "-"}}]
```

`победить` carries `futr_1sg: "-"` as an **explicit override argument**. Its
first-person *future* singular is the defect (`*побежу` is avoided); its
first-person *present* singular is `-` merely because it is perfective.

So `gap.fill-defective-1sg` (rename it from `gap.fill-1sg`) targets the 1sg
future of defective perfectives, and its documentation must say which gap it
fills and which it must never touch. Write this correction back into
`docs/specs/ruthenian-core.md` §7 and `docs/specs/ruthenian-extract.md` §2.

## Deliverable 1: the grammatical vocabulary

`Case`, `Number`, `Gender`, `Person`, `Tense`, `Aspect`, `Animacy`, `Slot`,
`VerbSlot`, `PronounStyle`, `ZaliznyakVerbClass`, `NounClass`, `AdjClass`,
`StressPattern`. Owned here; every other crate imports them from here.

`Slot` is exhaustive by construction, with `Pronoun` and `Numeral` as their own
variants (closed in spec §12): the post-prepositional `n-` series is not a case
of a noun, and numeral government is a property of the numeral.

## Deliverable 2: the rule engine

```rust
pub fn noun(stem: &Ruthenian, class: NounClass, g: Gender, a: Animacy, slot: Slot)
    -> Option<Prediction>;
pub fn adjective(stem: &Ruthenian, slot: Slot) -> Option<Prediction>;
pub fn verb(parts: &PrincipalPartsRef<'_>, class: ZaliznyakVerbClass, slot: VerbSlot)
    -> Option<Prediction>;
```

**`verb` takes principal parts, not a lemma.** Where the class does not determine
the present stem, the caller supplies it; the engine never guesses a stem it
cannot derive. That is law 8 expressed in the type.

**Watch the ownership direction here.** `ruthenian-lexicon` (Phase 3) depends on
this crate, so this crate cannot import from it. Core therefore defines the
*borrowed input shape* `PrincipalPartsRef<'_>`, and the lexicon defines the
*owned stored shape* and converts into it. Defining an owned `PrincipalParts`
here, or reaching for the lexicon's type, produces a dependency cycle that will
only surface in Phase 3.

`Prediction` carries a non-empty `Trace` naming the rules that fired. The
evaluator uses it to attribute mismatches and the CLI to explain deviations;
returning a bare string forces both to re-derive what you already knew.

Coverage, in implementation order:

1. **Nouns** — 6 cases × 2 numbers over the seven stem classes and six accent
   patterns, animacy in the accusative, reducible stems, irregular plurals.
2. **Adjectives** — long forms, short forms, comparative, superlative; the
   `ж ш ч щ ц` spelling rules, which in Ruthenian are `zz sz cz szcz c` and
   should be stated that way.
3. **Verbs** — classes in frequency order, present/past/future, imperative,
   infinitive, 4 participles, 2 gerunds, reflexive `-sja`, conditional.
4. **Pronouns and numerals** — full paradigms, the `n-` prefix after
   prepositions, and numeral government returned **as structure** (the case and
   number a count imposes), never as a string the caller must re-analyse.

**One morphophonology module** — the mutation table above, palatalization,
fleeting vowels, the sibilant spelling rules — used by every part of speech. A
second copy of a seam rule means it is in the wrong place; `interslavic-phrase`
paid for that lesson with a rewrite.

## Deliverable 3: the paradigm fixture

This crate cannot read the dump — that is Phase 4, and this crate is offline and
dependency-free. Extraction is a one-off developer step producing a vendored TSV,
not a crate dependency.

Extract **60–80 real lemmas with their full attested paradigms** into
`tests/paradigms/fixture.tsv`, covering:

- the top classes by frequency (`1a`, `4a/b/c` with and without `+p`, `2a`, `3a`,
  `5b`, `6c`), so the coverage number means something;
- at least one verb per mutation row in the table above, `ov → u` included;
- every noun stem class and every accent pattern a–f;
- adjectives with `*`, `①`, `②`;
- a perfective/imperfective pair, an `irreg` verb, a suppletive verb,
  `победить` (the `futr_1sg: "-"` case), a reducible-stem noun, a pluralia
  tantum, and an indeclinable.

Record provenance in `tests/paradigms/README.md`: dump fingerprint, page title
per lemma, and why each was chosen. **Real records only** — never hand-written
paradigms, which encode what you believe rather than what is attested.

**Score the fixture two ways, and report both**: *segmental* (letters only,
diacritics ignored) and *strict* (including stress placement). Early on the
endings will be right long before the accent patterns are, and a single
all-or-nothing number would hide all the progress and all the stress bugs at
once. This mirrors what `ruthenian-eval` will do in Phase 6.

Expect failures for lemmas whose principal parts the class does not determine —
that is the point. **Report them as a list, not a number**: each one names
something Phase 3 must store.

## Deliverable 4: the regularization registry

`RuleId`, `Policy::attested()`, `Policy::regularized()`, `.with()`/`.without()`.

Every rule is **off in both presets** and reachable only through
`Policy::attested().with(rule)` — no rule ships enabled before Phase 6 prices it.
`Policy::attested()` must reproduce Russian; it is the evaluator's baseline, and
if it drifts every later accuracy number is meaningless.

`docs/REGULARIZATION.md` is **generated** from the registry, not hand-written.

## The guards

All nine from spec §9, plus three this phase's findings require:

```text
slot_exhaustive          regular_rules_golden      trace_non_empty
policy_isolation         attested_is_pure          output_is_valid_ruthenian
stress_placed            morphophonology_single_owner
no_lexical_data          no_dependencies
```

New, all three non-negotiable:

- **`structural_gaps_are_derived`** — for every perfective verb in the fixture,
  the six present-tense slots and the present participles/gerund return `None`
  with no data consulted. Witness: make a present-tense slot of a perfective
  return a form.
- **`class_codes_parse`** — every observed code parses or is a named error, with
  `irreg` and `-` parsing to their own variants. Witness: add a code the parser
  silently defaults on.
- **`mutation_is_class_conditioned`** — a class `1a` verb with a labial-final
  stem (`пробивать`, `налаживать`) takes no epenthesis and no mutation. Witness:
  key the mutation on the stem's final consonant instead of the class; hundreds
  of `-ивать`/`-ывать` verbs break.

For **each** guard: apply the mutation named as its failure witness, confirm the
guard fails, revert, and record the outcome. Phase 1 found two stale guards this
way — one that sourced its expectations from the table it was checking, one that
passed because an unconsumed character survived by accident. Assume yours have
the same problem until the mutation proves otherwise.

## Gates

- `cargo test --workspace`, `cargo test --doc --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
  (`legacy/` is excluded from the workspace; leave it that way.)
- **Zero third-party dependencies**; `ruthenian-orthography` only. Enforced by a
  test, not by inspection.
- `#![forbid(unsafe_code)]`; no `unwrap`/`expect`/`panic!` reachable from public
  input. Phase 1's precedent: even a "cannot happen" table lookup returns a
  fallback rather than panicking.
- Doc test on every public function.
- No lexical data in the crate — no lemma lists, no exception tables.

## House rules

- **Assert nothing you have not executed.** In Phase 1, three claims taken from
  reading code were wrong when run; in preparing this prompt, one spec claim (the
  gap correction) and one of my own mutation-table omissions were wrong when
  measured. Every number in your report needs a command behind it.
- **Root cause, not symptom.** A form that comes out wrong is a rule that is
  wrong. Adding a lemma-specific branch to fix it is the one change this crate
  cannot accept — it is the boundary between this crate and the lexicon.
- **Frequency order, honestly reported.** Shipping Phase 2 with the long tail of
  the 127 classes unimplemented is fine. Leaving that implicit is not: state the
  covered share, and make an unimplemented class an explicit error rather than a
  wrong form.
- One commit per deliverable. Do not push, publish, tag, or open a PR unless
  asked.

## Report

State: coverage by part of speech and by verb class, as a share of the full-dump
distribution; the fixture pass rate **segmental and strict**, with failures
listed by lemma and the reason each needs a principal part; the twelve guards
with the outcome of each mutation test; every place the spec was wrong or
underspecified and what you did instead; and the list of things Phase 3 must
store because the rules provably cannot derive them — that list is this phase's
real output.
