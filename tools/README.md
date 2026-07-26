# tools

Measurement and fixture generation over the **source languages**. Both read a
complete record set, never a sample — see
[`docs/COMPARATIVE_GRAMMAR.md`](../docs/COMPARATIVE_GRAMMAR.md) "Method".

> **These measure Russian, not Ruthenian.** Ruthenian is specified rather than
> attested (I7), so nothing here produces a figure about it. The fixtures
> `build_fixture.py` writes are source-language evidence for `ruthenian-extract`
> and are not used by `ruthenian-core`, which is measured against
> `docs/RUTHENIAN.md` instead.

```bash
cd ~/Desktop/code/wikidata
LC_ALL=C grep -F '"lang_code": "ru"' raw-wiktextract-data.jsonl > /tmp/ru_all.jsonl

cd ~/Desktop/code/ruthenian
python3 tools/measure.py /tmp/ru_all.jsonl        # every published figure
python3 tools/build_fixture.py < /tmp/ru_all.jsonl # regenerate the fixture
```

The `grep` filter is a superset — every record whose `lang_code` is `ru`
contains that literal — so the cache is provably complete. It matches 517 691
lines, of which 441 629 parse as Russian records; the difference is nested
occurrences, which is why the records are JSON-parsed rather than pattern-matched.

`endings.py` produces the comparative ending tables in
`docs/COMPARATIVE_GRAMMAR.md` — noun and verb endings, case syncretism, dual and
vocative inventories — for Russian, Ukrainian, Belarusian, Old Church Slavonic,
Sanskrit and Proto-Indo-European. It needs the per-language record sets named in
its docstring, each from a full scan.

`measure.py` prints the class-code census, gap counts by aspect, the `+p`
precision/recall check, the mutation table, stem-class and accent distributions,
and writes `class-codes-full.txt` plus the random held-out samples.
