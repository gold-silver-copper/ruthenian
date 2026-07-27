# Open questions

What [`RUTHENIAN.md`](RUTHENIAN.md) still owes. Each entry names **what it
blocks** — an open question is not a note to self, it is a paradigm or a phase
that cannot be finished.

**One question about the language remains open**, and it blocks nothing. The rest
of this file is prose the specification is missing, not decisions it is waiting
on.

**These are decisions, not options.** Nothing here is modelled as a runtime
switch: when one closes, the specification changes, the code changes with it, and
the changelog enumerates the affected slots.

The bulk of this document has been answered and moved into the specification.
See §13 there for the settled list, and the changelog for the reasoning.

Lexicon questions — cognate grouping, source-language inventories — are out of
scope while the work is `ruthenian-core`, which needs no lexicon at all. They
belong to whichever phase builds one.

---

## The supine

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

- **The hidden-consonant verb list** (§7.3). Verbs whose root-final consonant the
  infinitive does not show — `zzitj` → `zziv-`, `plytj` → `plyv-` — take a listed
  present stem. A small closed set, identifiable from Russian present stems in
  the dump.

- **The aspect-partner mapping** (§7.2). Which prefix bleaches for each
  imperfective — `czitatj` → `proczitatj`, `pisatj` → `napisatj`. Also derivable
  from Russian pairs, and also a stored list.
