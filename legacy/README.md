# The reference implementation

The original `ruthenian` crate, unchanged, at commit `49d3af7`. It is kept in the
tree — not deleted — for one reason: the head-to-head in
[`docs/ORTHOGRAPHY.md`](../docs/ORTHOGRAPHY.md) is then reproducible from a
single checkout.

```bash
cd legacy && cargo run --release --example test    # the original round-trip harness
```

It is **excluded from the workspace**, so it is not built, tested or linted by
the workspace gates. Its lints do not pass `-D warnings`, and fixing them is not
worth doing to code that is superseded.

What supersedes it: [`crates/ruthenian-orthography`](../crates/ruthenian-orthography),
which round-trips the same corpus with 0 failures against this implementation's 3
(lines 12695, 13444, 31725). Every defect is catalogued with its witness in the
decision record.

The corpora `biblija_ru.txt` and `biblija_ukr.txt` stay at the repository root:
they are data, and the new crate's full-corpus guard reads the Russian one.
