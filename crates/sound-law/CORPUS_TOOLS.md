# Optional corpus inventory

Normal tests are deterministic and offline. The `corpus_inventory` binary
computes conservative aggregate metrics from a user-supplied Git checkout; it
does not download, cache, or copy upstream data.

```sh
cargo run -p sound-law --bin corpus_inventory -- \
  mixtecaso /path/to/MixteCaSo

cargo run -p sound-law --bin corpus_inventory -- \
  diachronica /path/to/diachronica
```

The tool refuses any revision other than the pins in
`UPSTREAM_CONFORMANCE.md`. MixteCaSo reporting separates parsed/malformed rows,
unconditioned segment correspondences, conservatively representable literal
rows, and locally executed minimal witnesses. It reports every tone row as
unsupported because `sound-law` has no tone-bearing-unit model.

Diachronica reporting parses only the documented five-field processed format.
It counts contexts, excluded contexts, parallel changes, alternatives,
malformed rows, and a deliberately narrow unconditioned-literal subset. It does
not print or vendor source rows, and it makes zero linguistic-correctness
claims. A successful minimal witness means only that the extracted literal
`from` and `to` fields can execute in the current engine.

These metrics are sensitive to the conservative classifier. Changes to the
classifier must update the recorded output in `UPSTREAM_CONFORMANCE.md` after
both pinned checkouts are rerun.
