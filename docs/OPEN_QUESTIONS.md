# Open questions

What [`RUTHENIAN.md`](RUTHENIAN.md) still owes. Each entry blocks something
concrete, and the blocked thing is named — an open question is not a note to
self, it is a paradigm or a phase that cannot be finished.

**These are decisions, not options.** Nothing here is modelled as a runtime
switch: when one closes, the specification changes, the code changes with it, and
the changelog enumerates the affected slots (`DIRECTION.md`, "There is no
configuration axis").

**Two conventions.** Where a reflex can be counted in the modern languages,
count it — `COMPARATIVE_GRAMMAR.md` is where such counts live. Historical sound
laws cannot be measured from a synchronic corpus, so claims about them rest on
citation instead, and must be **marked as citation-based** in the spec rather
than presented as though they were counted.

---

## A. Sound correspondences — the largest gap

**The spec never says which reflex Ruthenian takes for any Common Slavic
divergence.** This affects the shape of every inherited word, and the
multi-source lexicon (§12) cannot be built without it: choosing between a
Russian, Ukrainian, Polish and OCS cognate *is* choosing a reflex.

> **Blocks:** `ruthenian-extract` entirely. Reconstruction has no defined output
> until this table exists.

One prior question decides most of the rest:

> **What does "conservative" mean here — closest to Proto-Slavic, or closest to
> the oldest attested East Slavic?**
>
> They diverge. Proto-Slavic `*golvà` gives OCS `glava` (South Slavic metathesis)
> and Russian `golova` (East Slavic pleophony). Both innovate on Proto-Slavic;
> only one is *East* Slavic. The spec calls Ruthenian "a Latin-script East Slavic
> literary language", which argues for pleophony — but that makes it
> phonologically *less* archaic than OCS, against a brief of "maximally
> conservative" throughout.

### A1. Pleophony (`*TorT`, `*TolT`, `*TerT`)

| | reflex | example |
|---|---|---|
| Russian, Ukrainian, Belarusian | `-oro-`, `-olo-`, `-ere-` | `golova`, `gorod`, `moloko` |
| OCS, South Slavic | `-ra-`, `-la-`, `-rě-` | `glava`, `grad`, `mlěko` |
| Polish, West Slavic | `-ro-`, `-ło-`, `-rze-` | `głowa`, `gród`, `mleko` |

**Measured: ~5 % of lemmas** — 22 927 of 416 038 Russian and 2 929 of 50 598
Ukrainian show a pleophonic sequence.

*Recommendation: pleophony (`golova`).* It is the defining East Slavic
innovation and the spec commits to East Slavic. Note that Russian already carries
the OCS variants as a learned register (`gorod`/`grad`), so Ruthenian could keep
**both** as a register distinction — which would suit a language already offering
the aorist as high style.

### A2. `*tj` and `*dj`

| | `*tj` | `*dj` | example |
|---|---|---|---|
| OCS | `szt` | `zzd` | `svěšta`, `mežda` |
| Russian | `cz` | `zz` | `svjecza`, `mjezza` |
| Ukrainian | `cz` | `zz` | `svicza`, `mezza` |
| Polish | `c` | `dz` | `świeca`, `miedza` |

*Recommendation: `cz`/`zz`,* the East Slavic reflex, consistent with A1.

### A3. The nasal vowels `*ǫ`, `*ę`

Lost everywhere except Polish; OCS `ѫ`/`ѧ` → East Slavic `u`/`ja`. The pure-ASCII
alphabet has no nasal letters, so the standard language is effectively forced to
`u`/`ja` — but an etymological notation (D1) would restore them, and that is
where the decision actually bites.

### A4. The jers `*ъ`, `*ь`

Havlík's law: in a chain of jers, counting back from the end, odd-numbered jers
fall and even-numbered ones vocalize. This is *the* source of the fleeting vowel
(§3.9), which the spec currently states over consonant clusters instead — a
description of the symptom.

*Recommendation: adopt the jer analysis,* and let §2's phoneme inventory carry
it, making the alternation derivable rather than listed.

> Havlík, A., "K otázce jerové v staré češtině", *Listy filologické* 16, 1889.
> Lunt, H. G., *Old Church Slavonic Grammar*, 7th ed., 2001, §2.

### A5. `*dl`, `*tl`

East and South Slavic simplify (`*mydlo` → `mylo`); West Slavic keeps the stop
(Polish `mydło`). *Recommendation: simplify,* with A1.

### A6. Initial `*je-` and `*o-`

OCS `jelenj`, `jedinъ` against East Slavic `olenj`, `odin`. *Recommendation: East
Slavic `o-`,* with A1 — though this is where OCS is most audibly more archaic.

### A7. The law of open syllables

One paragraph in §2 would make A1, A4 and A5 look like the single decision they
are.

> Schenker, A. M., *The Dawn of Slavic*, Yale, 1995, ch. 2.
> Meillet, A., *Le slave commun*, 2nd ed., Paris, 1934.

### A8. The palatalizations need their conditioning environments

§2.4 gives the outputs but not the conditions. Add the environments, the relative
chronology, and why the second and third produce the same consonants.

> Lunt, *OCS Grammar*, §3. Shevelov, *A Prehistory of Slavic*, chs. 20–22.

**General sources for A1–A8.** Shevelov, G. Y., *A Prehistory of Slavic*,
Heidelberg, 1964 (the standard treatment); Carlton, T. R., *Introduction to the
Phonological History of the Slavic Languages*, Slavica, 1991 (table-oriented);
Townsend & Janda, *Common and Comparative Slavic*, Slavica, 1996. For the
Ukrainian reflexes the spec already adopts: Shevelov, *A Historical Phonology of
the Ukrainian Language*, 1979.

---

## B. Holes in the rule-derived aspect system

§7.2 claims aspect is fully computable from surface form. Three cases break it.

> **Blocks:** the aspect implementation in `ruthenian-core`. B1 in particular —
> the rule as written produces the wrong answer for a closed class of common
> verbs.

### B1. Inherently perfective simplex verbs

`datj` "give", `sjestj` "sit down", `statj` "become", `dvinutj` "move once" are
perfective in every Slavic language **without a prefix**. Under "bare stem =
imperfective" they come out imperfective, which is wrong.

Options: (a) accept it and require `po-` on them like any other verb — regular
but semantically odd; (b) admit a small closed class of inherently perfective
roots, listed in the lexicon — which reintroduces exactly the lexical storage
§7.2 exists to avoid; (c) treat `-nu-` as a perfectivizing suffix, which handles
`dvinutj` but not `datj`.

**This is the one place "fully rule-derived" is currently false.**

### B2. Does `-yva-` apply to `po-` perfectives?

`napisatj` → `napisyvatj` is clear. Is `poczitatj` → `poczityvatj` well-formed?
If yes, `po-` behaves like a lexical prefix after all and the "empty
perfectivizer" claim weakens. If no, `po-` needs a special case.

### B3. Determinate and indeterminate motion verbs

Pan-Slavic and entirely unaddressed: `idti`/`hoditj`, `njesti`/`nositj` — a third
axis crossing aspect, present in Russian, Ukrainian, Belarusian, Polish and OCS
alike. Keep it (more grammar, more conservative) or drop it (one fewer irregular
subsystem)?

---

## C. Remaining morphology

| # | Question | Blocks | Note |
|---|---|---|---|
| C1 | **Aorist types.** OCS had three — root, sigmatic, new sigmatic (Lunt §14). The spec has one. | the aorist paradigm | Root aorists are stem-specific and partly irregular |
| C2 | **Third-person imperative.** OCS used `da` + present (`da idetъ`). Not specified. | the imperative paradigm | Cheap to add |
| C3 | **Pluperfect auxiliary** — aorist `bjeh czital` or imperfect `bjah czital`? OCS used both, with a meaning difference | the pluperfect | |
| C4 | **Predicate adjective: long or short?** §4 restores both; §10 does not say which the predicate takes. OCS used the short form | sentence generation | |
| C5 | **Ablative plural.** No attested language distinguishes it; §3.1 follows. A maximally conservative variant could revive PIE `*-i̯os` | the plural paradigm | Currently abl = dat |
| C6 | **Clitic pronouns.** OCS, Sanskrit and Interslavic all have a full/clitic opposition; Ruthenian does not | the pronoun paradigm | Fits the maximum-grammar brief |
| C7 | **The middle voice.** Lost in all Slavic, its work done by `-sja` | every verb paradigm | The most radical available conservatism |
| C8 | **Vowel quantity.** Proto-Slavic had it; Czech and Slovak keep it. Not marked | nothing yet | Would need the etymological alphabet |

C5, C6 and C7 are `RUTHENIAN.md` §13's items 2, 3 and 4.

---

## D. Scope

| # | Question |
|---|---|
| D1 | **The etymological alphabet** — ASCII plus diacritics for dictionaries, distinguishing yat, the nasals, the jers, stress and quantity. Now the only place A3, A4 and C8 can be expressed. |
| D2 | **Serbo-Croatian and Czech as lexical sources.** Both preserve what the current five do not: pitch accent and a productive aorist in Serbo-Croatian, vowel length in Czech. Polish already crossed the East Slavic line, so the boundary is one of degree. |
| D3 | **Cognate grouping** (§12.2) — the unsolved lexicon problem. 5 517 etyma carry explicit Proto-Slavic links, only 88 across all five source languages and 2 700 in just one. Needs phonological matching and gloss pivoting. **Blocks `ruthenian-extract`**, and is its own phase. |

---

## E. Internal contradictions

Found by implementing the specification. Each is a place where two parts of
`RUTHENIAN.md` disagree, so one of them is stale.

### E1. The animate accusative — prose says genitive, every table shows ablative

§3.7 and the animacy footnotes say an animate noun "takes the genitive form in
the accusative". But every paradigm table shows the **ablative** in the singular:

| | accusative sg (animate) | genitive sg | ablative sg |
|---|---|---|---|
| `dom` | `doma` | `domogo` | `doma` |
| `konj` | `konja` | `konjego` | `konja` |
| `drug` | `druga` | `drugogo` | `druga` |

The tables are self-consistent across three paradigms and are the historically
coherent reading: Slavic's animate accusative has always used the `-a` form, and
§3.1's whole argument is that this `-a` **is** the inherited ablative. The prose
appears to use "genitive" in the traditional Slavic sense, which this language
has redefined out from under it.

In the plural the question does not arise — ablative = dative there, and the
tables show the genitive (`drugov`, `zzenov`).

*Recommendation: amend the §3.7 prose to say ablative in the singular, genitive
in the plural.*

### E2. The supine — specified in §7.10a, still listed as open in §13

§7.10a specifies it completely (infinitive `-tj` against supine `-t`, governing
the **genitive**: `idu lovit zvjerjej`), and "Written in this revision" claims it.
§13 item 7 still calls it unspecified.

*Recommendation: §13 item 7 is stale; delete it.*

---

## F. Written but unfinished

- **A fleeting-vowel rule stated over jer positions** rather than consonant
  clusters, now that §2 gives a phoneme inventory (see A4).
- **Worked example texts.** The spec has paradigms and no connected prose, and
  nothing tests a grammar like translating a page into it. Suggested: the OCS
  Lord's Prayer (attested in every source language, so all six columns can stand
  side by side), a narrative passage for the aorist/imperfect contrast, and a
  technical paragraph to exercise derivation and borrowing. This will surface
  gaps no paradigm table can.
- **A frequency-ordered core vocabulary** — a Swadesh or Leipzig–Jakarta list
  realized in Ruthenian, as the minimum demonstration that A1–A8's
  correspondences actually produce usable words.
- **Clitic placement and Wackernagel's law.** §10.4 already puts `li` in second
  position without saying why; second-position placement is inherited from PIE
  and systematic in OCS. Needed if C6 is answered yes, useful regardless.
  > Wackernagel, J., *Indogermanische Forschungen* 1, 1892.
  > Radanović-Kocić, V., *The Grammar of Serbo-Croatian Clitics*, 1988.
