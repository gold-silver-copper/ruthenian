# Open questions

What [`RUTHENIAN.md`](RUTHENIAN.md) still owes. Each entry names **what it
blocks** — an open question is not a note to self, it is a paradigm or a phase
that cannot be finished.

**These are decisions, not options.** Nothing here is modelled as a runtime
switch: when one closes, the specification changes, the code changes with it, and
the changelog enumerates the affected slots.

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

> **Blocks:** the lexicon's coverage, though not its existence. The anchored path
> can be built first and the fallback added later. It blocks nothing in
> `ruthenian-core`, which needs no lexicon at all.

## 4. Do short adjectives mark animacy?

§4.2's long table gives the masculine accusative as `dobryj` / `dobrogo`, the
second marked "animate". §4.1's short table gives `dobr` with no such split. But
§4 says the short form "declines as a **noun**", and nouns do mark animacy
(§3.7).

So either the §4.1 table is incomplete, or short adjectives genuinely do not
mark it — in which case `short_adjective()` needs no `animacy` parameter and the
two adjective functions have different signatures.

> **Blocks:** `short_adjective`'s signature, and any sentence with an animate
> masculine head and an indefinite adjective (`vizzu dobr'` or `vizzu dobra`?).

## 5. Does the past passive participle double its `n`?

§7.12 gives the suffix as `-nnyj` but its short form as `poczitan`, one `n` — the
Russian pattern (`прочитанный` / `прочитан`). If the doubling is real, the long
and short participles have **different stems**, so one derivation function cannot
feed both `adjective()` and `short_adjective()`.

Regularizing to a single `-n-` throughout (`poczitanyj` / `poczitan`) gives one
stem and costs nothing the language needs, which would suit "more regular than
either".

> **Blocks:** whether `past_passive_participle()` can return one lemma. Everything
> else in the participle design depends on it returning exactly one.

## 6. Czech and Serbo-Croatian lemma counts

§12.1 lists both as secondary sources with their inventories marked *not yet
measured*. `COMPARATIVE_GRAMMAR.md`'s method requires a full scan per language
code, so `tools/measure.py` must be run for `cs` and `sh`/`sr`/`hr` before those
figures are quoted anywhere.

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

- **The hidden-consonant verb list** (§7.3). Verbs whose root-final consonant the
  infinitive does not show — `zzitj` → `zziv-`, `plytj` → `plyv-` — take a listed
  present stem. A small closed set, identifiable from Russian present stems in
  the dump.

- **The aspect-partner mapping** (§7.2). Which prefix bleaches for each
  imperfective — `czitatj` → `proczitatj`, `pisatj` → `napisatj`. Also derivable
  from Russian pairs, and also a stored list.
