# Implement `ruthenian-core`

Build the second crate in this workspace: the inflection engine described by
[`DIRECTION.md`](DIRECTION.md), implementing the language specified by
[`docs/RUTHENIAN.md`](docs/RUTHENIAN.md).

**Give it a word and some grammar; it gives you the form.** No dictionary, no
data files, no I/O. A word the crate has never seen inflects exactly as well as
one it has.

---

## 0. Read these first, in this order

| Document | What it is | Authority |
|---|---|---|
| [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md) | the language | **normative** — where it states a form, that form is correct by definition |
| [`DIRECTION.md`](DIRECTION.md) | the crate: API, laws, scope, totality | **binding on design** |
| [`docs/COMPARATIVE_GRAMMAR.md`](docs/COMPARATIVE_GRAMMAR.md) | the measured evidence behind the design | **background only** |
| [`crates/ruthenian-orthography/`](crates/ruthenian-orthography/) | the alphabet, and the house style to match | dependency + template |

**The authority order matters.** `COMPARATIVE_GRAMMAR.md` Part 13 is
*reasoning*, written before the specification settled, and it had drifted from
it — §13.3's `dom` gave `domi` for the nominative plural and `domje` for the
locative singular, §13.6 inflected `tri domi`, and §13.4 listed the long/short
adjective among the *removals* that §4 restores. Those are corrected as of this
branch, and the correction is the illustration: **never take a form from Part
13.** Where the two documents disagree, §RUTHENIAN.md is right and Part 13 is a
stale argument about it.

Read `crates/ruthenian-orthography/README.md` and `tests/guards.rs` before
writing anything: the guard-with-named-witness convention, the doc-test density,
the committed-fixture pattern and the prose register of this repository are all
established there, and `ruthenian-core` matches them.

---

## 1. Deliverable

```
crates/ruthenian-core/
  Cargo.toml            # ruthenian-orthography path dep, and NOTHING else
  README.md             # what it does, the totality table, the guard table
  src/lib.rs            # types, the free functions, module docs with doctests
  src/noun.rs
  src/adjective.rs
  src/pronoun.rs
  src/numeral.rs
  src/verb.rs
  src/spelling.rs       # §3.8's automatic adjustments + §2.4's palatalizations
  tests/conformance.rs  # the engine against the committed corpus
  tests/guards.rs       # the structural guards, each with its witness
  tests/corpus/
    paradigms.tsv       # every cell §RUTHENIAN.md tabulates, extracted once
    README.md           # what is in it, where each row came from
tools/extract_paradigms.py   # spec -> paradigms.tsv, run by hand, output committed
```

Add the crate to the workspace `members`. `edition = "2024"`,
`rust-version = "1.85"`, all inherited from `[workspace.package]` as the
orthography crate does.

---

## 2. The API — implement exactly this

From `DIRECTION.md` "The API". Reproduced here so there is no ambiguity about
the signatures; read that section for *why* each one has the shape it does.

```rust
pub fn noun(word: &str, case: Case, number: Number) -> String;

pub fn adjective(word: &str, case: Case, number: Number,
                 gender: Gender, animacy: Animacy) -> String;
pub fn short_adjective(word: &str, case: Case, number: Number,
                       gender: Gender, animacy: Animacy) -> String;

pub fn comparative(word: &str) -> String;
pub fn superlative(word: &str) -> String;

pub fn verb(word: &str, person: Person, number: Number, tense: FiniteTense) -> String;
pub fn imperative(word: &str, person: Person, number: Number) -> String;
pub fn infinitive(word: &str) -> String;
pub fn l_participle(word: &str, gender: Gender, number: Number) -> String;

pub fn byti(person: Person, number: Number, tense: FiniteTense) -> String;
pub fn future_auxiliary(person: Person, number: Number) -> String;

pub fn present_active_participle(word: &str) -> String;
pub fn past_active_participle(word: &str) -> String;
pub fn present_passive_participle(word: &str) -> String;
pub fn past_passive_participle(word: &str) -> String;
pub fn present_gerund(word: &str) -> String;
pub fn past_gerund(word: &str) -> String;

// A pronoun is identified by its agreement features, not by a name — see the
// note below. Gender is inert outside the third-person singular.
pub fn pronoun(person: Person, number: Number, gender: Gender, case: Case) -> String;
pub fn clitic_pronoun(person: Person, number: Number, gender: Gender,
                      case: Case) -> String;
pub fn reflexive(case: Case) -> String;
pub fn clitic_reflexive(case: Case) -> String;

pub fn numeral(value: u64, case: Case, gender: Gender, animacy: Animacy) -> String;
```

Six types, and no seventh:

```rust
pub enum Case { Nominative, Vocative, Accusative, Genitive,
                Ablative, Dative, Instrumental, Locative }
pub enum Number { Singular, Dual, Plural }
pub enum Gender { Masculine, Feminine, Neuter }
pub enum Animacy { Animate, Inanimate }
pub enum Person { First, Second, Third }
pub enum FiniteTense { NonPast, Aorist, Imperfect }
```

### There is no `Pronoun` enum

`DIRECTION.md` used to declare a seventh type, `Pronoun { Ja, Ty, On, Ona, Ono,
My, Vy, Oni, Vje, Va }`, and its own justification was the reason to delete it:
*"Which pronoun, the way `word: &str` says which noun."* That is an admission
that the enum is a **lexical identity**, not a paradigm dimension — so it failed
the rule the same document states two paragraphs earlier, that what survives is
"types, each a genuine dimension of a paradigm". It was a third mechanism for
naming a word, beside `word: &str` and the function name.

`DIRECTION.md` is already updated — see "A pronoun has no name" there. The
reasoning is repeated here because it decides how `src/pronoun.rs` is keyed.

**A pronoun is fully identified by the features it agrees in.** The mapping is
exact and total — every variant is one cell of person × number × gender, and
every cell is a variant:

| | Singular | Dual | Plural |
|---|---|---|---|
| **1st** | `ja` | `vje` | `my` |
| **2nd** | `ty` | `va` | `vy` |
| **3rd** | `on` / `ono` / `ona` by gender | `ona` | `oni` |

Two defects go with the enum:

- **`Third.Dual` was reachable three ways.** §5.1's third-person dual column
  (`ona`, `ja`, `jeju`, `jima`) has no gender, so `On`, `Ono` and `Ona` each
  plus `Dual` all denote it — three spellings of one cell, and nothing said
  which was canonical.
- **The identity and the parameter could contradict each other.** `pronoun(Ja,
  _, Plural, _)` asked for a first-person *singular* pronoun in the plural. The
  new signature cannot express the question, which is why gap C1 below is now
  closed rather than answered.

**Gender is inert outside the third-person singular**, and that is correct
rather than a wart: `pronoun(First, Singular, Feminine, Nominative)` is `ja`,
because Ruthenian's first person does not inflect for gender. Document it on the
function; do not add a `Gender::Unspecified`, and do not make the parameter an
`Option` — law 4 forbids the second and law 5's concern (a caller omitting a
lexical fact and getting a plausible wrong answer) does not arise when the
argument is ignored rather than guessed.

The rule `DIRECTION.md` states is unchanged and is what decides this — an enum
that selects a paradigm becomes a function, an enum that indexes within one
stays a parameter, and an enum that **names a word** is neither.

Plus the bound-lexical-facts form, which **calls the free functions** and never
recomputes anything (law 2):

```rust
let dom = Noun::new("dom");
dom.form(Genitive, Singular);          // "domogo"
for (case, number, form) in dom.paradigm() { … }
```

`paradigm()` on each part of speech is how "every possible form" is reached, and
it is the same code path as `form()`.

> `DIRECTION.md`'s Totality table used to write `imperative(w, c, Third,
> Singular)` — a stray `c` left over from an earlier signature that carried the
> conjugation class, since §7.3 now derives it. Corrected there; noted here
> because the two-argument form is what the corpus rows assume.

---

## 3. The laws

All eight are in `DIRECTION.md` "The laws" and each is falsifiable by a test.
The four that will actually shape your code:

1. **The spec decides; the code conforms.** Where §RUTHENIAN.md states a form,
   a disagreeing engine is wrong. Where the spec is silent, the gap is reported
   as a **spec gap and closed in the spec** — never patched with a guess in
   code. See §7: you will hit several.
2. **One generation path.** `paradigm()` calls `form()` calls the free function.
3. **Derive state; never store it.** Gender, animacy, declension, hardness,
   stem, conjugation class and palatalization are all read off the lemma
   (§2.1, §3.2, §7.3). A struct field duplicating any of them fails review.
4. **Every function is total, and every fallback is declared.** No `Option`, no
   `Result`, no panic, for any input including `""`, a bare `'`, and
   non-Ruthenian text. Adding a fallback means adding a row to the totality
   table in `DIRECTION.md` and exercising it in a test — not quietly picking
   something in the code.

Also: `#![forbid(unsafe_code)]`, zero third-party dependencies, pure functions.

---

## 4. Build order

Each milestone is a commit that leaves the crate green.

| | Milestone | Contents |
|---|---|---|
| **M0** | skeleton + corpus | the seven enums, the crate wired into the workspace, `tools/extract_paradigms.py`, `paradigms.tsv` committed, `conformance.rs` reading it and asserting `todo`-free rows are skipped-with-a-count rather than silently passing |
| **M1** | `spelling.rs` | §3.8's five adjustments and §2.4's three palatalizations, as functions over (stem, ending) — tested directly against `drug`/`kniga`'s worked cells before any paradigm uses them |
| **M2** | nouns | §3.2's lemma reading (gender, declension, hardness, animacy, the mark), then §3.3–§3.6 — `dom`, `konj`, `drug`, `okno`, `polje`, `zzena`, `kniga`, `zjemlja`, `sluga'`, `noczj` |
| **M3** | adjectives | `short_adjective` (§4.1, the nominal endings **including the animate accusative**), `adjective` (§4.2, the pronominal endings), then `comparative`/`superlative` (§4.3) |
| **M4** | pronouns | §5.1 personal and third person keyed on person × number × gender, §5.1a the clitic series, §5.2 the reflexive |
| **M5** | numerals | §6.2–6.3's spelling of any `u64`, §6.4's declension of the last word, §6.1's government documented but **not** applied (that is syntax) |
| **M6** | verbs | §7.3's class derivation from the citation form, §7.4 present, §7.5 aorist, §7.6 imperfect, §7.11 mutation, §7.10 imperative, `byti` (§7.9) and `future_auxiliary` (§7.8) |
| **M7** | participles | §7.7's l-participle, §7.12's four participles and two gerunds — each returning an **adjective stem** that `adjective()`/`short_adjective()` then declines |
| **M8** | `paradigm()` + guards | the enumerations, then the guard suite of §6, each with its witness applied, observed to fail, and reverted |

---

## 5. Correctness: the committed corpus

The crate is measured against **the specification**, because there is no other
authority — Ruthenian is specified, not attested, and no corpus of it exists or
ever will.

**Extraction is a separate step from assertion, deliberately.** Parsing the spec
inside the test was tried in an earlier attempt and failed silently: a heading
match found `noczj` in §3.2's declension summary rather than §3.6's paradigm,
compared `dom`'s forms against it, and reported a clean run. A committed
artifact makes prose reformatting a reviewable diff instead of a test that
quietly checks less.

**Format.** TSV, one cell per row, with the section it came from so a failure is
traceable:

```
pos     lemma   features                form      section
noun    dom     Genitive.Singular       domogo    3.3
noun    drug    Vocative.Singular       druzzje   3.3
verb    czitatj First.Dual.NonPast      czitajevje 7.4
```

**What goes in.** Every cell the spec tabulates *and* every paradigm it gives in
prose — the prose ones are as normative as the tables and are the easy ones to
miss: `polje` (§3.4), `kniga`, `zjemlja`, `nacija`, `sluga'` (§3.5), `sjej`
(§5.4), `byti`'s participle/infinitive/imperative (§7.9). Also §3.3's `= nom`
and `= dat` shorthand must be expanded into real cells, not dropped.

`nacija` is the one to get right first: it is the **vowel-final stem**, so it
proves the engine appends endings rather than special-casing a Russian-shaped
`-ija` paradigm, and its genitive `nacii` is the doubled vowel that a
well-meaning contraction rule would silently eat.

**Currency.** A `spec_currency` guard fails when `docs/RUTHENIAN.md` changes and
the corpus has not been regenerated: the extractor writes a checksum of the spec
into the TSV header, and the guard recomputes it in pure Rust (a hand-rolled
FNV-1a over the file bytes — no dependency, and the value is a reviewable
constant in a diff). A spec edit then forces a deliberate `python3
tools/extract_paradigms.py` and a visible corpus diff, which is the whole point.

**Row count is pinned.** The guard asserts the exact number of rows, so cells
cannot silently disappear from the corpus during a refactor.

---

## 6. The guards

Law 8: **every guard ships only after its named mutation has been applied,
observed to fail it, and reverted.** A guard that survives its own witness is
stale and is deleted, not left in place looking reassuring. State the witness in
a comment above each, exactly as `ruthenian-orthography/tests/guards.rs` does,
and record the verification in the PR.

| # | Guard | Witness |
|---|---|---|
| 1 | `conformance` — every corpus row reproduced exactly | change any one ending in `spelling.rs` |
| 2 | `spec_currency` — corpus matches the spec's checksum | touch one byte of `docs/RUTHENIAN.md` |
| 3 | `corpus_row_count` | delete a row from the TSV |
| 4 | `no_option_no_result` — the public API contains neither | change one signature to `-> Option<String>` |
| 5 | `every_fallback_exercised` — each row of the totality table has a test | add a row without a test |
| 6 | `paradigm_is_form` — `paradigm()` agrees with `form()` for every cell | give `paradigm()` its own ending table |
| 7 | `totality_no_panic` — every function over a hostile input set (`""`, `"'"`, `"'''"`, `"дом"`, `"x".repeat(10_000)`, every enum combination) | remove a guard clause in the lemma reader |
| 8 | `output_is_lowercase` — `noun("Drug", Nominative, Singular) == "drug"` | pass the lemma's case through |
| 9 | `no_dependencies` — parse `Cargo.toml`, assert `[dependencies]` holds only the workspace path | add any crates.io entry |
| 10 | `no_stored_derivable_state` — `Noun`/`Verb` hold the lemma and nothing else | add a `gender: Gender` field |
| 11 | `every_public_fn_has_a_doctest` | delete one doctest |

Guard 7 is the one that will find real bugs. Write it early.

---

## 7. Spec gaps you will hit — report them, do not guess

Law 1 forbids patching a silent spec with a guess in code. These are the places
the spec is silent, self-contradictory or unimplementable as written. **Every
one of them is blocking**: none can be resolved by picking something reasonable
in `spelling.rs`.

Collect them in the PR description, and where the resolution is mechanical,
propose the `docs/RUTHENIAN.md` edit as a **separate commit** in the same PR so
the spec change and the code change are reviewable apart. Do not start the
affected milestone until its gap has a decision.

### A. Internal contradictions — the spec disagrees with itself

| | Where | The contradiction |
|---|---|---|
| **A1** | §3.5 vs §3.8.1 | §3.5 gives `sluga'`'s plural as `slugy`, but §3.8 rule 1 writes `y` as `i` after a velar — and §3.5's own `kniga` obeys it (`knigi`). One of the two is wrong; `slugi` is the consistent reading. |
| **A2** | §7.3 vs §7.4 + §7.11 | The spec's prose uses `vizzu` "I see" (§3.7, §9.5) from `vidjetj`, a **class 5** verb. §7.3 marks only classes 4 and 6 as mutating, and §7.4 gives the 2nd-conjugation 1sg as `-ju` — so the rules as written predict `vidju`. Either class 5 mutates in the 1sg too, or `vizzu` is wrong. |
| **A3** | §7.4 vs §7.11 | When the 1sg mutation applies, what is the ending? `ljubitj` → `ljubl-` (`b`→`blj`) + 2nd-conj `-ju` gives `*ljubljju`. `vizzu` shows `-u` after a mutated consonant. The spec never states that the mutated 1sg takes the bare vowel. |
| **A4** | §7.12 vs `DIRECTION.md` | §7.12 lists the participle suffixes in their **long** shapes (`-uszczij`, `-vszij`, `-jemyj`), but `DIRECTION.md` requires the participle functions to return an adjective **stem** for `adjective()`/`short_adjective()` to decline — as its own `poczitatj` → `poczitan` example does. The functions must return `czitajuszcz`, `czitavsz`, `czitajem`, `poczitan`. Confirm and fix §7.12's presentation. |

### B. Rules the spec uses but never states

| | Where | What is missing |
|---|---|---|
| **B1** | §3.8 rule 2 | "after `zz sz cz szcz c`, **unstressed** `o` is written `je`" is **unimplementable**: §2.1 says stress is real but *not written*, so the engine cannot see it. No paradigm in the spec exercises the rule, so nothing pins the intended output — `otjec` instrumental singular is `otjecom` or `otjecjem` and the spec does not say. Either the condition drops the stress clause, or the rule is deleted, or `-jec` stems get a worked paradigm. **This blocks M1.** |
| **B2** | §7.3's own example | `pisatj'` → `piszeszj`, not `*piszjeszj`: a `je`-initial ending loses its `j` after a hushing consonant. The example depends on the rule; §3.8's five adjustments are stated over nominal endings only and never mention it. It needs to be stated, and stated as applying to verb endings too. |
| **B3** | §4.3 | The comparative "triggers the first palatalization" — on a velar-final stem, `dorog` → `dorozzjejsz`? No example is given, and the superlative's `naj-` prefixing is unexemplified on a stem that palatalizes. Derivable from §2.4's trigger column, but should be pinned by a worked example in the corpus. |
| **B4** | §6.2–6.3 | The spelling of a compound numeral is illustrated (`dvadcatj dva`) but never specified: what separates the words, whether hundreds and tens join, and what `numeral(0, …)` is — the spec has no zero. |

### C. Cells the API can ask for that the language has no answer to

`DIRECTION.md`'s totality table covers seven of these. These are **not** in it
and each needs a row added there before the code returns anything:

| | The call | Why it is undefined |
|---|---|---|
| ~~C1~~ | ~~`pronoun(Ja, _, Plural, _)`~~ | **Closed by dropping the `Pronoun` enum** (§2). The signature can no longer state a person/number contradiction, so there is nothing to define a fallback for. Left here so the change is traceable. |
| **C2** | `clitic_pronoun(_, _, _, Genitive \| Instrumental \| Locative \| Vocative)` | §5.1a gives clitics for the **accusative and dative only**. Returning the full form is the obvious substitute and is what OCS practice implies, but it must be written down. |
| **C3** | `pronoun(_, _, _, Vocative)` | §5.1's table has no vocative row. §3.1's convention (nominative stands in) covers it, but the row belongs in the table. |
| **C4** | any function on an unparseable lemma | `""`, `"'"`, `"дом"`, `"x'y'z"`. `Ruthenian::parse` returns `Err`; these functions return `String`. The substitute must be defined and identical across all of them. |
| **C5** | `numeral(value, …)` for `value` with no spelling | see B4. |

### D. A hole the pronoun section makes visible

**D1 — the non-personal pronouns have no entry point at all.** §5.4's `toj` and
`sjej`, §5.5's `kto`, `czto` and `izzje`, and §5.6's `ni-`/`nje-`/`-libo` series
are all tabulated or specified, so §5 tells the extractor to put `toj`'s full
table in the corpus — and **no function in the API can produce a single cell of
it**. The corpus would carry rows the conformance test can only skip.

This predates the change above; the enum never covered them either.
`adjective()` does not absorb them: it builds the long adjective, whose
masculine nominative singular is stem + `-yj`, so `toj` is not `adjective("t",
…)`.

`DIRECTION.md` now records the hole under "One thing it is not yet, and should
be", and names the resolution: **expose the pronominal declension itself** — one
entry point that `toj`, `sjej`, `izzje` and the long adjective all route
through, since §4.2's whole claim is that `dobryj`'s endings *are* `toj`'s. One
paradigm, one function, one table.

What is left to you is the signature and whether it lands in this PR. It needs a
decision before M4, because it changes whether §5.4's corpus rows are checkable
or skipped. If it is deferred, say so in the PR and leave the rows in with the
skip counted — never delete a corpus row to make a test pass.

---

## 8. Out of scope

Named explicitly, because each is a plausible-looking thing to add and all of
them are wrong here:

- **§8 word formation.** No `-nije`, `-tjelj`, `-ostj`, no compounding, no
  prefixes. `DIRECTION.md`'s API has no derivation functions beyond degree and
  the participles, and adding them would make the crate a word-formation engine.
- **The periphrastic tenses.** The perfect, pluperfect and imperfective future
  are composed by the caller from `byti`, `future_auxiliary`, `l_participle` and
  `infinitive`. Doing the composition here means doing agreement and word order,
  which is syntax, and the return value stops being a word.
- **The two stored lexical classes** (§7.2's inherently perfective verbs, §7.2a's
  motion pairs) and the §7.3 residue (`zzitj` → `zziv-`). These are word data.
  They arrive as arguments or they live in a lexicon crate that does not exist
  yet. **Do not add a table.**
- **Aspect.** It changes what a form *means*, never what it looks like. There is
  no `Aspect` type.
- **Analysis.** Generation only. Form → grammar is a different problem.
- **Anything reading a file at runtime.** The corpus is a *test* fixture.

---

## 9. Done when

From `DIRECTION.md`, and each is checkable:

- Every cell of every paradigm the spec tabulates is reproduced exactly, checked
  against the committed corpus.
- Nouns, adjectives, verbs, pronouns and numerals each resolve **every** slot for
  every class, in all three numbers.
- `paradigm()` enumerates a complete table for each part of speech, via the same
  code path as `form()`.
- Every public function carries a doc test showing a real form.
- Zero third-party dependencies; `#![forbid(unsafe_code)]`; no panic on any
  public path, for any input.
- Every guard demonstrated to fail under its stated witness, and the
  verification recorded in the PR.
- Every gap in §7 either closed by a spec commit in this PR or listed, with what
  it blocks, in `docs/RUTHENIAN.md` §13 "Still to write".

## 10. Conventions

- `cargo test`, `cargo clippy -- -D warnings` and `cargo fmt --check` all clean.
- `CHANGELOG.md` under `## Unreleased` → `### Added`, in the register already
  there: state what the thing *is* and what it settles, not that a file was
  created.
- `crates/ruthenian-core/README.md` carries the totality table and the guard
  table, as the orthography crate's README carries its measurements and its
  defect table.
- Commit messages are the repository's existing style — a claim, not a summary:
  "Nouns carry their own gender and animacy", "Past passive participle: a single
  -n-, so one stem serves both forms".
- Work on a branch off `main`; one PR.
