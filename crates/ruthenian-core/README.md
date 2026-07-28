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

The language is [`docs/RUTHENIAN.md`](../../docs/RUTHENIAN.md), which is
normative; the crate's design is [`DIRECTION.md`](../../DIRECTION.md). Section
references throughout the source are to the specification.

## State

| | |
|---|---|
| **nouns** (§3.3–§3.6) | complete — all 8 cases × 3 numbers, three declensions, hard and soft |
| **adjectives** (§4) | complete — both declensions × 3 genders, plus §4.3's degrees |
| **pronouns** (§5.1, §5.1a, §5.2) | complete — personal, clitic and reflexive. §5.4–§5.6's non-personal series have **no entry point yet** |
| numerals (§6) | not yet — blocked on §6 having no word for zero and no rule for compounds |
| **verbs** (§7.3–§7.12) | complete — six classes, three synthetic tenses, the imperative, `byti`, `budu`, the `l`-participle, four participles and two gerunds |

## Measurements

| | Result |
|---|---|
| Corpus cells reproduced (`tests/corpus/paradigms.tsv`) | **458 of 458** |
| Nominal paradigms covered | 11 — `dom`, `Konj`, `Drug`, `okno`, `polje`, `zzena`, `kniga`, `zjemlja`, `nacija`, `sluga'`, `noczj'` |
| Adjective paradigms covered | 2 — `dobr` long and short, all three genders |
| Pronoun paradigms covered | 11 personal + 14 clitics + the reflexive |
| Guards, each verified to fail under its stated mutation | **12 of 12** |
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

All but the last are implemented; the imperative's arrives with M6. The
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
cargo test -p ruthenian-core                  # 45 tests: 4 unit, 1 corpus, 11 guards, 29 doc
cargo clippy -p ruthenian-core --all-targets  # clean
python3 tools/extract_paradigms.py            # regenerate the corpus after a spec edit
```
