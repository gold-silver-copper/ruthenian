# Ruthenian orthography

Normative specification of the Ruthenian writing system, as implemented in
`crates/ruthenian-orthography`. Every claim here is executed by a test; the
numbers were measured on 2026-07-25.

## The contract

```text
to_cyrillic(to_latin(s)) == s      for every well-formed Cyrillic string s
```

This holds **by construction**, not by accumulated special cases. The reader
defines how Ruthenian is read; the writer emits a separator exactly where
re-reading its own output would diverge.

Measured: 41 462 corpus lines (38 623 non-empty), **0 failures**; 62 single
letters (31 × 2 cases — `ъ` and `ь` cannot stand alone), 966 well-formed ordered
pairs, 30 101 well-formed ordered triples, and 14 144 random well-formed strings.
Every count is printed by the guard that produced it.

## The alphabet

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

Also in the alphabet: the combining acute U+0301 (stress), and neutral
characters — digits, punctuation, whitespace — which pass through untouched.
Deliberately **not** in it: `'` as literal text, Latin letters, pre-reform
letters, non-Russian Cyrillic, control characters other than whitespace.

## Well-formedness is part of the alphabet

The alphabet is a set of *strings*, not just characters. Three context rules make
the mapping bijective without the reverse direction ever guessing. Each was
validated against the corpus before being declared:

| Rule | Why it exists | Evidence |
|---|---|---|
| `ъ` is followed by `е ё ю я и` | `'` before `j`/`i` is the hard sign; `'` elsewhere is a pure separator | every one of 458 instances; the `и` environment comes from `предъидешь`, `предъизбранным`, `предъизбрал` |
| `ь` follows a consonant | `ь` and `й` are both written `j`; this is half of how the reader tells them apart | 50 036 instances, none after a vowel |
| `й` does not follow a consonant | the other half | 31 285 instances, none after a consonant |
| a hard sign agrees in case with the letter after it | `'` is caseless, so `Ъ`/`ъ` is recovered from its neighbour | no counter-example; `подЪезд` is not orthography |

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
| Иён | `Ijon` | reads back as И + ё |
| Ийон | `Ij'on` | without it, `jo` reads as ё |
| щи | `szczi` | `szcz` is щ |
| шчи | `sz'czi` | without it, ш + ч reads as щ |
| сзади | `s'zadi` | `sz` would read as ш |
| зж | `z'zz` | з + ж |
| жз | `zz'z` | ж + з — same naive string, different separator position |
| подъезд | `pod'jezd` | the hard sign *is* the separator, at a morpheme boundary |
| подезд | `podjezd` | no boundary, so no separator; the pair stays distinct |
| батальон | `batalj'on` | without it, `jo` reads as ё and `батальён` comes back |

The decision is **local**. The longest digraph is four characters, and the window
reaches one letter each way: leftward for the class the reader needs to decide a
bare `j`, rightward **only for the hard sign**, whose reading depends on what
follows it. Restricting the right-hand lookahead matters — including it
unconditionally assumes no separator will be inserted after the current letter,
which is exactly what the next step has yet to decide, and it makes `Ийон` come
out `I'j'on`. The exhaustive triples guard is what proves the window suffices.

**3. Case is a separate layer.** Encode by token: an ALL-CAPS token gets
ALL-CAPS digraphs, anything else Title-case ones. Decode per unit: a unit whose
first character is uppercase came from an uppercase letter. The two rules agree
everywhere, which is what lets mixed case round-trip with no special path.

```text
Щука  → Szczuka    ЩУКА → SZCZUKA    ЩуКа → SzczuKa    ПРЕДЪИДЕШЬ → PRJED'IDJESZJ
```

## Stress

Ruthenian marks stress with a combining acute on the Latin vowel: `писа́ть` ↔
`pisátj`, decomposed on both sides. Stressed and unstressed spellings are
**different strings**; nothing normalizes one into the other, and nothing
composes the mark into a precomposed codepoint. A mark not on a vowel is
`StrayStress`.

## Mixed-script text

Two entry points, and the type distinguishes them:

- `to_latin(&Cyrillic) -> Ruthenian` — strict. Input is validated first, so the
  round-trip contract applies.
- `to_latin_mixed(&str) -> (String, Vec<SkippedSpan>)` — lenient. Transliterates
  maximal runs of declared characters, leaves everything else byte-identical, and
  reports what it skipped. It returns a plain `String`, **not** a `Ruthenian`,
  because its output contains text this crate makes no claims about and does not
  round-trip.

## Decision record

### The reference implementation

<https://github.com/gold-silver-copper/ruthenian>, commit `49d3af7`. Executed
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
break the round-trip (it is ugly, not broken); combining stress marks already
pass through the reference cleanly. Nothing in this document is asserted that has
not been run.

### Closed decisions

- **Stress: stored, rendered on request** (`DIRECTION.md`). The mark is part of
  the alphabet and is carried in both directions.
- **Mixed-script input: two entry points**, strict and lenient, as above.
- **The apostrophe: one glyph, one rule.** `'` means "the next character starts a
  new letter", and Russian ъ is that rule at a morpheme boundary. This is
  coherent *only* because the alphabet constrains where ъ may appear — see the
  finding below.

### Findings from implementation

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
3. **The hard sign is caseless, so its case is borrowed.** `'` carries no case,
   so `Ъ` is recovered from the following letter, and the alphabet requires the
   two to agree. A property test found this: `кюдфЪюГ` round-tripped to
   `кюдфъюГ`.
4. **`ё` is preserved here even though extraction will normalize it away.** The
   contract is totality over the declared Cyrillic alphabet, which includes `ё`,
   independently of what any downstream lexicon holds.
