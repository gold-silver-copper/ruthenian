# The Ruthenian language — specification

Normative description of Ruthenian: a Latin-script East Slavic literary language,
**more conservative than Russian or Ukrainian and more regular than either**.

Ruthenian restores three categories the modern East Slavic languages lost — the
**ablative**, the **vocative** and the **dual** — and removes the irregularities
that make Russian hard, without removing the structure that makes it Slavic.

Every restored feature is attested in the family and every regularization has a
precedent in a sister language or in Interslavic. Nothing here is invented; the
novelty is in the combination. The evidence is in
[`COMPARATIVE_GRAMMAR.md`](COMPARATIVE_GRAMMAR.md), measured over full corpora of
Proto-Indo-European, Sanskrit, Old Church Slavonic, Russian, Ukrainian and
Belarusian. What this document still owes is in §13.

All forms below are in the Ruthenian Latin alphabet, specified in §2.1.

---

# 1. The design in one page

| | Ruthenian | Russian | Ukrainian | OCS | Sanskrit |
|---|---|---|---|---|---|
| cases | **8** | 6 (+4 marginal) | 7 | 7 | 8 |
| numbers | **3** | 2 | 2 | 3 | 3 |
| genders | 3 | 3 | 3 | 3 | 3 |
| noun declensions | **3** | 8 | 4 | 8 | ~8 |
| verb classes | **6** | 16 | 13 | 5 | 10 |
| stress | fixed, **not written** | 10 patterns | mobile | mobile | pitch |
| adjective long/short | **both** | both | both | both | n/a |
| past tenses | **3** (aorist, imperfect, perfect) | 1 | 1 | 3 | 4 |
| copula | **full, with dual** | invariant, omitted | full | full | full |
| clitic pronouns | **kept** | lost | relics | full | full |
| aspect | lexical + derived | lexical | lexical | emerging | n/a |
| 2nd palatalization | **kept** | lost (0 %) | kept (99 %) | kept (66 %) | n/a |
| yat distinction | **kept** (as `-i`) | lost | kept (as `i`) | kept (as `ě`) | n/a |

**Restored:** the ablative, the vocative, the dual, dual pronouns, dual verb
agreement, the aorist, the imperfect, the OCS long/short adjective, the full
copula, and the **full/clitic pronoun opposition** with second-position
placement.
**Removed:** mobile stress, heteroclitics, marginal cases, soft adjective stems,
indeclinables, **the fleeting vowel entirely**, irregular numeral government,
verb classes 7–16, and the doubled `n` of the past passive participle.
**Kept:** aspect, animacy, reflexives, iotation, participles,
determinate/indeterminate motion pairs, **all three palatalizations** (Ukrainian
99 %, Russian 0 %), and the yat distinction via the Ukrainian reflex `-i`.
**Not adopted:** the post-prepositional *n-* prefix (§5.1), which every Slavic
language has and neither PIE nor Sanskrit does.

**The three axes, stated once.** Ruthenian is not uniformly conservative or
uniformly regular; it is conservative in one dimension and Russian in another,
and every decision below follows from this:

| | follows | because |
|---|---|---|
| **grammar** | Old Church Slavonic | conservatism is spent on categories: cases, numbers, tenses, the clitic system |
| **phonology** | Russian | a language's branch is audible in its reflexes — `golova`, not `glava` |
| **vocabulary** | East Slavic (Russian-centred), plus an OCS learned layer | §12 |

The two exceptions to "phonology follows Russian" are **yat** and the **second
palatalization**, both of which Russian merged away and both of which the
OCS-shaped grammar needs in order to keep eight cases distinguishable (§2.4,
§3.1).

---

# 2. Phonology and orthography

## 2.1 The alphabet is pure ASCII

Ruthenian is written in **unaccented ASCII**. There are no diacritics, no
combining marks, and no letters outside `a`–`z`. The digraphs `cz sz zz szcz ja
je jo ju` and the separator `'` carry everything a diacritic would.

The alphabet, against the Cyrillic it corresponds to:

| Cyr | Ruth | Cyr | Ruth | Cyr | Ruth |
|---|---|---|---|---|---|
| а | `a` | к | `k` | ч | `cz` |
| б | `b` | л | `l` | ш | `sz` |
| в | `v` | м | `m` | щ | `szcz` |
| г | `g` | н | `n` | ъ | `'` |
| д | `d` | о | `o` | ы | `y` |
| е | `je` | п | `p` | ь | `j` |
| ё | `jo` | р | `r` | э | `e` |
| ж | `zz` | с | `s` | ю | `ju` |
| з | `z` | т | `t` | я | `ja` |
| и | `i` | у | `u` | | |
| й | `j` | ф | `f` | | |
| | | х | `h` | | |

**The separator `'` is one glyph with one rule**: *the next character starts a
new letter*. `sz'czi` is ш + ч rather than щ; `s'zadi` is с + з rather than ш.
Russian's hard sign is that same rule applied at a morpheme boundary
(`pod'jezd`), so the two are one idea and not two.

The correspondence is **bijective for every word transliterated from Cyrillic**,
which is what makes the source languages mechanically readable. Its
implementation, the context rules that make it invertible, and the round-trip
evidence are in
[`crates/ruthenian-orthography`](../crates/ruthenian-orthography/README.md).

### A citation form carries what its shape cannot predict

Two positions in a lemma carry morphology rather than sound, and between them
they let every inflectional fact be **read off the citation form** instead of
supplied alongside it. A lemma is therefore self-describing: `noun(word, case,
number)` and `verb(word, person, number, tense)` need nothing else.

**A trailing `'` marks a lemma that is not what its ending predicts.** Word-finally
there is no next character, so the separator rule is vacuous and the position is
free:

| | predicted | marked |
|---|---|---|
| verb, `-atj` | class 1 — `czitatj` → `czitaj-` | class 6 — `pisatj'` → `pisz-` |
| noun, `-j` | masculine — `konj` | feminine — `noczj'` |
| noun, `-a` | feminine — `zzena` | masculine — `sluga'` |

One mark suffices because each ambiguity is **binary**, never three-way: a verb
in `-atj` is class 1 or 6; a noun in `-j` is masculine or feminine, since no
neuter ends in `j`; a noun in `-a` is feminine or masculine, since no neuter ends
in `-a`. Where an ending predicts one gender absolutely — `-o`/`-je` is always
neuter, a non-`j` consonant always masculine — the mark has no legal use.

**A capital first letter marks an animate noun.**

```
drug     inanimate
Drug     animate      — vizzu druga, not vizzu drug
```

Capitalization is otherwise free in a lemma, because sentence position is not a
property of a word. The inflected output is **always lowercase**: `Drug` in the
nominative is `drug`, and a text that needs a capital applies it afterwards.

Two costs, and both are real:

- **A lemma list is not running text.** In a lemma a capital means animate; in
  text it means sentence-initial or proper. The two conventions cannot be mixed
  in one string, so a lemma is a dictionary object rather than a word ready to
  paste.
- **Transliteration does not produce lemmas.** A Cyrillic source word carries no
  animacy in its capitalization — `друг` is lowercase and animate — so the
  extractor must supply the capital when it builds a lemma. `to_latin` gives a
  word, not a citation form.

Two honest consequences:

- **`'` now has two jobs**, not one. They are in complementary distribution — the
  separator only ever occurs *between* letters, the mark only ever at the end —
  so no string is ambiguous, but the "one glyph, one rule" formulation is no
  longer literally true. What the mark means is uniform across parts of speech,
  though: *this lemma is not what its ending predicts*. Which fact that is
  depends on what the word is, and the caller already knows that, since it calls
  `noun` or `verb`.
- **The mark has no Cyrillic counterpart.** No Cyrillic word can produce a
  word-final `'`, because `ъ` may only stand before `е ё ю я и`, so the mark never
  appears in a transliterated word and the round-trip contract is untouched on
  its actual domain. Converting a *marked* lemma back to Cyrillic is simply
  undefined: the mark is Ruthenian's own, and Ruthenian's orthography is not
  obliged to be expressible in someone else's.

**Stress is not written.** It is real, lexical and fixed per word, but ordinary
text does not mark it — as in Russian, Ukrainian and Polish orthography, and as
in Interslavic, which does not encode stress at all. Dictionaries and teaching
materials may mark it with a combining acute on the vowel (`pisátj`); running
text never does, and the marked and unmarked spellings are different strings.

## 2.2 Consonants

| | labial | dental | alveolar | palatal | velar |
|---|---|---|---|---|---|
| stop | `p b` | `t d` | | | `k g` |
| affricate | | `c` | `cz` `dzz` | | |
| fricative | `f v` | `s z` | `sz zz` | `szcz` | `h` |
| nasal | `m` | `n` | | | |
| liquid | | `l r` | | `j` | |

Every consonant except `j`, `cz`, `szcz`, `zz` and `sz` has a **hard** and a
**soft** (palatalized) value; softness before a vowel is written with the
`j`-digraphs (`ja je jo ju`) and word-finally or before a consonant with `j`
alone (`konj`).

**The five exceptions are exceptions for two different reasons.** `j` is itself
the palatal; `cz` and `szcz` are inherently palatal, so there is no hard value to
contrast with. `zz` and `sz` are the opposite case: they are inherently **hard**,
as `ж` and `ш` are in Russian, and have no soft value. Either way the `j` has
nothing to mark, which is why none of the five ever takes one (§3.8 rule 2a).

## 2.3 Vowels

`a e i o u y` — six, with `y` the back counterpart of `i`. The iotated series
`ja je jo ju` are `j` + vowel, not separate phonemes.

## 2.4 The three palatalizations

Inherited, productive, and fully automatic:

| | before | `k` | `g` | `h` | **`c`** | trigger |
|---|---|---|---|---|---|---|
| **first** | front vowels, `j` | `cz` | `zz` | `sz` | **`cz`** | vocative `-je`, present stem, comparative |
| **second** | yat-derived `-i` | `c` | `z` | `s` | — | locative sg, feminine dative sg, dual |
| **third** | after `i`, `j`, `r` | `c` | `z` | `s` | — | certain derivational suffixes |

Russian levelled the second away entirely (0 %); Ukrainian keeps it at 99 % and
OCS at 66 %. Ruthenian keeps all three.

**The first palatalization also applies to `c`**, which is itself the output of an
earlier palatalization and reverts before a front vowel: `otjec` → vocative
`otjecze`, exactly as OCS `otьcь` → `otьče`. This is not a marginal case — it
governs the whole `-jec` class (`otjec`, `hlopjec`, `konjec`, `kupjec`), which is
large and frequent.

Note that Ruthenian has **no `dz`** [dz]. It does have `dzz` [dʒ], which is a
different consonant and arrives by a different route: `dzz` is the additive
output of iotation (§2.6, §7.11 — `mjedzza`, `vidzzu`), while `dz` would have to
be a *palatalization* output, and it is not one. OCS had it as the
second-palatalization
output of `g` (`kъnędzь`); Ruthenian's second palatalization gives `z` instead
(`drug` → `druzi`), following East Slavic. There is therefore no `dz → zz` rule,
because there are no inputs for it.

## 2.5 Phonotactics

Syllables are maximally `CCCVCC`. **There is no fleeting vowel**: a stem is
invariant across its whole paradigm, so `son` gives `sona`, `sonu`, `sonom`, and
`otjec` gives `otjeca`, `otjecu` (§3.9). Voicing assimilates regressively within
a word; final devoicing is **not** written.

One consequence worth stating, because it simplifies everything downstream: **the
citation form is always the stem.** Nothing has to be reconstructed from an
oblique form, and no lexical entry needs a reducibility flag.

## 2.6 Sound correspondences

Which reflex Ruthenian takes for each Common Slavic divergence. **Phonology
follows Russian** (§1), with the two exceptions noted there.

| | Proto-Slavic | Ruthenian | OCS has | Example |
|---|---|---|---|---|
| pleophony `*TorT`, `*TolT`, `*TerT` | `*golvà`, `*gordъ`, `*melkò` | `-oro-`, `-olo-`, `-jerje-` | `-ra-`, `-la-`, `-rje-` | `golova`, `gorod`, `moloko`, `bjerjeg` |
| `*tj`, `*dj` | `*světja`, `*medja` | `tcz`, `dzz` | `szt`, `zzd` | `svjetcza`, `mjedzza` |
| nasals `*ǫ`, `*ę` | `*rǫka`, `*pętь` | `u`, `ja` | `ǫ`, `ę` | `ruka`, `pjatj` |
| `*dl`, `*tl` | `*mydlo` | simplified to `l` | simplified | `mylo` |
| initial `*je-`, `*o-` | `*edinъ`, `*elenь` | `o-` | `je-` | `odin`, `oljenj` |
| **yat `*ě`** | `*lěsъ`, `*domě` | **`-i`** (Ukrainian) | `ě` | `lis`, `domi` |

The `*tj`/`*dj` row is **additive**, the same operation §7.11 applies to the
present stem: `t` and `d` are stops, so both survive in front of their reflex. This is a
third departure from "follow Russian", and it is taken for transparency rather
than for a category — `svjetcza` "candle" keeps `svjet-` "light" visible where
Russian's `свеча` hides it, and `mjedzza` "boundary" keeps `mjed-`. Russian's
outcome is the result of the stop assimilating away; nothing in the grammar
depends on its having done so.

The last row is the first exception to "follow Russian": Russian merged yat into
`e`, which would make the locative singular `dome` identical to the vocative
`domje` and collapse a case distinction the grammar needs. The second exception
is the second palatalization (§2.4), which Russian levelled to 0 %.

*These correspondences are stated from the comparative literature rather than
measured; historical sound laws cannot be counted from a synchronic corpus. See
`COMPARATIVE_GRAMMAR.md` for what is measured.*

### 2.6a The OCS learned layer is productive

Russian carries inherited and Church Slavonic forms side by side — `golova` /
`glava`, `gorod` / `grad`, `gorozzanin` / `grazzdanin`, `odin` / `jedinyj` —
with the OCS member marking elevated, abstract or technical register. Ruthenian
generalizes this: **any root eligible for one of the correspondences above may
form its OCS-shaped doublet by rule.**

| Ruthenian | learned doublet |
|---|---|
| `golova`, `gorod`, `moloko`, `bjerjeg`, `djerjevo` | `glava`, `grad`, `mljeko`, `brjeg`, `drjevo` |
| `svjetcza`, `gorozzanin` | `osvjeszczjenije`, `grazzdanin` |
| `odin` | `jedinyj` |

Two consequences, both deliberate:

- The rule generates **both** members for every eligible root, including roots
  where only one is attested in Russian. Russian has only the learned `vrjemja`;
  Ruthenian also has native `vjerjemja`. This is regularization by addition, the
  same move as declining the indeclinables (§12.3).
- Where Russian's two members have drifted into separate lexemes — `storona`
  "side" against `strana` "country" — Ruthenian treats them as one lexeme in two
  registers. The semantic split is not inherited; if a language needs both
  senses, they are separate entries by §12.4's ordering, not by the doublet rule.

The layer is **register, not meaning**: the two forms denote the same thing, and
the learned member is the marked one.

---

# 3. Nouns

## 3.1 The eight cases

| Case | Abbrev. | Function |
|---|---|---|
| nominative | nom | subject; citation form |
| vocative | voc | direct address |
| accusative | acc | direct object |
| genitive | gen | possession, relation |
| **ablative** | abl | source, origin, motion *from*, cause, comparison |
| dative | dat | indirect object, recipient |
| instrumental | ins | means, accompaniment, predicate |
| locative | loc | location, topic ("about") |

### Why Ruthenian has an ablative, and where it comes from

Slavic is usually said to have "lost the ablative". It did not: **it lost the
genitive and reassigned the ablative form to genitive function.**

- PIE thematic genitive singular was `*-osyo`; the ablative singular was `*-ōd`.
- Slavic's `o`-stem genitive singular `-a` (Russian `дома`, OCS `доma`) is the
  regular reflex of **`*-ōd`** — the ablative.
- PIE `*-osyo` disappeared from Slavic nouns but survives intact in the
  pronominal and adjectival declension as **`-ogo`**: Russian `того`, `доброго`;
  OCS `того`, `доброѥго`. Every Slavic language still has it.

So both endings are alive in every Slavic language today; only their distribution
changed. Ruthenian returns them to their inherited functions:

| | Ruthenian | Source |
|---|---|---|
| genitive sg (masc/neut) | `-ogo` | PIE `*-osyo`, preserved in the Slavic adjective |
| **ablative sg (masc/neut)** | `-a` | PIE `*-ōd`, the inherited ablative form |

`doma` therefore means *from the house* — its original meaning — and `domogo`
means *of the house*. A speaker of any Slavic language reads both without
instruction.

### Where the ablative is distinct — and where it is not

The ablative is **not** a case that appears everywhere. Measured across the
family:

| | ablative distinct from… | PIE | Sanskrit |
|---|---|---|---|
| singular, masculine | genitive | ✓ | 81 % |
| singular, neuter | genitive | ✓ | 85 % |
| **singular, feminine** | genitive | ✗ | **1 %** |
| dual | dative | ✗ | 0 % |
| plural | dative | ✗ | 0 % |

Ruthenian follows this exactly. The ablative is a **distinct form only in the
masculine and neuter singular**; elsewhere it is syncretic:

- **feminine singular** — ablative = genitive (as in PIE and Sanskrit, 99 %);
- **dual** — ablative = dative = instrumental;
- **plural** — ablative = dative.

This is why adding a case costs so little: it adds **one cell** to two of the six
paradigms.

### The vocative

Singular only, in every language that has it: `nom=voc` runs 90–100 % in the
plural (PIE 100 %, Sanskrit 99 %, OCS 90 %, Ukrainian 99 %). Ruthenian therefore
has no vocative plural — the nominative is used.

Endings are the OCS and Ukrainian consensus, which agree closely:

| | Ruthenian | OCS | Ukrainian |
|---|---|---|---|
| masculine hard | `-je` | `-e` (34 %) | `-e` (51 %) |
| masculine soft | `-ju` | `-ju` (9 %) | `-ju` (7 %) |
| feminine | `-o` | `-o` (42 %) | `-o` (25 %) |
| neuter | = nominative | `-o` = nom (45 %) | = nom |

### The dual

Three distinct forms, never more. In **both** Sanskrit and OCS — an eight-case
language and a seven-case one — the dual collapses to the same three groups at
99–100 %:

| Group | Cases | Ruthenian ending (hard masc) |
|---|---|---|
| direct | nom = voc = acc | `-a` |
| adnominal | gen = loc | `-u` |
| oblique | dat = ins = abl | `-oma` |

Sources: OCS `-a` / `-u` / `-oma`, cognate with Sanskrit `-au` / `-yoḥ` /
`-bhyām`. The dual is used for exactly two of something, and is **obligatory**
with the numeral `dva`.

### Yat, the locative, and the two palatalizations

OCS distinguished the vocative and the locative singular by origin:

| | OCS | from |
|---|---|---|
| vocative sg | `dome` | PIE `*-e`, the bare e-grade stem vowel |
| locative sg | `domě` | PIE `*-oi`, a diphthong that became **yat** (`ě`) |

Two different endings. The East Slavic languages then split on yat:

| | ě becomes | Effect |
|---|---|---|
| Russian | `e` | **merger** — the distinction is lost |
| Belarusian | `e` | merger |
| **Ukrainian** | **`i`** | **distinction preserved**, as a different vowel |

Ruthenian takes the **Ukrainian reflex**: every yat-derived ending is written
`-i`. This is not a borrowing in place of conservatism — it is the one East
Slavic reflex in which the OCS distinction survived. `lěsъ` → `lis`,
`chlěbъ` → `chlib`, `domě` → `domi`.

So the locative singular is `-i` and the vocative singular is `-je`, and they are
distinct as they were in OCS.

**The second palatalization does the same work in the consonant.** Yat from
`*-oi` triggered the *second* palatalization of velars, while the vocative `-e`
triggered the *first*. Velar stems therefore distinguish the two cases twice
over — in the vowel and in the consonant:

| | first palatalization (voc `-je`) | second palatalization (loc `-i`) |
|---|---|---|
| `k` | `cz` | `c` |
| `g` | `zz` | `z` |
| `h` | `sz` | `s` |

`drug` → vocative `druzze`, locative `druzi`.

Whether to keep the second palatalization at all is a real choice, and the
measured answer is that **Ruthenian sides with Ukrainian and OCS against
Russian**. In velar-stem feminine dative/locative singulars:

| | second palatalization applied |
|---|---|
| Ukrainian | **99 %** (`sobaka` → `sobaci`) |
| Old Church Slavonic | 66 % (`noga` → `nozě`) |
| **Russian** | **0 %** (`sobaka` → `sobake`) |

Russian levelled it away completely. Ruthenian keeps it: it is inherited, it is
regular, and it carries a case distinction that would otherwise rest on the
vowel alone.

## 3.2 The three declensions

Ruthenian has three declensions, not Russian's eight. The velar, sibilant, `c`
and vowel stem-classes of Russian are **not separate declensions** — they are the
same endings with automatic spelling adjustments (§3.8).

| Declension | Contents | Example |
|---|---|---|
| **I** | nouns in `-a`, of either gender | `zzena` (woman), `zjemlja` (earth), `sluga'` (servant, masc.) |
| **II** | masculine, and neuter | `dom` (house), `konj` (horse), `okno` (window), `polje` (field) |
| **III** | feminine ending in a consonant | `noczj` (night), `kostj` (bone) |

Each has a **hard** and a **soft** variant; the soft variant substitutes `je` for
`o`, `ju` for `u`, `i` for `y` — a single alternation, applied everywhere.

### Declension, hardness and gender are all read off the citation form

Nothing about a noun's class has to be stated separately:

| Citation form ends in | Gender | Declension | Hardness |
|---|---|---|---|
| `-a` | feminine | I | hard |
| `-ja` | feminine | I | soft |
| `-o` | neuter | II | hard |
| `-je` | neuter | II | soft |
| `-j` | **masculine** | II | soft |
| `-j` **+ `'`** | **feminine** | III | — |
| `-a` **+ `'`** | **masculine** | I | hard |
| any other consonant | masculine | II | hard |

`konj` and `noczj'` are the pair that makes the mark necessary: both end in `j`,
and nothing else distinguishes a soft masculine of declension II from a feminine
of declension III. Everywhere else the ending decides, and **hardness is simply
whether the form ends in `j`, `ja` or `je`.**

## 3.3 Declension II — masculine

### Hard: `dom` "house" (stem `dom-`)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `dom` | `doma` | `domy` |
| vocative | `domje` | = nom | = nom |
| accusative | `dom` / `doma` ¹ | = nom | `domy` / `domov` ¹ |
| genitive | `domogo` | `domu` | `domov` |
| **ablative** | `doma` | = dat | = dat |
| dative | `domu` | `domoma` | `domom` |
| instrumental | `domom` | `domoma` | `domami` |
| locative | `domi` | `domu` | `domah` |

¹ animate nouns take the genitive form in the accusative (§3.7).

### Soft: `konj` "horse" (stem `kon-`)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `konj` | `konja` | `konji` |
| vocative | `konju` | = nom | = nom |
| accusative | `konja` (anim.) | = nom | `konjev` |
| genitive | `konjego` | `konju` | `konjev` |
| **ablative** | `konja` | = dat | = dat |
| dative | `konju` | `konjema` | `konjem` |
| instrumental | `konjem` | `konjema` | `konjami` |
| locative | `konji` | `konju` | `konjah` |

### Velar: `drug` "friend" (animate, stem `drug-`)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `drug` | `druga` | `drugi` ² |
| vocative | **`druzze`** ³ | = nom | = nom |
| accusative | `druga` | = nom | `drugov` |
| genitive | `drugogo` | `drugu` | `drugov` |
| **ablative** | `druga` | = dat | = dat |
| dative | `drugu` | `drugoma` | `drugom` |
| instrumental | `drugom` | `drugoma` | `drugami` |
| locative | **`druzi`** ⁴ | `drugu` | `drugah` |

² spelling rule: `y` is written `i` after a velar. ³ first palatalization
`g` → `zz`. ⁴ second palatalization `g` → `z`. The vocative and locative differ
in both the consonant and the vowel.

**Note on syncretism.** `doma` is both ablative singular and nominative dual.
This is inherited, not a defect: OCS has exactly the same collision (genitive
singular `-a` = nominative dual `-a`), because both continue different PIE
endings that fell together regularly. It is disambiguated by agreement — a dual
noun takes dual modifiers and a dual verb.

## 3.4 Declension II — neuter

### Hard: `okno` "window" (stem `okn-`)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `okno` | `okni` | `okna` |
| vocative | = nom | = nom | = nom |
| accusative | `okno` | `okni` | `okna` |
| genitive | `oknogo` | `oknu` | `oknov` |
| **ablative** | `okna` | = dat | = dat |
| dative | `oknu` | `oknoma` | `oknom` |
| instrumental | `oknom` | `oknoma` | `oknami` |
| locative | `okni` | `oknu` | `oknah` |

### Soft: `polje` "field" (stem `pol-`)

Nominative `polje`, genitive `poljego`, ablative `polja`, dative `polju`,
instrumental `poljem`, locative `polji`; dual `polji` / `polju` / `poljema`;
plural `polja` / `poljev` / `poljem` / `poljami` / `poljah`.

The neuter dual `-i` continues OCS `-ě` (`dvě selě`), and the neuter vocative is
the nominative in every language measured — Sanskrit 84 % ∅, OCS 45 %.

## 3.5 Declension I — nouns in `-a`

### Hard: `zzena` "woman" (stem `zzen-`)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `zzena` | `zzeni` | `zzeny` |
| vocative | `zzeno` | = nom | = nom |
| accusative | `zzenu` | `zzeni` | `zzeny` / `zzenov` ¹ |
| genitive | `zzeny` | `zzenu` | `zzenov` |
| **ablative** | `zzeny` ² | = dat | = dat |
| dative | `zzeni` ³ | `zzenama` | `zzenam` |
| instrumental | `zzenoj` | `zzenama` | `zzenami` |
| locative | `zzeni` ³ | `zzenu` | `zzenah` |

¹ animate. ² **= genitive**, as in PIE and Sanskrit (99 %). ³ dative **=**
locative: both continue OCS `-ě`, and they are identical in OCS, Russian and
Ukrainian alike (Ukrainian `-i` 55 % in both cells). Keeping them apart would be
an innovation, not a conservatism.

### Velar feminine: `kniga` "book" (stem `knig-`)

Nominative `kniga`, vocative `knigo`, accusative `knigu`, genitive/ablative
`knigi` (spelling rule only), **dative/locative `knizi`** (second palatalization
`g` → `z`), instrumental `knigoj`; dual `knizi` / `knigu` / `knigama`; plural
`knigi` / `knigov` / `knigam` / `knigami` / `knigah`.

Note that the genitive `knigi` and the dative/locative `knizi` are distinguished
by the palatalization alone — the vowel is the same. This is exactly the
Ukrainian pattern (`knyhy` / `knyzi`) and is lost in Russian, where both are
`книги`/`книге` with the velar intact.

### Soft: `zjemlja` "earth" (stem `zjeml-`)

Nominative `zjemlja`, vocative `zjemljo`, accusative `zjemlju`, genitive/ablative
`zjemli`, dative `zjemlji`, instrumental `zjemljoj`, locative `zjemli`; dual
`zjemlji` / `zjemlju` / `zjemljama`; plural `zjemli` / `zjemljev` / `zjemljam` /
`zjemljami` / `zjemljah`.

### Vowel-final stems: `nacija` "nation" (stem `naci-`)

A stem may end in a vowel, and nothing about the declension changes. §3.8's
rule 3 puts the soft sign in the ending rather than the stem, so `nacija` is
`naci-` plus the same endings `zjemlja` takes:

Nominative `nacija`, vocative `nacijo`, accusative `naciju`, genitive/ablative
`nacii`, dative `naciji`, instrumental `nacijoj`, locative `nacii`; dual
`naciji` / `naciju` / `nacijama`; plural `nacii` / `nacijev` / `nacijam` /
`nacijami` / `nacijah`.

**The doubled `i` of `nacii` is regular and is not repaired.** It is simply
`naci-` + `-i`, and Ruthenian has no rule contracting a vowel sequence. Russian
gives this class a sub-pattern of its own (`нация`, `нации`, `нацией`), and
Ruthenian does not: that would be a fourth declension to learn, for a class the
third already handles. The class is not marginal — §12.3's `-cija` and `-ija`
borrowings are large and productive — which is the reason to state it here
rather than leave it to be inferred at each loan.

### Masculine nouns in `-a`

`sluga'` "servant", `vojevoda'` "commander", `junosza'` "youth" — masculine in
meaning and in agreement, but `-a` in form. Pan-Slavic, and OCS has them.

They **decline as declension I and agree as masculine**, and the split is the
whole point:

```
moj sluga'         my servant        — masculine adjective (moj, not moja)
vizzu slugu        I see the servant — declension I accusative -u, not the
                                       masculine ablative
o sluzi            about the servant — declension I locative, with the second
                                       palatalization g -> z
```

Form follows the declension; agreement follows the gender. Nothing else is
special: the paradigm is `zzena`'s throughout, so `sluga'` gives vocative
`slugo`, dative and locative `sluzi`, dual `sluzi` / `slugu` / `slugama`, plural
`slugi` / `slugov` / `slugam` / `slugami` / `slugah`, and instrumental singular
`slugoj`.

(`slugi`, not `slugy`: §3.8's first rule writes `y` as `i` after a velar, and
`kniga` → `knigi` two paradigms above is the same rule on the same consonant.)

The `'` is what says so (§2.1): `-a` predicts feminine, and a masculine noun in
`-a` is exactly the case the mark exists for. An animate one carries both marks —
`Sluga'`.

## 3.6 Declension III — feminine in a consonant

### `noczj` "night" (stem `nocz-`)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `noczj` | `noczi` | `noczi` |
| vocative | `noczi` | = nom | = nom |
| accusative | `noczj` | `noczi` | `noczi` |
| genitive | `noczi` | `noczu` | `noczev` |
| **ablative** | `noczi` | = dat | = dat |
| dative | `noczi` | `noczjma` | `noczam` |
| instrumental | `noczjju` | `noczjma` | `noczami` |
| locative | `noczi` | `noczu` | `noczah` |

The `j` of the endings is a **softness marker**, so §3.8's rule 2 removes it
after `cz`: the plural is `noczev`, `noczam`, `noczami`, `noczah` and not
`*noczjev`, and this is Russian's own `ночей`, `ночам`, `ночами`, `ночах`, none
of which carries a `ь`. Where the `j` is *not* a softness marker it stays — the
nominative is the bare ending `-j`, and the instrumental `-jju` is the sign plus
the ending.

A stem that is not inherently palatal keeps them all: `kostj'` gives `kostjev`,
`kostjam`, `kostjami`, `kostjah`.

This is the inherited PIE *i*-stem declension. Its singular is heavily syncretic
(`-i` for genitive, ablative, dative and locative) in Ruthenian as it is in
Russian, Ukrainian and OCS; the instrumental `-jju` keeps the soft sign *and*
takes the ending, as in Russian `ночью`.

## 3.6a A note on the masculine dative

Ukrainian generalized the u-stem `-ovi` to **65 %** of masculines (`domovi`), and
it is a tempting regularization: it would make the dative unmistakable against
the ablative `-a` and locative `-i`. Ruthenian nonetheless takes **`-u`**.

`-u` is the OCS ending (37 %), it is what Russian, Belarusian, Polish and OCS all
have, and `-ovi` is a Ukrainian innovation from a minor stem class. Where the
brief is "most OCS, most pan-Slavic", `-u` wins on both counts. The ambiguity it
creates — dative singular `domu` = genitive/locative dual `domu` — is inherited
from OCS, which has exactly the same collision.

## 3.7 Animacy

**Animate** nouns take an oblique form in the accusative; inanimates take the
nominative. Inherited, pan-Slavic, information-bearing, and kept unchanged in
scope — it applies to any animate noun, in both the singular and the plural.

Animacy is not derivable from a word's shape — `dom` and `drug` are identical in
form and differ only in what they denote — so a lemma **marks it with a capital
first letter** (§2.1): `Drug` is animate, `drug` is not. Inflected output is
always lowercase.

Which oblique form, however, is a question Ruthenian has to answer and the other
Slavic languages do not, because Ruthenian has split the genitive from the
ablative (§3.1):

| | singular | plural |
|---|---|---|
| animate accusative = | **ablative** | **genitive** |

```
dom   → vizzu dom       I see the house     (inanimate sg: acc = nom)
Konj  → vizzu konja     I see the horse     (animate sg:   acc = ABL, konjego is genitive)
dom   → vizzu domy      I see the houses    (inanimate pl: acc = nom)
Drug  → vizzu drugov    I see the friends   (animate pl:   acc = gen)
```

**Why the singular takes the ablative.** The construction is inherited: OCS forms
it with `raba`, and every Slavic language uses that `-a` form. But `-a` continues
PIE `*-ōd`, which is the **ablative** — the entire argument of §3.1. When
Ruthenian gives the two endings back their original names, the animate accusative
follows the *form* it was actually built on, not the label the tradition later
attached to it. `konjego` would be a form no Slavic language has ever had here.

The plural does not raise the question: ablative and dative are syncretic there
(§3.1), so the distinct oblique available is the genitive, and `drugov` is what
the paradigm gives.

## 3.8 Automatic spelling adjustments

Not declensions — a single set of rules applied to every ending:

1. after `k g h` and `zz sz cz szcz`, `y` is written `i` (`knigi`, not `*knigy`);
2. after `cz szcz zz sz`, an ending's initial `j` is **not written** — §2.2 gives
   none of the four a hard/soft contrast, so the glide has nothing to mark:
   `otjecz` + `-je` → `otjecze`, `druzz` + `-je` → `druzze`,
   `pisz` + `-jeszj` → `piszeszj`;
3. a stem-final soft sign belongs to the ending, not the stem (`kon` + `j`);
4. **first palatalization** before the vocative `-je`: `k`→`cz`, `g`→`zz`,
   `h`→`sz`, `c`→`cz` (`drug` → `druzze`, `otjec` → `otjecze`);
5. **second palatalization** before any yat-derived `-i` — the locative
   singular, the feminine dative singular, the neuter and feminine dual:
   `k`→`c`, `g`→`z`, `h`→`s` (`drug` → `druzi`, `kniga` → `knizi`).

Rules 4 and 5 are morphophonemic rather than orthographic — they change the
consonant, not just its spelling — but they are fully automatic and belong with
the others.

**There used to be a rule between 1 and 3, and deleting it is what let rule 2
become simple.** It read "after `zz sz cz szcz c`, *unstressed* `o` is written
`je`", following Russian's `ножом` against `товарищем`. Two things were wrong
with it. It cannot be implemented or checked, because §2.1 makes stress real but
unwritten and no paradigm here exercised the rule. And its output was
phonologically wrong: `nozzjem` claims a palatalized `zz`, and §2.2 has no such
consonant — `zz` and `sz` are hard, as `ж` and `ш` are in Russian.

With the rule gone the endings are simply invariant, and the forms it was
reaching for come out right anyway: `nozzom` and `otjecom` are Russian's `ножом`
and `отцом`. What the rule captured was the *unstressed* `товарищем` type, which
is precisely the part Ruthenian cannot see.

**Rule 2 is what remains, and it is now one statement about four consonants.**
An ending never marks softness on a consonant that has no soft value. That
covers the vocative (`otjecze`, `druzze`), the present endings (`piszeszj`,
§7.3), and the `-jem`/`-jego` series, which after these stems is simply `-om`
and `-ogo` — there was never a `j` to drop, because rule 2 no longer puts one
there.

Its one bound is that it applies to an ending's **initial** `j` and not to a `j`
anywhere in an ending: everywhere else a leading `j` is rule 3's soft sign rather
than a glide, so §3.6's instrumental `-jju` keeps both, and a wider rule makes
`noczjju` into `noczju`.

These replace Russian's velar-, sibilant-, `ц`- and vowel-stem declensions, which
differ from the hard type *only* by these automatic effects.

## 3.9 What was removed, and why

| Removed | Was | Justification |
|---|---|---|
| mobile stress | 6 patterns + 4 primed | stress is fixed, lexical and unwritten |
| **the fleeting vowel, entirely** | `okno` → `okon`, `son` → `sna` | the stem is invariant; see below |
| heteroclitics | `vremja/vremeni` | ~15 lemmas memorized individually in every Slavic language |
| partitive, 2nd locative, count form | 436 Russian lemmas | the ablative does their semantic work |
| indeclinables | 1 193 Russian nouns | regularized **by addition**: `mjetro`, `mjetrogo`, `mjetru` |
| plural-/singular-only defectiveness | 2 003 nouns | every noun has all three numbers |

**The fleeting vowel goes further than the genitive plural.** Two facts made the
full removal cheap. The genitive plural is uniformly `-ov`, so the zero-ending
environment that produced `okno` → `okon` no longer exists in the paradigm at
all; and the alternation that survives elsewhere (`son`/`sna`, `otjec`/`otca`) is
not predictable from the surface — `son` alternates and `nos` does not, and
nothing distinguishes them without etymological information the lexicon cannot
supply for most lemmas (§12.2).

Ruthenian therefore has invariant stems: `son`, `sona`, `sonu`, `sonom`;
`otjec`, `otjeca`, `otjecu`. This is regularization by removal, in the same class
as dropping mobile stress and the heteroclitics, and it makes the citation form
identical to the stem for every noun.

---

# 4. Adjectives

Ruthenian keeps the **OCS long/short opposition**, which is a definiteness
contrast — the only one the language has. There is no article.

| | Form | Meaning | Declines |
|---|---|---|---|
| **short** | `dobr mǫzz` | *a* good man — indefinite | as a **noun** |
| **long** | `dobryj mǫzz` | *the* good man — definite | **pronominally** |

The long form *is* the short form plus the anaphoric pronoun `j-`: OCS `dobrъ` +
`jь` → `dobrъjь`. That is where the `-ogo` genitive came from in the first place
(§3.1), so the adjective is the origin of the noun's new genitive and the two are
now visibly the same system.

Unlike Russian, the short form is **not** restricted to the predicate. It is the
indefinite adjective and declines fully.

### The predicate takes either, and the contrast survives

Because the opposition is definiteness rather than syntactic position, both forms
are grammatical after the copula and they mean different things:

```
on jestj dobr        he is good            (indefinite predication)
on jestj dobryj      he is the good one    (definite, identifying)
```

OCS put the short form in the predicate and Russian restricts it there, but both
are describing a *positional* rule. Ruthenian has made the contrast semantic, so
banning one form in a position would throw away a distinction the system already
encodes. The predicate is simply another place where definiteness is expressed.

The copula agrees in gender and number as usual, and the predicate adjective
agrees with the subject in the **nominative** in both forms — there is no
predicate instrumental (§10.2).

## 4.1 Short (indefinite) — nominal declension

`dobr` "good", masculine. Endings are the noun's, **exactly** — including the
animacy syncretism, which belongs to the nominal declension rather than to nouns
as a word class.

| Case | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `dobr` | `dobro` | `dobra` | `dobra` | `dobry` |
| vocative | `dobrje` | = nom | `dobro` | = nom | = nom |
| accusative | `dobr` / `dobra` ¹ | `dobro` | `dobru` | = nom | `dobry` / `dobrov` ¹ |
| genitive | `dobrogo` | `dobrogo` | `dobry` | `dobru` | `dobrov` |
| **ablative** | `dobra` | `dobra` | `dobry` | = dat | = dat |
| dative | `dobru` | `dobru` | `dobri` | `dobroma` | `dobrom` |
| instrumental | `dobrom` | `dobrom` | `dobroj` | `dobroma` | `dobrami` |
| locative | `dobri` | `dobri` | `dobri` | `dobru` | `dobrah` |

## 4.2 Long (definite) — pronominal declension

`dobryj`. Endings are the pronoun `toj`'s.

| Case | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `dobryj` | `dobroje` | `dobraja` | `dobraja` | `dobryje` |
| accusative | `dobryj` / `dobra` ¹ | `dobroje` | `dobruju` | `dobraja` | `dobryje` / `dobryh` ¹ |
| genitive | `dobrogo` | `dobrogo` | `dobroj` | `dobru` | `dobryh` |
| **ablative** | `dobra` | `dobra` | `dobroj` | = dat | = dat |
| dative | `dobromu` | `dobromu` | `dobroj` | `dobryma` | `dobrym` |
| instrumental | `dobrym` | `dobrym` | `dobroj` | `dobryma` | `dobrymi` |
| locative | `dobrom` | `dobrom` | `dobroj` | `dobru` | `dobryh` |

¹ animate. Long adjectives have **no vocative** — the nominative is used, as in
every language measured.

The two declensions differ in the nominative, accusative, dative, instrumental
and locative, and coincide in the genitive and ablative. That coincidence is
inherited: contraction of the long forms (`dobrajego` → `dobrogo`) merged them in
exactly those cells across all of Slavic.

### Both forms mark animacy, and both agree with the noun

An adjective agrees with its head in case, so where §3.7 puts an animate noun in
the **ablative** in the singular and the **genitive** in the plural, an agreeing
adjective goes there too — in either form:

```
vizzu dobr dom          I see the good house      (inanimate: acc = nom)
vizzu dobra druga       I see the good friend     (animate sg: acc = ABL)
vizzu dobrov drugov     I see the good friends    (animate pl: acc = gen)
```

Two consequences fall out of the tables above.

**The short form marks animacy too.** §4.1's endings are the noun's, and the
animate accusative is a syncretism *of the nominal declension* rather than a
property of nouns as a word class — so anything declining nominally inherits it.
`COMPARATIVE_GRAMMAR.md` §2 records OCS's short adjective as declining like a
noun, which is the same statement.

**The two forms coincide in the animate accusative singular**, both giving
`dobra`, because they already coincide in the ablative. So `vizzu dobra druga`
is well-formed whether the adjective is definite or not — the definiteness
contrast is neutralized in exactly that cell, which is the sort of thing a case
system does and not a defect.

## 4.3 Degrees

Regular, no suppletion. The comparative triggers the **first** palatalization.

| Degree | Formation | Example |
|---|---|---|
| positive | — | `dobr` / `dobryj` |
| comparative | `-jejsz-` | `dobrjejszij` |
| superlative | `naj-` + comparative | `najdobrjejszij` |

`naj-` follows OCS, Ukrainian, Belarusian, Polish and Interslavic against
Russian's analytic `самый`.

Comparatives and superlatives exist in both long and short forms, and the
comparative governs the **ablative** for the standard of comparison:
`dobrjejszij brata` "better than the brother" — the inherited ablative of
comparison, which Russian expresses with the genitive and Sanskrit with the
ablative proper.

# 5. Pronouns

Pronouns decline **pronominally** — the declension the long adjective borrows
(§4.2). All have dual forms.

## 5.1 Personal

| | 1sg | 2sg | **1du** | **2du** | 1pl | 2pl |
|---|---|---|---|---|---|---|
| nominative | `ja` | `ty` | `vje` | `va` | `my` | `vy` |
| accusative | `mjenja` | `tjebja` | `na` | `va` | `nas` | `vas` |
| genitive | `mjenjego` | `tjebjego` | `naju` | `vaju` | `nas` | `vas` |
| **ablative** | `mjenja` | `tjebja` | = dat | = dat | = dat | = dat |
| dative | `mnje` | `tjebje` | `nama` | `vama` | `nam` | `vam` |
| instrumental | `mnoj` | `toboj` | `nama` | `vama` | `nami` | `vami` |
| locative | `mnje` | `tjebje` | `naju` | `vaju` | `nas` | `vas` |

`vje` "we two" and `va` "you two" are the OCS duals, restored.

### 5.1a The clitic series

Every personal pronoun has a **short, unstressed** form beside the full one, as
in OCS and Sanskrit. Russian lost this opposition entirely; OCS, Czech, Polish
and Interslavic keep it.

| | full acc | **clitic acc** | full dat | **clitic dat** |
|---|---|---|---|---|
| 1sg | `mjenja` | `mja` | `mnje` | `mi` |
| 2sg | `tjebja` | `tja` | `tjebje` | `ti` |
| reflexive | `sjebja` | **`sja`** | `sjebje` | `si` |
| 3sg masc/neut | `jego` | `go` | `jemu` | `mu` |
| 3sg fem | `ju` | `ju` | `jej` | `ji` |
| 1pl | `nas` | `ny` | `nam` | `ni` |
| 2pl | `vas` | `vy` | `vam` | `vi` |
| 3pl | `jih` | `jih` | `jim` | `jim` |

**Clitics are unstressed and stand in second position** — after the first
stressed constituent of the clause, which is Wackernagel's law and the same rule
that places the question particle `li` (§10.4). They cannot be stressed, cannot
be focused, cannot stand alone as an answer, and cannot follow a preposition:
each of those environments requires the full form.

```
on mi go dal          he gave it to me        (clitics second, dat before acc)
mnje on go dal        to ME he gave it        (focused: full form, fronted)
u jego                at his                  (after a preposition: full form)
```

Within the cluster the order is **dative before accusative**, and the reflexive
`sja` comes last.

**This is what the bound reflexive was.** `sja` is not a verbal suffix that
happens to resemble a pronoun; it is the accusative clitic of the reflexive,
which Russian fused to the verb and Ruthenian keeps free (§5.2).

### Third person

| | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `on` | `ono` | `ona` | `ona` | `oni` |
| accusative | `jego` | `jego` | `ju` | `ja` | `jih` |
| genitive | `jego` | `jego` | `jeje` | `jeju` | `jih` |
| **ablative** | `jego` | `jego` | `jeje` | = dat | = dat |
| dative | `jemu` | `jemu` | `jej` | `jima` | `jim` |
| instrumental | `jim` | `jim` | `jeju` | `jima` | `jimi` |
| locative | `jem` | `jem` | `jej` | `jeju` | `jih` |

**No post-prepositional *n-*.** A pronoun after a preposition is the plain form:

```
u jego        at his          (not *u njego)
s jim         with him        (not *s njim)
k jej         to her          (not *k njej)
o jih         about them      (not *o njih)
```

Every Slavic language has this prefix — OCS `u njego`, Russian `у него`,
Ukrainian, Belarusian, Polish and Interslavic alike — so declining it is the one
regularization here without a Slavic precedent. It has a deeper one instead.

The `n-` is not inherited. It is a **reanalysis**: the prepositions `vъn`, `sъn`,
`kъn` once ended in a nasal, and when that nasal was lost the boundary was
misparsed, `vъn jego` becoming `vъ njego`. The prefix is therefore a
Slavic-internal accident, and PIE and Sanskrit — the two languages this spec
takes the ablative and the dual from — have nothing corresponding to it.

Removing it is conservative in the same sense those restorations are: it returns
the pronoun to the shape it had before a change that only Slavic made. It also
removes an allomorph, which costs nothing in expressiveness — the `nj-` forms
were never contrastive, only positional.

## 5.2 Reflexive

No nominative — the cell does not exist.

| | |
|---|---|
| accusative / ablative | `sjebja` |
| genitive | `sjebjego` |
| dative / locative | `sjebje` |
| instrumental | `soboj` |

The reflexive clitic is `sja`, and it is a **free second-position clitic** like
every other (§5.1a) — not a bound verbal suffix:

```
on sja myjet          he washes himself
myjet li sja on?      does he wash himself?
```

This follows OCS `sę`, Czech `se` and Polish `się`. Russian's `-sja`, written
attached, is a later fusion; Ruthenian does not carry it, because doing so would
give one morph two grammars — a suffix in the verb and a pronoun in the
paradigm — when it is a single clitic doing a single job.

## 5.3 Possessive

Possessives are **adjectives** and decline as such, in both long and short forms.

| | Singular | Dual | Plural |
|---|---|---|---|
| 1st | `moj` | `naju` | `nasz` |
| 2nd | `tvoj` | `vaju` | `vasz` |
| reflexive | `svoj` — "one's own", any person | | |

`svoj` is obligatory when the possessor is the subject: `on czitajet svoju
knigu` "he reads his own book" against `on czitajet jego knigu` "he reads
someone else's".

The third person has **no** possessive adjective: the genitive of the personal
pronoun is used, indeclinably — `jego dom`, `jeje dom`, `jih dom`.

## 5.4 Demonstrative

Two degrees, as OCS had: `sjej` "this (near)" and `toj` "that (far)". Russian
lost the near deixis; Ruthenian keeps it.

`toj` is the model for the whole pronominal declension:

| | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `toj` | `to` | `ta` | `ta` | `ti` |
| accusative | `toj` / `togo` ¹ | `to` | `tu` | `ta` | `ti` / `tjeh` ¹ |
| genitive | `togo` | `togo` | `toj` | `toju` | `tjeh` |
| **ablative** | `toga` | `toga` | `toj` | = dat | = dat |
| dative | `tomu` | `tomu` | `toj` | `tjema` | `tjem` |
| instrumental | `tjem` | `tjem` | `toj` | `tjema` | `tjemi` |
| locative | `tom` | `tom` | `toj` | `toju` | `tjeh` |

¹ animate. `sjej` declines identically on the stem `sj-`: `sjego`, `sjemu`,
`sjim`, `sjem`.

## 5.5 Interrogative and relative

| | "who" | "what" |
|---|---|---|
| nominative | `kto` | `czto` |
| accusative | `kogo` | `czto` |
| genitive | `kogo` | `czjego` |
| **ablative** | `koga` | `czjega` |
| dative | `komu` | `czjemu` |
| instrumental | `kjem` | `czjem` |
| locative | `kom` | `czjem` |

`kto` is animate and `czto` inanimate, which is why `kto` has the
genitive-accusative and `czto` does not.

Also `czij` "whose", `kotoryj` "which", `kakyj` "what kind of" — all adjectival.

**The relative pronoun is `izzje`** (OCS `иже`), inflecting as `toj` plus the
invariant `-zzje`: `izzje`, `jegozzje`, `jemuzzje`. Russian lost it in favour of
`который`; Ruthenian keeps both, `izzje` for restrictive clauses and `kotoryj`
for non-restrictive.

## 5.6 Negative and indefinite

Built by prefix from the interrogatives — fully regular, no suppletion:

| Prefix | Sense | Example |
|---|---|---|
| `ni-` | negative | `nikto`, `nicztozze`, `niczij`, `nikakyj` |
| `nje-` | indefinite-specific ("a certain") | `njekto`, `njeczto`, `njekyj` |
| `-libo` | indefinite-nonspecific ("any") | `kto-libo`, `czto-libo` |
| `vjesj-` | universal | `vsjakyj` "every" |

Negative pronouns require **double negation**: `nikto nje czitajet` "nobody
reads" — the negative particle stays on the verb. Pan-Slavic and obligatory.

When a preposition intervenes, the prefix separates, as in OCS and Russian:
`ni u kogo` "at nobody's".

# 6. Numerals

## 6.1 Government — the payoff of the dual

Russian's genitive singular after 2–4 is petrified **dual** agreement. With a
real dual the rule is not simplified — it disappears:

| Numeral | Governs | Example |
|---|---|---|
| `odin` | agrees, singular | `odin dom` |
| **`dva`** | **the dual** | `dva doma` |
| `tri`, `czjetyrje` | nominative plural | `tri domy` |
| `pjatj` and above | genitive plural | `pjatj domov` |

`dva doma` is the nominative **dual**, which is what it originally was. There is
no 11–14 exception and no last-digit rule: five and above always take the
genitive plural, and a compound numeral is governed by its **last word** —
`dvadcatj dva doma` (dual), `dvadcatj pjatj domov` (genitive plural).

In oblique cases the numeral and noun simply agree: `s dvuma domoma` "with two
houses", `s pjatju domami` "with five houses".

## 6.2 Cardinals 1–10

| | | | |
|---|---|---|---|
| 1 `odin` | 2 `dva` | 3 `tri` | 4 `czjetyrje` |
| 5 `pjatj` | 6 `szjestj` | 7 `sjedmj` | 8 `osmj` |
| 9 `djevjatj` | 10 `djesjatj` | | |

## 6.3 Teens, tens, hundreds — regularized

Teens are transparently "N on ten", contracted to `-nadcatj`:

`odinnadcatj, dvanadcatj, trinadcatj, czjetyrnadcatj, pjatnadcatj,
szjestnadcatj, sjedmnadcatj, osmnadcatj, djevjatnadcatj`

Tens are "N tens", with **no exceptions**:

| | | | |
|---|---|---|---|
| 20 `dvadcatj` | 30 `tridcatj` | 40 **`czjetyrjedjesjat`** | 50 `pjatjdjesjat` |
| 60 `szjestjdjesjat` | 70 `sjedmjdjesjat` | 80 `osmjdjesjat` | 90 **`djevjatjdjesjat`** |

Russian's `сорок` (40) and `девяносто` (90) are lexical oddities with no
transparent structure; Ruthenian regularizes both onto the pattern.

Hundreds: `sto, dvjestje, trista, czjetyrjesta, pjatjsot, szjestjsot,
sjedmjsot, osmjsot, djevjatjsot`. Then `tysjacza` (1 000), `miljon`, `miljard`.

## 6.4 Declension

`odin` declines as a **long adjective** and agrees in gender, number and case:
`odin dom`, `odnogo doma`, `odnoj zzeny`.

`dva` is a **dual** form and has only dual endings:

| | Masc/neut | Fem |
|---|---|---|
| nom / acc | `dva` | `dvje` |
| gen / loc | `dvu` | `dvu` |
| dat / ins / abl | `dvjema` | `dvjema` |

`tri` and `czjetyrje` decline as plurals:

| | `tri` | `czjetyrje` |
|---|---|---|
| nominative | `tri` | `czjetyrje` |
| genitive / locative | `trjeh` | `czjetyrjeh` |
| dative | `trjem` | `czjetyrjem` |
| instrumental | `trjemi` | `czjetyrjmi` |

`pjatj` and above decline as **declension III** nouns (`noczj`): `pjatj`,
`pjati`, `pjati`, `pjatj`, `pjatjju`, `pjati`. This is inherited — the higher
numerals were feminine *i*-stem nouns in OCS and still behave like them.

## 6.5 Ordinals

Adjectives, long or short: `pjervyj, vtoryj, trjetij, czjetvjortyj, pjatyj,
szjestyj, sjedmyj, osmyj, djevjatyj, djesjatyj`, then `odinnadcatyj` and so on;
`sotyj`, `tysjacznyj`.

## 6.6 Collectives and fractions

Collective numerals count groups and mixed-gender sets, and govern the genitive
plural: `dvoje, troje, czjetvjero, pjatjero, szjestjero, sjedmjero`. `dvoje
djetjej` "two children".

Fractions: `polovina` (½), `trjetj` (⅓), `czjetvjertj` (¼), thereafter the
ordinal — `pjataja czastj` (⅕). `poltora` "one and a half" takes the dual.

# 7. Verbs

## 7.1 Categories

| Category | Values |
|---|---|
| aspect | imperfective, perfective — **derived, never listed** |
| tense | present, **aorist**, **imperfect**, perfect, pluperfect, future |
| mood | indicative, imperative, conditional |
| voice | active, passive |
| person | 1, 2, 3 |
| number | singular, **dual**, plural |

Ruthenian has **three past tenses**, as OCS did, and they divide by function
rather than by aspect:

| Tense | Function | Formation |
|---|---|---|
| **aorist** | a single completed event — narrative past | synthetic, `-h-`/`-s-` |
| **imperfect** | ongoing or repeated past | synthetic, `-jah-` |
| **perfect** | a past state still relevant now | `l`-participle + copula |
| pluperfect | past before the past | `l`-participle + past copula |

Aspect and the past tenses are **independent** axes, as in OCS: a perfective verb
has an imperfect (`poczitaszje` "he kept finishing"), an imperfective has an
aorist (`czita` "he read, once"). This is a large grammar, deliberately.

## 7.2 Aspect: mostly derived, with two stored classes

Aspect is read off the verb's shape wherever it can be, and stored where it
cannot. The default rules cover the great majority of verbs:

| Shape | Aspect | Example |
|---|---|---|
| bare stem | **imperfective** | `czitatj` "to read" |
| any prefix | **perfective** | `napisatj`, `poczitatj` |
| prefix + `-yva-`/`-iva-` | **imperfective** again | `napisyvatj` |

`-yva-` outranks the prefix: a secondary imperfective satisfies both conditions
and the suffix wins.

### What is not derivable, and why the spec says so plainly

An earlier revision claimed aspect was *fully* computable and that `po-` was an
empty perfectivizer forming every pair. Neither survives contact with the data.

**1. A closed class of simplex verbs is inherently perfective.** `datj` "give",
`statj` "become", `sjestj` "sit down", `pastj` "fall" are perfective with no
prefix in every Slavic language, and OCS already has `dati` (pf) against `dajati`
(impf). The class is larger than those: much of class 4 behaves the same way —
`kupitj` "buy", `brositj` "throw", `rjeszitj` "decide", `konczitj` "finish", with
class-1 imperfective partners.

There is no surface property that identifies them. The obvious criterion — a root
with no theme vowel — fails at once, since `datj` is perfective while `pitj`,
`mytj`, `bitj` and `bytj` are identically shaped and imperfective; and class does
not work either, since `kupitj` and `govoritj` are both class 4. This is exactly
why every Slavic dictionary stores aspect. **Ruthenian stores it too, for this
class only.**

**2. `po-` is delimitative, not empty.** `poczitatj` means "read for a while",
and it re-imperfectivizes to `poczityvatj` "read now and then". That form would
be pointless if `po-` contributed nothing — you would simply be back at
`czitatj`. Since `po-` carries meaning, it cannot serve as the universal pair
former.

**3. So the aspect partner is stored.** Which prefix bleaches for a given verb is
unpredictable and language-specific: `czitatj` → `proczitatj`, `pisatj` →
`napisatj`, `djelatj` → `sdjelatj`. Each imperfective records its perfective
partner, as OCS and every modern Slavic language do.

There are no biaspectual verbs. What Ruthenian removes is not lexical aspect but
lexical *irregularity* around it: no suppletive aspect pairs, no verb whose
partner cannot be written in the lexicon as a plain reference to another entry.

### 7.2a Determinate and indeterminate motion

A third axis, crossing aspect and inherited from OCS. Both members are
**imperfective**; they differ in directionality.

| determinate — one occasion, one direction | indeterminate — habitual, multidirectional |
|---|---|
| `idti` "be going" | `hoditj` "go about, go regularly" |
| `njesti` "be carrying" | `nositj` "carry around" |
| `vjesti` "be leading" | `voditj` "lead around" |
| `bjezzatj` "be running" | `bjegatj` "run about" |

About fourteen pairs, closed and stored. They are suppletive — `idti` and
`hoditj` share no root — which is the one place §1's removal of suppletion does
not reach, and it is kept because the distinction is pan-Slavic, present in OCS,
and carries meaning nothing else expresses.

## 7.3 Conjugation classes

Six, corresponding to Zaliznyak's 1–6; his 7–16 are regularized onto them.

**A class is defined by what it does to the stem, not by how the infinitive
ends.** Two classes share `-atj` and differ only in their operation, so the
ending was never the definition:

| Class | Present stem | Infinitives | Conj. | Example |
|---|---|---|---|---|
| 1 | theme vowel **stays**, `-j-` added | `-atj`, `-ytj`, monosyllabic `-itj` and `-jetj` | 1st | `czitatj` → `czitaj-`, `mytj` → `myj-`, `pitj` → `pij-` |
| 2 | `ova` → `uj` | `-ovatj` | 1st | `njegodovatj` → `njegoduj-` |
| 3 | theme drops | `-nutj` | 1st | `dvinutj` → `dvin-` |
| 4 | theme drops, 1sg mutates | polysyllabic `-itj` | 2nd | `govoritj` → `govor-` |
| 5 | theme drops | polysyllabic `-jetj` | 2nd | `vidjetj` → `vid-` |
| 6 | theme drops, stem mutates | `-atj` | 1st | `pisatj` → `pisz-` |

### Monosyllabic stems keep their vowel

A verb whose stem is **one syllable** has no theme vowel to drop — the vowel *is*
the root — so it takes class 1's operation regardless of which vowel it is:

```
mytj   → myj-    myju, myjeszj, myjet …      (OCS myti, myjǫ)
pitj   → pij-    piju, pijeszj, pijet …      (OCS piti, pijǫ)
bitj   → bij-    biju, bijeszj, bijet …      (OCS biti, bijǫ)
krytj  → kryj-   kryju …                     (OCS kryti, kryjǫ)
grjetj → grjej-  grjeju …                    (OCS grěti, grějǫ)
```

Without this, `-ytj` verbs belong to no class at all, and `pitj` would be read as
class 4 and yield `*p-` — a stem of one consonant.

### The class is derived from the citation form

Stated by operation, every ending decides its own class:

| Ending | Class |
|---|---|
| `-ovatj` | 2 |
| `-nutj` | 3 |
| `-itj` | 1 if the stem is monosyllabic, else 4 |
| `-jetj` | 1 if the stem is monosyllabic, else 5 |
| `-ytj` | 1 |
| `-atj` | 1, **or 6 if the lemma carries the final `'`** |

`-atj` was the one genuinely undecidable case — `czitatj` → `czitaj-` against
`pisatj` → `pisz-`, with nothing on the surface to separate them. The word-final
mark (§2.1) supplies exactly the one bit needed, so **the citation form alone
determines the class** and nothing has to be told to the inflection engine:

```
czitatj   → czitaj-   czitaju, czitajeszj, czitajet …
pisatj'   → pisz-     piszu, piszeszj, piszet …
```

The mark is part of the lemma, not an argument, so it travels with the word
wherever the word goes — in the lexicon, in a dictionary entry, in a citation.
Its cost is that the *unmarked* spelling of a class-6 verb is a well-formed
lemma of a different verb: `pisatj` would inflect as class 1 and yield `pisaju`.
A lemma is written with its mark or it is a different word.

### The residue: stems with a hidden consonant

A small closed set has a root-final consonant the infinitive does not show, so
the monosyllabic rule predicts the wrong stem:

| | infinitive | actual present stem | rule would give |
|---|---|---|---|
| "live" | `zzitj` | `zziv-` (OCS `žiti`, `živǫ`) | `*zzij-` |
| "swim" | `plytj` | `plyv-` | `*plyj-` |

These take a **listed present stem**, the same treatment §7.2 gives the
inherently perfective verbs — a stored class, small and closed, not a rule. They
are the only verbs where the class plus the infinitive is not enough.

## 7.4 Present

| | Singular | **Dual** | Plural |
|---|---|---|---|
| **1st conjugation** | | | |
| 1 | `-u` | `-jevje` | `-jem` |
| 2 | `-jeszj` | `-jeta` | `-jetje` |
| 3 | `-jet` | `-jetje` | `-ut` |
| **2nd conjugation** | | | |
| 1 | `-ju` | `-ivje` | `-im` |
| 2 | `-iszj` | `-ita` | `-itje` |
| 3 | `-it` | `-itje` | `-jat` |

`czitatj`: `czitaju, czitajeszj, czitajet` · `czitajevje, czitajeta, czitajetje` ·
`czitajem, czitajetje, czitajut`.

## 7.5 Aorist

The OCS sigmatic aorist. Built on the **infinitive** stem.

| | Singular | Dual | Plural |
|---|---|---|---|
| 1 | `-h` | `-hovje` | `-hom` |
| 2 | `-∅` | `-sta` | `-stje` |
| 3 | `-∅` | `-stje` | `-sza` |

`czitatj`: `czitah, czita, czita` · `czitahovje, czitasta, czitastje` ·
`czitahom, czitastje, czitasza`.

The second and third singular are bare stem — the inherited shape, and the reason
the aorist is instantly recognizable.

## 7.6 Imperfect

| | Singular | Dual | Plural |
|---|---|---|---|
| 1 | `-jah` | `-jahovje` | `-jahom` |
| 2 | `-jasze` | `-jaszeta` | `-jaszetje` |
| 3 | `-jasze` | `-jaszetje` | `-jahu` |

`czitatj`: `czitajah, czitajasze, czitajasze` · … · `czitajahom, czitajaszetje,
czitajahu`.

## 7.7 Perfect and pluperfect

The `l`-participle, agreeing in gender and number, with the copula:

| | Masculine | Feminine | Neuter | Dual | Plural |
|---|---|---|---|---|---|
| | `czital` | `czitala` | `czitalo` | `czitala` | `czitali` |

- **perfect**: `jesmj czital` "I have read"
- **pluperfect**: `byh czital` / `bjah czital` — see below

Unlike Russian, the copula is **not** dropped: `jesmj czital`, not `*czital`.

**Two pluperfects, and the auxiliary chooses between them.** OCS formed the
pluperfect with either the aorist or the imperfect of `byti`, and the two are not
synonymous. Ruthenian keeps both, with the auxiliary's own tense doing exactly
the work it does elsewhere:

| | Auxiliary | Sense |
|---|---|---|
| `byh czital` | aorist of `byti` | "I had read" — the anterior state is a completed point |
| `bjah czital` | imperfect of `byti` | "I had been reading" — the anterior state was ongoing |

This costs nothing: both auxiliaries already exist in §7.9's paradigm, and the
contrast is the same aorist/imperfect opposition the language draws everywhere
else, applied one layer up.

## 7.8 Future

| | Perfective | Imperfective |
|---|---|---|
| | present endings, future sense — `poczitaju` | `budu` + infinitive — `budu czitatj` |

`budu, budjeszj, budjet` · `budjevje, budjeta, budjetje` · `budjem, budjetje,
budut`.

## 7.9 The copula `byti`

Irregular, and the most frequent verb in the language. The full OCS paradigm,
dual included, and **never omitted**.

| | Singular | Dual | Plural |
|---|---|---|---|
| **present** | `jesmj`, `jesi`, `jestj` | `jesvje`, `jesta`, `jestje` | `jesm`, `jestje`, `sutj` |
| **aorist** | `byh`, `by`, `by` | `byhovje`, `bysta`, `bystje` | `byhom`, `bystje`, `bysza` |
| **imperfect** | `bjah`, `bjasze`, `bjasze` | `bjahovje`, `bjaszeta`, `bjaszetje` | `bjahom`, `bjaszetje`, `bjahu` |
| **future** | `budu`, `budjeszj`, `budjet` | `budjevje`, `budjeta`, `budjetje` | `budjem`, `budjetje`, `budut` |

Participle `byl/byla/bylo/byli`; infinitive `byti`; imperative `budi`.

Russian's zero copula (`он врач`) is an East Slavic innovation. Ruthenian follows
OCS, Polish and Ukrainian: `on jestj vracz`.

### `byti` is the language's one suppletive verb, deliberately

§1 removes suppletion everywhere else. `byti` keeps it, because it is the most
frequent verb in the language and every Indo-European language tolerates
suppletion in exactly this word — Latin `sum`/`fui`, English `is`/`was`/`been`,
OCS `jesmь`/`byxъ`/`bǫdǫ`.

The stems, and their sources:

| slot | stem | from | regular? |
|---|---|---|---|
| present | `jes-`, 3pl `s-` | PIE `*h₁es-` "be, exist" | no — athematic |
| aorist | `by-` | PIE `*bʰuH-` | **yes** — §7.5's endings on the infinitive stem |
| l-participle | `by-` | `*bʰuH-` | **yes** — §7.7's rule on the same stem |
| imperfect | `bja-` | `*bʰuH-` | no — regular would be `byja-` |
| future | `bud-` | `*bʰuH-`, nasal present | no |
| imperative | `bud-` | `*bʰuH-` | no |

**Splitting it into two verbs was considered and declined.** `*h₁es-` supplies
only the present, so the paradigm could be read as a stative copula `jesm`
(present only) beside a dynamic verb `bytj` "be, become" — a real typological
distinction (Irish `is`/`tá`, Spanish `ser`/`estar`), and one that would make
§7.7's two auxiliaries motivated rather than arbitrary: the perfect takes the
stative copula, the pluperfect the dynamic verb.

It was declined because the split does not actually remove the suppletion — the
`*bʰuH-` verb is itself `by-` against `bud-` — so paying for it would mean
regularizing onto one of them, and either choice loses pan-Slavic material that
every Slavic speaker knows on sight: onto `bud-` costs `byti`, `byl` and the
aorist, giving a past `budjel`; onto `by-` costs `budu`, the most recognizable
future marker in Slavic. One suppletive verb is cheaper than either.

## 7.10 Imperative and conditional

Imperative: present stem + `-i`, or the bare stem after `j`.

| | Singular | Dual | Plural |
|---|---|---|---|
| 2 | `czitaj` | `czitajta` | `czitajtje` |
| 1 (hortative) | — | `czitajvje` | `czitajm` |

**Third person: particle + present indicative.** No Slavic language builds a
synthetic third imperative, and Ruthenian does not either. Three particles are
permitted and they are interchangeable — this is a matter of register and taste,
not grammar:

```
da idjet         let him go      (OCS; the default)
nehaj idjet      let him go      (Ukrainian, Polish, Interslavic)
pustj idjet      let him go      (Russian)
```

`da` is listed first and is what a generator produces when nothing else is asked
for. Where `da` is already serving as the complementizer of a purpose clause
(§10.5), context disambiguates, exactly as it does in OCS.

Conditional: `l`-participle + the invariant particle `by` — `czital by`.

## 7.11 Present-stem mutation

Applied **by class**, never by stem shape: a class-1 verb with a labial-final
stem takes no mutation at all, verified across 1 977 Russian verbs without
exception.

**A stop keeps its place before its reflex; a fricative merges with its own.**

| | | | |
|---|---|---|---|
| `t` → `tcz` | `d` → `dzz` | `k` → `kcz` | `g` → `gzz` |
| `p` → `plj` | `b` → `blj` | `v` → `vlj` | `m` → `mlj` |
| `s` → `sz` | `z` → `zz` | `h` → `sz` | |

The first two rows are **additive** — the consonant survives and the reflex
follows it. The third is **replacive**.

```
vidjetj  → vidzz-    vidzzu     I see        (root vid- still visible)
voditj   → vodzz-    vodzzu     I lead
vozitj   → vozz-     vozzu      I carry
letjetj  → letcz-    letczu     I fly
leczitj  → lecz-     leczu      I heal      (no mutation: cz is already palatal)
pisatj'  → pisz-     piszu      I write
mahatj'  → masz-     maszu      I wave
ljubitj  → ljublj-   ljublju    I love
```

**Why the split falls where it does.** A stop stays audible in front of its
reflex, so writing it costs nothing and buys a legible root: `vidzzu` [vidʒu]
shows `vid-` where Russian's `вижу` does not, and `dzz` [dʒ] is the voiced
counterpart of `cz` [tʃ], which already contains its own stop. Two fricatives in
sequence do not survive: `s` + `sz` would be [sʃ], which is no Slavic sound and
collapses to [ʃː], so only the reflex is written.

The labial rules were always additive — `p` → `plj` keeps the `p` — so the stops
are following a rule this table already had, and the fricatives are the ones that
never could.

Two homographs disappear, and they disappear *because* of where the split falls.
`voditj` and `vozitj` both gave `vozzu` when `d` was replacive; `d` is a stop and
`z` a fricative, so they now separate as `vodzzu` and `vozzu`. Likewise `letjetj`
"fly" collided with `leczitj` "heal" at `leczu`, and now gives `letczu`.

Two rules disappear with it. `st` → `szcz` and `sk` → `szcz` were cluster special
cases; the general rule applies to the cluster's last consonant, and `t` and `k`
are both stops, so `krjestitj` → `krjestcz-` and `iskatj'` → `iskcz-` need no
rules of their own.

No output needs the separator `'`: `tcz`, `dzz`, `kcz` and `gzz` are each
unambiguous under the greedy reader.

`ov` → `u` is **not** in the table. It is class 2's stem formation (§7.3,
`njegodovatj` → `njegoduj-`), not iotation, and it replaces.

**Additive applies to iotation only — never to the palatalizations of §2.4.**
Those are positional changes before a front vowel and they *replace*: `drug` →
vocative `druzze`, not `*drugzze`. The two processes look alike in their
outputs and are not the same rule.

## 7.12 Participles and gerunds

| | Suffix | Example |
|---|---|---|
| present active | `-uszczij` / `-jaszczij` | `czitajuszczij` |
| past active | `-vszij` | `czitavszij` |
| present passive | `-jemyj` / `-imyj` | `czitajemyj` |
| past passive | `-nyj` / `-jonyj` / `-tyj` | `poczitanyj` |
| present gerund | `-ja` | `czitaja` |
| past gerund | `-v` | `czitav` |

Participles decline as adjectives and have **both long and short forms**, like
any adjective — the short passive participle is how the passive is built:
`dom jestj poczitan` "the house is read".

**The past passive `n` is single, not doubled.** Russian writes `-nnyj` long
against `-n` short (`прочитанный` / `прочитан`), which gives the long and short
forms *different stems*. Ruthenian writes one `n` throughout, so there is one
stem and the participle behaves like every other adjective:

| verb | stem | long | short |
|---|---|---|---|
| `poczitatj` | `poczitan` | `poczitanyj` | `poczitan` |
| `rjeszitj` | `rjeszjon` | `rjeszjonyj` | `rjeszjon` |
| `bitj` | `bit` | `bityj` | `bit` |

The doubling in Russian is orthographic convention rather than a distinction the
language uses — nothing is told apart by it — so removing it costs no contrast
and buys a participle that is a plain adjective stem. Which of `-n-`, `-jon-` and
`-t-` applies is decided by the conjugation class, not stored.

Their **existence** is structural: a perfective verb has no present participles
or present gerund; an intransitive verb has no passive participle.

# 8. Word formation

§12.4 ranks a native coinage above a Graeco-Latin borrowing. This section is the
machinery that makes that possible: a fixed, productive inventory of affixes with
which any root can be extended. Every suffix below is inherited and every one is
regular — the seam alternations are those of §2.4 and nothing else.

## 8.1 Nouns from verbs

| Suffix | Meaning | Gender | Example |
|---|---|---|---|
| `-nije` | the action, verbal noun | neut. II | `czitatj` → `czitanije` "reading" |
| `-tjelj` | the agent | masc. II | `czitatj` → `czitatjelj` "reader" |
| `-tjeljstvo` | the agent's activity | neut. II | → `czitatjeljstvo` |
| `-tjeljka` | female agent | fem. I | → `czitatjeljka` |
| `-ba` | the action, concrete | fem. I | `boritj` → `borjba` "struggle" |
| `-ok` | a single instance | masc. II | `brositj` → `brosok` "a throw" |
| `-lo` | the instrument | neut. II | `mytj` → `mylo` "soap" |

`-nije` is formed on the **past** stem and takes iotation: `poczitatj` →
`poczitanije`, `vidjetj` → `vidjenije`.

## 8.2 Nouns from nouns and adjectives

| Suffix | Meaning | Gender | Example |
|---|---|---|---|
| `-ostj` | abstract quality | fem. III | `dobr` → `dobrostj` "goodness" |
| `-stvo` | abstract or collective | neut. II | `czjelovjek` → `czjelovjeczstvo` |
| `-nik` | agent, person concerned with | masc. II | `uk-` → `ucznik` "pupil" |
| `-ica` | female, or the thing | fem. I | `car` → `carica` |
| `-ka` | female, diminutive | fem. I | `ruka` → `ruczka` |
| `-jec` | member, agent | masc. II | `borjec` "fighter" |
| `-iszcze` | augmentative, place | neut. II | `dom` → `domiszcze` |
| `-ok`, `-jek` | diminutive | masc. II | `dom` → `domok` |
| `-cze` | diminutive | neut. II | `okno` → `okoncze` |

`-ostj` is the productive abstract suffix and attaches to the **short**
adjective: `dobr` → `dobrostj`, not `*dobryjostj`.

## 8.3 Adjectives

| Suffix | Meaning | Example |
|---|---|---|
| `-n-` | general relational | `dom` → `domnyj` |
| `-sk-` | relational, of a place or people | `russ` → `russkyj` |
| `-ov-`, `-in-` | possessive, of a specific owner | `otjec` → `otcov`, `zzena` → `zzenin` |
| `-liv-` | inclined to | `czastj` → `czastlivyj` "fortunate" |
| `-at-` | provided with | `boroda` → `borodatyj` "bearded" |
| `-jenn-` | made of | `djerjevo` → `djerjevjennyj` |

Possessive adjectives in `-ov`/`-in` are **short-form only** and are the normal
way to say "X's" for a specific person, in place of a genitive: `otcov dom`
"father's house".

## 8.4 Verbs

| Suffix | Meaning | Example |
|---|---|---|
| `-ova-` | denominal, do/be an X | `car` → `carovatj` |
| `-i-` | causative / transitive | `bjel` → `bjelitj` "to whiten" |
| `-nu-` | semelfactive, one instance | `dvig-` → `dvinutj` |
| `-a-` | durative | `sjed-` → `sjedatj` |
| `-yva-` | secondary imperfective (§7.2) | `napisatj` → `napisyvatj` |

## 8.5 Prefixes

Verbal prefixes carry meaning **and** perfectivize (§7.2):

| | | | |
|---|---|---|---|
| `po-` empty perfectivizer | `na-` onto, a quantity | `za-` behind, beginning | `pri-` arrival |
| `pjerje-` across, re- | `vy-` out | `do-` up to, completion | `ot-` away |
| `pod-` under | `nad-` over | `raz-` apart | `s-` together, down |
| `u-` away, completion | `v-` into | `iz-` out of | `pro-` through |

Nominal and adjectival prefixes: `nje-` (negation, `njedobryj`), `bjez-`
(without, `bjezdomnyj`), `so-` (co-, `sobjesjednik`), `pra-` (ancestral,
`pradjed`), `naj-` (superlative).

## 8.6 Compounding

Two roots join with a **linking vowel** — `-o-` after a hard stem, `-je-` after a
soft or sibilant one:

`vod-o-provod` "water-conduit", `pol-je-vodstvo` "field-husbandry",
`czjern-o-zjem` "black-earth".

Compounding is fully productive and is the preferred device for coining
technical vocabulary, ahead of Graeco-Latin borrowing (§12.4). `zjemljeopisanije`
"geography" is available where `gjeografija` would otherwise be borrowed; both
are well-formed and the choice is one of register.

---

# 9. The closed classes

## 9.1 Adverbs

Formed from the **short** adjective in `-o`, or `-je` after a soft stem:
`dobro` "well", `iskrjennje` "sincerely". Comparatives follow the adjective:
`dobrjejje`. Adverbs of place and time distinguish location from goal, as OCS
did: `tu` "here" against `sjemo` "to here", `tam` / `tamo`.

## 9.2 Prepositions and their government

Government is lexical but the inventory is closed, so it is listed here in full.
Ruthenian's ablative absorbs the "source" senses that Russian expresses with the
genitive, which makes the system more transparent, not less.

| Case | Prepositions | Sense |
|---|---|---|
| **ablative** | `iz`, `ot`, `s` (down from), `do` (starting from) | **source, origin, motion away** |
| genitive | `u`, `bjez`, `dlja`, `okolo`, `protiv`, `vmjesto`, `kromje`, `radi` | possession, relation, absence |
| dative | `k`, `po` (along), `blagodarja` | goal, recipient |
| accusative | `v`, `na`, `za`, `pod`, `czjerjez`, `pro`, `skvozj` | motion **into**, direction |
| instrumental | `s` (together with), `za`, `pod`, `nad`, `mjezzdu`, `pjerjed` | accompaniment, location behind/above |
| locative | `v`, `na`, `o`, `pri`, `po` (after) | location **in**, topic |

Four prepositions govern two cases and the contrast is meaningful:

| | + accusative | + locative |
|---|---|---|
| `v` | `v dom` "into the house" | `v domi` "in the house" |
| `na` | `na stol` "onto the table" | `na stoli` "on the table" |
| | **+ accusative** | **+ instrumental** |
| `za` | `za dom` "to behind the house" | `za domom` "behind the house" |
| `pod` | `pod dom` "to under the house" | `pod domom` "under the house" |

And the ablative/instrumental pair on `s`:

`s doma` "down from the house" (ablative) against `s domom` "with the house"
(instrumental). Russian collapses the first into the genitive and loses the
symmetry.

## 9.3 Conjunctions

Coordinating: `i` "and", `a` "and/but (contrastive)", `no` "but", `ili` "or",
`ni…ni` "neither…nor". Subordinating: `czto` "that", `jesli` "if", `kogda`
"when", `poka` "while", `jako` "as, since", `da` "so that".

## 9.4 Particles

`zzje` (emphatic), `li` (interrogative), `by` (conditional), `nje` (negation),
`ni` (emphatic negation), `-to` (definitizing), `vot` (presentative), `li…li`
(alternative). All are second-position clitics except `nje`, which is proclitic
to the verb.

## 9.5 Negation

`nje` immediately precedes the verb. **Negation takes the genitive** for the
direct object: `nje vizzu domogo` "I do not see the house" — the genitive of
negation, which OCS has and Polish still requires obligatorily. It is pan-Slavic,
not a Russian complication.

Negative pronouns require double negation (§5.6): `nikto nje czitajet`.

# 10. Syntax

## 10.1 Word order

Free, and used for **information structure** rather than grammatical relations —
the case system already marks those. The unmarked order is **SVO**; anything
fronted is topical, anything final is focal.

```
Ivan czitajet knigu.        Ivan reads a book.        (neutral)
Knigu czitajet Ivan.        It is Ivan who reads the book.
Czitajet Ivan knigu.        Ivan is READING the book.
```

Because the accusative `knigu` is distinct from the nominative `kniga`, no order
is ambiguous. Within a noun phrase, order is fixed: demonstrative — possessive —
adjective — noun (`ta moja dobraja kniga`).

### 10.1a Clitics stand in second position

Word order is free for full words. **Clitics are not free**: the pronoun clitics
(§5.1a), the reflexive `sja` and the question particle `li` all occupy the
position immediately after the first stressed constituent of the clause. This is
Wackernagel's law, inherited from PIE and systematic in OCS.

```
Ivan mi ju dal.              Ivan gave it to me.
Vczera mi ju Ivan dal.       Yesterday Ivan gave it to me.
Dal li mi ju Ivan?           Did Ivan give it to me?
On sja myjet.                He washes himself.
```

The cluster has a fixed internal order — **`li` · dative · accusative ·
reflexive** — and it is a single unit: nothing may be inserted into it, and the
constituent it follows may be of any size (`Ta moja dobraja kniga mi sja
nravitj`).

A clitic cannot begin a clause, cannot be stressed or focused, cannot stand alone
as an answer, and cannot follow a preposition. Each of those requires the full
form (§5.1a).

> Wackernagel, J., "Über ein Gesetz der indogermanischen Wortstellung",
> *Indogermanische Forschungen* 1, 1892. Radanović-Kocić, V., *The Grammar of
> Serbo-Croatian Clitics*, 1988, for the modern Slavic treatment. *Citation-based;
> not measured.*

## 10.2 Agreement

| Agrees with the head in | |
|---|---|
| adjectives, participles, ordinals, `odin` | gender, number, case, **and definiteness** |
| the verb, present and future | person, number |
| the verb, past (`l`-participle) | gender, number |
| the copula | person, number |

Definiteness agreement is the feature Russian lacks: a definite noun phrase takes
long-form modifiers throughout — `ta dobraja kniga`, not `*ta dobra kniga`.

**Dual agreement is obligatory.** Two of anything takes dual modifiers and a dual
verb: `ta dobraja knizi jesta` "those two good books are".

## 10.3 Definiteness without an article

The long/short adjective (§4) is the only definiteness marker:

```
dobr czjelovjek jestj zdjesj.     A good man is here.
dobryj czjelovjek jestj zdjesj.   The good man is here.
```

Where a noun phrase has no adjective, definiteness is unmarked — as in every
Slavic language except Bulgarian and Macedonian. The particle `-to` may
optionally definitize a bare noun (`dom-to`), which is colloquial rather than
grammatical.

## 10.4 Questions

Three devices, in ascending formality:

1. **Intonation alone** — `ty czitajeszj?`
2. **The clitic `li`**, in second position — `czitajeszj li ty?` This is the
   neutral written question. `li` is a clitic and shares the second-position
   cluster with the pronoun clitics, standing first within it (§10.1a).
3. **An interrogative word**, fronted — `czto ty czitajeszj?`

## 10.5 Subordination

| Conjunction | Use |
|---|---|
| `czto` | statement complement — `znaju, czto on czitajet` |
| `cztoby` | purpose or irrealis complement — `hoczu, cztoby on czital` |
| `jesli` | condition — `jesli by on czital…` |
| `kogda`, `poka` | time |
| `jako` | cause, manner |

Relative clauses use `izzje` when restrictive and `kotoryj` when
non-restrictive (§5.5). The relative agrees with its antecedent in gender and
number but takes its case from **its own clause**:

```
czjelovjek, jegozzje vizzu…        the man whom I see…      (acc in its clause)
czjelovjek, izzje czitajet…        the man who reads…       (nom in its clause)
```

Purpose after a verb of motion uses the infinitive or a `da` clause: `idu
lovitj`, `idu, da lovju`. OCS distinguished a **supine** here — `idǫ lovitъ`
against `idǫ loviti` — which Ruthenian does not keep (§13).

## 10.6 The cases in use

| Case | Bare use | With a preposition |
|---|---|---|
| nominative | subject; predicate noun with the copula | — |
| vocative | address | — |
| accusative | direct object; duration | motion into (§9.2) |
| genitive | possession; the object of negation; after 5+ | absence, relation |
| **ablative** | standard of comparison; cause | **source, motion away** |
| dative | indirect object; the logical subject of impersonals | goal |
| instrumental | means; the predicate of `byti` in the past | accompaniment |
| locative | — (never bare) | location, topic |

The locative is the only case that **cannot** occur without a preposition —
inherited, and true of every Slavic language.

The instrumental predicate is worth noting: `on jestj vracz` (nominative,
permanent) against `on byl vraczom` (instrumental, temporary or past role).

---

# 11. Summary of paradigm sizes

Distinct **surface forms**, after syncretism — 24 nominal cells (8 cases × 3
numbers) never yield 24 forms:

| Word class | Singular | Dual | Plural | Total |
|---|---:|---:|---:|---:|
| noun, declension II masculine (`dom`) | 7 | 3 | 5 | **15** |
| noun, declension II neuter (`okno`) | 6 | 3 | 5 | **14** |
| noun, declension I feminine (`zzena`) | 6 | 3 | 5 | **14** |
| noun, declension III (`noczj`) | 3 | 3 | 5 | **11** |
| adjective (`dobryj`) | — | — | — | 24 across all genders |
| verb, one aspect | 9 present + 5 past + 3 imperative + 6 participles |

Compare Russian, which has 12 nominal cells and typically 9–11 distinct forms.
Ruthenian doubles the categories and adds four to six forms, because the added
categories are heavily syncretic by design — the dual contributes three forms
regardless of how many cases exist, and the ablative contributes one, in two
paradigms out of four.

That is the whole design: **conservatism in the inventory, regularity in the
realization.** A Ruthenian noun expresses more distinctions than a Russian one
and is barely larger, because what was removed — mobile stress, fleeting vowels,
heteroclitic stems, eight declensions — cost more than what was added.

---

# 12. The lexicon: sources and borrowing

## 12.1 Why the lexicon is multi-source

Ruthenian's vocabulary is drawn from all of East Slavic, plus Polish, Czech,
Serbo-Croatian and Old Church Slavonic, plus a rule-governed borrowing system.
This is not eclecticism: **Russian alone cannot supply the language specified
above.**

| What the spec needs | Russian | Recoverable from |
|---|---|---|
| yat (`-i` endings, `chlib`) | merged into `e` | Ukrainian `i`, Polish `ie`/`ia`, OCS `ě` |
| second palatalization | levelled to **0 %** | Ukrainian **99 %**, OCS 66 % |
| the vocative | 40 relic forms | Ukrainian 25 180, OCS 6 186 |
| the dual | lost | OCS 77 714 forms |
| nasal vowels (etymology) | lost | **Polish only** (`ą`, `ę`) |

Measured lemma inventories, each from a full scan:

| Language | Tier | Single-word lemmas |
|---|---|---:|
| Russian | primary | 419 283 |
| Ukrainian | primary | 52 223 |
| Belarusian | primary | 6 899 |
| Polish | secondary | 152 325 |
| Old Church Slavonic | secondary | 4 311 |
| Czech | secondary | *not yet measured* |
| Serbo-Croatian | secondary | *not yet measured* |

Czech and Serbo-Croatian are added for **lexical breadth**, not for a feature:
the arguments that once justified them — Czech vowel length, Serbo-Croatian pitch
accent and a living aorist — are all moot, since Ruthenian marks neither length
nor pitch (§2.1) and takes its aorist from OCS (§7.5). They supply attested
cognates where East Slavic has gaps, and nothing else. Their counts must be
measured by a full scan before they appear here.

## 12.2 How a Ruthenian word is chosen

A Ruthenian lemma is not "the Russian word transliterated". It is the reflex the
**Proto-Slavic etymon** would have in Ruthenian's phonology, with the attested
cognates as evidence.

**The method is Russian-anchored.** Since the vocabulary is East Slavic and
Russian-centred (§1), Ruthenian is not the average of seven languages — it is
Russian, adjusted wherever Russian destroyed something the grammar needs:

1. **Take the Russian lemma as the spine.** It is by far the largest inventory
   (419 283), and it is what the vocabulary brief asks for.
2. **Consult the other six only at known gaps.** Russian's mergers are a short,
   enumerable list, and each has a designated source:

   | Russian lost | consult |
   |---|---|
   | yat | Ukrainian, Polish, OCS |
   | the nasals (etymology only) | Polish |
   | the second palatalization | Ukrainian, OCS |
   | the dual, the aorist, the imperfect | OCS |
   | the vocative | Ukrainian, OCS |

3. **Apply §2.6's correspondences** to derive the Ruthenian form.
4. **Record the evidence** — which languages attested it, and how confidently the
   derivation follows.

This turns an n-way clustering problem into a lookup keyed on a fixed list, which
is what makes the lexicon buildable at all. **Where Russian has no lemma**, the
entry falls back to grouping across the remaining six, and `Provenance` records
that it did.

**An honest limit, measured.** Explicit Proto-Slavic etymology links are thin:
5 517 distinct etyma, of which only 88 have reflexes tagged in all five original
languages and 2 700 in just one. The anchored method sidesteps most of this — it
needs correspondence, not etymology tags — but the fallback path does not, and
that path remains the hardest unsolved problem in the lexicon.

Where reconstruction is uncertain, the entry records that. A form derived from
one language's reflex is not the same claim as one attested across four, and
`Provenance` must distinguish them.

## 12.3 Borrowing: the regularized system

International vocabulary is adapted by rule rather than borrowed ad hoc, so that
`nacija` is predictable from *natio* rather than memorized from Russian.

### Latin and Greek — the learned layer

| Source ending | Ruthenian | Gender / declension | Example |
|---|---|---|---|
| `-tiō, -tiōnem` | `-cija` | fem., I | *natio* → `nacija` |
| `-tās, -tātem` | `-tet` | masc., II | *universitas* → `universitet` |
| `-or, -ōrem` | `-or` | masc., II | *doctor* → `doktor` |
| `-us` (2nd decl.) | `-∅` | masc., II | *circus* → `cirk` |
| `-um` (2nd decl.) | `-um`, unchanged | masc., II | *museum* → `museum` |
| `-a` (1st decl.) | `-a` | fem., I | *forma* → `forma` |
| Gk `-ισμός` | `-izm` | masc., II | *organismos* → `organizm` |
| Gk `-ία` | `-ija` | fem., I | *philosophia* → `filosofija` |
| Gk `-της` | `-t` | masc., II | *poiētēs* → `pojet` |

The Latin **oblique** stem is the base, not the nominative — *natio* borrows
from *natiōn-* and not from *natio*, exactly as Slavic has always done it,
giving the citation form `nacija`. Once adapted, a loan declines as a native
word of its class, and **strictly**: `nacija`, genitive `nacii`, dative
`naciji`, instrumental `nacijoj` — soft declension I on the vowel-final stem
`naci-` (§3.5). Russian's `-ия` sub-pattern is not imported; the endings are the
ones `zjemlja` takes.

### The learned layer stays close to its source

**The source shape is kept wherever Ruthenian's own letters can write it.** A
source `e` is borrowed as `e`, not `je`: Latin and Greek `e` does not palatalize
a preceding consonant, and `e` is precisely the Ruthenian letter for that vowel
(§2.1) — so `universitet`, not `univjersitjet`. The learned layer is borrowed
**from Latin and Greek directly**, and re-spelling it as though it had arrived
through Cyrillic would import a palatalization the source does not have.

For the same reason a Latin ending that Ruthenian can spell is simply kept.
*Museum* is `museum`, not `muzeo`: the `-um` is writable, it is recognizable —
Russian had `музеум` before it had `музей` — and reshaping it to `-o` buys
nothing the declension does not already give. A loan is naturalized by
**declining** it (§3.9), not by resegmenting it.

Two consequences, both accepted:

- **A learned loan is not the transliteration of its Russian counterpart.**
  `universitet` reads back as университэт rather than университет. No contract
  is broken: §2.1's bijection is claimed over words transliterated *from*
  Cyrillic, and these are not — they are borrowed from the source language, as
  §12.2 borrows everything else from its etymon rather than from Russian's
  spelling of it.
- **The shape decides the class, so gender follows the ending and not the
  source.** `universitet` ends in a plain consonant, which is masculine
  declension II by §3.2, though *universitas* is feminine; `museum` is
  masculine II for the same reason, though *museum* is neuter. This is what
  "once adapted, a loan declines as a native word of its class" means; keeping
  the Latin gender would take a word-final `j` and the mark
  (`universitetj'`), which is a Slavic diacritic added to a form whose whole
  point is to stay close to the source.

**The one exception is a referent that has a sex.** A man's name, a male animal,
a word for a woman: there the gender is a fact about the thing rather than about
the ending, and agreement follows the thing. This needs no new machinery,
because §2.1's two marks already say exactly this much — a **capital** first
letter for animacy (§3.7), and the word-final **`'`** where the ending predicts
the wrong gender (§3.2). A borrowed male name in `-a` is therefore `Seneka'`, on
precisely the model of native `sluga'`: declension I in form, masculine and
animate in agreement. Everything without a sex — and that is nearly the whole
learned layer — takes its gender from its ending and carries no mark.

### Sanskrit — the purist layer

Available for coining where a native or Graeco-Latin form is unwanted. Sanskrit
is not a *source* of everyday vocabulary but of learned formations, adapted
through the same regular correspondences: `-a` → `-∅` masculine, `-ā` → `-a`
feminine, `-am` → `-o` neuter, `-tva` → `-stvo`, `-tā` → `-ostj`.

Because Sanskrit and Slavic are cognate, some Sanskrit borrowings would collide
with inherited words (`dāna-` "gift" beside inherited `dan-`). Where they do, the
inherited word wins and the Sanskrit form is not borrowed — a rule that keeps the
layer from corrupting the core.

### English, French, German — the modern layer

| Pattern | Ruthenian | Example |
|---|---|---|
| ends in a consonant | masc., II, unchanged | *computer* → `kompjutjer` |
| ends in `-a` | fem., I | *pizza* → `pica` |
| ends in `-o`, `-u`, `-i`, `-e` | neut., II, **declined** | *metro* → `mjetro`, `mjetrogo`, `mjetru` |
| `-tion` (Fr./Eng.) | `-cija` | *station* → `stancija` |
| `-ing` | `-ing`, masc. | *marketing* → `markjeting` |

The third row is the notable one: Ruthenian has **no indeclinables**. Russian
leaves 1 193 loans undeclined; Ruthenian declines them all. This is
regularization by *addition* — the loan is naturalized rather than quarantined.

### Stress in loans

Loans take **fixed stress on the syllable stressed in the source**, and keep it
throughout the paradigm. No mobile patterns, ever.

## 12.4 The layers are ordered

When two sources could supply a word, the earlier layer wins:

1. inherited Slavic, attested in two or more source languages
2. inherited Slavic, attested in one
3. Old Church Slavonic, for learned and abstract vocabulary
4. a native coinage from Ruthenian roots
5. a Graeco-Latin borrowing
6. a modern borrowing

Within layers 1 and 2, the **tier** decides: a primary-tier attestation
(Russian, Ukrainian, Belarusian) outranks a secondary one (Polish, OCS, Czech,
Serbo-Croatian). This is what keeps a seven-source lexicon East Slavic in
character rather than pan-Slavic.

Note that layer 3 is a *lexical* layer — OCS supplying a word East Slavic never
had — and is distinct from §2.6a's productive doublets, which are register
variants of a word Ruthenian already has.

This makes Ruthenian **purist by default but not dogmatic**: a native word is
preferred where one exists, and an international word is adopted where it does
not, by rule rather than by taste.

---

# 13. Open questions

**Settled in this revision.** What "conservative" means (§1: grammar from OCS,
phonology from Russian, vocabulary East Slavic with an OCS learned layer); the
sound correspondences (§2.6) and the productive learned layer (§2.6a); the
fleeting vowel, abolished (§3.9); the animate accusative (§3.7); the predicate
adjective (§4); the clitic pronoun series and Wackernagel placement (§5.1a,
§10.1a); the reflexive as a free clitic (§5.2); aspect, with its two stored
classes (§7.2, §7.2a); one aorist formation (§7.5); the two pluperfects (§7.7);
the third-person imperative (§7.10); seven lexical sources with tiers (§12.1);
and the Russian-anchored reconstruction method (§12.2).

**Closed against restoration.** The middle voice — lost in all Slavic, its work
done by the reflexive clitic, and restoring it would roughly double the verb
paradigm for a category no Slavic language attests. Vowel quantity — Russian has
none, the pure-ASCII alphabet cannot write it, and Ruthenian has one orthography
rather than a second diacritic notation to hide it in. A distinct ablative plural —
PIE does not have one either (ablative and dative are syncretic in the plural
throughout), so there is nothing to restore.

The **supine** — OCS distinguished `idǫ lovitъ` "I go in order to hunt" from the
infinitive `idǫ loviti`, governing the genitive, and Ruthenian does not keep it.
It is the one category this document declines that OCS had and the brief would
otherwise want, so the reason is worth stating: purpose after a verb of motion is
already expressible two ways (§10.5), the distinction it draws is one no living
Slavic language maintains, and a second infinitive-like form differing from the
first by a single letter buys a contrast that would be inaudible in speech.

**Nothing is open**, and the last thing that was is worth recording because
implementing §3 is what settled it.

Rule 2a used to admit `cz` and `szcz` only, on the strength of §2.2's exception
list, which made `otjecze` and `druzzje` both derivable but left §7.3's
`piszeszj` underivable. The resolution was that §2.2's list was wrong: `zz` and
`sz` are inherently **hard**, as `ж` and `ш` are in Russian, so they have no soft
value for a `j` to mark either. Both kinds of consonant reject the glide, for
opposite reasons, and one rule now covers all four — `otjecze`, `druzze`,
`piszeszj`.

Three consequences fell out, and each turned out to move *towards* Russian rather
than away:

| | was | is | Russian |
|---|---|---|---|
| vocative of `drug` | `druzzje` | `druzze` | `друже` |
| instrumental of `nozz` | `nozzjem` | `nozzom` | `ножом` |
| plural of `noczj'` | `noczjev`, `noczjam` | `noczev`, `noczam` | `ночей`, `ночам` |

Everything else this document raised has an answer above.

## Written in this revision

Word formation (§8), the full numeral system (§6), the full pronoun system
including possessives, demonstratives, relatives and the negative/indefinite
series (§5), the preposition-government table (§9.2), and syntax (§10).

## Still to write

Not decisions — prose this document is missing, listed so it is not forgotten.

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
  paradigm table can** — it is the highest-value item here.

- **A frequency-ordered core vocabulary** — a Swadesh or Leipzig–Jakarta list
  realized in Ruthenian, as the minimum demonstration that §2.6's correspondences
  produce usable words.

Three closed lists the grammar calls for, each enumerable from Russian data:

- **The inherently perfective verbs** (§7.2) — roughly 100–200, identifiable from
  aspect metadata.
- **The aspect partners** (§7.2) — which prefix bleaches for each imperfective,
  `czitatj` → `proczitatj`, `pisatj` → `napisatj`.
- **The hidden-consonant verbs** (§7.3) — `zzitj` → `zziv-`, `plytj` → `plyv-`,
  which take a listed present stem.
