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

Sampled from `~/Desktop/code/wikidata/raw-wiktextract-data.jsonl` on 2026-07-25:
1.8 GB of 22 GB (~8%), 1 669 verb lemmas carrying `ru-conj`. Re-measure on a
larger sample if you want tighter numbers; these are estimates, not exact counts.

**Verbs — 127 distinct class codes, very heavily skewed:**

| Class | Share | Cumulative |
|---|---:|---:|
| `1a` | 38.6 % | 38.6 % |
| `4a+p` | 4.3 % | 43.0 % |
| `4b+p` | 4.0 % | 47.0 % |
| `4c+p` | 4.0 % | 50.9 % |
| `4b` | 3.3 % | 54.2 % |
| `2a+p` | 3.1 % | 57.3 % |
| `4a` | 2.8 % | 60.0 % |
| `1a+p` | 2.8 % | 62.8 % |
| `4c` | 2.7 % | 65.5 % |
| `a(2)` | 2.2 % | 67.7 % |
| … 12 more | | 79.9 % |
| `irreg` | 1.0 % | |
| `-` (no class given) | 1.6 % | |

Classes 1, 2 and 4 with stress patterns a/b/c cover roughly two thirds of all
verbs. **Implement in frequency order** and report coverage as you go — that
number is this phase's headline.

**Nouns — a small closed inventory, which is why they are the easiest part:**

- stem classes: `hard-stem` (1 258), `velar-stem` (716), `soft-stem` (336),
  `i-stem` (235), `sibilant-stem` (107), `ц-stem` (106), `vowel-stem` (73);
- accent patterns: `a` (2 524), `b` (275), `c` (58), `d` (57), `e` (26), `f` (12).

**Adjectives:** `a*` (287), `a` (103), `a*①` (35), `a*②` (14), `c`, `cʹ*`, `b*②`.
`*` marks a fleeting vowel in the short form, `①`/`②` short-form irregularities,
`ʹ` a softness distinction.

### Parsing the class code is part of this crate

The notation is not a simple enum. Observed shapes include `1a`, `4b+pжд`,
`7b/b(9)+p`, `a(2)`, `6°b`, `irreg`, `-`. Write a real parser, with the full list
of distinct codes as its test corpus — regenerate it rather than trusting the
table above:

```bash
cd ~/Desktop/code/wikidata
for skip in 4000 11000 19000; do
  dd if=raw-wiktextract-data.jsonl bs=1m skip=$skip count=600 2>/dev/null | tail -n +2
done | python3 -c 'your_filter'   # collect inflection_templates[0].args["2"]
```

**`irreg` and `-` are not parse failures.** They are valid codes meaning "the
rules cannot derive this verb" — a signal that `ruthenian-lexicon` must supply
the forms. Parse them into an explicit variant. An *unrecognized* code is a
different thing and must be an error, never a silent default to some class.

**Verified: `+p` means the verb forms a past passive participle.** Of verbs whose
code contains `+p`, 1 488 have an attested PPP and 1 without; of verbs without
it, 61 have one and 3 284 do not. So `+p` predicts the PPP with ~99.9 % precision
and ~96 % recall — reliable enough to drive generation, and the 61 exceptions are
exactly the kind of residue the lexicon exists to hold. In `4b+pжд` the trailing
`жд` is the participle's stem mutation (`победить` → `побеждённый`).

## The present-stem mutations, measured

This is the heart of the crate, and it is directly expressible in Ruthenian.
Counts and examples are from the same sample; the Ruthenian column is the same
rule written in the alphabet you actually emit.

| Cyrillic | Ruthenian | Count | Examples |
|---|---|---:|---|
| ов → у | `ov` → `u` | 146 | негодовать/негодую, семплировать/семплирую |
| с → ш | `s` → `sz` | 16 | превозносить/превозношу, заносить/заношу |
| т → ч | `t` → `cz` | 15 | рокотать/рокочу, колотить/колочу |
| д → ж | `d` → `zz` | 13 | садить/сажу, восходить/восхожу |
| з → ж | `z` → `zz` | 11 | отвозить/отвожу, грезить/грежу |
| п → пл | `p` → `plj` | 17 | тупить/туплю |
| в → вл | `v` → `vlj` | 6 | кривить/кривлю, травить/травлю |
| м → мл | `m` → `mlj` | 6 | знакомить/знакомлю, экономить/экономлю |
| б → бл | `b` → `blj` | 5 | долбить/долблю, клубить/клублю |
| ст → щ | `st` → `szcz` | 6 | хрустеть/хрущу, хлестать/хлещу |
| т → щ | `t` → `szcz` | 3 | трепетать/трепещу, тяготить/тягощу |
| х → ш | `h` → `sz` | 2 | пахать/пашу |
| ск → щ | `sk` → `szcz` | 1 | рыскать/рыщу |
| к → ч | `k` → `cz` | 1 | мурлыкать/мурлычу |

Two things this table teaches that a grammar book states less sharply:

- **`ов` → `u` is the single most common mutation**, by an order of magnitude. It
  is the `-овать`/`-ировать` class, not an exotic case — do not leave it for
  later.
- **Mutation is conditioned on the class, not on the stem's final consonant.**
  1 251 verbs in the sample show *no* mutation at all, and 670 labial-final stems
  show no epenthesis — because they are class `1a` `-ивать`/`-ывать` verbs, where
  the theme vowel intervenes and nothing mutates. A rule keyed on "stem ends in a
  labial" will corrupt hundreds of common verbs. Key on the class.
- `д` → `zz` and `з` → `zz` collide, exactly as they do in Russian
  (водить/возить both → вожу). That is a real homograph, not a bug.

## Correction to the spec: two kinds of gap, and only one is fillable

`docs/specs/ruthenian-core.md` §7 says the dump marks defective slots `"-"` so
the affected set is "enumerated exactly by Phase 4". **That is wrong as written,
and acting on it would break the aspect system.** Measured, from 2 941 verbs:

| Aspect | Verbs | Gap slots |
|---|---:|---:|
| perfective (incl. `pf-intr`) | 1 459 | 13 922 |
| imperfective (incl. `impf-intr`) | 1 477 | 2 509 |

The commonest gap slots are `participle passive present` (2 199),
`participle passive past` (2 013), `adverbial participle present` (1 532), and
each of the six present-tense person/number slots at ~1 519 — almost exactly the
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
first-person *present* singular is `-` merely because it is perfective. Only 21
imperative gaps appear in the entire sample, which is the order of magnitude of
real defectiveness.

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

State: coverage by part of speech and by verb class, as a share of the sampled
distribution; the fixture pass rate **segmental and strict**, with failures
listed by lemma and the reason each needs a principal part; the twelve guards
with the outcome of each mutation test; every place the spec was wrong or
underspecified and what you did instead; and the list of things Phase 3 must
store because the rules provably cannot derive them — that list is this phase's
real output.
