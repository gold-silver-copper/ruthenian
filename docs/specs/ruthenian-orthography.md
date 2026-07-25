# Spec: `ruthenian-orthography`

Phase 1. Depends on nothing.

## 1. Purpose

The single place in the system where a Cyrillic string becomes a Latin one or
the reverse. It owns the Ruthenian alphabet: what characters exist, how they map,
and what it means for a string to be well-formed Ruthenian.

It is a *writing system*, not a linguistics crate. Wrong to put here: anything
that knows about lemmas, stems, paradigms, parts of speech, or the dump. If a
function needs to know that `-tj` is an infinitive ending, it belongs in
`ruthenian-core`.

The crate's reason for existing is one property, and every design choice serves
it: **`to_cyrillic(to_latin(s)) == s` for every `s` over the declared alphabet**,
true by construction rather than by accumulated special cases.

## 2. Public API sketch

```rust
/// A string verified to contain only declared-alphabet characters.
/// Its existence makes "unmapped character reached the mapper" unrepresentable.
pub struct Cyrillic(String);
pub struct Ruthenian(String);

impl Cyrillic {
    pub fn parse(s: &str) -> Result<Self, AlphabetError>;
}

pub struct AlphabetError {
    pub offset: usize,      // byte offset of the first offending character
    pub found: char,
    pub kind: Unmapped,     // PreReform | ForeignCyrillic | LatinInCyrillic | Control
}

pub fn to_latin(s: &Cyrillic) -> Ruthenian;
pub fn to_cyrillic(s: &Ruthenian) -> Cyrillic;

/// Convenience for mixed text: transliterate only maximal runs of declared
/// characters, leave everything else byte-identical, and report what was
/// skipped so a caller can refuse if it wants to.
pub fn to_latin_mixed(s: &str) -> (String, Vec<SkippedSpan>);

/// The reader, exposed because it is the definition of "how Ruthenian is read".
pub fn tokenize(s: &Ruthenian) -> Vec<Grapheme>;

pub struct Alphabet;                 // the declared inventory, queryable
impl Alphabet {
    pub fn contains(c: char) -> bool;
    pub fn digraphs() -> &'static [&'static str];   // ordered, longest first
}
```

`Cyrillic`/`Ruthenian` are newtypes rather than `String` aliases because the
round-trip contract is only claimed over the declared alphabet. A caller that
wants to convert arbitrary text must go through `parse` and handle the error, or
use `to_latin_mixed` and see the skipped spans. There is no entry point that
silently accepts anything.

## 3. Inputs and outputs

In: Rust strings. Out: Rust strings. No files, no configuration, no I/O.

## 4. Data owned

- The mapping table (below) — the only copy in the workspace.
- The ordered digraph list that defines greedy reading.
- The separator rule.
- The case-restoration rule.
- The declared alphabet.

## 5. Dependencies allowed

**Zero** non-dev dependencies. This is a gate: a `Cargo.toml` with a
`[dependencies]` entry fails the phase. Dev-dependencies may include a property
testing crate.

## 6. The mapping

Lowercase; uppercase parallel via the case layer.

| Cyr | Ruth | Cyr | Ruth | Cyr | Ruth |
|---|---|---|---|---|---|
| а | `a` | к | `k` | ч | `cz` |
| б | `b` | л | `l` | ш | `sz` |
| в | `v` | м | `m` | щ | `szcz` |
| г | `g` | н | `n` | ъ | `'` |
| д | `d` | о | `o` | ы | `y` |
| е | `je` | п | `p` | ь | `j` |
| ё | `jo` | р | `r` | э | `e` |
| ж | `zz` | с | `s` | ю | `ju` |
| з | `z` | т | `t` | я | `ja` |
| и | `i` | у | `u` | | |
| й | `j` | ф | `f` | | |
| | | х | `h` | | |

This inventory is inherited from the reference implementation
(<https://github.com/gold-silver-copper/ruthenian>, commit `49d3af7`) and is not
up for redesign. What *is* up for redesign is everything that makes it
consistent.

### The reference implementation's defects

All verified by executing the reference crate on 2026-07-25 (probe run against
the cloned repo, corpus `biblija_ru.txt`: 41 462 lines, 38 623 non-empty).

**Bijectivity breaks — these are the reason for the rewrite:**

| # | Defect | Verified witness |
|---|---|---|
| D1 | `й` + vowel is indistinguishable from an iotified vowel. Both write `j`+vowel. | `Ийон` → `Ijon` → `Иён`; `Йод` → `Jod` → `Ёд`. Corpus: **3 failures in 38 623 non-empty lines**, at lines 12695, 13444, 31725 — all of this shape. |
| D2 | `шч` collides with `щ`. The contextual separator covers only з/ж after s/z/c/zz, not this pair. | `шчи` → `szczi` → `щи` |
| D3 | Latin-script input is consumed as Ruthenian, so mixed text is destroyed. | `"cat дом"` → `"cat dom"` → `"цат дом"`; bare `"sz"` → `ш` |

**Quality defects — round-trip survives, output is wrong or undeclared:**

| # | Defect | Verified witness |
|---|---|---|
| D4 | No case layer: an all-caps word renders with a Title-case digraph. | `ЩУКА` → `SzczUKA` (should be `SZCZUKA`); round-trip back to `ЩУКА` does succeed |
| D5 | Unmapped characters fall through a catch-all and emit raw Cyrillic inside Latin output, invisibly. | `мѣсто` → `mѣsto`; `ѳита` → `ѳita`; `ґанок` → `ґanok` — each "round-trips" while producing mixed-script garbage |
| D6 | `'` carries two meanings (hard sign; digraph separator) and `Ъ`/`ъ` have asymmetric spellings `''`/`'`. | Both round-trip correctly — this is inelegance, not breakage, and the fix must not pretend otherwise |
| D7 | The reference's own verification is a `println!` loop with no assertions. | `test_roundtrip_from_file_ru` returns `Ok(())` regardless of failures |

**Corrections to earlier drafts of this analysis**, recorded so they are not
reintroduced: `Ъ`→`''` does *not* break the round-trip; `ЩУКА` does *not* break
the round-trip; combining stress marks (U+0301) pass through cleanly and land
naturally on the Latin vowel (`писа́ть` → `pisátj`). Three claims asserted from
code reading alone turned out to be wrong when executed — assert nothing here
that has not been run.

**What already works and must keep working:** the separator handles з/ж clusters
in both cases (`сзади` → `s'zadi`, `изжить` → `iz'zzitj`, `зз` → `z'z`, `жж` →
`zz'zz`, `СЗАДИ` → `S'ZADI`); `подъезд` → `pod'jezd` and `подезд` → `podjezd`
stay distinct; stress marks survive.

## 7. The design

Three mechanisms, in this order of authority.

**1. The reader is the definition.** A greedy longest-match tokenizer over the
ordered digraph table (`szcz`, `sz`, `cz`, `zz`, `ja`, `je`, `jo`, `ju`, then
singles) is the single source of truth for how a Ruthenian string is read. Every
other component is defined in terms of it.

**2. The writer inserts separators only where the reader would disagree.** Emit
the naive mapping, then run the reader over the result; wherever re-reading would
not reproduce the input, insert `'` and re-check. This makes the round-trip a
property of the construction rather than a list of patched cases, and it fixes
D1 and D2 with the same mechanism that already fixes сз:

- `Ийон` → `ij'on` (reader sees й, о) while `Иён` → `ijon`
- `шчи` → `sz'czi` while `щи` → `szczi`
- `подъезд` → `pod'jezd` — the hard sign is the same rule at a morpheme boundary

**D6 is closed: one glyph, one rule.** `'` means *"the next character starts a
new letter"*, and Russian ъ is exactly that rule applied at a morpheme boundary.
The hard sign is not a separate concept with a separate symbol; `pod'jezd` is the
separator doing its ordinary job. This means the writer has a single
separator-insertion pass with nothing special-cased for ъ, and the reader has one
rule to explain. The asymmetric `Ъ`→`''` spelling of the reference is dropped:
case is the case layer's business, so `Ъ` is `'` under an uppercase token like
any other character.

**3. Case is a separate layer.** Fold to lowercase, map, then restore the token's
case pattern: all-lower → lower, Title → Title, ALL-CAPS → ALL-CAPS. `ЩУКА` →
`SZCZUKA`, `Щука` → `Szczuka`. Mixed-case tokens (`МоСкВа`) get one documented,
tested behaviour — do not guess per character.

**Alphabet declaration** closes D5: the inventory is explicit, and anything
outside it is an `AlphabetError` with a byte offset and a reason, never a silent
passthrough. Pre-reform letters (ѣ ѳ і ѵ), foreign Cyrillic (ґ є ї ў) and Latin
runs each get their own `Unmapped` variant so a caller can decide.

## 8. Invariants

1. `to_cyrillic(to_latin(c)) == c` for every `Cyrillic` value.
2. `to_latin(to_cyrillic(r)) == r` for every *canonical* `Ruthenian` value, where
   canonical means "contains no separator the writer would not have emitted".
   Non-canonical input reads correctly but normalizes; this is documented, not
   silently true.
3. `Cyrillic::parse` accepts exactly the declared alphabet — no character both
   parses and fails to map.
4. Case restoration is exact: `to_latin` of an all-caps input contains no
   lowercase letter.
5. Combining stress marks are preserved in both directions and attach to the same
   vowel.
6. `to_latin_mixed` leaves every skipped span byte-identical.
7. Non-Cyrillic runs — digits, punctuation, whitespace — pass through unchanged.

## 9. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `roundtrip_exhaustive_singles` | Inv. 1 for every declared character | Remove one row from the mapping table | required | ms | crate |
| `roundtrip_exhaustive_pairs` | Inv. 1 for every ordered pair | Delete the separator insertion for `j`+vowel — `ij'on` regresses | required | <1 s | crate |
| `roundtrip_exhaustive_triples` | Inv. 1 for every ordered triple | Make the digraph list unordered so `sz` shadows `szcz` | required | seconds | crate |
| `roundtrip_corpus` | Inv. 1 over every Russian string in the extracted lexicon | Reintroduce the reference's catch-all passthrough arm | required (from Phase 4) | seconds | crate |
| `reference_defect_witnesses` | D1–D5 are fixed and stay fixed | Revert any one fix: `Ийон`, `Йод`, `шчи`, `"cat дом"`, `ЩУКА`, `мѣсто` each pinned with the corrected output | required | ms | crate |
| `alphabet_totality` | Every `char` either parses or yields a typed `AlphabetError` | Add a character to the mapping without adding it to `Alphabet::contains` | required | ms | crate |
| `case_restoration` | Inv. 4 | Map before folding instead of after | required | ms | crate |
| `stress_preserved` | Inv. 5 | Strip combining marks during normalization | required | ms | crate |
| `stress_is_distinguishing` | A stressed and an unstressed spelling are different strings and both round-trip | Normalize `pisátj` to `pisatj` anywhere in the pipeline | required | ms | crate |
| `no_dependencies` | Section 5 | Add any `[dependencies]` entry | required | ms | workspace |
| `property_roundtrip` | Inv. 1 over random declared-alphabet strings | Any of the above | required | ~1 s | crate |

Ten guards, each with a witness that must break it.

## 10. Out of scope

- Any knowledge of morphology, lemmas or parts of speech → `ruthenian-core`.
- Any knowledge of the dump or its `roman` field → `ruthenian-extract`.
- Ukrainian, Church Slavonic, Proto-Slavic, pre-reform Russian as *supported*
  scripts. They are recognized only well enough to be rejected with a specific
  error.
- Phonetic transcription. This is orthography; `zz` is a spelling of ж, not a
  claim about /ʐ/.

## 11. Done criteria

- All ten guards present, each demonstrated to fail under its witness (run the
  mutation, record that it failed, revert).
- 0 round-trip failures over the corpus, with the count of strings tested stated
  in the crate docs.
- The reference's 3 corpus failures reproduced against the *reference* and shown
  fixed against ours — the comparison is the headline number for this phase.
- `docs/ORTHOGRAPHY.md` written: the table, the reader, the separator rule, the
  case rule, the alphabet, and a decision record for D1–D7 and the two open
  decisions below.
- Zero dependencies; `#![forbid(unsafe_code)]`; no `unwrap`/`expect`/`panic!` on
  any public path.

## 12. Closed decisions

All three questions this spec opened are now closed.

- **Stress: stored, rendered on request.** Ruthenian marks stress with a
  combining acute on the Latin vowel (`pisátj`), the lexicon keeps it, and the
  CLI prints it only when asked. Verified that the reference already preserves
  U+0301 for free, so this costs nothing to carry. Consequences for this crate:
  the combining acute is part of the declared alphabet, `Alphabet::contains` must
  accept it, and the round-trip guards must cover stressed and unstressed
  variants of the same word as distinct strings. A form differing only in stress
  is a *different string*, and the crate must never normalize one into the other.
- **Mixed-script input: two entry points.** Strict `to_latin` returns
  `AlphabetError` with a byte offset; `to_latin_mixed` transliterates only
  declared-alphabet runs and reports the spans it skipped. The CLI uses the
  strict one.
- **The apostrophe: one glyph, one rule.** See §7.

## 13. Open questions

None. All three questions this spec opened are closed in §12.

One consequence worth recording even though it is enforced elsewhere:
extraction normalizes ё to е (see `ruthenian-extract.md` §10), so **no lexicon
entry will contain the `jo` digraph**. `jo` nonetheless stays in the reader and
the mapping table, because this crate's contract is round-trip totality over the
declared Cyrillic alphabet — which includes ё — independently of what the
lexicon happens to hold. Do not "optimize" it out.
