# ruthenian-orthography

Bijective Cyrillic↔Latin conversion for Ruthenian — Latin-script Russian. The
first crate of the [Ruthenian](../../DIRECTION.md) workspace, and the only place
in the system where a script conversion exists.

Zero dependencies. `#![forbid(unsafe_code)]`. No panic on any public path.

```rust
use ruthenian_orthography::{Cyrillic, to_latin, to_cyrillic};

let c = Cyrillic::parse("подъезд")?;
assert_eq!(to_latin(&c).as_str(), "pod'jezd");
assert_eq!(to_cyrillic(&to_latin(&c)).as_str(), "подъезд");
```

## The contract

`to_cyrillic(to_latin(s)) == s` for every well-formed Cyrillic string, true by
construction: the reader defines how Ruthenian is read, and the writer inserts a
separator exactly where re-reading its own output would diverge.

| Measurement | Result |
|---|---|
| Corpus round-trip (`biblija_ru.txt`, 41 462 lines, 38 623 non-empty) | **0 failures** |
| The same corpus through the reference implementation | 3 failures (lines 12695, 13444, 31725) |
| Every letter (62 cased forms), ordered pair (966) and ordered triple (30 101) | 0 failures |
| Random well-formed strings (14 144) | 0 failures |
| Guards, each verified to fail under its stated mutation | 11 of 11 |

See [`docs/ORTHOGRAPHY.md`](../../docs/ORTHOGRAPHY.md) for the normative spec,
the alphabet's context rules, and the decision record.

## Running the guards

```bash
cargo test                                                    # all but the full corpus
RUTHENIAN_CORPUS=../../biblija_ru.txt cargo test --release -- --ignored
```

The corpus is in the repository root, and the reference implementation is kept
in [`legacy/`](../../legacy), so the head-to-head needs no second clone.

The per-PR suite uses a 1 621-line fixture that includes the three lines the
reference fails, so the comparison is provable without carrying a 7.4 MB file.
The full-corpus test fails loudly when `RUTHENIAN_CORPUS` is unset rather than
passing silently.
