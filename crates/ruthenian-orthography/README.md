# ruthenian-orthography

Bijective Cyrillic↔Latin conversion for Ruthenian, and the only place in the
system where a script conversion exists.

The alphabet itself is specified in [`docs/RUTHENIAN.md`](../../docs/RUTHENIAN.md)
§2, which is normative. This document is the implementation: how the mapping is
made bijective, and what it cost to get there.

Zero dependencies. `#![forbid(unsafe_code)]`. No panic on any public path.

```rust
use ruthenian_orthography::{Cyrillic, to_latin, to_cyrillic};

let c = Cyrillic::parse("подъезд")?;
assert_eq!(to_latin(&c).as_str(), "pod'jezd");
assert_eq!(to_cyrillic(&to_latin(&c)).as_str(), "подъезд");
```

## The contract

```text
to_cyrillic(to_latin(s)) == s      for every well-formed Cyrillic string s
```

True **by construction**, not by accumulated special cases: the reader defines
how Ruthenian is read, and the writer inserts a separator exactly where
re-reading its own output would diverge.

**Note the direction the contract is stated in.** It quantifies over *Cyrillic*
strings, and that is deliberate: the guarantee needed is that transliterating
source material loses nothing, so two Cyrillic words can never collapse into one
Ruthenian word.

It does **not** quantify over every Ruthenian string, and since
`RUTHENIAN.md` §2.1 a Ruthenian word may end in `'` to mark an unpredicted
inflectional class (`pisatj'`). No Cyrillic input can produce that — `ъ` may only
stand before `е ё ю я и`, so a word-final hard sign is ill-formed Cyrillic — which
means the mark never appears in a transliterated word and the contract above is
untouched on its actual domain. Converting a *marked* lemma back to Cyrillic is
undefined rather than wrong: the mark is Ruthenian's own morphology, and
Ruthenian's orthography is not obliged to be expressible in another language's.

| Measurement | Result |
|---|---|
| Corpus round-trip (`biblija_ru.txt`, 41 462 lines, 38 623 non-empty) | **0 failures** |
| The same corpus through the reference implementation | 3 failures (lines 12695, 13444, 31725) |
| Every letter (60 cased forms), ordered pair and ordered triple | 0 failures |
| Random well-formed strings (14 144) | 0 failures |
| Guards, each verified to fail under its stated mutation | 15 of 15 |

Every count is printed by the guard that produced it. Measured 2026-07-25.

## The word-final mark

`RUTHENIAN.md` §2.1 gives a word-final `'` a second job: it marks a lemma that is
**not what its ending predicts** — class 6 rather than 1 on a verb (`pisatj'`),
feminine rather than masculine on a noun in `-j` (`noczj'`). The position is free
because the separator rule is about what follows a `'`, and word-finally nothing
does. This crate reports the mark; interpreting it is morphology's job.

| | |
|---|---|
| `Ruthenian::parse("pisatj'")` | **Ok** — the mark was already inside the allowed character set |
| `Ruthenian::parse("pisatj''")` | **Err(Apostrophe)** — two marks are neither separator nor mark |
| `marked.is_marked()` | `true`; a word-internal `pod'jezd` is `false` |
| `marked.word()` | `"pisatj"` — the word without its mark |
| `to_cyrillic(marked)` | `писать` — the mark is morphology, not sound |

**Two lemmas differing only in the mark share a Cyrillic form, and that is not a
round-trip failure.** The contract quantifies over *Cyrillic* strings, and
`to_latin` can never emit a mark: `ъ` may only stand before `е ё ю я и`, so a
word-final hard sign is ill-formed Cyrillic and no source word reaches that
position. `transliteration_never_emits_a_mark` pins it. A caller who needs
the distinction asks `is_marked` before converting.

## Well-formedness is part of the alphabet

The alphabet is a set of *strings*, not just characters. Four context rules make
the mapping bijective without the reverse direction ever guessing, and each was
validated against the corpus before being declared:

| Rule | Why it exists | Evidence |
|---|---|---|
| `ъ` is followed by `е ё ю я и` | `'` before `j`/`i` is the hard sign; `'` elsewhere is a pure separator | all 458 instances; the `и` environment is `предъидешь`, `предъизбранным`, `предъизбрал` |
| `ь` follows a consonant | `ь` and `й` are both written `j`; this is half of how the reader tells them apart | 50 036 instances, none after a vowel |
| `й` does not follow a consonant | the other half | 31 285 instances, none after a consonant |
| a hard sign agrees in case with the letter after it | `'` is caseless, so `Ъ`/`ъ` is recovered from its neighbour | no counter-example; `подЪезд` is not orthography |
| `ж ш ч щ` are not followed by `э` | they take `e` rather than `je`, so `e` after one of them reads back as `е` | **0** occurrences against 33 308 of `ж ш ч щ` + `е` |

Violations are `AlphabetError { offset, found, kind }` — never a silent
passthrough. `Unmapped` names the reason: `PreReform`, `ForeignCyrillic`,
`LatinInCyrillic`, `CyrillicInLatin`, `NotInAlphabet`, `Control`, `Apostrophe`,
`ForeignMark`, `StrayStress`, `HardSignContext`, `HardSignCase`,
`SoftSignContext`, `ShortIContext`.

## The three mechanisms

**1. The reader is the definition.** A greedy longest-match tokenizer over the
ordered digraph list — `szcz`, `sz`, `cz`, `zz`, `ja`, `je`, `jo`, `ju`, then
singles — is the single source of truth for how Ruthenian is read.

**2. The writer inserts a separator only where re-reading would diverge.**

| Source | Ruthenian | Why |
|---|---|---|
| Ийе | `Ijje` | `j` then the `je` digraph |
| щи | `szczi` | `szcz` is щ |
| шчи | `sz'czi` | without it, ш + ч reads as щ |
| сзади | `s'zadi` | `sz` would read as ш |
| зж | `z'zz` | з + ж |
| жз | `zz'z` | ж + з — same naive string, different separator position |
| подъезд | `pod'jezd` | the hard sign *is* the separator, at a morpheme boundary |
| подезд | `podjezd` | no boundary, so no separator; the pair stays distinct |
| батальон | `bataljon` | no separator: `jo` is `j` + `o`, there being no `ё` |

The decision is **local**. The longest digraph is four characters, and the window
reaches one letter each way: leftward for the class the reader needs to decide a
bare `j`, rightward **only for the hard sign**, whose reading depends on what
follows. Restricting the right-hand lookahead matters — including it
unconditionally assumes no separator will be inserted after the current letter,
which is exactly what the next step has yet to decide, and it makes `Ийон` come
out `I'j'on`. The exhaustive triples guard is what proves the window suffices.

**3. Case is a separate layer.** Encode by token: an ALL-CAPS token gets ALL-CAPS
digraphs, anything else Title-case ones. Decode per unit: a unit whose first
character is uppercase came from an uppercase letter. The two rules agree
everywhere, which is what lets mixed case round-trip with no special path.

```text
Щука → Szczuka    ЩУКА → SZCZUKA    ЩуКа → SzczuKa    ПРЕДЪИДЕШЬ → PRJED'IDJESZJ
```

## Mixed-script text

Two entry points, and the type distinguishes them:

- `to_latin(&Cyrillic) -> Ruthenian` — strict. Input is validated first, so the
  round-trip contract applies.
- `to_latin_mixed(&str) -> (String, Vec<SkippedSpan>)` — lenient. Transliterates
  maximal runs of declared characters, leaves everything else byte-identical, and
  reports what it skipped. It returns a plain `String`, **not** a `Ruthenian`,
  because its output contains text this crate makes no claims about and does not
  round-trip.

## What the reference implementation got wrong

Kept in the tree at [`legacy/`](../../legacy), commit `49d3af7`. Executed
2026-07-25; it round-trips 38 620 of 38 623 non-empty corpus lines.

| # | Defect | Witness | Resolution |
|---|---|---|---|
| D1 | `й`+vowel collides with the iotated vowel | `Ийон` → `Ijon` → `Иён`; `Йод` → `Jod` → `Ёд`; corpus lines 12695, 13444, 31725 | separator: `Ij'on`, `J'od` |
| D2 | `шч` collides with `щ` | `шчи` → `szczi` → `щи` | separator: `sz'czi` |
| D3 | Latin input consumed as Ruthenian | `"cat дом"` → `"цат дом"` | `LatinInCyrillic` error; `to_latin_mixed` for the lenient path |
| D4 | no case layer | `ЩУКА` → `SzczUKA` | token-level case: `SZCZUKA` |
| D5 | unmapped characters emit raw Cyrillic inside Latin | `мѣсто` → `mѣsto` | declared alphabet; `PreReform` / `ForeignCyrillic` errors |
| D6 | `'` overloaded; `Ъ`→`''` vs `ъ`→`'` | both round-trip in the reference | one glyph, one rule; `Ъ` is `'` |
| D7 | verification is a `println!` loop with no assertions | `test_roundtrip_from_file_ru` returns `Ok(())` regardless | 11 guards, each mutation-tested |

**Corrections to the pre-implementation analysis.** Three claims made from
reading the reference's code were wrong when executed, and are recorded so they
are not reintroduced: `Ъ`→`''` does *not* break the round-trip; `ЩУКА` does *not*
break it (it is ugly, not broken); combining stress marks already pass through
the reference cleanly. Nothing here is asserted that has not been run.

## Findings

1. **"One glyph, one rule" needs the alphabet to constrain `ъ`.** If `ъ` could
   appear anywhere, `'` would be ambiguous between hard sign and separator
   (`съз` and `сз` both naively give `s'z`). Declaring that `ъ` occurs only
   before `е ё ю я и` — which is what Russian does — makes `'`+`j`/`i` the hard
   sign and `'` elsewhere the separator, with no overlap. The decision and the
   constraint are one thing, not two.
2. **The `ь`/`й` ambiguity is resolved by declaration, not by a rule of thumb.**
   The reference guessed from a hardcoded consonant list. Here the alphabet
   declares the environments each letter occurs in, and the reader reads the
   declaration. The evidence is 81 321 corpus instances with no counter-example.
3. **The hard sign is caseless, so its case is borrowed.** `Ъ` is recovered from
   the following letter, and the alphabet requires the two to agree. A property
   test found this: `кюдфЪюГ` round-tripped to `кюдфъюГ`.
4. **`ё` is preserved here even though extraction normalizes it away.** The
   contract is totality over the declared Cyrillic alphabet, which includes `ё`,
   independently of what any downstream lexicon holds.

## Running the guards

```bash
cargo test                                                    # all but the full corpus
RUTHENIAN_CORPUS=../../biblija_ru.txt cargo test --release -- --ignored
```

The corpus is in the repository root and the reference implementation is in
[`legacy/`](../../legacy), so the head-to-head needs no second clone.

The per-PR suite uses a 1 621-line fixture that includes the three lines the
reference fails, so the comparison is provable without carrying a 7.4 MB file.
The full-corpus test fails loudly when `RUTHENIAN_CORPUS` is unset rather than
passing silently.
