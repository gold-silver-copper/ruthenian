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
Belarusian.

Orthography is specified separately in [`ORTHOGRAPHY.md`](ORTHOGRAPHY.md); all
forms below are in the Ruthenian Latin alphabet.

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
| 2nd palatalization | **kept** | lost (0 %) | kept (99 %) | kept (66 %) | n/a |
| yat distinction | **kept** (as `-i`) | lost | kept (as `i`) | kept (as `ě`) | n/a |

**Restored:** ablative, vocative, dual, dual pronouns, dual verb agreement, the
aorist, the imperfect, the OCS long/short adjective, the full copula.
**Removed:** mobile stress, heteroclitics, marginal cases, soft adjective stems,
indeclinables, fleeting vowels in the genitive plural, irregular numeral
government, verb classes 7–16, **lexical aspect pairing**.
**Kept:** aspect, animacy, the *n-* prefix, reflexives, iotation, participles,
**both palatalizations** (Ukrainian 99 %, Russian 0 %), and the yat distinction
via the Ukrainian reflex `-i`.

---

# 2. Phonology and orthography

## 2.1 The alphabet is pure ASCII

Ruthenian is written in **unaccented ASCII**. There are no diacritics, no
combining marks, and no letters outside `a`–`z`. The digraphs `cz sz zz szcz ja
je jo ju` and the separator `'` carry everything a diacritic would
(`ORTHOGRAPHY.md`).

**Stress is not written.** It is real, lexical and fixed per word, but ordinary
text does not mark it — as in Russian, Ukrainian and Polish orthography, and as
in Interslavic, which does not encode stress at all. Dictionaries and teaching
materials may mark it; running text never does.

> **Planned: an etymological alphabet.** A second, diacritic-bearing notation for
> dictionaries and etymology — ASCII with diacritics, on the model of
> Interslavic's etymological alphabet — would let one spelling distinguish what
> the standard orthography merges: yat (`ě`) from `e`, the nasals (`ę`, `ǫ`), the
> jers, and stress. It is **not** part of the standard language and is deferred
> until the lexicon can supply the etymological information (§10).

## 2.2 Consonants

| | labial | dental | alveolar | palatal | velar |
|---|---|---|---|---|---|
| stop | `p b` | `t d` | | | `k g` |
| affricate | | `c` | `cz` | | |
| fricative | `f v` | `s z` | `sz zz` | `szcz` | `h` |
| nasal | `m` | `n` | | | |
| liquid | | `l r` | | `j` | |

Every consonant except `j`, `cz`, `szcz` has a **hard** and a **soft**
(palatalized) value; softness before a vowel is written with the `j`-digraphs
(`ja je jo ju`) and word-finally or before a consonant with `j` alone (`konj`).

## 2.3 Vowels

`a e i o u y` — six, with `y` the back counterpart of `i`. The iotated series
`ja je jo ju` are `j` + vowel, not separate phonemes.

## 2.4 The three palatalizations

Inherited, productive, and fully automatic:

| | before | `k` | `g` | `h` | trigger |
|---|---|---|---|---|---|
| **first** | front vowels, `j` | `cz` | `zz` | `sz` | vocative `-je`, present stem, comparative |
| **second** | yat-derived `-i` | `c` | `z` | `s` | locative sg, feminine dative sg, dual |
| **third** | after `i`, `j`, `r` | `c` | `z` | `s` | certain derivational suffixes |

Russian levelled the second away entirely (0 %); Ukrainian keeps it at 99 % and
OCS at 66 %. Ruthenian keeps all three.

## 2.5 Phonotactics

Syllables are maximally `CCCVCC`. Word-final consonant clusters are broken by a
fleeting vowel where the historical jers stood (§3.9). Voicing assimilates
regressively within a word; final devoicing is **not** written.

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

`drug` → vocative `druzzje`, locative `druzi`.

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
same endings with automatic spelling adjustments (§2.7).

| Declension | Contents | Example |
|---|---|---|
| **I** | feminine in `-a` | `zzena` (woman), `zjemlja` (earth) |
| **II** | masculine, and neuter | `dom` (house), `konj` (horse), `okno` (window), `polje` (field) |
| **III** | feminine ending in a consonant | `noczj` (night), `kostj` (bone) |

Each has a **hard** and a **soft** variant; the soft variant substitutes `je` for
`o`, `ju` for `u`, `i` for `y` — a single alternation, applied everywhere.

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

¹ animate nouns take the genitive form in the accusative (§2.6).

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
| vocative | **`druzzje`** ³ | = nom | = nom |
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

## 3.5 Declension I — feminine in `-a`

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

## 3.6 Declension III — feminine in a consonant

### `noczj` "night" (stem `nocz-`)

| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `noczj` | `noczi` | `noczi` |
| vocative | `noczi` | = nom | = nom |
| accusative | `noczj` | `noczi` | `noczi` |
| genitive | `noczi` | `noczju` | `noczjev` |
| **ablative** | `noczi` | = dat | = dat |
| dative | `noczi` | `noczjma` | `noczjam` |
| instrumental | `noczjju` | `noczjma` | `noczjami` |
| locative | `noczi` | `noczju` | `noczjah` |

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

Masculine and plural **animate** nouns take the genitive form in the accusative;
inanimates take the nominative. Inherited, pan-Slavic, and information-bearing —
Ruthenian keeps it unchanged.

```
vizzu dom       I see the house    (inanimate: acc = nom)
vizzu konja     I see the horse    (animate:   acc = gen)
```

## 3.8 Automatic spelling adjustments

Not declensions — a single set of rules applied to every ending:

1. after `k g h` and `zz sz cz szcz`, `y` is written `i` (`knigi`, not `*knigy`);
2. after `zz sz cz szcz c`, unstressed `o` is written `je`;
3. a stem-final soft sign belongs to the ending, not the stem (`kon` + `j`);
4. **first palatalization** before the vocative `-je`: `k`→`cz`, `g`→`zz`,
   `h`→`sz` (`drug` → `druzzje`);
5. **second palatalization** before any yat-derived `-i` — the locative
   singular, the feminine dative singular, the neuter and feminine dual:
   `k`→`c`, `g`→`z`, `h`→`s` (`drug` → `druzi`, `kniga` → `knizi`).

Rules 4 and 5 are morphophonemic rather than orthographic — they change the
consonant, not just its spelling — but they are fully automatic and belong with
the others.

These replace Russian's velar-, sibilant-, `ц`- and vowel-stem declensions, which
differ from the hard type *only* by these automatic effects.

## 3.9 What was removed, and why

| Removed | Was | Justification |
|---|---|---|
| mobile stress | 6 patterns + 4 primed | stress is fixed, lexical and unwritten |
| fleeting vowel in gen. pl. | `okno` → `okon` | genitive plural is uniformly `-ov` |
| heteroclitics | `vremja/vremeni` | ~15 lemmas memorized individually in every Slavic language |
| partitive, 2nd locative, count form | 436 Russian lemmas | the ablative does their semantic work |
| indeclinables | 1 193 Russian nouns | regularized **by addition**: `metro`, `metrogo`, `metru` |
| plural-/singular-only defectiveness | 2 003 nouns | every noun has all three numbers |

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

## 4.1 Short (indefinite) — nominal declension

`dobr` "good", masculine. Endings are the noun's, exactly.

| Case | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `dobr` | `dobro` | `dobra` | `dobra` | `dobry` |
| vocative | `dobrje` | = nom | `dobro` | = nom | = nom |
| accusative | `dobr` | `dobro` | `dobru` | = nom | `dobry` |
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
| accusative | `dobryj` / `dobrogo` ¹ | `dobroje` | `dobruju` | `dobraja` | `dobryje` / `dobryh` ¹ |
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

## 5.1 Personal

The dual pronouns are restored from OCS (`vě` "we two", `va` "you two").

| | 1sg | 2sg | **1du** | **2du** | 1pl | 2pl |
|---|---|---|---|---|---|---|
| nominative | `ja` | `ty` | `vje` | `va` | `my` | `vy` |
| accusative | `mjenja` | `tjebja` | `na` | `va` | `nas` | `vas` |
| genitive | `mjenjego` | `tjebjego` | `naju` | `vaju` | `nas` | `vas` |
| **ablative** | `mjenja` | `tjebja` | = dat | = dat | = dat | = dat |
| dative | `mnje` | `tjebje` | `nama` | `vama` | `nam` | `vam` |
| instrumental | `mnoj` | `toboj` | `nama` | `vama` | `nami` | `vami` |
| locative | `mnje` | `tjebje` | `naju` | `vaju` | `nas` | `vas` |

Third person `on` / `ona` / `ono`, plural `oni`, dual `ona`, declining like
`dobryj` with the pronominal stem `j-`: `jego`, `jemu`, `jim`, `jem`.

**The *n-* prefix** is kept: after a preposition, third-person forms beginning in
`j-` take `nj-` — `u njego`, `s njim`, `k njej`. This is a Slavic innovation from
a reanalysed preposition-final nasal, shared by all four East Slavic lects.

## 5.2 Reflexive

`sjebja` (acc/abl), `sjebjego` (gen), `sjebje` (dat/loc), `soboj` (ins). No
nominative — the cell does not exist. The bound reflexive verb suffix is `-sja`.

## 5.3 Demonstrative and interrogative

`toj` / `ta` / `to` "this, that"; `kto` "who", `czto` "what", declining
pronominally: `kogo`, `komu`, `kjem`, `kom`.

---

# 6. Numerals

## 6.1 Government — the payoff of the dual

Russian's genitive singular after 2–4 is petrified **dual** agreement. With a real
dual, the rule is not simplified — it disappears:

| Numeral | Governs | Example |
|---|---|---|
| `odin` | agrees, singular | `odin dom` |
| **`dva`** | **the dual** | `dva doma` |
| `tri`, `czjetyrje` | nominative plural | `tri domi` |
| `pjatj` and above | genitive plural | `pjatj domov` |

`dva doma` is not "genitive singular after two" — it is the nominative **dual**,
which is what it originally was. There is no 11–14 exception and no last-digit
rule: 5 and above always take the genitive plural.

## 6.2 Cardinals

`odin`, `dva`, `tri`, `czjetyrje`, `pjatj`, `sjestj`, `sjedjem`, `vosjem`,
`djevjatj`, `djesjatj`, `sto`, `tysjacza`.

`odin` declines as an adjective and agrees. `dva` has dual forms (`dva` /
`dvuh` / `dvuma`). `tri` and `czjetyrje` decline. From `pjatj` upward the
numerals decline as declension III nouns — inherited behaviour, kept.

## 6.3 Ordinals

`pjervyj`, `vtoryj`, `trjetij`, `czjetvjortyj`, `pjatyj` … declining as
adjectives.

---

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

## 7.2 Aspect is derived, not listed

Every Slavic language stores aspect pairs in the dictionary because pairing is
unpredictable. **Ruthenian derives them.** Aspect is a function of the verb's
shape alone:

| Shape | Aspect | Example |
|---|---|---|
| bare stem | **imperfective** | `czitatj` "to read" |
| any prefix | **perfective** | `poczitatj`, `napisatj` |
| prefix + `-yva-`/`-iva-` | **imperfective** again | `napisyvatj` |

Three rules, no exceptions, no lexical entry:

1. **`po-` is the empty perfectivizer.** Every imperfective forms its perfective
   with `po-` and nothing else. `czitatj` → `poczitatj`. `po-` adds no meaning; it
   is a grammatical marker, not a lexical prefix.
2. **Lexical prefixes perfectivize as a side effect.** `na-`, `za-`, `pri-`,
   `pjerje-`, `vy-`, `do-`, `ot-`, `pod-`, `raz-`, `s-`, `u-`, `v-`, `iz-`,
   `pro-` each add meaning *and* make the verb perfective.
3. **`-yva-` re-imperfectivizes.** Any prefixed perfective becomes imperfective
   with `-yva-` (`-iva-` after a soft stem): `napisatj` → `napisyvatj` "to be
   writing down".

There are no biaspectual verbs and no suppletive pairs. Aspect is computable from
the surface form, which is what lets `ruthenian-core` treat it as a rule rather
than as data.

## 7.3 Conjugation classes

Six, Zaliznyak's 1–6; classes 7–16 are regularized onto them.

| Class | Infinitive | Present stem | Conj. | Example |
|---|---|---|---|---|
| 1 | `-atj` | stem + `j` | 1st | `czitatj` → `czitaj-` |
| 2 | `-ovatj` | `ov` → `uj` | 1st | `njegodovatj` → `njegoduj-` |
| 3 | `-nutj` | theme drops | 1st | `dvinutj` → `dvin-` |
| 4 | `-itj` | theme drops, 1sg mutates | 2nd | `govoritj` → `govor-` |
| 5 | `-jetj` | theme drops | 2nd | `vidjetj` → `vid-` |
| 6 | `-atj` | theme drops, stem mutates | 1st | `pisatj` → `pisz-` |

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
- **pluperfect**: `bjeh czital` "I had read"

Unlike Russian, the copula is **not** dropped: `jesmj czital`, not `*czital`.

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
| **aorist** | `bjeh`, `bje`, `bje` | `bjehovje`, `bjesta`, `bjestje` | `bjehom`, `bjestje`, `bjesza` |
| **imperfect** | `bjah`, `bjasze`, `bjasze` | `bjahovje`, `bjaszeta`, `bjaszetje` | `bjahom`, `bjaszetje`, `bjahu` |
| **future** | `budu`, `budjeszj`, `budjet` | `budjevje`, `budjeta`, `budjetje` | `budjem`, `budjetje`, `budut` |

Participle `byl/byla/bylo/byli`; infinitive `byti`; imperative `budi`.

Russian's zero copula (`он врач`) is an East Slavic innovation. Ruthenian follows
OCS, Polish and Ukrainian: `on jestj vracz`.

## 7.10 Imperative and conditional

Imperative: present stem + `-i`, or the bare stem after `j`.

| | Singular | Dual | Plural |
|---|---|---|---|
| 2 | `czitaj` | `czitajta` | `czitajtje` |
| 1 (hortative) | — | `czitajvje` | `czitajm` |

Conditional: `l`-participle + the invariant particle `by` — `czital by`.

## 7.11 Present-stem mutation

Applied **by class**, never by stem shape: a class-1 verb with a labial-final
stem takes no mutation at all, verified across 1 977 Russian verbs without
exception.

| | | | |
|---|---|---|---|
| `ov` → `u` | `s` → `sz` | `t` → `cz` | `d` → `zz` |
| `z` → `zz` | `st` → `szcz` | `k` → `cz` | `h` → `sz` |
| `p` → `plj` | `b` → `blj` | `v` → `vlj` | `m` → `mlj` |

## 7.12 Participles and gerunds

| | Suffix | Example |
|---|---|---|
| present active | `-uszczij` / `-jaszczij` | `czitajuszczij` |
| past active | `-vszij` | `czitavszij` |
| present passive | `-jemyj` / `-imyj` | `czitajemyj` |
| past passive | `-nnyj` / `-jonnyj` / `-tyj` | `poczitannyj` |
| present gerund | `-ja` | `czitaja` |
| past gerund | `-v` | `czitav` |

Participles decline as adjectives and have **both long and short forms**, like
any adjective — the short passive participle is how the passive is built:
`dom jestj poczitan` "the house is read".

Their **existence** is structural: a perfective verb has no present participles
or present gerund; an intransitive verb has no passive participle.

# 8. The closed classes

**Adverbs** are formed from adjectives in `-o`: `dobro` "well". Comparatives
follow the adjective: `dobrjejje`.

**Prepositions** govern case, and which case is lexical — it is listed in the
dictionary, not derived. Ruthenian's ablative gives several prepositions a more
transparent government than Russian's: `iz doma` "out of the house" takes the
ablative, not the genitive.

**Conjunctions** (`i`, `a`, `no`, `czto`, `jesli`), **particles** (`zzje`, `li`,
`by`, `nje`) and **interjections** are indeclinable and lexical.

**Negation takes the genitive.** `nje vizzu domogo` "I do not see the house" —
the genitive of negation, which OCS has and which Polish still requires
obligatorily. It is pan-Slavic, not a Russian complication, so Ruthenian keeps
it.

---

# 9. Summary of paradigm sizes

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

# 10. The lexicon: sources and borrowing

## 10.1 Why the lexicon is multi-source

Ruthenian's vocabulary is drawn from all of East Slavic, plus Polish and Old
Church Slavonic, plus a rule-governed borrowing system. This is not eclecticism:
**Russian alone cannot supply the language specified above.**

| What the spec needs | Russian | Recoverable from |
|---|---|---|
| yat (`-i` endings, `chlib`) | merged into `e` | Ukrainian `i`, Polish `ie`/`ia`, OCS `ě` |
| second palatalization | levelled to **0 %** | Ukrainian **99 %**, OCS 66 % |
| the vocative | 40 relic forms | Ukrainian 25 180, OCS 6 186 |
| the dual | lost | OCS 77 714 forms |
| nasal vowels (etymology) | lost | **Polish only** (`ą`, `ę`) |

Measured lemma inventories, each from a full scan:

| Language | Single-word lemmas |
|---|---:|
| Russian | 419 283 |
| Polish | 152 325 |
| Ukrainian | 52 223 |
| Belarusian | 6 899 |
| Old Church Slavonic | 4 311 |

## 10.2 How a Ruthenian word is chosen

A Ruthenian lemma is not "the Russian word transliterated". It is the reflex the
**Proto-Slavic etymon** would have in Ruthenian's phonology, with the attested
cognates as evidence.

1. **Group by etymon.** Cognates across the source languages that continue the
   same Proto-Slavic form make one entry.
2. **Reconstruct the Ruthenian form** by regular sound correspondence from the
   etymon, using the cognates to resolve what any single language lost.
3. **Record the evidence** — which languages attest it, and how confidently the
   reconstruction follows.

**An honest limit, measured.** Explicit Proto-Slavic etymology links in the
source are thin: 5 517 distinct etyma, of which only 88 have reflexes tagged in
all five languages and 2 700 in just one. Cognate grouping therefore cannot rely
on etymology templates alone; it will need phonological matching and the English
gloss as a pivot, as slovowiki does. **This is the hardest unsolved problem in
the lexicon** and it should be scoped as its own phase, not assumed away.

Where reconstruction is uncertain, the entry records that. A form derived from
one language's reflex is not the same claim as one attested across four, and
`Provenance` must distinguish them (`docs/specs/ruthenian-lexicon.md`).

## 10.3 Borrowing: the regularized system

International vocabulary is adapted by rule rather than borrowed ad hoc, so that
`nacija` is predictable from *natio* rather than memorized from Russian.

### Latin and Greek — the learned layer

| Source ending | Ruthenian | Gender / declension | Example |
|---|---|---|---|
| `-tiō, -tiōnem` | `-cija` | fem., I | *natio* → `nacija` |
| `-tās, -tātem` | `-tetj` | fem., III | *universitas* → `univjersitetj` |
| `-or, -ōrem` | `-or` | masc., II | *doctor* → `doktor` |
| `-us` (2nd decl.) | `-∅` | masc., II | *circus* → `cirk` |
| `-um` (2nd decl.) | `-o` | neut., II | *museum* → `muzjeo` |
| `-a` (1st decl.) | `-a` | fem., I | *forma* → `forma` |
| Gk `-ισμός` | `-izm` | masc., II | *organismos* → `organizm` |
| Gk `-ία` | `-ija` | fem., I | *philosophia* → `filosofija` |
| Gk `-της` | `-t` | masc., II | *poiētēs* → `pojet` |

The Latin **oblique** stem is the base, not the nominative — *natio* borrows as
`nacij-` from *natiōn-*, exactly as Slavic has always done it. Once adapted, a
loan declines as a native word of its class: `nacija`, `nacijy`, `naciji`,
`nacijej`.

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

## 10.4 The layers are ordered

When two sources could supply a word, the earlier layer wins:

1. inherited Slavic (attested in two or more source languages)
2. inherited Slavic (attested in one)
3. Old Church Slavonic, for learned and abstract vocabulary
4. a native coinage from Ruthenian roots
5. a Graeco-Latin borrowing
6. a modern borrowing

This makes Ruthenian **purist by default but not dogmatic**: a native word is
preferred where one exists, and an international word is adopted where it does
not, by rule rather than by taste.

---

# 11. Open questions

Settled in this revision: the copula (full, with dual), aspect (rule-derived),
the masculine dative (`-u`), negation (genitive), the alphabet (pure ASCII, no
yat letter), the aorist and imperfect (core, not register), and the long/short
adjective (restored, carrying definiteness).

Still open:

1. **The etymological alphabet.** A diacritic notation for dictionaries — ASCII
   plus diacritics, on the model of Interslavic's — distinguishing yat, the
   nasals, the jers and stress. Deferred until the lexicon can supply the
   etymological information (§10.2). The standard orthography stays pure ASCII
   regardless.
2. **Ablative in the plural.** No attested language distinguishes it; Ruthenian
   follows. A maximally conservative variant could revive PIE `*-ios`.
3. **Clitic pronouns.** OCS, Sanskrit and Interslavic all have a full/clitic
   opposition; Ruthenian does not. Restoring it is coherent and would fit the
   "maximum grammar" brief.
4. **The middle voice.** Lost in all Slavic, its work done by `-sja`. Restoring
   it would be the most radical available conservatism.
5. **Cognate grouping.** §10.2 — the unsolved lexicon problem. Explicit
   Proto-Slavic links cover 5 517 etyma with only 88 attested across all five
   source languages. Needs phonological matching and gloss pivoting; its own
   phase.
6. **Serbo-Croatian and Czech as further sources.** Both preserve features the
   chosen five do not — pitch accent and a productive aorist in Serbo-Croatian,
   vowel length in Czech. Polish already crossed the East Slavic line, so the
   boundary is one of degree.
7. **The supine.** OCS had it alongside the infinitive, for purpose after verbs
   of motion (`idǫ loviti` vs `idǫ lovitъ`). Not yet specified; it would fit the
   brief.

## Still to write

Not questions — work outstanding on this document:

- **derivation** (§10.4 layer 4 assumes it): agent, abstract, diminutive and
  verbal-noun suffixes, the prefix inventory, compounding;
- **numerals in full**: 11–20, tens, hundreds, the declension of `dva`/`tri`/
  `czjetyrje`, collectives, fractions;
- **pronouns in full**: possessives, the declined demonstratives and
  interrogatives, the negative and indefinite series;
- **a preposition/case government table**, now that there are eight cases;
- **syntax**: word order, agreement, questions, subordination.
