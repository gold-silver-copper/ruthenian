# Comparative grammar by word class

Proto-Indo-European · Sanskrit · Old Church Slavonic · Russian · Ukrainian ·
Belarusian · Interslavic — and the design of Ruthenian

Seven languages, one word class at a time, with measured ending inventories, and
a final part designing a **conservative but regular** Ruthenian that carries the
ablative, the vocative and the dual.

The order is **genetic, not alphabetical**, because the interesting fact in almost
every table is the shape of the erosion from left to right. PIE is the
reconstructed ancestor, Sanskrit the conservative early attestation, OCS the
Slavic starting point, the East Slavic three the immediate design space, and
Interslavic the only column representing a *choice* rather than an inheritance.

**Nothing here imports Interslavic data into the crate.** This is a grammar
comparison; the lexicon's single source remains the English Wiktionary dump.

## Method

Every countable claim is counted over the **whole** dump (`INVARIANTS.md` I1 and
I7). Full scans: 441 629 Russian records, 124 791 across `uk`/`be`/`cu`/`sa`, and
1 894 Proto-Indo-European reconstructions (`ine-pro`).

**How the ending tables were built.** For each lemma, the stem is the longest
common prefix over all its attested forms with combining marks stripped; the
ending is the form minus that stem. Percentages are the share of that
language's lemmas taking that ending in that cell. This over-segments where a
paradigm has stem alternation — a velar stem shows `ки` rather than `и`, because
palatalization moved the boundary — so consonant-initial "endings" in the tables
below are stem-final consonants travelling with the true ending. That is a
property of the method and is visible rather than hidden.

PIE figures are counts of **reconstructed** forms: they describe what comparative
reconstruction posits, not a corpus. Interslavic has no corpus here, so its
column is descriptive.

## References

- Fortson, B. W. IV, *Indo-European Language and Culture: An Introduction*, 2nd
  ed., Wiley-Blackwell, 2010.
- Beekes, R. S. P., *Comparative Indo-European Linguistics: An Introduction*, 2nd
  ed., Benjamins, 2011.
- Whitney, W. D., *Sanskrit Grammar*, 2nd ed., 1889.
- Lunt, H. G., *Old Church Slavonic Grammar*, 7th rev. ed., Mouton de Gruyter,
  2001.
- Comrie, B. & Corbett, G. G. (eds), *The Slavonic Languages*, Routledge, 1993.
- Vaillant, A., *Grammaire comparée des langues slaves*, 1950–77.
- Zaliznyak, A. A., *Русское именное словоизменение*, 1967; *Грамматический
  словарь русского языка*, 1977 (6th ed. 2010).
- *Русская грамматика*, ed. N. Yu. Shvedova, Academy of Sciences, 1980.
- Jakobson, R., "Russian Conjugation", *Word* 4, 1948; Townsend, C. E., *Russian
  Word Formation*, 1975.
- van Steenbergen, J. & Merunka, V., *Interslavic* normative grammar.

---

# 1. Nouns

## 1.1 Case inventory

| Case | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| nominative | 3 906 | 23 138 | 6 004 | ✓ | 35 961 | 7 941 | ✓ |
| accusative | 3 757 | 21 174 | 6 142 | ✓ | 25 838 | 5 505 | ✓ |
| genitive | 1 851 | 19 075 | 6 417 | ✓ | 51 542 | 11 233 | ✓ |
| dative | 2 638 | 17 638 | 6 135 | ✓ | 31 654 | 5 501 | ✓ |
| instrumental | 2 343 | 21 269 | 6 165 | ✓ | 25 185 | 6 519 | ✓ |
| locative / prepositional | 2 745 | 18 512 | 6 187 | ✓ | 31 034 | 5 512 | ✓ |
| **vocative** | 3 698 | 23 227 | 6 186 | 40 (relic) | **25 180** | 21 (relic) | form kept, case denied |
| **ablative** | **2 614** | **18 656** | — | — | — | — | — |
| partitive | — | — | — | 206 | — | — | — |
| 2nd locative | — | — | — | 199 | — | — | — |
| count form | — | — | — | 31 | — | — | — |

PIE and Sanskrit have eight. Slavic loses the **ablative** into the genitive and
arrives at OCS's seven with a productive vocative. Then East Slavic splits:
Ukrainian keeps the vocative (25 180 forms — one per noun), Belarusian lost it
(21), Russian lost it and grew four *new* marginal cases instead.

## 1.2 Case syncretism — measured

The most important table in the document, because it constrains what a revived
case system can look like. Percentages are how often two cases are **identical**
in the same lemma.

| | number | abl=dat | abl=gen | dat=ins | gen=loc | nom=acc | nom=voc |
|---|---|---|---|---|---|---|---|
| **PIE** | singular | **0 %** | 68 % | 0 % | 0 % | 29 % | 45 % |
| | dual | — | — | — | — | 100 % | 100 % |
| | plural | **100 %** | 0 % | 0 % | 0 % | 19 % | 100 % |
| **Sanskrit** | singular | **0 %** | 35 % | 0 % | 0 % | 27 % | 5 % |
| | dual | **100 %** | 0 % | **100 %** | **100 %** | 99 % | 99 % |
| | plural | **100 %** | 0 % | 0 % | 0 % | 40 % | 99 % |
| **OCS** | singular | — | — | 0 % | 14 % | 74 % | 21 % |
| | dual | — | — | **99 %** | **99 %** | 100 % | 99 % |
| | plural | — | — | 0 % | 0 % | 54 % | 90 % |
| **Russian** | singular | — | — | 0 % | 23 % | 49 % | 0 % |
| | plural | — | — | 0 % | — | 72 % | — |
| **Ukrainian** | singular | — | — | 0 % | 25 % | 50 % | 14 % |
| | plural | — | — | 0 % | 1 % | 74 % | 99 % |
| **Belarusian** | singular | — | — | 0 % | 18 % | 49 % | — |
| | plural | — | — | 0 % | 1 % | 72 % | — |

Three findings that decide the Ruthenian design:

1. **The ablative is only ever distinct in the singular.** PIE: `abl=dat` 0 % in
   the singular, **100 % in the plural**. Sanskrit: identical picture, plus
   `abl=dat` 100 % in the dual. No attested Indo-European language of this group
   maintains a distinct ablative outside the singular. A Ruthenian ablative that
   is distinct in all three numbers would be a novelty, not a restoration.
2. **The dual has exactly three distinct forms.** In both Sanskrit and OCS:
   NOM=ACC=VOC, GEN=LOC, DAT=INS(=ABL). This holds at 99–100 % in both, across
   completely different case inventories — an eight-case language and a
   seven-case one converge on a three-form dual.
3. **The vocative plural does not exist.** `nom=voc` is 90–100 % in the plural
   everywhere it is attested (PIE 100 %, Sanskrit 99 %, OCS 90 %, Ukrainian 99 %)
   but only 5–45 % in the singular. The vocative is a **singular-only** category.

## 1.3 Noun endings — measured

Most common endings per cell, with the share of lemmas taking them. Cyrillic as
attested; the Ruthenian column in Part 13 converts.

### Old Church Slavonic, masculine (*o*-stem dominant)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nom | `-ъ` 35 % | `-a` 36 % | `-i` 46 % |
| voc | `-e` 34 %, `-če` 35 % | `-a` | `-i` 45 % |
| acc | `-ъ` 35 % | `-a` | `-y` 37 % |
| gen | `-a` 37 % | `-u` 38 % | `-ъ` 36 % |
| dat | `-u` 37 % | `-oma` 33 % | `-omъ` 33 % |
| ins | `-omь` 33 % | `-oma` 33 % | `-y` 35 % |
| loc | `-ě` 34 % | `-u` 38 % | `-ěxъ` 33 % |

### Old Church Slavonic, feminine (*a*-stem dominant)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nom | `-a` 50 % | `-i` 51 %, `-ě` 42 % | — |
| voc | `-o` 42 %, `-i` 30 % | — | — |
| acc | `-ǫ` 52 % | `-i`/`-ě` | — |
| gen | `-y` 42 %, `-i` 29 % | `-u` 53 % | — |
| dat | `-i` 50 %, `-ě` 42 % | `-ama` 52 % | — |
| ins | `-ojǫ` 42 % | `-ama` | — |
| loc | `-i` 51 %, `-ě` 42 % | `-u` | — |

### Russian

| Case | Masc sg | Masc pl | Fem sg | Fem pl | Neut sg | Neut pl |
|---|---|---|---|---|---|---|
| nom | `-∅` 74 % | `-y` 52 %, `-i` 31 % | `-a` 37 % | `-i` 38 %, `-y` 28 % | `-e` 66 %, `-o` 20 % | `-ja` 63 %, `-a` 20 % |
| acc | `-∅` 50 %, `-a` 24 % | `-y` 36 %, `-ov` 23 % | `-u` 37 % | `-i` 35 % | = nom | = nom |
| gen | `-a` 74 %, `-ja` 8 % | `-ov` 71 % | `-i` 40 %, `-y` 28 % | `-∅` 36 % | `-ja` 63 %, `-a` 23 % | `-i` 62 %, `-∅` 21 % |
| dat | `-u` 74 % | `-am` 77 % | `-e` 39 %, `-i` 29 % | `-am` 36 % | `-ju` 63 % | `-jam` 63 % |
| ins | `-om` 72 %, `-em` 10 % | `-ami` 77 % | `-oj` 30 %, `-ej` 24 % | `-ami` 36 % | `-em` 66 % | `-jami` 63 % |
| prep | `-e` 83 % | `-ah` 77 % | `-e` 39 %, `-i` 29 % | `-ah` 36 % | `-i` 62 % | `-jah` 63 % |
| *2nd loc* | `-u` 81 % | — | `-i` 100 % | — | — | — |
| *partitive* | `-u` 77 % | — | — | — | — | — |
| *vocative* | `-če` 27 %, `-e` 17 % | — | `-∅` 44 %, `-o` 33 % | — | — | — |

### Ukrainian

| Case | Masc sg | Masc pl | Fem sg |
|---|---|---|---|
| nom | `-∅` 70 % | `-y` 66 %, `-i` 12 % | `-a` 28 %, `-ja` 24 % |
| voc | **`-e` 51 %, `-u` 18 %** | = nom (99 %) | **`-o` 25 %, `-je` 16 %** |
| acc | `-∅` 49 %, `-a` 20 % | `-y` 46 %, `-iv` 23 % | `-u` 28 %, `-ju` 24 % |
| gen | `-a` 41 %, **`-u` 27 %** | `-iv` 76 % | `-i` 29 %, `-y` 25 % |
| dat | **`-ovi` 65 %** | `-am` 70 % | `-i` 55 % |
| ins | `-om` 65 % | `-amy` 70 % | `-oju` 26 % |
| loc | `-i` 27 %, `-u` 22 %, `-ovi` 19 % | `-ah` 70 % | `-i` 55 % |

Ukrainian's masculine dative `-ovi` (65 %) is the old *u*-stem ending
generalized — a natural-language regularization of exactly the kind Ruthenian is
considering, and evidence that it works.

### Sanskrit, masculine *a*-stem (the thematic paradigm)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nom | `-aḥ` | `-au` | `-āḥ` |
| voc | `-a` (∅) | `-au` | `-āḥ` |
| acc | `-am` | `-au` | `-ān` |
| gen | `-asya` | `-ayoḥ` | `-ānām` |
| **abl** | **`-āt`** | `-ābhyām` | `-ebhyaḥ` |
| dat | `-āya` | `-ābhyām` | `-ebhyaḥ` |
| ins | `-ena` | `-ābhyām` | `-aiḥ` |
| loc | `-e` | `-ayoḥ` | `-eṣu` |

The ablative singular `-āt` is the only cell where it is distinct; everywhere
else it equals the dative.

### PIE, thematic

Reconstructed: nom sg `*-os`, voc `*-e` (29 %), acc `*-om`, gen `*-osyo`, **abl
`*-ōd`**, dat `*-ōi`, ins `*-oh₁`, loc `*-oi`; dual nom/voc/acc `*-oh₁` (32 %);
plural nom `*-ōs`, acc `*-ons`, gen `*-ōm`, dat/abl `*-obʰos`, ins `*-ōis`, loc
`*-oisu`.

**The single most important etymological fact for this project:** Slavic's
`o`-stem genitive singular `-a` continues **PIE `*-ōd`, the ablative**. The PIE
genitive `*-osyo` did not survive in Slavic nouns — it survives only in the
pronominal and adjectival declension, as `-ogo` (`togo`, `dobrogo`). So Slavic
did not "lose the ablative"; it lost the *genitive* and reassigned the ablative
form to genitive function. That fact makes a Ruthenian ablative recoverable from
material the language already has, and Part 13 uses it.

## 1.4 Number

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| singular | 14 968 | 144 285 | 97 919 | ✓ | ✓ | 29 799 | ✓ |
| **dual** | **9 052** | **143 771** | **77 714** | — | — | — | — |
| plural | 15 403 | 153 641 | 92 189 | ✓ | ✓ | 38 248 | ✓ |

In Sanskrit the dual is **as frequent as the singular**. In OCS it is ~30 % of
forms. In all four modern lects it is gone, surviving as fossils: Russian
`два часа́`, `глаза`, `рога`, and the genitive singular after 2–4 — petrified dual
agreement, and the origin of the count form.

### Dual endings — measured

| | NOM=ACC=VOC | GEN=LOC | DAT=INS(=ABL) |
|---|---|---|---|
| PIE (thematic) | `*-oh₁` 32 % | — | — |
| Sanskrit | `-au` 40 %, `-e` 32 % | `-yoḥ` 84 % | `-bhyām` 73 % |
| OCS | `-a` (masc), `-ě`/`-i` (fem) | `-u` 46 % | `-oma` 25 %, `-ama` 17 % |

Two independent languages, three forms each, the same three groupings. This is
the strongest structural constraint in the document.

## 1.5 Gender and animacy

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian |
|---|---|---|---|---|---|---|
| masculine | 113 | 3 356 | 153 | 105 388 | 11 204 | 1 365 |
| feminine | 104 | 1 556 | 114 | 73 791 | 11 142 | 1 365 |
| neuter | 115 | 1 962 | 87 | 27 363 | 3 031 | 367 |

PIE's three are near-equal (and are usually reconstructed as later than an
original animate/inanimate split — Fortson §6). Slavic neuter has eroded
steadily.

**Animacy** is a Slavic innovation absent from PIE and Sanskrit: masculine
accusative = genitive for animates, = nominative for inanimates. The measured
`nom=acc` rates above (49–50 % singular in all three East Slavic) are this rule
showing up as a statistic. Present in all four Slavic lects; Interslavic keeps it.

## 1.6 Declension classes

| | Classes |
|---|---|
| PIE | thematic (*o*-stem) vs athematic (consonant, *i*-, *u*-, *r*-, *n*-stems); ablaut-graded |
| Sanskrit | vowel stems (a, ā, i, ī, u, ū, ṛ) and consonant stems |
| OCS | *o*-, *jo*-, *a*-, *ja*-, *i*-, *u*-, consonant, *ū*-stems |
| Russian | 8 types (Zaliznyak), keyed on the **graphic** stem ending |
| Ukrainian | 4 declensions |
| Belarusian | 3 declensions plus indeclinables |
| Interslavic | 3 (hard/soft × gender) |

One trajectory run to different depths: a **stem-based** system (what the stem
historically ended in) reanalysed as a **gender-based** one. Interslavic completes
it; Russian is mid-transition, which is why Zaliznyak's types are defined
orthographically.

## 1.7 Stress and ablaut

| | System |
|---|---|
| PIE | free **pitch** accent; mobile paradigms; **ablaut** (e/o/zero) grammatically productive |
| Sanskrit | pitch accent in Vedic, lost in Classical |
| OCS | free, mobile; the Proto-Slavic accent paradigms |
| Russian | free, mobile — 6 patterns + 4 primed (`RUSSIAN_GRAMMAR.md` §2) |
| Ukrainian | free, mobile |
| Belarusian | free, mobile, **orthographically load-bearing** (*akanne*: unstressed `o` written `a`) |
| Interslavic | **unspecified**; not marked |

**PIE ablaut is the ancestor of Slavic fleeting vowels.** The e/o/zero alternation
became, via the jers, the appearing and vanishing vowels of `okno`/`okon`. What
looks like a Russian irregularity is a very old morphological device in its final
decayed state.

---

# 2. Adjectives

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| agreement | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| declines like a noun | ✓ | ✓ | (short form) | (short form) | vestigial | vestigial | — |
| long/short | — | — | **both productive** | long + short (predicative) | short vestigial | short vestigial | long only |
| comparative | synthetic | synthetic | synthetic | synth. + analytic | synth. + analytic | synth. + analytic | regular `-ějši` |
| superlative | — | synthetic | prefix | `самый` + long | `най-` | `най-` | `naj-` |

**Russian long-form endings** (hard stem): masc sg nom `-yj`, gen `-ogo`, dat
`-omu`, ins `-ym`, prep `-om`; fem sg nom `-aja`, gen/dat/ins/prep `-oj`, acc
`-uju`; neut nom/acc `-oje`; plural nom `-yje`, gen/prep `-yh`, dat `-ym`, ins
`-ymi`. Soft stems substitute `-ij/-jego/-jemu/-im/-jem` etc. — and are **155 of
9 999 adjectives = 1.6 %**.

The long/short split is a **Slavic innovation**: PIE and Sanskrit adjectives
simply decline like nouns. OCS had a real indefinite/definite opposition
(`dobrъ` vs `dobrъjь`), the definite formed by suffixing the anaphoric pronoun —
the ancestor of the modern long form, and the source of the `-ogo` ending that
Part 13 puts to work. Modern East Slavic has bleached this to
predicative/attributive, and the short form is receding: **4 571 of 9 999**
Russian adjectives have short forms, with no rule predicting which.

---

# 3. Pronouns

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| 3 persons | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **dual personal** | ✓ | ✓ | ✓ | — | — | — | — |
| clitic / enclitic series | ✓ | ✓ (`mā`, `tvā`, `naḥ`) | ✓ (`mę`, `tę`, `sę`) | — (bound `-sja` only) | — | — | ✓ (restored) |
| post-prepositional *n-* | — | — | ✓ | ✓ | ✓ | ✓ | ✓ |
| reflexive | ✓ | ✓ (`ātman-`) | ✓ | ✓ | ✓ | ✓ | ✓ |

Clitics run PIE → Sanskrit → OCS → *lost in East Slavic* → **deliberately restored
by Interslavic**. The ***n-* prefix** (`u njego`) is a Slavic innovation from a
reanalysed preposition-final nasal, absent from PIE and Sanskrit.

Pronouns are also where the PIE genitive `*-osyo` survives in Slavic, as `-ogo`.

---

# 4. Numerals

| | After the numeral |
|---|---|
| PIE | 1–4 adjectival and agreeing; higher numerals nominal |
| Sanskrit | 1–4 decline and agree; 5+ decline without agreeing |
| OCS | 1–4 agree, **dual for 2**; 5+ feminine *i*-stem nouns taking the genitive |
| **Russian** | 1 agrees; **2–4 genitive singular** (petrified dual); 5+ genitive plural; 11–14 override |
| Ukrainian | 1 agrees; 2–4 **nominative plural**; 5+ genitive plural |
| Belarusian | as Ukrainian |
| Interslavic | as Ukrainian |

**The clearest case where Russian is the odd one out**, and the one rule that most
reliably defeats learners. Note the interaction with Part 13: if Ruthenian
restores the dual, the Russian genitive singular after 2 stops being a fossil and
becomes recoverable as *real dual agreement*.

---

# 5. Verbs

## 5.1 Categories

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| present | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **aorist** | ✓ | ✓ | ✓ | — | — | — | optional |
| **imperfect** | ✓ | ✓ | ✓ | — | — | — | optional |
| perfect | ✓ | ✓ | periphrastic | — | — | — | ✓ |
| past (l-participle) | — | — | ✓ | ✓ | ✓ | ✓ | ✓ |
| future | ✓ | ✓ | periphrastic | ✓ | ✓ | ✓ | ✓ |
| conditional | — | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| imperative | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **subjunctive** | ✓ (3 686) | ✓ | — | — | — | — | — |
| **optative** | ✓ (3 854) | ✓ | — | — | — | — | — |
| **injunctive** | ✓ | ✓ | — | — | — | — | — |
| **middle voice** | ✓ | ✓ | — | — | — | — | — |
| passive | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **aspect** | — | — | emerging | **pervasive** | ✓ | ✓ | ✓ |
| supine | — | — | ✓ | — | — | — | optional |

**The great trade.** Slavic exchanged a rich tense–mood system for a pervasive
aspect system. PIE's measured mood counts — indicative 7 325, **optative 3 854,
subjunctive 3 686** — show how central those moods were. Modern Slavic has one
past tense and instead marks perfective/imperfective on every verb: Russian
5 517 imperfective / 4 902 perfective. The **middle voice**'s work went to the
reflexive, which is why Russian has 3 182 reflexive verbs.

## 5.2 Personal endings — measured

### PIE present (thematic)

| | Singular | Dual | Plural |
|---|---|---|---|
| 1 | `*-oh₂` 61 % | `*-owos` 62 % | `*-omos` 62 % |
| 2 | `*-esi` 62 % | `*-etes` 65 % | `*-ete` 62 % |
| 3 | `*-eti` 61 % | `*-etes` 65 % | `*-onti` 62 % |

Past: 1sg `*-om`, 2sg `*-es`.

### OCS present

| | Singular | Dual | Plural |
|---|---|---|---|
| 1 | `-ǫ` 44 %, `-jǫ` 13 % | `-evě` 40 % | `-emъ` 40 % |
| 2 | `-eši` 40 %, `-iši` 22 % | `-eta` 40 % | `-ete` 40 % |
| 3 | `-etъ` 40 %, `-itъ` 22 % | `-ete` 40 % | `-ǫtъ` 40 %, `-ętъ` 22 % |

OCS aorist (`byti`): 1sg `-běxъ`, 2/3sg `-bě`, 1du `-běxově`, 2du `-běsta`, 3du
`-běste`, 1pl `-běxomъ`, 2pl `-běste`, 3pl `-běšę`. Imperfect: 1sg `-ěaxъ`, 2/3sg
`-ěaše`, 1pl `-ěaxomъ`, 3pl `-ěaxǫ`.

### Russian present

| | Singular | Plural |
|---|---|---|
| 1 | `-ju` 65 %, `-u` 4 % | `-em` 64 %, `-im` 7 % |
| 2 | `-esh` 64 %, `-ish` 7 % | `-ete` 64 %, `-ite` 7 % |
| 3 | `-et` 64 %, `-it` 7 % | `-jut` 61 %, `-jat` 5 % |

The reflexive `-sja`/`-sj` appears as a suffix on ~20 % of every cell — visible
in the measurement as `-jusj`, `-etsja` and so on.

### Sanskrit future

1sg `-āmi`, 2sg `-si`, 3sg `-ti`; 1du `-āvaḥ`, 2du `-thaḥ`, 3du `-taḥ`; 1pl
`-āmaḥ`, 2pl `-tha`, 3pl `-nti`.

Note the **three-way dual in the verb too**, and that the OCS dual verb endings
(`-evě`, `-eta`, `-ete`) are cognate with Sanskrit's (`-vaḥ`, `-thaḥ`, `-taḥ`).

## 5.3 Conjugation classes

| | Classes |
|---|---|
| PIE | thematic vs athematic; present-stem formations (root, `*-ye/o-`, nasal-infix, reduplicated) |
| Sanskrit | **10 present classes**, plus causative/desiderative/intensive |
| OCS | 5 present classes on the theme vowel |
| Russian | 2 conjugations on the surface; **16 Zaliznyak classes** underneath (1–6 = 90.7 %) |
| Ukrainian | 2 conjugations, 13 classes |
| Belarusian | 2 conjugations |
| Interslavic | 2, fully regular |

---

# 6. Participles and verbal adverbs

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| present active | ✓ | ✓ | ✓ | ✓ | restricted | restricted | ✓ |
| past active | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| present passive | ✓ | ✓ | ✓ | ✓ | rare | rare | ✓ |
| past passive | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| gerund / absolutive | — | ✓ | ✓ | ✓ (2) | ✓ | ✓ | ✓ |
| declines | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

Russian suffixes: present active `-uszczij`/`-jaszczij`, past active `-vszij`,
present passive `-jemyj`/`-imyj`, past passive `-nnyj`/`-jonnyj`/`-tyj`, gerunds
`-ja` and `-v`.

Ukrainian **restricts** the present active participle prescriptively in favour of
relative clauses — a standardization decision made by a natural language.

---

# 7–11. The closed classes

**Adverbs.** Formed from adjectives throughout: Slavic `-o`/`-e`, Sanskrit and PIE
the neuter accusative singular — the same device.

**Prepositions.** In PIE, adverbial particles, not yet a word class; the case
system did the work, which is exactly why PIE needs eight cases and Russian needs
prepositions. Slavic grammaticalized them into case-governing prepositions, and
the governed case is **lexical** — stored, not derived.

**Conjunctions, particles, interjections.** Indeclinable everywhere. Russian's
`že`, `li`, `by`, `-to` are closest to OCS; Sanskrit's `ca`, `vā`, `hi` are
enclitic, and both continue PIE second-position clitics.

---

# 12. Summary: what each language did

| Feature | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| cases | 8 | 8 | 7 | 6 (+4 marginal) | 7 | 6 | 6 |
| numbers | 3 | 3 | 3 | 2 | 2 | 2 | 2 |
| genders | 3 | 3 | 3 | 3 | 3 | 3 | 3 |
| ablative | ✓ sg only | ✓ sg only | — | — | — | — | — |
| vocative | ✓ sg only | ✓ sg only | ✓ sg only | relic | ✓ | relic | form only |
| dual | ✓ 3 forms | ✓ 3 forms | ✓ 3 forms | — | — | — | — |
| aspect | — | — | emerging | ✓ | ✓ | ✓ | ✓ |
| moods beyond imperative | 3 | 3 | — | — | — | — | — |
| verb conj. classes | thematic/athematic | 10 | 5 | 16 | 13 | ~2 | 2 |
| noun decl. classes | thematic/athematic | ~8 | 8 | 8 | 4 | 3 | 3 |
| stress | pitch, mobile | pitch (Vedic) | mobile | mobile, 10 patterns | mobile | mobile + *akanne* | unmarked |

---

# 13. Designing Ruthenian: conservative but regular

> The conclusions of this part are specified normatively, with full paradigms and
> example words, in [`RUTHENIAN.md`](RUTHENIAN.md). What follows is the reasoning.

The brief: **more conservative than Russian or Ukrainian, but more regular than
either**, carrying the **ablative, vocative and dual**. The tables above make
that a solvable problem rather than a wish, because they say precisely what a
realistic version looks like.

## 13.1 Three constraints the data imposes

1. **The ablative is a singular-only category.** PIE `abl=dat` 0 % singular,
   100 % plural; Sanskrit the same plus 100 % in the dual. Giving Ruthenian a
   distinct ablative in all three numbers would be inventing, not restoring.
2. **The dual has three forms, not six or eight.** NOM=ACC=VOC, GEN=LOC,
   DAT=INS(=ABL) — at 99–100 % in both Sanskrit and OCS. A fully differentiated
   dual is unattested anywhere in the family.
3. **The vocative is singular-only.** `nom=voc` is 90–100 % in the plural
   everywhere. A vocative plural would be an innovation.

Following all three gives a system that is *more conservative* than any modern
Slavic language while being *smaller and more regular* than a naive
eight-case × three-number grid: **8 cases × 3 numbers = 24 cells nominally, but
only 15 distinct forms**.

## 13.2 The ablative problem, and its solution

Slavic did not simply lose the ablative. **The Slavic `o`-stem genitive singular
`-a` *is* the PIE ablative `*-ōd`**; the PIE genitive `*-osyo` was lost from
nouns and survives only in the pronominal/adjectival `-ogo` (`togo`, `dobrogo`).
Slavic merged the two cases by keeping the ablative form and giving it genitive
function.

That gives Ruthenian a restoration rather than an invention:

- **ablative singular `-a`** — the inherited form, returned to its inherited
  function;
- **genitive singular `-ogo`** — the inherited PIE genitive, already present in
  every Slavic language on adjectives and pronouns, generalized to nouns.

Both endings already exist in Russian and Ukrainian; only their distribution
changes. Nothing is coined. A speaker of any Slavic language would recognize
`dobrogo` as a genitive and `doma` as "from home" without instruction.

In the plural and dual the ablative merges with the dative, exactly as in PIE and
Sanskrit.

## 13.3 The proposed noun paradigm

Hard masculine stem `dom-`, in Ruthenian orthography.

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `dom` | `doma` | `domi` |
| vocative | `domje` | = nom | = nom |
| accusative | `dom` / `doma` (anim.) | = nom | `domy` |
| genitive | `domogo` | `domu` | `domov` |
| **ablative** | `doma` | = dat | = dat |
| dative | `domu` | `domoma` | `domom` |
| instrumental | `domom` | `domoma` | `domami` |
| locative | `domje` | `domu` | `domah` |

Distinct forms: 8 singular, 3 dual, 6 plural. Sources: the dual from OCS
(`-a`/`-u`/`-oma`), the vocative from OCS and Ukrainian (which agree: `-e`, 34 %
and 51 % respectively), the ablative and genitive as §13.2, everything else the
East Slavic consensus.

Feminine `žen-`: vocative `-o` (OCS 42 %, Ukrainian 25 %), dual NAV `-ě`→`-je`,
GL `-u`, DAT/INS `-ama`. Neuter: vocative = nominative throughout, as in OCS
(45 % `-o` = the nominative ending) and Sanskrit (84 % ∅).

## 13.4 Where the regularity comes from

Conservative in *inventory*, regular in *realization*. Ruthenian keeps more
categories than any modern Slavic language and still has fewer forms to learn,
because the irregularity is removed rather than the categories:

| Removed | Justification from the tables |
|---|---|
| mobile stress (patterns `c`–`f`, primed) | Interslavic marks no stress at all; 1 465 Russian nouns affected |
| heteroclitics (`-mę`, `dětę`, `mati`) | ~15 lemmas, memorized individually in every Slavic language |
| the four Russian marginal cases | absent from every sister language; the partitive and 2nd locative are absorbed by the ablative, which does their semantic work |
| short/long adjective split | a Slavic innovation absent from PIE and Sanskrit; lexically unpredictable in Russian (4 571 / 9 999) |
| soft adjective stems | 1.6 % of adjectives carrying a parallel ending set |
| indeclinable loans | regularized by **addition** — `metro`, `metra`, `metru` |
| Russian numeral government | replaced by real dual agreement (§13.6) |
| verb classes 7–16 | 9.3 % of verbs; regularized onto 1–6 |

| Kept | Justification |
|---|---|
| aspect | the defining Slavic innovation, in all four lects |
| animacy | shared by all four, information-bearing |
| the *n-* prefix, reflexive, participle structure | pan-Slavic |
| iotation | 675 `ov→u` alone; removing it would make the verb unrecognizable |

## 13.5 The verb

Keep the two-stem model and Zaliznyak classes 1–6, add the **dual** from OCS —
`-evě`, `-eta`, `-ete`, cognate with Sanskrit `-vaḥ`, `-thaḥ`, `-taḥ`:

| | Singular | Dual | Plural |
|---|---|---|---|
| 1 | `-u` | `-jevje` | `-jem` |
| 2 | `-jeszj` | `-jeta` | `-jetje` |
| 3 | `-jet` | `-jetje` | `-ut` |

The aorist and imperfect are available as an **optional archaic register**, on
Interslavic's model, rather than required — the OCS forms are fully attested
(`-běxъ`, `-ěaxъ`) and cost nothing to specify while remaining unused by default.

## 13.6 The payoff: numeral agreement stops being a fossil

Russian's genitive singular after 2–4 is petrified dual agreement (§4). A
Ruthenian **with a dual** does not need the rule at all:

- `dva doma` — *nominative dual*, not "genitive singular"
- `tri domi` / `pjatj domov` — nominative plural / genitive plural

The rule that most reliably defeats Russian learners disappears not by
simplification but by **restoring the category that made it make sense**. That is
the clearest single argument that "conservative" and "regular" are not in tension
here — for this feature they are the same move.

## 13.7 What this costs

Honest accounting. The dual adds a third number to every nominal and verbal
paradigm: three forms per noun, three per verb tense. The ablative adds one cell
in the singular. The vocative adds one. Against that, the removals in §13.4 take
away far more than they add — but the added categories are **obligatory**, and
obligatory categories are what make a language hard, not the size of a paradigm
table.

The design is defensible precisely because every restored feature is attested in
the family and every regularization has a sister-language or constructed-standard
precedent. Nothing here is invented; the novelty is only in the combination.
