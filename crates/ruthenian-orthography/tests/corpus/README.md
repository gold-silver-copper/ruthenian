# Corpus fixture

`sample.tsv` — 1 621 lines drawn from `biblija_ru.txt` (Russian Synodal Bible) at
the repository root, as of commit `49d3af7`. The source file is 41 462 lines,
38 623 of them non-empty.

Format: `<original line number>\t<text>`. The line number is kept so a failure
can be traced back to the full corpus.

## Why these lines

- **Lines 12695, 13444 and 31725 — the three the reference implementation fails.**
  Executed against the reference on 2026-07-25: it round-trips 38 620 of 38 623
  non-empty lines and breaks on exactly these three, each an `й`+vowel sequence
  read back as an iotated vowel (`Ийон` → `Ijon` → `Иён`). They are in the
  fixture so the head-to-head comparison is provable per-PR without carrying a
  7.4 MB file in the repository.
- Every line containing `ъи`, the environment that widened the hard-sign rule
  (`предъидешь`, `предъизбранным`, `предъизбрал`).
- Lines containing `ё`, so the `jo` digraph is exercised.
- An even sample across the whole file for general coverage.

## The full corpus

The fixture is the per-PR guard; it is not a substitute for the whole file. Run
the full corpus with:

```bash
RUTHENIAN_CORPUS=../../biblija_ru.txt cargo test --release -- --ignored
```

When `RUTHENIAN_CORPUS` is unset, the full-corpus test reports that it did not
run rather than passing silently.
