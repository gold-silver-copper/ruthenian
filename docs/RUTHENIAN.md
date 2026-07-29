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
| past tenses | **2**, both periphrastic | 1 | 1 | 3 | 4 |
| copula | **full, with dual** | invariant, omitted | full | full | full |
| clitic pronouns | **kept** | lost | relics | full | full |
| aspect | lexical + derived | lexical | lexical | emerging | n/a |
| 2nd palatalization | **kept** | lost (0 %) | kept (99 %) | kept (66 %) | n/a |
| yat distinction | **kept** (as `-i`) | lost | kept (as `i`) | kept (as `ě`) | n/a |

**Restored:** the ablative, the vocative, the dual, dual pronouns, dual verb
agreement, the OCS long/short adjective, the full
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
| ж | `zz` | р | `r` | э | `e` |
| з | `z` | с | `s` | ю | `ju` |
| и | `i` | т | `t` | я | `ja` |
| й | `j` | у | `u` | | |
| | | ф | `f` | | |
| | | х | `h` | | |

Thirty-two letters, not thirty-three: **`ё` is not one of them** (§2.3). It is
stressed `е`, and Ruthenian writes `je` for both.

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
`j`-digraphs — **`ja je ji jo ju jy`**, one for each vowel of §2.3 — and
word-finally or before a consonant with `j` alone (`konj`).

Two of the six carry no sound of their own. `ji` is redundant, since `i` implies
a soft consonant before it anyway; `jy` is worse than redundant, since Slavic has
no palatalized consonant before a back vowel and `jy` is therefore pronounced
exactly as `ji` is. Both are written all the same, because §3.2 builds the soft
endings as `j` + the hard ones and a digraph that vanished in two cells would put
the genitive `zjemljy` and the dative `zjemlji` back together. The orthography
is morphophonemic: it spells the ending and leaves the merger to the reader, as
§2.5 does for final devoicing and §3.9 for invariant stems.

**The five exceptions are exceptions for two different reasons.** `j` is itself
the palatal; `cz` and `szcz` are inherently palatal, so there is no hard value to
contrast with. `zz` and `sz` are the opposite case: they are inherently **hard**,
as `ж` and `ш` are in Russian, and have no soft value. Either way the `j` has
nothing to mark, which is why none of the five ever takes one (§3.8 rule 2a).

## 2.3 Vowels

`a e i o u y` — six, with `y` the back counterpart of `i`. The iotated series
`ja je ju` are `j` + vowel, not separate phonemes.

**There is no `jo`.** Russian's `ё` is not a vowel of its own: it is stressed
`е` after the East Slavic `*e > o` shift, so `нёс` and `несу` are one root
differing only in where the accent falls. §2.1 does not write stress, so
spelling the shift would encode an alternation the language cannot see, and the
stem would stop being invariant (§2.5) — the same objection that removed the
fleeting vowel and the stress clause of §3.8's rule 2. Ruthenian writes `je`
throughout: `zzeltyj`, `njes` beside `njesu`, `czetvjertyj`.

Russian's own orthography prints `е` for `ё` outside dictionaries, so this is
also what a Russian text mostly looks like already. The sequence `jo` is
therefore free to be `j` + `o`, which is what `bataljon` and the soft vocative
`-jo` (§3.5's `zjemljo`) need it to be.

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
| `svjetcza`, `gorozzanin` | `osvjeszczenije`, `grazzdanin` |
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

Each has a **hard** and a **soft** variant, and the soft one is not a second set
of endings. It is **`j` + the hard ending, with `o` written `e` after the `j`**:

```
""      -> j        ogo -> jego     oj -> jej      y -> jy
a       -> ja       om  -> jem      i  -> ji       ov -> jev
u       -> ju       oma -> jema     o  -> je       am -> jam
```

One operation, applied to every cell of every declension that has a soft
variant. The vocative singular is the only exception, and §3.1 states it: `konju`
and `zjemljo`, neither of which the rule reaches.

The `o`-to-`e` clause is not an extra rule but §2.2's: `jo` is written `je`
except where a following hard consonant demands otherwise, which no ending has.
And note that the rule leaves `-jy` alone. `jy` and `ji` are pronounced alike —
Slavic has no palatalized consonant before a back vowel, which is precisely why
Russian writes `и` after a soft one — so the genitive `zjemljy` and the dative
`zjemlji` differ on paper and not in speech. **That is intended.** Ruthenian
spells the morphology and lets the reader apply the mergers, the same choice
§2.5 makes for final devoicing and §3.9 for invariant stems; a soft declension
that wrote what was said would lose a case distinction the hard one keeps.

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

<!-- render:noun-dom -->
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

¹ animate nouns take this form in the accusative (§3.7).
<!-- /render:noun-dom -->

### Soft: `konj` "horse" (stem `kon-`)

<!-- render:noun-konj -->
| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `konj` | `konja` | `konjy` |
| vocative | `konju` | = nom | = nom |
| accusative | `konja` | = nom | `konjev` |
| genitive | `konjego` | `konju` | `konjev` |
| **ablative** | `konja` | = dat | = dat |
| dative | `konju` | `konjema` | `konjem` |
| instrumental | `konjem` | `konjema` | `konjami` |
| locative | `konji` | `konju` | `konjah` |
<!-- /render:noun-konj -->

### Velar: `drug` "friend" (animate, stem `drug-`)

<!-- render:noun-drug -->
| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `drug` | `druga` | `drugi` |
| vocative | `druzze` | = nom | = nom |
| accusative | `druga` | = nom | `drugov` |
| genitive | `drugogo` | `drugu` | `drugov` |
| **ablative** | `druga` | = dat | = dat |
| dative | `drugu` | `drugoma` | `drugom` |
| instrumental | `drugom` | `drugoma` | `drugami` |
| locative | `druzi` | `drugu` | `drugah` |
<!-- /render:noun-drug -->

The nominative plural `drugi` is rule 1's spelling of `-y`; the vocative
`druzze` is the **first** palatalization and the locative `druzi` the
**second** — the two cells differ in both the consonant and the vowel (§3.8).

**Note on syncretism.** `doma` is both ablative singular and nominative dual.
This is inherited, not a defect: OCS has exactly the same collision (genitive
singular `-a` = nominative dual `-a`), because both continue different PIE
endings that fell together regularly. It is disambiguated by agreement — a dual
noun takes dual modifiers and a dual verb.

## 3.4 Declension II — neuter

### Hard: `okno` "window" (stem `okn-`)

<!-- render:noun-okno -->
| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `okno` | `okni` | `okna` |
| vocative | = nom | = nom | = nom |
| accusative | = nom | = nom | `okna` / `oknov` ¹ |
| genitive | `oknogo` | `oknu` | `oknov` |
| **ablative** | `okna` | = dat | = dat |
| dative | `oknu` | `oknoma` | `oknom` |
| instrumental | `oknom` | `oknoma` | `oknami` |
| locative | `okni` | `oknu` | `oknah` |

¹ animate nouns take this form in the accusative (§3.7).
<!-- /render:noun-okno -->

### Soft: `polje` "field" (stem `pol-`)

Nominative `polje`, genitive `poljego`, ablative `polja`, dative `polju`,
instrumental `poljem`, locative `polji`; dual `polji` / `polju` / `poljema`;
plural `polja` / `poljev` / `poljem` / `poljami` / `poljah`.

The locative and the dual are `polji`, not `polje`, and this is where §3.2's
rule earns its keep. Both are the yat `-i`, so both soften to `-ji`; had they
softened to `-je` they would have merged with the nominative, and `polje` would
have covered seven cells — nominative, vocative and accusative singular, all
three dual cells, and the locative singular. The hard neuter keeps `okno` apart
from `okni`, and the soft one now keeps `polje` apart from `polji`.

The neuter dual `-i` continues OCS `-ě` (`dvě selě`), and the neuter vocative is
the nominative in every language measured — Sanskrit 84 % ∅, OCS 45 %.

## 3.5 Declension I — nouns in `-a`

### Hard: `zzena` "woman" (stem `zzen-`)

<!-- render:noun-zzena -->
| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `zzena` | `zzeni` | `zzeny` |
| vocative | `zzeno` | = nom | = nom |
| accusative | `zzenu` | = nom | `zzeny` / `zzenov` ¹ |
| genitive | `zzeny` | `zzenu` | `zzenov` |
| **ablative** | `zzeny` | = dat | = dat |
| dative | `zzeni` | `zzenama` | `zzenam` |
| instrumental | `zzenoj` | `zzenama` | `zzenami` |
| locative | `zzeni` | `zzenu` | `zzenah` |

¹ animate nouns take this form in the accusative (§3.7).
<!-- /render:noun-zzena -->

The ablative singular **is the genitive form**, as in PIE and Sanskrit
(99 %). The dative and locative coincide: both continue OCS `-ě`, and they
are identical in OCS, Russian and Ukrainian alike (Ukrainian `-i` 55 % in
both cells) — keeping them apart would be an innovation, not a conservatism.

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
`zjemljy`, dative/locative `zjemlji`, instrumental `zjemljej`; dual
`zjemlji` / `zjemlju` / `zjemljama`; plural `zjemljy` / `zjemljev` / `zjemljam` /
`zjemljami` / `zjemljah`.

### Vowel-final stems: `nacija` "nation" (stem `naci-`)

A stem may end in a vowel, and nothing about the declension changes. §3.8's
rule 3 puts the soft sign in the ending rather than the stem, so `nacija` is
`naci-` plus the same endings `zjemlja` takes:

Nominative `nacija`, vocative `nacijo`, accusative `naciju`, genitive/ablative
`nacijy`, dative/locative `naciji`, instrumental `nacijej`; dual
`naciji` / `naciju` / `nacijama`; plural `nacijy` / `nacijev` / `nacijam` /
`nacijami` / `nacijah`.

**The soft declension is exactly parallel to the hard one, cell for cell.**
That is the test §3.2's rule has to pass, and it does:

<!-- render:decl-i-parallel -->
| | genitive sg | dative/locative sg | dual | nominative pl |
|---|---|---|---|---|
| hard `zzena` | `zzeny` | `zzeni` | `zzeni` | `zzeny` |
| soft `zjemlja` | `zjemljy` | `zjemlji` | `zjemlji` | `zjemljy` |
| soft `nacija` | `nacijy` | `naciji` | `naciji` | `nacijy` |
<!-- /render:decl-i-parallel -->

The dative and locative coincide as §3.5's table note requires, the genitive stands
apart from them, and the plural groups with the genitive — the same four-cell
arrangement in both series, because the soft endings *are* the hard ones with a
`j`. No cell had to be chosen; every one falls out.

Earlier revisions decided these cells one at a time and kept trading one
collision for another. Giving the dative `-ji` and the locative `-i` broke the
syncretism this section exists to justify; making the yat ending `-je` after a
soft stem repaired that, but then `polje` swallowed its own locative and dual,
and the softened `-y` ended up written `-i` in this section and `-ji` in §3.3
without either being wrong on its own terms. A single derivation settles all of
them at once, which is the argument for having one.

**Ruthenian and Russian part company here, in favour of Old Church Slavonic.**
Russian has `земле`, `коне`, `поле` where Ruthenian has `zjemlji`, `konji`,
`polji`. OCS has `земли`, `кони`, `поли` — the soft stems took the *jo*-stem
locative `-i`, not the *o*-stem yat — so the older language is on Ruthenian's
side, and §1 takes grammar from OCS and phonology from Russian. This is grammar.

**The soft vocatives are stated, not derived, and that is now a choice rather
than a repair.** §3.2's rule would give `zjemlje` from the hard `-o` and `konje`
from the hard `-je`, and — since the dative and locative moved to `-ji` — neither
would collide with anything. The exception survives on its own merits: `-jo`
keeps the hard vocative `-o` visible where `-je` would hide it, and `konju` is
OCS's `коню` directly. §3.1 gives both endings, and they stay given.

This is worth flagging because the earlier justification is gone. While the
dative was `zjemlje`, the vocative *had* to avoid `-je` or lose a category §3.1
exists to restore; that pressure no longer exists, and the two endings are kept
because they are the better forms, not because the alternative is unavailable.

**The instrumental `-jej` is not an exception.** It is `oj` with the `o` written
`e` after the `j`, exactly as `ogo` gives `jego`. Russian's `землёй` is stressed
`землей` and §2.3 does not spell the shift, so the two agree without a rule of
their own.

**`nacija` needs nothing beyond a vowel-final stem.** §3.8's rule 3 puts the
soft sign in the ending, so `naci-` takes the same endings `zjeml-` does and the
sequences `ija`, `ijy`, `iji` simply occur. Russian gives this class a
sub-pattern of its own (`нация`, `нации`, `нацией`); Ruthenian does not, because
that would be a fourth declension to learn for a class the third already
handles. The class is not marginal — §12.3's `-cija` and `-ija` borrowings are
large and productive — which is the reason to state it here rather than leave it
to be inferred at each loan.

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

<!-- render:noun-noczj -->
| Case | Singular | Dual | Plural |
|---|---|---|---|
| nominative | `noczj` | `noczi` | `noczi` |
| vocative | `noczi` | = nom | = nom |
| accusative | = nom | = nom | `noczi` / `noczev` ¹ |
| genitive | `noczi` | `noczu` | `noczev` |
| **ablative** | `noczi` | = dat | = dat |
| dative | `noczi` | `noczjma` | `noczam` |
| instrumental | `noczjju` | `noczjma` | `noczami` |
| locative | `noczi` | `noczu` | `noczah` |

¹ animate nouns take this form in the accusative (§3.7).
<!-- /render:noun-noczj -->

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
nominative. Inherited, pan-Slavic, information-bearing, and kept at its
inherited scope, which differs by number. In the **plural** it reaches every
animate noun of every gender. In the **singular** it belongs to declension II
masculine alone: declension I keeps its own accusative (`vizzu slugu`, §3.5),
declension III patterns with the other feminines (`vizzu myszj`, as Russian
`вижу мышь`), and the neuter keeps its nominative (`vizzu czudoviszcze`) — no
Slavic language says otherwise, and an adjective has one plural column and one
neuter singular, so a noun that deviated could not be agreed with.

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

1. after `k g h` and `zz sz cz szcz`, `y` is written `i` (`knigi`, not `*knigy`)
   — applied **after** rule 2, since rule 2 can expose the `y` it governs;
2. **`j` is never written after `cz szcz zz sz`** — §2.2 gives none of the four
   a hard/soft contrast, so the glide has nothing to mark. This is a fact about
   the alphabet rather than about endings, and it holds inside a root as much as
   at a seam: `zzena`, `czelovjek`, `szestj`, and `otjecz` + `-je` → `otjecze`,
   `pisz` + `-jesz` → `piszesz`;
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
covers the vocative (`otjecze`, `druzze`), the present endings (`piszesz`,
§7.3), and the `-jem`/`-jego` series, which after these stems is simply `-om`
and `-ogo` — there was never a `j` to drop, because rule 2 no longer puts one
there.

Its one bound is that it applies to an ending's **initial** `j` and not to a `j`
anywhere in an ending: everywhere else a leading `j` is rule 3's soft sign rather
than a glide, so §3.6's instrumental `-jju` keeps both, and a wider rule makes
`noczjju` into `noczju`.

**Rule 2 runs before rule 1, and the order is load-bearing.** §3.2's soft series
contains `-jy`, so a soft hushing stem meets rule 1 only after rule 2 has taken
the glide off: `nozzj` + `-jy` → `nozz` + `-y` → `nozzi`, which is the plural the
hard `nozz` has. Run the other way round, rule 1 never sees the `y` and the form
stays `nozzy` — a spelling rule 1 forbids outright. No other pair of these rules
interacts, and this pair did not either until the soft endings gained a `y`.

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
on jest dobr        he is good            (indefinite predication)
on jest dobryj      he is the good one    (definite, identifying)
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

<!-- render:adj-short -->
| Case | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `dobr` | `dobro` | `dobra` | `dobra` | `dobry` |
| vocative | `dobrje` | = nom | `dobro` | = nom | = nom |
| accusative | `dobr` / `dobra` ¹ | = nom | `dobru` | = nom | `dobry` / `dobrov` ¹ |
| genitive | `dobrogo` | `dobrogo` | `dobry` | `dobru` | `dobrov` |
| **ablative** | `dobra` | `dobra` | `dobry` | = dat | = dat |
| dative | `dobru` | `dobru` | `dobri` | `dobroma` | `dobrom` |
| instrumental | `dobrom` | `dobrom` | `dobroj` | `dobroma` | `dobrami` |
| locative | `dobri` | `dobri` | `dobri` | `dobru` | `dobrah` |

¹ animate (§3.7).
<!-- /render:adj-short -->

## 4.2 Long (definite) — pronominal declension

`dobryj`. Endings are the pronoun `toj`'s.

<!-- render:adj-long -->
| Case | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `dobryj` | `dobroje` | `dobraja` | `dobraja` | `dobryje` |
| accusative | `dobryj` / `dobra` ¹ | = nom | `dobruju` | = nom | `dobryje` / `dobryh` ¹ |
| genitive | `dobrogo` | `dobrogo` | `dobroj` | `dobroju` | `dobryh` |
| **ablative** | `dobra` | `dobra` | `dobroj` | = dat | = dat |
| dative | `dobromu` | `dobromu` | `dobroj` | `dobryma` | `dobrym` |
| instrumental | `dobrym` | `dobrym` | `dobroj` | `dobryma` | `dobrymi` |
| locative | `dobrom` | `dobrom` | `dobroj` | `dobroju` | `dobryh` |

¹ animate (§3.7).
<!-- /render:adj-long -->

Long adjectives have **no vocative** — the nominative is used, as in every
language measured.

**The Dual and Plural columns above are the masculine.** Both tables give one
column where the declension needs three, and §4.1's rule decides the rest: its
endings are the noun's, so the feminine dual is `dobrama` against the masculine
`dobroma` (§3.5's `zzenama` against §3.3's `domoma`), and the neuter plural
nominative is `dobra` against `dobry`. The long declension genuinely has one dual
and one plural for all genders, as `toj` does (§5.4).

**"The pronoun `toj`'s" means the declension *type*, not the same endings.**
Thirteen of the seventeen differ, because the long adjective is the contracted
`short + jь` form: `dobryj` against `toj`, `dobryje` against `ti`, `dobrym`
against `tjem`. What the two share is every ending that begins in `o` — the
genitive `-ogo`, the dative `-omu`, the locative `-om`, the feminine oblique
`-oj` and the dual `-oju`. The dual was `-u` in an earlier revision, the one
`o`-initial cell where the tables disagreed, and it was the nominal dual `domu`
borrowed by mistake.

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

**On a velar stem the suffix loses its own glide.** `dorog` palatalizes to
`dorozz`, and `zz` is hard (§2.2), so §3.8's rule 2 removes the `j` of `-jejsz-`:

```
dobr   → dobrjejsz     dobrjejszij      (rule 1: y is written i after sz)
dorog  → dorozzejsz    dorozzejszij     (first palatalization, then rule 2)
tih    → tiszejsz      tiszejszij
```

The long form is `dobrjejszij` and not `*dobrjejszyj` for the same reason at the
other seam — §3.8's first rule writes `y` as `i` after `sz`.

Comparatives and superlatives exist in both long and short forms, and the
comparative governs the **ablative** for the standard of comparison:
`dobrjejszij brata` "better than the brother" — the inherited ablative of
comparison, which Russian expresses with the genitive and Sanskrit with the
ablative proper.

# 5. Pronouns

Pronouns decline **pronominally** — the declension the long adjective borrows
(§4.2). All have dual forms.

## 5.1 Personal

<!-- render:pron-personal -->
| | 1sg | 2sg | **1du** | **2du** | 1pl | 2pl |
|---|---|---|---|---|---|---|
| nominative | `ja` | `ty` | `vje` | `va` | `my` | `vy` |
| accusative | `mjenja` | `tjebja` | `na` | `va` | `nas` | `vas` |
| genitive | `mjenjego` | `tjebjego` | `naju` | `vaju` | `nas` | `vas` |
| **ablative** | `mjenja` | `tjebja` | = dat | = dat | = dat | = dat |
| dative | `mnje` | `tjebje` | `nama` | `vama` | `nam` | `vam` |
| instrumental | `mnoj` | `toboj` | `nama` | `vama` | `nami` | `vami` |
| locative | `mnje` | `tjebje` | `naju` | `vaju` | `nas` | `vas` |
<!-- /render:pron-personal -->

`vje` "we two" and `va` "you two" are the OCS duals, restored.

### 5.1a The clitic series

Every personal pronoun has a **short, unstressed** form beside the full one, as
in OCS and Sanskrit. Russian lost this opposition entirely; OCS, Czech, Polish
and Interslavic keep it.

<!-- render:pron-clitic -->
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
<!-- /render:pron-clitic -->

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

<!-- render:pron-third -->
| | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `on` | `ono` | `ona` | `ona` | `oni` |
| accusative | `jego` | `jego` | `ju` | `ja` | `jih` |
| genitive | `jego` | `jego` | `jeje` | `jeju` | `jih` |
| **ablative** | `jego` | `jego` | `jeje` | = dat | = dat |
| dative | `jemu` | `jemu` | `jej` | `jima` | `jim` |
| instrumental | `jim` | `jim` | `jeju` | `jima` | `jimi` |
| locative | `jem` | `jem` | `jej` | `jeju` | `jih` |
<!-- /render:pron-third -->

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

<!-- render:pron-tot -->
| | Masc sg | Neut sg | Fem sg | Dual | Plural |
|---|---|---|---|---|---|
| nominative | `tot` | `to` | `ta` | `ta` | `tje` |
| accusative | `tot` / `toga` ¹ | `to` | `tu` | `ta` | `tje` / `tjeh` ¹ |
| genitive | `togo` | `togo` | `toj` | `toju` | `tjeh` |
| **ablative** | `toga` | `toga` | `toj` | = dat | = dat |
| dative | `tomu` | `tomu` | `toj` | `tjema` | `tjem` |
| instrumental | `tjem` | `tjem` | `toj` | `tjema` | `tjemi` |
| locative | `tom` | `tom` | `toj` | `toju` | `tjeh` |

¹ animate (§3.7).
<!-- /render:pron-tot -->

The animate accusative is the **ablative** `toga`, not the genitive `togo`:
§3.7 puts the animate accusative singular on the ablative, because the `-a`
form every Slavic language uses there continues PIE `*-ōd`; `togo` was a
leftover from before §3.1 split the two cases apart, and §4.2's `dobra` had
it right already.

`sjej` declines identically on the stem `sj-`: `sjego`, `sjemu`, `sjim`, `sjem`,
`sji`, `sjih`, `sjimi`. The stem is invariant across the whole paradigm, as §2.5
requires of every word — Russian's `сим` and `сии` write `с-` there, but that
would truncate a stem Ruthenian keeps whole.

## 5.5 Interrogative and relative

<!-- render:pron-kto-czto -->
| | "who" | "what" |
|---|---|---|
| nominative | `kto` | `czto` |
| accusative | `koga` | `czto` |
| genitive | `kogo` | `czego` |
| **ablative** | `koga` | `czega` |
| dative | `komu` | `czemu` |
| instrumental | `kjem` | `czem` |
| locative | `kom` | `czem` |
<!-- /render:pron-kto-czto -->

**The masculine nominative `tot` is reduplicated, and that one cell is the whole
irregularity.** The declension gives `toj`, which is what OCS `тъи` and
Ukrainian `той` have; Russian doubled `тъ` onto itself and `tot` is the form a
reader knows. It reaches nothing else — the neuter is `to` and the feminine
`ta`, not `*toto` or `*tota` — and `sjej` does not share it, exactly as Russian's
`сей` does not. `sjej` is therefore wholly regular and `tot` is a one-cell
exception, the same shape as §5.5's `kto` and `czto`.

The plural nominative is `tje` and not `ti`: the rest of the plural is built on
`tje-` — `tjeh`, `tjem`, `tjemi` — and the nominative was the one cell that was
not. OCS had `ти` there, and Russian levelled it to `те` for the same reason.

`czto`'s instrumental and locative are one form, `czem`. Russian writes `чем`
and `чём`, but that `ё` is stressed `е` (§2.3) and Ruthenian does not spell the
shift, so the two fall together — as they did before a revision that split them
on the strength of a distinction the alphabet cannot carry.

`kto` is animate and `czto` inanimate, which is why `kto` has an oblique
accusative and `czto` does not. It is the **ablative** `koga`, not the genitive
`kogo` — §3.7 again, and the same correction §5.4's `toga` needed.

Also `czij` "whose", `kotoryj` "which", `kakyj` "what kind of" — all adjectival.

**The relative pronoun is `izze`** (OCS `иже`), inflecting as `toj` plus the
invariant `-zze`: `izze`, `jegozze`, `jemuzze`. Russian lost it in favour of
`который`; Ruthenian keeps both, `izze` for restrictive clauses and `kotoryj`
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
| `tri`, `czetyrje` | nominative plural | `tri domy` |
| `pjatj` and above | genitive plural | `pjatj domov` |

`dva doma` is the nominative **dual**, which is what it originally was. There is
no 11–14 exception and no last-digit rule: five and above always take the
genitive plural, and a compound numeral is governed by its **last word** —
`dvadjesjat dva doma` (dual), `dvadjesjat pjatj domov` (genitive plural).

In oblique cases the numeral and noun simply agree: `s dvuma domoma` "with two
houses", `s pjatju domami` "with five houses".

## 6.2 Cardinals 1–10

<!-- render:num-cardinals -->
| | | | |
|---|---|---|---|
| 0 `nolj` | 1 `odin` | 2 `dva` | 3 `tri` |
| 4 `czetyrje` | 5 `pjatj` | 6 `szestj` | 7 `sjedmj` |
| 8 `osmj` | 9 `djevjatj` | 10 `djesjatj` |  |
<!-- /render:num-cardinals -->

`nolj` is a soft masculine of declension II and governs the genitive plural,
`nolj domov`.

## 6.3 Teens, tens, hundreds — regularized

**One rule per rank, and no exceptions in any of them.**

Teens are "N on ten", on the unit's stem — the cardinal less a final `j` or
`je`:

`odinnadjesjat, dvanadjesjat, trinadjesjat, czetyrnadjesjat, pjatnadjesjat,
szestnadjesjat, sjedmnadjesjat, osmnadjesjat, djevjatnadjesjat`

Tens are "N tens", on the unit whole:

<!-- render:num-tens -->
| | | | |
|---|---|---|---|
| 20 `dvadjesjat` | 30 `tridjesjat` | 40 `czetyrjedjesjat` | 50 `pjatjdjesjat` |
| 60 `szestjdjesjat` | 70 `sjedmjdjesjat` | 80 `osmjdjesjat` | 90 `djevjatjdjesjat` |
<!-- /render:num-tens -->

Hundreds are "N hundred", likewise:

`sto, dvjesto, tristo, czetyrjesto, pjatjsto, szestjsto, sjedmjsto, osmjsto,
djevjatjsto`

Russian has four separate formations here and Ruthenian has three, one per
rank. Gone with them: `сорок` and `девяносто`, which have no transparent
structure at all; `-дцать`, a second tens formation beside `-десят`; and the
`-сти`/`-ста`/`-сот` variation in the hundreds. Two hundred keeps `dvje` rather
than `dva`, because `sto` was historically a dual there.

Above that the scale words are **nouns**, and the short scale is used, as in
English: each step is a thousand times the last.

<!-- render:num-scales -->
| | | | |
|---|---|---|---|
| 10³ `tysjacza` (fem. I) | 10⁶ `miljon` | 10⁹ `biljon` | 10¹² `triljon` |
| 10¹⁵ `kvadriljon` | 10¹⁸ `kvintiljon` |  |  |
<!-- /render:num-scales -->

There is no `miljard`: it belongs to the long scale, where `biljon` would be
10¹². `q` is not a letter (§2.1), so *quadrillion* is `kvadriljon`.

Being nouns, they are governed by their count like any other: `dvje tysjaczi`
(dual), `pjatj tysjaczov` (genitive plural), `tri miljony` (nominative plural).

A compound is written as separate words, one per rank — `sto tridjesjat dva`,
`djevjatjsto djevjatjdjesjat djevjatj` — and **only its last word declines**.
§6.1 already makes a compound's *government* its last word's; the declension
follows it, so `dvadjesjat pjati` and not Russian's `двадцати пяти`, where every
part inflects.

## 6.4 Declension

`odin` declines **pronominally** and agrees in gender, number and case:
`odin dom`, `odina tysjacza`, `odinogo doma`, `odinoj zzeny`. Its masculine
nominative is the bare stem, as `tot`'s is.

An earlier revision said "as a long adjective". The two declensions differ in
thirteen of their seventeen endings (§4.2), and both forms this paragraph cites
are among the four they share, so the citation did not decide it. The cell that
does is the feminine nominative — the long adjective would give `odinaja` where
the pronominal gives `odina`, and `odina tysjacza` is the form Russian has.

The stem is `odin-` throughout and not `odn-`: §3.9 abolished the fleeting vowel
for every word, and a numeral is not an exception to it. Russian's `одного`
against `один` is exactly the alternation `son`/`sona` no longer has.

`dva` is a **dual** form and has only dual endings — the plain nominal ones, so
it declines exactly as `dom` does in the dual (`doma` / `domu` / `domoma`):

<!-- render:num-dva -->
| | Masc/neut | Fem |
|---|---|---|
| nom / acc | `dva` | `dvje` |
| gen / loc | `dvu` | `dvu` |
| dat / ins / abl | `dvoma` | `dvoma` |
<!-- /render:num-dva -->

`tri` and `czetyrje` decline as plurals:

<!-- render:num-tri-czetyrje -->
| | `tri` | `czetyrje` |
|---|---|---|
| nominative | `tri` | `czetyrje` |
| genitive / locative | `trjeh` | `czetyrjeh` |
| dative | `trjem` | `czetyrjem` |
| instrumental | `trjemi` | `czetyrjmi` |
<!-- /render:num-tri-czetyrje -->

`pjatj` and above decline as **declension III** nouns (`noczj`): `pjatj`,
`pjati`, `pjati`, `pjatj`, `pjatjju`, `pjati`. This is inherited — the higher
numerals were feminine *i*-stem nouns in OCS and still behave like them.

## 6.5 Ordinals

Adjectives, long or short: `pjervyj, vtoryj, trjetyj, czetvjertyj, pjatyj,
szestyj, sjedmyj, osmyj, djevjatyj, djesjatyj`, then `odinnadcatyj` and so on;
`sotyj`, `tysjacznyj`.

## 6.6 Collectives and fractions

Collective numerals count groups and mixed-gender sets, and govern the genitive
plural: `dvoje, troje, czetvjero, pjatjero, szestjero, sjedmjero`. `dvoje
djetjej` "two children".

Fractions: `polovina` (½), `trjetj` (⅓), `czetvjertj` (¼), thereafter the
ordinal — `pjataja czastj` (⅕). `poltora` "one and a half" takes the dual.

# 7. Verbs

## 7.1 Categories

| Category | Values |
|---|---|
| aspect | imperfective, perfective — **derived, never listed** |
| tense | present, perfect, pluperfect, future |
| mood | indicative, imperative, conditional |
| voice | active, passive |
| person | 1, 2, 3 |
| number | singular, **dual**, plural |

**Every past is periphrastic.** Ruthenian has no synthetic past at all:

| Tense | Function | Formation |
|---|---|---|
| **perfect** | the past | `l`-participle + the copula — `jesm czital` |
| **pluperfect** | past before the past | the copula + `byl` + the `l`-participle |

OCS had three synthetic pasts and an earlier revision of this document restored
two of them, on the argument that the aorist/imperfect opposition is what makes a
conservative Slavic standard conservative. Both are gone, and for the same reason
each time: neither was carrying a distinction the language could not get
otherwise.

The aorist went first. Its one diagnostic shape is the bare stem in the second
and third singular, and that shape **collides** — `dvinutj` gives `dvinu` for
both the first singular present and the aorist second and third singular, in
every verb of class 3.

The imperfect followed it. With the aorist gone it was not an *imperfect* any
more, only "the synthetic past", contrasting with nothing; and aspect already
marks completedness on every verb, which is what Slavic exchanged the old tense
system for in the first place. What it cost was visible in the paradigms: the
two pasts of one verb were built on different stems, so `dvinutj` had `dvinjah`
beside `dvinul` and `govoritj` had `govorjah` beside `govoril`, with the theme
vowel showing in one and not the other.

Nothing remains of it, not even for the copula. `bjah` was `bytj`'s own past and
went with the rest; the pluperfect is built instead from `bytj`'s `l`-participle
`byl`, which the ordinary machinery already produces (§7.9). So the language has
**one** synthetic tense — the non-past — and everything else is composed.

Aspect and the past are **independent** axes, as in OCS: a perfective verb has a
perfect and so does an imperfective.

*§7.5 and §7.6 held the aorist and the imperfect. The numbers are left vacant
rather than closed up, so that every reference to §7.7 and after — in this
document, in `COMPARATIVE_GRAMMAR.md` and throughout the engine — still points
where it did.*

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
mytj   → myj-    myju, myjesz, myjet …      (OCS myti, myjǫ)
pitj   → pij-    piju, pijesz, pijet …      (OCS piti, pijǫ)
bitj   → bij-    biju, bijesz, bijet …      (OCS biti, bijǫ)
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
czitatj   → czitaj-   czitaju, czitajesz, czitajet …
pisatj'   → pisz-     piszu, piszesz, piszet …
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

<!-- render:verb-nonpast -->
| | Singular | **Dual** | Plural |
|---|---|---|---|
| **1st conjugation** | | | |
| 1 | `-u` | `-jevje` | `-jemy` |
| 2 | `-jesz` | `-jeta` | `-jetje` |
| 3 | `-jet` | `-jetje` | `-ut` |
| **2nd conjugation** | | | |
| 1 | `-ju` | `-ivje` | `-imy` |
| 2 | `-isz` | `-ita` | `-itje` |
| 3 | `-it` | `-itje` | `-jat` |

`czitatj`: `czitaju, czitajesz, czitajet` · `czitajevje, czitajeta, czitajetje` · `czitajemy, czitajetje, czitajut`.
<!-- /render:verb-nonpast -->

## 7.7 Perfect and pluperfect

The `l`-participle, agreeing in gender and number, with the copula:

<!-- render:verb-l-participle -->
| | Masculine | Feminine | Neuter | Dual | Plural |
|---|---|---|---|---|---|
| | `czital` | `czitala` | `czitalo` | `czitala` | `czitali` |
<!-- /render:verb-l-participle -->

- **perfect**: `jesm czital` "I have read"
- **pluperfect**: `jesm byl czital` "I had read"

Unlike Russian, the copula is **not** dropped: `jesm czital`, not `*czital`.

The pluperfect stacks the same device on itself: `byl` is `bytj`'s
`l`-participle, so `jesm byl czital` is "I am having-been having-read". An
earlier revision had `byh czital` beside `bjah czital`, an aorist auxiliary
against an imperfect one; both auxiliaries went with the synthetic pasts that
carried them (§7.1), and what replaces them needs no form the language did not
already have.

## 7.8 Future

| | Perfective | Imperfective |
|---|---|---|
| | present endings, future sense — `poczitaju` | `budu` + infinitive — `budu czitatj` |

`budu, budjesz, budjet` · `budjevje, budjeta, budjetje` · `budjemy, budjetje,
budut`.

## 7.9 The copula `bytj`

Irregular, and the most frequent verb in the language. The full OCS paradigm,
dual included, and **never omitted**.

<!-- render:verb-bytj -->
| | Singular | Dual | Plural |
|---|---|---|---|
| **present** | `jesm`, `jesesz`, `jest` | `jesvje`, `jesta`, `jestje` | `jesmy`, `jestje`, `jesut` |
| **future** | `budu`, `budjesz`, `budjet` | `budjevje`, `budjeta`, `budjetje` | `budjemy`, `budjetje`, `budut` |
<!-- /render:verb-bytj -->

**There is no past row.** `bjah` went with the synthetic past (§7.1), and the
copula is not exempt from a rule the rest of the language keeps. The pluperfect
that used to rest on it now rests on `byl`, this verb's own `l`-participle.

Participle `byl/byla/bylo/byli`; infinitive `bytj`; imperative `budj`.

The infinitive is `bytj` and not `byti`: §7.3's classes make `-tj` the ending of
every Ruthenian infinitive, and §7.2 already writes `bytj` when it lists the
verbs shaped like `pitj` and `mytj`. `bytj` was the OCS citation form left
standing.

**One of these rows is regular and is not stored anywhere.** The participle is
§7.7's rule on the stem `by-`, so it comes out of the ordinary machinery given
the lemma `bytj`. What is genuinely suppletive is the present, the
future and the imperative — two roots and a third stem, and with no tense
parameter left in the language each takes a function of its own.

Russian's zero copula (`он врач`) is an East Slavic innovation. Ruthenian follows
OCS, Polish and Ukrainian: `on jest vracz`.

### `bytj` is the language's one suppletive verb, deliberately

§1 removes suppletion everywhere else. `bytj` keeps it, because it is the most
frequent verb in the language and every Indo-European language tolerates
suppletion in exactly this word — Latin `sum`/`fui`, English `is`/`was`/`been`,
OCS `jesmь`/`byxъ`/`bǫdǫ`.

The stems, and their sources. The present used to carry a third stem — OCS's
3pl `sǫtъ` gave `sutj` — until the cell was rebuilt as `jes-` + the regular
`-ut`: `jesut`. One row left the table, and the present became a single stem
with near-regular endings (`jesm`, `jesesz`, `jest`, `jesvje`, `jesta`,
`jestje`, `jesmy`, `jestje`, `jesut`):

| slot | stem | from | regular? |
|---|---|---|---|
| present | `jes-`, in every cell | PIE `*h₁es-` "be, exist" | half — athematic endings on one stem |
| l-participle | `by-` | `*bʰuH-` | **yes** — §7.7's rule on the same stem |
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
every Slavic speaker knows on sight: onto `bud-` costs `bytj`, `byl` and the
past, giving `budjel`; onto `by-` costs `budu`, the most recognizable
future marker in Slavic. One suppletive verb is cheaper than either.

## 7.10 Imperative and conditional

Imperative: present stem + `-i`, or the bare stem after `j`.

<!-- render:verb-imperative -->
| | Singular | Dual | Plural |
|---|---|---|---|
| 2 | `czitaj` | `czitajta` | `czitajtje` |
| 1 (hortative) | — | `czitajvje` | `czitajmy` |
<!-- /render:verb-imperative -->

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
| past passive | `-nyj` / `-jenyj` / `-tyj` | `poczitanyj` |
| present gerund | `-ja` | `czitaja` |
| past gerund | `-v` | `czitav` |

Participles decline as adjectives and have **both long and short forms**, like
any adjective — the short passive participle is how the passive is built:
`dom jest poczitan` "the house is read".

**The past passive `n` is single, not doubled.** Russian writes `-nnyj` long
against `-n` short (`прочитанный` / `прочитан`), which gives the long and short
forms *different stems*. Ruthenian writes one `n` throughout, so there is one
stem and the participle behaves like every other adjective:

| verb | stem | long | short |
|---|---|---|---|
| `poczitatj` | `poczitan` | `poczitanyj` | `poczitan` |
| `rjeszitj` | `rjeszen` | `rjeszenyj` | `rjeszen` |
| `bitj` | `bit` | `bityj` | `bit` |

The doubling in Russian is orthographic convention rather than a distinction the
language uses — nothing is told apart by it — so removing it costs no contrast
and buys a participle that is a plain adjective stem. Which of `-n-`, `-jen-` and
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
| `-stvo` | abstract or collective | neut. II | `czelovjek` → `czelovjeczstvo` |
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
`czern-o-zjem` "black-earth".

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
| accusative | `v`, `na`, `za`, `pod`, `czerjez`, `pro`, `skvozj` | motion **into**, direction |
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

`zze` (emphatic), `li` (interrogative), `by` (conditional), `nje` (negation),
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
dobr czelovjek jest zdjesj.     A good man is here.
dobryj czelovjek jest zdjesj.   The good man is here.
```

Where a noun phrase has no adjective, definiteness is unmarked — as in every
Slavic language except Bulgarian and Macedonian. The particle `-to` may
optionally definitize a bare noun (`dom-to`), which is colloquial rather than
grammatical.

## 10.4 Questions

Three devices, in ascending formality:

1. **Intonation alone** — `ty czitajesz?`
2. **The clitic `li`**, in second position — `czitajesz li ty?` This is the
   neutral written question. `li` is a clitic and shares the second-position
   cluster with the pronoun clitics, standing first within it (§10.1a).
3. **An interrogative word**, fronted — `czto ty czitajesz?`

## 10.5 Subordination

| Conjunction | Use |
|---|---|
| `czto` | statement complement — `znaju, czto on czitajet` |
| `cztoby` | purpose or irrealis complement — `hoczu, cztoby on czital` |
| `jesli` | condition — `jesli by on czital…` |
| `kogda`, `poka` | time |
| `jako` | cause, manner |

Relative clauses use `izze` when restrictive and `kotoryj` when
non-restrictive (§5.5). The relative agrees with its antecedent in gender and
number but takes its case from **its own clause**:

```
czelovjek, jegozze vizzu…        the man whom I see…      (acc in its clause)
czelovjek, izze czitajet…        the man who reads…       (nom in its clause)
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
| instrumental | means; the predicate of `bytj` in the past | accompaniment |
| locative | — (never bare) | location, topic |

The locative is the only case that **cannot** occur without a preposition —
inherited, and true of every Slavic language.

The instrumental predicate is worth noting: `on jest vracz` (nominative,
permanent) against `on byl vraczom` (instrumental, temporary or past role).

---

# 11. Summary of paradigm sizes

Distinct **surface forms**, after syncretism — 24 nominal cells (8 cases × 3
numbers) never yield 24 forms:

<!-- render:paradigm-sizes -->
| Word class | Singular | Dual | Plural | Total |
|---|---:|---:|---:|---:|
| noun, declension II masculine (`dom`) | 7 | 3 | 5 | **15** |
| noun, declension II neuter (`okno`) | 6 | 3 | 5 | **14** |
| noun, declension I feminine (`zzena`) | 6 | 3 | 5 | **14** |
| noun, declension III (`noczj`) | 3 | 3 | 5 | **11** |
| adjective, long (`dobryj`) | — | — | — | **15** across all genders |
| adjective, short (`dobr`) | — | — | — | **16** across all genders |
| verb, one aspect (`czitatj`) | — | — | — | **8** non-past + **5** imperative + **4** `l`-participle, six participle/gerund stems, every past periphrastic |
<!-- /render:paradigm-sizes -->

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
nor pitch (§2.1) and has no aorist at all (§7.1). They supply attested
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
   | the dual, the past tense | OCS |
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
word of its class, and **strictly**: `nacija`, genitive `nacijy`, dative
`naciji`, instrumental `nacijej` — soft declension I on the vowel-final stem
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

**Settled in this revision.** Both synthetic pasts, dropped (§7.1). What "conservative" means (§1: grammar from OCS,
phonology from Russian, vocabulary East Slavic with an OCS learned layer); the
sound correspondences (§2.6) and the productive learned layer (§2.6a); the
fleeting vowel, abolished (§3.9); the animate accusative (§3.7); the predicate
adjective (§4); the clitic pronoun series and Wackernagel placement (§5.1a,
§10.1a); the reflexive as a free clitic (§5.2); aspect, with its two stored
classes (§7.2, §7.2a); the periphrastic pasts (§7.1); the single pluperfect (§7.7);
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
`piszesz` underivable. The resolution was that §2.2's list was wrong: `zz` and
`sz` are inherently **hard**, as `ж` and `ш` are in Russian, so they have no soft
value for a `j` to mark either. Both kinds of consonant reject the glide, for
opposite reasons, and one rule now covers all four — `otjecze`, `druzze`,
`piszesz`.

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
  by side; a narrative passage for the past tenses; and a technical
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
