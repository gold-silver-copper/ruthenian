# Optional upstream differential checks

Normal tests do not require Haskell, Java, upstream executables, or the network.

The complete Latin-to-Portuguese translation has an ignored comparison against
the pinned Brassica CLI:

```sh
BRASSICA_BIN=/path/to/brassica \
BRASSICA_CHECKOUT=/path/to/brassica-checkout \
cargo test -p sound-law --test brassica_latin_portuguese \
  differential_against_pinned_brassica_cli -- --ignored --exact
```

The test verifies checkout revision
`f3a92ad0eb5a890fdf92208c48325371e5e770ff` before executing the source
`examples/latin2port.bsc` and `.lex`, then compares all ten lines with the Rust
translation. The current development environment has no GHC/Cabal or Brassica
binary, so the check is available but was not run here.

There is no vendored Lexurgy differential check. Its pinned build is GPL-3.0
and its feature language exceeds the shared subset; local cases instead use its
test-file taxonomy with independently authored stimuli. A future external
runner must verify revision `fa5027711cba3cd6a3f4b6defd0d38181fa98cd4`
and must not copy its declarations or test strings into this repository.
