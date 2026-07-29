# ruthenian-core

**Give it a word and some grammar; it gives you the form.**

```rust
use ruthenian_core::*;

assert_eq!(noun("dom", Case::Genitive, Number::Singular), "domogo"); // OF the house
assert_eq!(noun("dom", Case::Ablative, Number::Singular), "doma");   // FROM the house
assert_eq!(noun("dom", Case::Nominative, Number::Dual),   "doma");   // two houses

// Everything a shape cannot predict is in the lemma itself:
assert_eq!(noun("Drug", Case::Locative, Number::Singular), "druzi");  // capital = animate
assert_eq!(noun("noczj'", Case::Genitive, Number::Singular), "noczi"); // ' = not the
                                                                       // predicted gender
```

Everything is computed from rules. **No dictionary, no data files, no lookup
tables of word facts, no network, no I/O.** A word the crate has never seen
inflects exactly as well as one it has.

The language is [`docs/RUTHENIAN.md`](../../docs/RUTHENIAN.md), whose **prose**
is normative; its paradigm **tables** run the other way — they sit between
`<!-- render:ID -->` markers and are generated from this crate by
`cargo run -p ruthenian-core --example render_spec`, so the spec and the engine
cannot drift on anything tabular. The crate's design is
[`DIRECTION.md`](../../DIRECTION.md); the grammar itself is written in
`src/dsl.rs`'s notation — SPE rewrite rules (`"y" => "i" / [k g h] _ ;`) and
declension tables laid out exactly as the spec prints them. Section references
throughout the source are to the specification.

## State

| | |
|---|---|
| **nouns** (§3.3–§3.6) | complete — all 8 cases × 3 numbers, three declensions, hard and soft |
| **adjectives** (§4) | complete — both declensions × 3 genders, plus §4.3's degrees |
| **pronouns** (§5) | complete — personal, clitic, reflexive, the pronominal declension, the interrogatives and the relative |
| **numerals** (§6) | complete — cardinals to `u64::MAX`, one rule per rank, and §6.5's ordinals |
| **verbs** (§7.3–§7.12) | complete — six classes, the one synthetic tense, the imperative, the copula and its future, the `l`-participle, four participles and two gerunds |

## Measurements

| | Result |
|---|---|
| Corpus cells reproduced (`tests/corpus/paradigms.tsv`) | **637 of 637** |
| Cells matching Russian where Russian has one | `cargo run --example against_russian` |
| Sample for review | `cargo run --example review` — 35 paradigms |
| Guards, each verified to fail under its stated mutation | **14 core, 20 orthography** |
| Third-party dependencies | **0** |

## Totality

**Every function is total.** No `Option`, no `Result`, no panic: any combination
of arguments the types permit returns a string. Where the language has no form
for a cell, the function returns a *declared* substitute, and `src/fallback.rs`
is the only place one may be introduced.

| Call | Returns | Why |
|---|---|---|
| any function, on an unreadable lemma | `?` | Not a word at all: unparseable, empty, no vowel, or non-letters. `?` is outside the alphabet, so it cannot collide with a form. |
| `reflexive(Nominative)` | `sjebja` | §5.2 gives the reflexive no nominative — it cannot be a subject. |
| `pronoun(_, _, _, Vocative)` | the nominative | §5.1's table has no vocative row; §3.1's convention applies. |
| `clitic_pronoun` outside the cells §5.1a lists | the full form | §5.1a gives clitics for the accusative and dative, singular and plural — there are no dual clitics. |
| `clitic_reflexive` outside acc/dat | the full reflexive | §5.2 gives `sja` and `si` and nothing else. |
| `imperative(w, First \| Third, Singular)` | the present indicative | §7.10 builds these with a particle; this is the form it attaches to. |

All are implemented. The
`every_fallback_exercised` guard counts the rows in this table against the tests,
so a substitute cannot be added without being both written down and exercised.

**The cost, stated once.** A caller can ask a question the language does not
have an answer to and get a plausible-looking string back. That is deliberate:
these combinations do not arise in real use, and paying for them with `Option` at
every call site is worse than paying for them with a documented fallback.

## The guards

Law 8: **every guard ships only after its named mutation has been applied,
observed to fail it, and reverted.** A guard that survives its own witness is
stale and is deleted, not left in place looking reassuring.

| # | Guard | Witness |
|---|---|---|
| 1 | `conformance` — every corpus cell reproduced exactly | change one ending in `noun.rs` |
| 2 | `spec_currency` — the corpus was generated from this spec | change one byte of `docs/RUTHENIAN.md` |
| 3 | `corpus_row_count` | delete a row from `paradigms.tsv` |
| 4 | `no_option_no_result` | make one public signature return `Option<String>` |
| 5 | `every_fallback_exercised` | declare a sixth fallback with no test |
| 6 | `paradigm_is_form` — law 2, one generation path | give `paradigm()` its own table |
| 7 | `totality_no_panic` — 25 hostile inputs across every entry point | remove the `is_word` guard in `lemma.rs` |
| 8 | `output_is_lowercase` | stop folding the lemma's case |
| 9 | `no_dependencies` | add any crates.io entry |
| 10 | `no_stored_derivable_state` — law 3 | add a `gender` field to `Noun` |
| 11 | `every_public_fn_has_a_doctest` | strip an example block |
| 12 | `grammar_types_are_exhaustive` | drop a variant from `Case::ALL` |
| 13 | `paradigm_totality` — no `-` cell is reachable | blank declension I's accusative |
| 14 | `spec_tables_current` — the spec's tables are fresh engine output | edit a cell inside a render block |

Guard 7 is the one that finds real bugs, because it is the only one that does not
know what the answer should be — only that there must be one. It is what caught
`noun("!", ..)` returning `"!"`: the alphabet tolerates punctuation in running
text, but a lemma is not running text, so a citation form must be letters and
contain a vowel.

## Why the corpus is a committed artifact

The crate is measured against **the specification**, because there is no other
authority: Ruthenian is specified, not attested, and no corpus of it exists or
ever will.

Extraction is deliberately a separate step from assertion.
`tools/extract_paradigms.py` **transcribes** the paradigms and then verifies that
every form it emits occurs in the specification; it does not parse the tables.
Parsing was tried in an earlier attempt and failed *silently* — a heading match
found `noczj` in §3.2's declension summary rather than §3.6's paradigm, compared
`dom`'s forms against it, and reported a clean run.

The script's own first draft reproduced that class of bug: it scanned the whole
file for backticked spans, so a single unbalanced backtick flipped the parity of
every span after it and the scan silently read the *gaps* between spans instead.
The paradigm tables fell out of the vocabulary and all 107 checks failed at once
— loudly, which is the only reason it was caught. Splitting per line confines an
imbalance to its own line, and reports it.

Both directions of drift are now covered: the specification cannot move away
from the corpus (guard 2's checksum), and the corpus cannot claim a form the
specification does not contain (the script's verification).

## Running it

```bash
cargo test -p ruthenian-core                  # 70 tests (97 across the workspace)
cargo clippy -p ruthenian-core --all-targets  # clean
cargo run -p ruthenian-core --example render_spec  # republish the spec's tables
python3 tools/extract_paradigms.py            # regenerate the corpus after a spec edit
```
