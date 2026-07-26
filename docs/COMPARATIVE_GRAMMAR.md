# Comparative grammar by word class

Proto-Indo-European · Sanskrit · Old Church Slavonic · Russian · Ukrainian ·
Belarusian · Interslavic

Reference notes for Ruthenian's standardization decisions. Seven languages, one
word class at a time.

The order is **genetic, not alphabetical**, because the interesting fact in almost
every table is the shape of the erosion from left to right: PIE is the
reconstructed ancestor, Sanskrit the conservative early attestation, OCS the
Slavic starting point, the East Slavic three the immediate design space, and
Interslavic the worked example of a deliberately standardized Slavic lect —
the only column that represents a *choice* rather than an inheritance.

**Nothing here imports Interslavic data into the crate.** This is a grammar
comparison; the lexicon's single source remains the English Wiktionary dump.

## Method

Where a claim is countable it is counted over the **whole** dump
(`INVARIANTS.md` I1 and I7 — grammar claims are measured, not quoted). Counts
come from full scans: 441 629 Russian records, 124 791 across `uk`/`be`/`cu`/`sa`,
and 1 894 Proto-Indo-European reconstructions (`ine-pro`).

Counts are of *attested form-slots*, not lemmas: they show which categories a
language actually fills, not how large its dictionary is. PIE figures are counts
of **reconstructed** forms and carry the usual caveat — they describe what
comparative reconstruction posits, not an attested corpus. Interslavic is a
designed standard with no corpus here, so its column is descriptive throughout.

## References

Academic sources only.

- Fortson, B. W. IV, *Indo-European Language and Culture: An Introduction*,
  2nd ed., Wiley-Blackwell, 2010.
- Beekes, R. S. P., *Comparative Indo-European Linguistics: An Introduction*,
  2nd ed., Benjamins, 2011.
- Whitney, W. D., *Sanskrit Grammar*, 2nd ed., 1889.
- Lunt, H. G., *Old Church Slavonic Grammar*, 7th rev. ed., Mouton de Gruyter,
  2001.
- Comrie, B. & Corbett, G. G. (eds), *The Slavonic Languages*, Routledge, 1993 —
  the standard comparative reference, with per-language chapters.
- Vaillant, A., *Grammaire comparée des langues slaves*, 1950–77.
- Zaliznyak, A. A., *Русское именное словоизменение*, 1967; *Грамматический
  словарь русского языка*, 1977 (6th ed. 2010).
- *Русская грамматика*, ed. N. Yu. Shvedova, Academy of Sciences, 1980.
- Jakobson, R., "Russian Conjugation", *Word* 4, 1948; Townsend, C. E., *Russian
  Word Formation*, 1975.
- van Steenbergen, J. & Merunka, V., *Interslavic* normative grammar — for a
  constructed standard, the primary source.

---

# 1. Nouns

## 1.1 Case

Measured — attested (or reconstructed) noun form-slots per case:

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
| partitive (2nd gen.) | — | — | — | 206 | — | — | — |
| 2nd locative | — | — | — | 199 | — | — | — |
| count form | — | — | — | 31 | — | — | — |

**The shape of the loss.** PIE and Sanskrit have eight cases. Slavic lost the
**ablative** — merged into the genitive before OCS — and arrives at seven. OCS
still has a fully productive vocative. Then East Slavic diverges:

- **Ukrainian kept the vocative**: 25 180 forms, essentially one per noun. Live,
  taught, obligatory.
- **Belarusian lost it**: 21 forms, a relic on the order of Russian's 40.
- **Russian lost it too**, then grew four *new* marginal cases: the partitive
  (`снегу`), second locative (`в лесу`), a new colloquial vocative (`мам!` — a
  truncation, not the old `-е`), and the count form after numerals.

So the East Slavic three are **not** "six cases each": Ukrainian has seven,
Belarusian six, Russian six plus a marginal tail of four. That tail is the
messiest part of the Russian nominal system.

**Interslavic** takes six and prints the vocative in tables while explicitly
denying it case status — it exists only in the masculine and feminine singular,
never affects agreement, plays no syntactic role. Keeping the form while denying
the category is a clean piece of standardization.

## 1.2 Number

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| singular | 14 968 | 144 285 | 97 919 | ✓ | ✓ | 29 799 | ✓ |
| **dual** | **9 052** | **143 771** | **77 714** | — | — | — | — |
| plural | 15 403 | 153 641 | 92 189 | ✓ | ✓ | 38 248 | ✓ |

The most dramatic measured difference in the comparison. In Sanskrit the dual is
**as frequent as the singular** (143 771 vs 144 285) — a fully equal member of
the system. PIE reconstruction posits it robustly (9 052 against 14 968 singular).
In OCS it is still ~30 % of forms. In all four modern lects it is **gone**,
surviving as fossils: Russian `два часа́`, `глаза`, `рога`, and the
numeral-governed genitive singular after 2–4, which is petrified dual agreement.

Russian's count form (§1.1) is the dual's ghost. Worth knowing before deciding to
regularize numeral government away: it is not arbitrary but the last trace of a
lost number.

## 1.3 Gender and animacy

Three genders throughout, though PIE's are usually reconstructed as a later
development from an earlier animate/inanimate split (Fortson §6). Measured noun
form-slots by gender:

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian |
|---|---|---|---|---|---|---|
| masculine | 113 | 3 356 | 153 | 105 388 | 11 204 | 1 365 |
| feminine | 104 | 1 556 | 114 | 73 791 | 11 142 | 1 365 |
| neuter | 115 | 1 962 | 87 | 27 363 | 3 031 | 367 |

PIE's three are near-equal; Sanskrit's neuter is still large; Slavic neuter has
eroded steadily (Russian 27 363 against 105 388 masculine).

**Animacy** is a Slavic innovation absent from PIE and Sanskrit: the masculine
accusative copies the genitive for animates, the nominative for inanimates.
Present in all four Slavic lects, extended slightly further in the plural in
Ukrainian and Belarusian. Interslavic keeps it.

## 1.4 Declension classes

| | Classes |
|---|---|
| PIE | thematic (*o*-stem) vs athematic (consonant, *i*-, *u*-, *r*-, *n*-stems); ablaut-graded |
| Sanskrit | vowel stems (a, ā, i, ī, u, ū, ṛ) and consonant stems |
| OCS | inherited *o*-, *jo*-, *a*-, *ja*-, *i*-, *u*-, consonant, *ū*-stems |
| Russian | 8 types (Zaliznyak), keyed on the **graphic** stem ending |
| Ukrainian | 4 declensions |
| Belarusian | 3 declensions plus indeclinables |
| Interslavic | 3 (hard/soft × gender) |

One trajectory, run to different depths: an inherited **stem-based** system (what
the stem historically ended in) reanalysed as a **gender-based** one (what the
word looks like now). Interslavic completes it; Russian is mid-transition, which
is why Zaliznyak's types are defined orthographically rather than morphologically.

## 1.5 Stress and ablaut

| | System |
|---|---|
| PIE | free **pitch** accent; mobile paradigms; **ablaut** (e/o/zero grade) is grammatically productive |
| Sanskrit | pitch accent in Vedic, lost in Classical |
| OCS | free, mobile; the Proto-Slavic accent paradigms |
| Russian | free, mobile — 6 patterns + 4 primed (measured, `RUSSIAN_GRAMMAR.md` §2) |
| Ukrainian | free, mobile |
| Belarusian | free, mobile, **and orthographically load-bearing** — unstressed `o` is written `a` (*akanne*) |
| Interslavic | **unspecified**; not marked, no paradigms |

Two through-lines worth naming.

**PIE ablaut is the ancestor of Slavic fleeting vowels.** The e/o/zero alternation
that PIE used to build paradigms became, via the Slavic jers, the appearing and
vanishing vowels of `okno`/`okon` and `sovjestnyj`/`sovjesten`
(`RUSSIAN_GRAMMAR.md` §3.4). What looks like a Russian irregularity is a
5000-year-old morphological device in its final decayed state.

**Belarusian shows the cost of orthographic stress-dependence**: because *akanne*
is written, stress placement determines spelling, so the orthography cannot be
stress-neutral. Interslavic sidesteps the problem entirely by not encoding stress.
Ruthenian currently stores it and models six accent patterns — both precedents
are live options.

---

# 2. Adjectives

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| agreement (gender/number/case) | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| declines like a noun | ✓ | ✓ | (short form) | (short form) | vestigial | vestigial | — |
| long/short (definite/indefinite) | — | — | **both, productive** | long + short (predicative) | long; short vestigial | long; short vestigial | long only |
| comparative | synthetic | synthetic | synthetic | synthetic + analytic | synthetic + analytic | synthetic + analytic | regular `-ějši` |
| superlative | — | synthetic | prefix | `самый` + long | `най-` | `най-` | `naj-` |

The long/short split is a **Slavic innovation**, not an inheritance: PIE and
Sanskrit adjectives simply decline like nouns. OCS had a genuine
indefinite/definite opposition (`dobrъ` vs `dobrъjь`), the definite formed by
suffixing the anaphoric pronoun — the ancestor of the modern long form. Modern
East Slavic has bleached this into a predicative/attributive split, and the short
form is receding: in Russian, **4 571 of 9 999 adjectives have short forms and
5 428 do not**, with no rule predicting which.

Interslavic drops the split. That makes short-form availability a strong
regularization candidate: lexically unpredictable, already dying, absent from
both the ancestor's function and the constructed standard.

Russian's soft adjectives (`-ний`) are **155 of 9 999 = 1.6 %** — an entire
parallel ending set for a rounding error.

---

# 3. Pronouns

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| personal, 3 persons | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **dual personal forms** | ✓ | ✓ | ✓ | — | — | — | — |
| clitic / enclitic series | ✓ | ✓ (`mā`, `tvā`, `naḥ`) | ✓ (`mę`, `tę`, `sę`) | — (only bound `-ся`) | — | — | ✓ (restored) |
| post-prepositional *n-* | — | — | ✓ | ✓ (`у него`) | ✓ | ✓ | ✓ |
| reflexive | ✓ | ✓ (`ātman-`) | ✓ | ✓ | ✓ | ✓ | ✓ |
| 3rd person from demonstrative | ✓ | `sa/tad` | `onъ` | `он` | `він` | `ён` | `on` |

**Clitic pronouns** run PIE → Sanskrit → OCS → *lost in East Slavic* →
**deliberately restored by Interslavic**. Russian retains only the bound
reflexive `-ся`. A standardized lect can plausibly go either way; Ruthenian
currently has the full series plus post-prepositional `n-` forms.

The ***n-* prefix** (`у него`, not `*у его`) is a Slavic innovation from a
reanalysed preposition-final nasal — absent from PIE and Sanskrit, present in all
four Slavic lects. Ruthenian implements it as its own `Slot` variant rather than
a case of a noun, which is the linguistically correct treatment.

---

# 4. Numerals

| | Behaviour after the numeral |
|---|---|
| PIE | 1–4 adjectival and agreeing; higher numerals nominal |
| Sanskrit | 1–4 decline and agree in gender/number/case; 5+ decline without agreeing |
| OCS | 1–4 agree, with **dual for 2**; 5+ are feminine *i*-stem nouns taking the genitive |
| **Russian** | 1 agrees; **2–4 take genitive singular** (petrified dual); 5+ genitive plural; 11–14 override |
| Ukrainian | 1 agrees; 2–4 **nominative plural**; 5+ genitive plural |
| Belarusian | as Ukrainian |
| Interslavic | 1 agrees; 2–4 nominative plural; 5+ genitive plural |

**The clearest case where Russian is the odd one out.** Ukrainian, Belarusian and
Interslavic all use the nominative plural after 2–4 — the straightforward
reading. Russian's genitive singular is the fossilized dual of §1.2, and it is
the rule that most reliably defeats learners.

For Ruthenian, adopting the Ukrainian/Belarusian/Interslavic pattern is attested
in two sister languages, adopted by the constructed standard, and a single-rule
change. The strongest regularization candidate in the comparison.

---

# 5. Verbs — categories

Measured verbal categories present in attested (or reconstructed) forms:

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| present | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **aorist** | ✓ | ✓ | ✓ | — | — | — | optional |
| **imperfect** | ✓ | ✓ | ✓ | — | — | — | optional |
| perfect | ✓ | ✓ | periphrastic | — | — | — | ✓ |
| past (l-participle) | — | — | ✓ | ✓ | ✓ | ✓ | ✓ |
| future | ✓ | ✓ | periphrastic | ✓ | ✓ (synth. + periphr.) | ✓ | ✓ |
| conditional | — | ✓ | ✓ | ✓ (`бы`) | ✓ | ✓ | ✓ |
| imperative | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **subjunctive** | ✓ | ✓ | — | — | — | — | — |
| **optative** | ✓ | ✓ | — | — | — | — | — |
| **injunctive** | ✓ | ✓ | — | — | — | — | — |
| **middle voice** | ✓ | ✓ | — | — | — | — | — |
| passive | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **aspect (ipf/pf)** | — | — | emerging | **✓ pervasive** | ✓ | ✓ | ✓ |
| infinitive | (verbal noun) | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| **supine** | — | — | ✓ | — | — | — | optional |

**The great trade.** Slavic exchanged a rich *tense–mood* system for a pervasive
*aspect* system. PIE and Sanskrit distinguish present/aorist/perfect stems
morphologically and carry three moods beyond the imperative — the measured PIE
counts bear this out: indicative 7 325, **optative 3 854, subjunctive 3 686**.
Modern Slavic has one past tense (the old l-participle) and instead marks
perfective/imperfective on **every** verb: Russian 5 517 imperfective against
4 902 perfective lemmas.

The **middle voice** of PIE and Sanskrit is gone from all Slavic; its functions
were taken over by the reflexive `-ся`, which is why Russian has 3 182 reflexive
verbs doing work Sanskrit did by inflection.

Interslavic is instructive: it offers aorist, imperfect and supine as **optional**
archaisms at a "scientific" register while running on the modern aspect system by
default. A standardized lect need not choose one historical layer.

## 5.1 Conjugation classes

| | Classes |
|---|---|
| PIE | thematic vs athematic; present-stem formations (root, *-ye/o-*, nasal-infix, reduplicated, …) |
| Sanskrit | **10 present classes**, plus derived stems (causative, desiderative, intensive) |
| OCS | 5 present classes on the theme vowel |
| Russian | 2 conjugations on the surface; **16 Zaliznyak classes** underneath |
| Ukrainian | 2 conjugations, 13 classes traditionally |
| Belarusian | 2 conjugations |
| Interslavic | 2, fully regular |

All four Slavic lects present a two-conjugation surface over a messier reality.
Russian's 16 classes are 90.7 % covered by classes 1–6 (measured).

---

# 6. Participles and verbal adverbs

| | PIE | Sanskrit | OCS | Russian | Ukrainian | Belarusian | Interslavic |
|---|---|---|---|---|---|---|---|
| present active | ✓ | ✓ | ✓ | ✓ | restricted | restricted | ✓ |
| past active | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| present passive | ✓ | ✓ | ✓ | ✓ | rare | rare | ✓ |
| past passive | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| gerund / verbal adverb | — | ✓ (absolutive) | ✓ | ✓ (2) | ✓ | ✓ | ✓ |
| participle declines | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

Ukrainian is the outlier in **restricting** active participles: prescriptive
Ukrainian grammar discourages the present active participle in favour of relative
clauses. That is a standardization decision made by a natural language, and a
direct precedent for Ruthenian doing the same deliberately.

Whether a participle exists is **structural**, not lexical: no present participles
for perfectives, no passive participles for intransitives. Ruthenian derives this
rather than storing it (`RUSSIAN_GRAMMAR.md` §3.6).

---

# 7–11. The closed classes

**Adverbs.** Formed from adjectives throughout: Slavic `-o`/`-e`, Sanskrit and PIE
the neuter accusative singular — the same device. Comparatives follow the
adjective. Little divergence, little to standardize.

**Prepositions.** In PIE these were **adverbial particles**, not yet a word class;
the case system did the work. Sanskrit is still largely at that stage. Slavic
grammaticalized them into true prepositions governing case, and the governed case
is **lexical** — it must be stored, not derived. The East Slavic three share
nearly the whole inventory with small government differences; Interslavic
regularizes some. Ruthenian will need a preposition→case table in the lexicon.

**Conjunctions, particles, interjections.** Indeclinable everywhere; lexicon-only,
no morphology. Russian's particle inventory (`же`, `ли`, `бы`, `-то`) is closest to
OCS; Sanskrit's `ca`, `vā`, `hi` are enclitic, as several OCS particles are, and
both continue PIE sentence-second clitics.

---

# 12. What this comparison says for Ruthenian

Sorted by how strong the case is for adopting a non-Russian solution.

**Adopt the sister-language solution.**

1. **Numeral government** — Ukrainian, Belarusian and Interslavic all use the
   nominative plural after 2–4. Russian's genitive singular is a fossilized dual.
   One rule, three precedents.
2. **Short-form adjectives** — a Slavic innovation, not an inheritance; dying in
   all East Slavic; lexically unpredictable in Russian (4 571 / 9 999); absent
   from Interslavic. Drop, or universalize.
3. **The vocative** — two clean options: Ukrainian's (productive seventh case,
   continuing PIE and OCS directly) or Interslavic's (keep the form, deny the
   category). Russian's relic-plus-truncation state is the one option to avoid.

**Regularize on Interslavic's model.**

4. **Declension classes** — collapse toward hard/soft × gender, completing a
   reanalysis already 2000 years old.
5. **Marginal cases** — Russian's partitive, second locative and count form exist
   in no sister language and no standard. Fold them in.
6. **Stress** — Interslavic does not encode it. Belarusian shows the cost of an
   orthography that depends on it. Ruthenian currently stores it.

**Keep — Slavic, not Russian quirks.**

7. **Aspect** — the defining Slavic innovation, pervasive in all four lects.
8. **Animacy** — shared by all four, information-bearing.
9. **The *n-* prefix**, the reflexive, participle structure.

**Deliberately available, not required.**

10. **Aorist, imperfect, supine, dual, clitic pronouns, the middle voice** — all
    recoverable from OCS or further back, all offered by Interslavic as optional
    registers. A purist Ruthenian could restore any of them. The comparison says
    this is a coherent choice rather than an eccentric one, and Interslavic shows
    how to offer it without imposing it.

**The one thing the deep history changes.** Fleeting vowels look like a Russian
irregularity worth deleting. They are the last state of PIE ablaut — the same
device that built `pater`/`patros` and Sanskrit's guṇa/vṛddhi grades. That is not
an argument for keeping them, but it does mean removing them severs Ruthenian
from a morphological system its sister languages all still carry, in exchange for
regularity in roughly one cell per paradigm.
