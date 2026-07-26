# Prompt: completing the Ruthenian specification

Two parts. **Part 1** is decisions only the language's designer can make — they
are listed with the evidence and a recommendation, but not answered. **Part 2**
is work that needs no decision: sections that can be written from the Slavic
comparative literature, each with the source that supports it.

Target: [`docs/RUTHENIAN.md`](docs/RUTHENIAN.md), 1 408 lines as of `438e309`.

**Convention.** `INVARIANTS.md` I7 says grammar claims are measured, not quoted.
Historical sound laws cannot be measured from a synchronic dump, so Part 2 claims
rest on citations instead — and every such claim must be **marked as
citation-based** in the spec, not presented as though it were counted. Where a
reflex *can* be counted in the modern languages, count it.

---

# Part 1 — Decisions

## A. Sound correspondences — the largest gap

**The spec never says which reflex Ruthenian takes for any Common Slavic
divergence.** This is the biggest hole in the document: it affects the shape of
every inherited word, and the multi-source lexicon (§12) cannot be built without
it, because choosing between a Russian, Ukrainian, Polish and OCS cognate *is*
choosing a reflex.

There is a prior question that decides most of these at once:

> **What does "conservative" mean here — closest to Proto-Slavic, or closest to
> the oldest attested East Slavic?**
>
> They diverge. Proto-Slavic `*golvà` gives OCS `glava` (South Slavic metathesis)
> and Russian `golova` (East Slavic pleophony). Both are innovations on
> Proto-Slavic; only one is *East* Slavic. The spec calls Ruthenian "a Latin-script
> East Slavic literary language", which argues for pleophony — but that makes
> Ruthenian phonologically *less* archaic than OCS, and the brief has been
> "maximally conservative" throughout.
>
> Answer this and A1–A6 mostly follow.

### A1. Pleophony (*TorT, *TolT, *TerT)

| | reflex | example |
|---|---|---|
| Russian, Ukrainian, Belarusian | `-oro-`, `-olo-`, `-ere-` | `golova`, `gorod`, `moloko` |
| OCS, South Slavic | `-ra-`, `-la-`, `-rě-` | `glava`, `grad`, `mlěko` |
| Polish, West Slavic | `-ro-`, `-ło-`, `-rze-` | `głowa`, `gród`, `mleko` |

**Measured: ~5 % of lemmas** — 22 927 of 416 038 Russian and 2 929 of 50 598
Ukrainian show a pleophonic sequence. About one word in twenty.

*Recommendation: pleophony (`golova`).* It is the defining East Slavic
innovation and the spec commits to East Slavic. But note that Russian already
borrows the OCS variants as a learned register (`gorod`/`grad`,
`golova`/`glava`), so Ruthenian could keep **both** as a register distinction —
which would suit a language that already offers the aorist as high style.

### A2. `*tj` and `*dj`

| | `*tj` | `*dj` | example |
|---|---|---|---|
| OCS | `szt` | `zzd` | `svěšta`, `mežda` |
| Russian | `cz` | `zz` | `svjecza`, `mjezza` |
| Ukrainian | `cz` | `zz` | `svicza`, `mezza` |
| Polish | `c` | `dz` | `świeca`, `miedza` |

*Recommendation: `cz`/`zz`, the East Slavic reflex,* consistent with A1.

### A3. The nasal vowels `*ǫ`, `*ę`

Lost everywhere except Polish. OCS `ѫ`/`ѧ` → East Slavic `u`/`ja`. Polish keeps
`ą`/`ę`. Ruthenian's pure-ASCII alphabet has no nasal letters, so the answer is
effectively forced to `u`/`ja` — but the **etymological alphabet** (§13.1) would
restore them, and that is where the decision actually bites.

### A4. The jers `*ъ`, `*ь`

Havlík's law: in a chain of jers, counting back from the end, odd-numbered jers
fall and even-numbered ones vocalize. This is *the* source of the fleeting vowel
(§3.9) and Ruthenian currently states the fleeting vowel over consonant clusters
instead of over jer positions — which works but is a description of the symptom.

*Recommendation: adopt the jer analysis,* and let §2's phoneme inventory carry
it. It makes the fleeting vowel derivable rather than listed, which fits the
rule-heavy brief.

### A5. `*dl`, `*tl`

East and South Slavic simplify (`*mydlo` → `mylo`); West Slavic keeps the stop
(Polish `mydło`). *Recommendation: simplify,* with A1.

### A6. Initial `*je-` and `*o-`

OCS `jelenj`, `jedinъ` against East Slavic `olenj`, `odin`. *Recommendation:
East Slavic `o-`,* with A1 — though this is the case where OCS is most audibly
more archaic.

## B. Holes in the rule-derived aspect system

§7.2 claims aspect is fully computable from surface form. Three cases break it.

### B1. Inherently perfective simplex verbs

`datj` "give", `sjestj` "sit down", `statj` "become", `dvinutj` "move once" are
perfective in every Slavic language **without a prefix**. Under the rule "bare
stem = imperfective" they come out imperfective, which is wrong.

Options: (a) accept it and require `po-` on them like any other verb, making
`datj` imperfective and `podatj` perfective — regular but semantically odd;
(b) admit a small closed class of inherently perfective roots, listed in the
lexicon — which reintroduces exactly the lexical storage §7.2 exists to avoid;
(c) treat `-nu-` as a perfectivizing suffix, which handles `dvinutj` but not
`datj`.

*This is the one place where "fully rule-derived" is currently false, and it
needs an answer before `ruthenian-core` implements aspect.*

### B2. Does `-yva-` apply to `po-` perfectives?

`napisatj` → `napisyvatj` is clear. Is `poczitatj` → `poczityvatj` well-formed?
If yes, `po-` behaves like a lexical prefix after all and the "empty
perfectivizer" claim weakens. If no, `po-` needs a special case.

### B3. Determinate and indeterminate motion verbs

Pan-Slavic and entirely unaddressed: `idti`/`hoditj`, `njesti`/`nositj` — a
third axis crossing aspect, present in Russian, Ukrainian, Belarusian, Polish and
OCS alike. Keep it (more grammar, more conservative) or drop it (one fewer
irregular subsystem)?

## C. Remaining morphology

| # | Question | Note |
|---|---|---|
| C1 | **Aorist types.** OCS had three — root, sigmatic, new sigmatic (Lunt §14). The spec has one. | More grammar, but root aorists are stem-specific and partly irregular |
| C2 | **Third-person imperative.** OCS used `da` + present (`da idetъ` "let him go"). Not specified. | Cheap to add |
| C3 | **Pluperfect auxiliary** — aorist `bjeh czital` or imperfect `bjah czital`? OCS used both, with a meaning difference | |
| C4 | **Predicate adjective: long or short?** §4 restores both but §10 does not say which the predicate takes. OCS used the short form | Blocks sentence generation |
| C5 | **Ablative plural.** No attested language distinguishes it. A maximally conservative variant could revive PIE `*-i̯os` | |
| C6 | **Clitic pronouns.** OCS, Sanskrit and Interslavic all have a full/clitic opposition; Ruthenian does not | Fits the maximum-grammar brief |
| C7 | **The middle voice.** Lost in all Slavic, its work done by `-sja` | The most radical available conservatism |
| C8 | **Vowel quantity.** Proto-Slavic had it; Czech and Slovak keep it. Ruthenian does not mark it | Would need the etymological alphabet |

## D. Scope

| # | Question |
|---|---|
| D1 | **The etymological alphabet** — ASCII plus diacritics for dictionaries, distinguishing yat, the nasals, the jers, stress and quantity. Now the only place A3, A4 and C8 can be expressed |
| D2 | **Serbo-Croatian and Czech as lexical sources.** Both preserve what the current five do not: pitch accent and a productive aorist in Serbo-Croatian, vowel length in Czech |
| D3 | **Cognate grouping method** — the unsolved lexicon problem (§12.2). 5 517 etyma have explicit Proto-Slavic links, only 88 across all five languages |

---

# Part 2 — Sections that can be written from the literature

No decisions required; each needs the named source and must be marked as
citation-based per I7.

## P1. A sound-correspondence table, PIE → Proto-Slavic → the six languages

The single most useful addition. A table giving, for each Common Slavic
divergence (A1–A6 plus the palatalizations, `*x`, the liquid diphthongs, initial
`*v-`), the reflex in OCS, Russian, Ukrainian, Belarusian, Polish and Ruthenian.
It turns the multi-source lexicon from an aspiration into a procedure: given
cognates in four languages, the table says what the Ruthenian form must be.

> Shevelov, G. Y., *A Prehistory of Slavic: The Historical Phonology of Common
> Slavic*, Heidelberg, 1964 — the standard treatment.
> Carlton, T. R., *Introduction to the Phonological History of the Slavic
> Languages*, Slavica, 1991 — more accessible, table-oriented.
> Townsend, C. E. & Janda, L. A., *Common and Comparative Slavic: Phonology and
> Inflection*, Slavica, 1996.

## P2. The fleeting vowel as a jer rule

Restate §3.9 over Havlík's law rather than over consonant clusters, using §2's
phoneme inventory. Makes the alternation derivable.

> Havlík, A., "K otázce jerové v staré češtině", *Listy filologické* 16, 1889.
> Lunt, H. G., *Old Church Slavonic Grammar*, 7th ed., Mouton de Gruyter, 2001,
> §2 — the clearest modern statement.

## P3. The three palatalizations, with conditioning environments

§2.4 gives the outputs but not the conditions. Add the environments, the relative
chronology, and why the second and third produce the same consonants.

> Lunt, *OCS Grammar*, §3.
> Shevelov, *A Prehistory of Slavic*, chs. 20–22.

## P4. OCS aorist typology

Needed if C1 is answered "all three". Root, sigmatic and new-sigmatic
formations, with the stem conditions on each.

> Lunt, *OCS Grammar*, §14.
> Vaillant, A., *Grammaire comparée des langues slaves*, vol. III, 1966.

## P5. Clitic placement and Wackernagel's law

Needed if C6 is answered yes, and useful regardless: §10.4 already puts `li` in
second position without saying why. Second-position clitic placement is
inherited from PIE and is systematic in OCS.

> Wackernagel, J., "Über ein Gesetz der indogermanischen Wortstellung",
> *Indogermanische Forschungen* 1, 1892.
> Radanović-Kocić, V., *The Grammar of Serbo-Croatian Clitics*, 1988, for the
> modern Slavic treatment.

## P6. Accent paradigms and their PIE origin

Ruthenian has fixed stress, so this is not needed for the standard language —
but the etymological alphabet (D1) would mark accent, and the accent paradigms
a/b/c are the framework for doing so.

> Stang, C. S., *Slavonic Accentuation*, Oslo, 1957 — the foundation.
> Dybo, V. A., *Славянская акцентология*, Moscow, 1981.
> Illič-Svityč, V. M., *Именная акцентуация в балтийском и славянском*, 1963.

## P7. The law of open syllables

Explains why Proto-Slavic reshaped so much inherited material, and underlies
A1, A4 and A5 together. One paragraph in §2 would make those three decisions
look like one decision, which is what they are.

> Schenker, A. M., *The Dawn of Slavic: An Introduction to Slavic Philology*,
> Yale, 1995, ch. 2.
> Meillet, A., *Le slave commun*, 2nd ed., Paris, 1934.

## P8. Ukrainian and Belarusian historical phonology

Ruthenian takes the Ukrainian yat reflex (§2.1) and the Ukrainian second
palatalization; the sources for those specific developments should be cited
where the spec relies on them.

> Shevelov, G. Y., *A Historical Phonology of the Ukrainian Language*,
> Heidelberg, 1979.
> Wexler, P., *A Historical Phonology of the Belorussian Language*, 1977.

## P9. Worked example texts

The spec has paradigms and no connected prose. Nothing tests a grammar like
translating a page into it. Suggested: the OCS Lord's Prayer (attested in every
source language, so all six columns can be shown side by side), a passage of
narrative for the aorist/imperfect contrast, and a technical paragraph to
exercise the derivation and borrowing rules.

This will surface gaps no paradigm table can.

## P10. A frequency-ordered core vocabulary

Deferred to the lexicon phase, but the spec should state the target: a Swadesh
or Leipzig–Jakarta list realized in Ruthenian, as the minimum demonstration that
the sound correspondences of P1 actually produce usable words.

---

# Working rules

- Answer Part 1 A first — the "what does conservative mean" question governs
  A1–A6 and much of Part 2.
- B1 blocks `ruthenian-core`'s aspect implementation and C4 blocks sentence
  generation; both are cheap to answer and expensive to leave.
- Mark every citation-based claim as such (I7). Where a reflex can be counted in
  the modern languages, count it and say so.
- Keep the spec's existing discipline: every restored feature attested, every
  regularization precedented, and the cost of each stated plainly.
