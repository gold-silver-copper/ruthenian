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
| stress | fixed, lexical | 10 patterns | mobile | mobile | pitch |
| adjective long/short | long only | both | both | both | n/a |
| 2nd palatalization | **kept** | lost (0 %) | kept (99 %) | kept (66 %) | n/a |
| yat distinction | **kept** (as `-i`) | lost | kept (as `i`) | kept (as `ě`) | n/a |

**Restored:** ablative, vocative, dual, dual pronouns, dual verb agreement.
**Removed:** mobile stress, heteroclitics, marginal cases, short adjectives, soft
adjective stems, indeclinables, fleeting vowels in the genitive plural,
irregular numeral government, verb classes 7–16.
**Kept:** aspect, animacy, the *n-* prefix, reflexives, iotation, participles,
**both palatalizations** (Ukrainian 99 %, Russian 0 %), and the yat distinction
via the Ukrainian reflex `-i`.

---

# 2. Nouns

## 2.1 The eight cases

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

## 2.2 The three declensions

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

## 2.3 Declension II — masculine

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

## 2.4 Declension II — neuter

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

## 2.5 Declension I — feminine in `-a`

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

## 2.6 Declension III — feminine in a consonant

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

## 2.7 Animacy

Masculine and plural **animate** nouns take the genitive form in the accusative;
inanimates take the nominative. Inherited, pan-Slavic, and information-bearing —
Ruthenian keeps it unchanged.

```
vizzu dom       I see the house    (inanimate: acc = nom)
vizzu konja     I see the horse    (animate:   acc = gen)
```

## 2.8 Automatic spelling adjustments

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

## 2.9 What was removed, and why

| Removed | Was | Justification |
|---|---|---|
| mobile stress | 6 patterns + 4 primed | stress is fixed and lexical; Interslavic marks none at all |
| fleeting vowel in gen. pl. | `okno` → `okon` | genitive plural is uniformly `-ov` |
| heteroclitics | `vremja/vremeni` | ~15 lemmas memorized individually in every Slavic language |
| partitive, 2nd locative, count form | 436 Russian lemmas | the ablative does their semantic work |
| indeclinables | 1 193 Russian nouns | regularized **by addition**: `metro`, `metrogo`, `metru` |
| plural-/singular-only defectiveness | 2 003 nouns | every noun has all three numbers |

---

# 3. Adjectives

Long form only — the short form is dropped. It is a Slavic innovation absent from
PIE and Sanskrit, lexically unpredictable in Russian (4 571 of 9 999 adjectives
have one), and absent from Interslavic. Soft adjective stems are also dropped:
they are 1.6 % of Russian adjectives carrying an entire parallel ending set.

Adjectives agree in gender, number and case — including the ablative. They have
**no vocative**: the nominative is used, as in every language measured.

### `dobryj` "good"

| Case | Masc sg | Neut sg | Fem sg | Dual (all) | Plural (all) |
|---|---|---|---|---|---|
| nominative | `dobryj` | `dobroje` | `dobraja` | `dobraja` | `dobryje` |
| accusative | `dobryj` / `dobrogo` | `dobroje` | `dobruju` | `dobraja` | `dobryje` / `dobryh` |
| genitive | `dobrogo` | `dobrogo` | `dobroj` | `dobru` | `dobryh` |
| **ablative** | `dobra` | `dobra` | `dobroj` | = dat | = dat |
| dative | `dobromu` | `dobromu` | `dobroj` | `dobroma` | `dobrym` |
| instrumental | `dobrym` | `dobrym` | `dobroj` | `dobroma` | `dobrymi` |
| locative | `dobrom` | `dobrom` | `dobroj` | `dobru` | `dobryh` |

The adjectival `-ogo` here is the *same ending* the noun now uses for the
genitive — the two declensions are reunited, which is what they were in PIE.

## 3.1 Degrees

Regular, with no suppletion:

| Degree | Formation | Example |
|---|---|---|
| positive | — | `dobryj` |
| comparative | `-jejszij` | `dobrjejszij` |
| superlative | `naj-` + comparative | `najdobrjejszij` |

`naj-` follows Ukrainian, Belarusian and Interslavic against Russian's analytic
`самый`.

---

# 4. Pronouns

## 4.1 Personal

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

## 4.2 Reflexive

`sjebja` (acc/abl), `sjebjego` (gen), `sjebje` (dat/loc), `soboj` (ins). No
nominative — the cell does not exist. The bound reflexive verb suffix is `-sja`.

## 4.3 Demonstrative and interrogative

`toj` / `ta` / `to` "this, that"; `kto` "who", `czto` "what", declining
pronominally: `kogo`, `komu`, `kjem`, `kom`.

---

# 5. Numerals

## 5.1 Government — the payoff of the dual

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

## 5.2 Cardinals

`odin`, `dva`, `tri`, `czjetyrje`, `pjatj`, `sjestj`, `sjedjem`, `vosjem`,
`djevjatj`, `djesjatj`, `sto`, `tysjacza`.

`odin` declines as an adjective and agrees. `dva` has dual forms (`dva` /
`dvuh` / `dvuma`). `tri` and `czjetyrje` decline. From `pjatj` upward the
numerals decline as declension III nouns — inherited behaviour, kept.

## 5.3 Ordinals

`pjervyj`, `vtoryj`, `trjetij`, `czjetvjortyj`, `pjatyj` … declining as
adjectives.

---

# 6. Verbs

## 6.1 Categories

| Category | Values |
|---|---|
| aspect | imperfective, perfective |
| tense | present, past, future |
| mood | indicative, imperative, conditional |
| voice | active, passive |
| person | 1, 2, 3 |
| number | singular, **dual**, plural |

**Aspect is kept unchanged.** It is the defining Slavic innovation, pervasive in
all four East Slavic lects, and removing it would produce a different language
rather than a standardized one.

The aorist, imperfect and supine are **available as an optional archaic
register**, on Interslavic's model — fully specified, never required. Their forms
are the OCS ones (`-běxъ`, `-ěaxъ`).

## 6.2 Conjugation

Two conjugations, six classes — Zaliznyak's 1–6, which cover 90.7 % of Russian
verbs; classes 7–16 are regularized onto them.

| Class | Infinitive | Present stem | Example |
|---|---|---|---|
| 1 | `-atj` | stem + `j` | `czitatj` → `czitaj-` |
| 2 | `-ovatj` | `ov` → `uj` | `nyegodovatj` → `nyegoduj-` |
| 3 | `-nutj` | theme drops | `dvinutj` → `dvin-` |
| 4 | `-itj` | theme drops, 1sg mutates | `govoritj` → `govor-` |
| 5 | `-jetj` | theme drops | `vidjetj` → `vid-` |
| 6 | `-atj` | theme drops, stem mutates | `pisatj` → `pisz-` |

Classes 1, 2, 3 and 6 take first-conjugation endings; 4 and 5 take the second.

## 6.3 Present endings

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

The dual endings are the OCS ones (`-evě`, `-eta`, `-ete`), cognate with Sanskrit
`-vaḥ`, `-thaḥ`, `-taḥ` — the same three-way dual as in the noun.

`czitatj` "to read": `czitaju`, `czitajeszj`, `czitajet`; dual `czitajevje`,
`czitajeta`, `czitajetje`; plural `czitajem`, `czitajetje`, `czitajut`.

## 6.4 Past

The `l`-participle, agreeing in gender and number:

| | Masculine | Feminine | Neuter | Dual | Plural |
|---|---|---|---|---|---|
| | `czital` | `czitala` | `czitalo` | `czitala` | `czitali` |

## 6.5 Future

Perfective verbs use the present endings with future meaning. Imperfective verbs
use the auxiliary `budu` + infinitive: `budu czitatj`, dual `budjevje czitatj`,
plural `budjem czitatj`.

## 6.6 Imperative and conditional

Imperative: present stem + `-i` (or bare stem after `j`), plus `-tje` in the
plural and `-ta` in the dual — `czitaj`, `czitajta`, `czitajtje`.

Conditional: `l`-participle + the invariant particle `by`.

## 6.7 Present-stem mutation

Kept — removing it would make the verb unrecognizable. Applied **by class**,
never by stem shape: a class-1 verb with a labial-final stem takes no mutation at
all, verified across 1 977 Russian verbs without exception.

| | | | |
|---|---|---|---|
| `ov` → `u` | `s` → `sz` | `t` → `cz` | `d` → `zz` |
| `z` → `zz` | `st` → `szcz` | `k` → `cz` | `h` → `sz` |
| `p` → `plj` | `b` → `blj` | `v` → `vlj` | `m` → `mlj` |

## 6.8 Participles and gerunds

| | Suffix | Example |
|---|---|---|
| present active | `-uszczij` / `-jaszczij` | `czitajuszczij` |
| past active | `-vszij` | `czitavszij` |
| present passive | `-jemyj` / `-imyj` | `czitajemyj` |
| past passive | `-nnyj` / `-jonnyj` / `-tyj` | `proczitannyj` |
| present gerund | `-ja` | `czitaja` |
| past gerund | `-v` | `czitav` |

Participles decline as adjectives. Their **existence** is structural, not
lexical: a perfective verb has no present participles or present gerund; an
intransitive verb has no passive participle. These are absences of the category,
not gaps in the lexicon.

---

# 7. The closed classes

**Adverbs** are formed from adjectives in `-o`: `dobro` "well". Comparatives
follow the adjective: `dobrjejje`.

**Prepositions** govern case, and which case is lexical — it is listed in the
dictionary, not derived. Ruthenian's ablative gives several prepositions a more
transparent government than Russian's: `iz doma` "out of the house" takes the
ablative, not the genitive.

**Conjunctions** (`i`, `a`, `no`, `czto`, `jesli`), **particles** (`zzje`, `li`,
`by`, `nje`) and **interjections** are indeclinable and lexical.

---

# 8. Summary of paradigm sizes

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

# 9. The lexicon: sources and borrowing

## 9.1 Why the lexicon is multi-source

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

## 9.2 How a Ruthenian word is chosen

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

## 9.3 Borrowing: the regularized system

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

## 9.4 The layers are ordered

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

# 10. Open questions

Recorded rather than decided.

1. **Yat as a written letter.** Ruthenian already carries the yat *distinction*
   through the Ukrainian reflex `-i` (§2.1) and the second palatalization, so
   nothing is lost in the grammar. Writing `ě` as a letter would additionally
   restore it **lexically** — `hlib` would be spelled `chlěb` — and that is the
   maximally conservative option.

   It is deferred, and the reason is lexicographic rather than orthographic. Yat
   is not one ending but a phoneme in roughly **15 % of the vocabulary** (700 of
   4 505 OCS lemmas). Writing it requires knowing where it stood in every word,
   and the Russian source has almost none: 360 pre-reform headwords in 441 629
   records, mostly proper nouns like `Dnjeprъ`. It *is* recoverable — Ukrainian
   `i` and OCS `ě` both encode it and both are in the dump — but that means
   admitting a second language as a lexical source, which is a scope decision
   about the lexicon (`DIRECTION.md`), not about the alphabet.
2. **Ablative in the plural.** No attested language distinguishes it. Ruthenian
   follows, but a maximally conservative variant could revive PIE `*-ios`.
3. **Clitic pronouns.** OCS, Sanskrit and Interslavic all have a full/clitic
   opposition; Ruthenian currently does not. Restoring it is coherent.
4. **The middle voice.** Lost in all Slavic, its work done by `-sja`. Restoring
   it would be the most radical available conservatism.
5. **Aorist and imperfect as default rather than optional register.**
6. **Cognate grouping.** §9.2 — the unsolved problem. Explicit Proto-Slavic
   links cover 5 517 etyma with only 88 attested across all five languages, so
   grouping needs phonological matching and gloss pivoting. Its own phase.
7. **Serbo-Croatian and Czech as further sources.** Both preserve features the
   chosen five do not — Serbo-Croatian has pitch accent and a productive aorist,
   Czech has vowel length. Neither is East Slavic, so both are excluded for now,
   but Polish is already a West Slavic admission and the line is one of degree.
