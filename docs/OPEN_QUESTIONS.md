# Open questions

What [`RUTHENIAN.md`](RUTHENIAN.md) still owes. Each entry names **what it
blocks** — an open question is not a note to self, it is a paradigm or a phase
that cannot be finished.

**These are decisions, not options.** Nothing here is modelled as a runtime
switch: when one closes, the specification changes, the code changes with it, and
the changelog enumerates the affected slots (`DIRECTION.md`, "There is no
configuration axis").

The bulk of this document has been answered and moved into the specification.
See §13 there for the settled list, and the changelog for the reasoning.

---

## 1. The supine

OCS distinguished the supine from the infinitive for purpose after verbs of
motion, governing the **genitive**:

```
idǫ lovitъ zvěrii     I go in order to hunt beasts   (supine, genitive object)
idǫ loviti            I go to hunt                   (infinitive)
```

Ruthenian was briefly specified with it (`-tj` infinitive against `-t` supine)
and the section was withdrawn as premature. §10.5 currently expresses purpose
with the infinitive or a `da` clause instead.

It would fit the brief — grammar follows OCS, and this is a category OCS had. It
is deferred rather than rejected.

> **Blocks:** nothing structurally. §10.5 has a working construction. Adopting it
> would add one `VerbSlot` and one government rule.

## 2. The etymological alphabet

A second, diacritic-bearing notation for dictionaries and etymology — ASCII plus
diacritics on the model of Interslavic's — distinguishing what the standard
orthography merges: yat behind `-i`, the nasals behind `u`/`ja`, the jers, and
stress.

Two decisions have made this **purely a presentation feature**. §2.6 fixes the
sound correspondences as rules, so nothing needs to look up an etymology to
inflect a word; and §3.9 abolished the fleeting vowel, which was the jers' only
remaining grammatical job. The grammar no longer depends on etymological
information at any point.

The standard orthography stays pure ASCII regardless.

> **Blocks:** nothing. It is a dictionary feature whose absence costs
> explanatory power, not correctness.

## 3. Cognate grouping where Russian has no lemma

§12.2's method is Russian-anchored: take the Russian lemma, consult the other six
only at Russian's known mergers, apply §2.6. That covers the ordinary case and
needs no etymology tags.

It does not cover a word Russian lacks entirely. There the fallback is n-way
grouping across the remaining six sources, and the data is thin: **5 517 etyma
carry explicit Proto-Slavic links, only 88 with reflexes tagged in all five
original languages, and 2 700 in just one.** Etymology templates alone will not
group them; it needs phonological matching and the English gloss as a pivot, as
slovowiki does.

> **Blocks:** `ruthenian-extract`'s coverage, though not its existence. The
> anchored path can be built first and the fallback added as its own phase.

## 4. Czech and Serbo-Croatian lemma counts

§12.1 lists both as secondary sources with their inventories marked *not yet
measured*. `DIRECTION.md` law 3 requires a full scan per language code, so
`tools/measure.py` must be run for `cs` and `sh`/`sr`/`hr` before those figures
are quoted anywhere.

> **Blocks:** the §12.1 table, and any claim about lexical coverage.

---

## Still to write

Not decisions — sections the specification is missing, listed so they are not
forgotten.

- **The conditioning environments of the palatalizations.** §2.4 gives the
  outputs but not the conditions, the relative chronology, or why the second and
  third produce the same consonants.
  > Lunt, *OCS Grammar*, §3. Shevelov, *A Prehistory of Slavic*, chs. 20–22.

- **The law of open syllables.** One paragraph in §2 would make pleophony, the
  jers and `*dl`/`*tl` look like the single development they are.
  > Schenker, *The Dawn of Slavic*, Yale, 1995, ch. 2. Meillet, *Le slave
  > commun*, 2nd ed., 1934.

- **Worked example texts.** The spec has paradigms and no connected prose, and
  nothing tests a grammar like translating a page into it. Suggested: the OCS
  Lord's Prayer, attested in every source language so all columns can stand side
  by side; a narrative passage for the aorist/imperfect contrast; and a technical
  paragraph to exercise derivation and borrowing. **This will surface gaps no
  paradigm table can** — it is the highest-value item on this list.

- **A frequency-ordered core vocabulary** — a Swadesh or Leipzig–Jakarta list
  realized in Ruthenian, as the minimum demonstration that §2.6's
  correspondences produce usable words.

- **The inherently perfective verb list** (§7.2). The class is closed and stored,
  so it has to be enumerated. Roughly 100–200 verbs, identifiable from Russian
  aspect metadata in the dump.

- **The aspect-partner mapping** (§7.2). Which prefix bleaches for each
  imperfective — `czitatj` → `proczitatj`, `pisatj` → `napisatj`. Also derivable
  from Russian pairs, and also a stored list.
