# Russian grammar: references, edge cases, and what Ruthenian does with them

Research notes for `ruthenian-core`. Two purposes: record what the standard
descriptive tradition says, and inventory every irregularity we have actually
measured, so that decisions about which of them Ruthenian keeps are made against
evidence rather than intuition.

Every count is measured over the **whole** dump (`INVARIANTS.md` I1): 441 629
Russian records — 29 689 nouns, 12 773 verbs with a conjugation index, 9 999
adjectives. Where a published source and the data disagree, the data wins and the
disagreement is noted.

---

# Part 1 — The reference landscape

## Zaliznyak, *Грамматический словарь русского языка* (1977; 6th ed. 2010)

The source everything else derives from: ~110 000 words, each tagged with a
compact index encoding declension/conjugation type and stress pattern. It is
"the standard reference for Russian inflection and forms the basis for most
Russian language processing algorithms."

Its architecture is the one we already use, at one remove: **Wiktionary's
`ru-noun+` and `ru-conj` templates are Zaliznyak indices**, which is why our class
codes look like `4b+pжд` and our accent labels like `accent-c`. We are not
choosing a classification; we inherited his.

- Nouns: **8 declension types**, keyed on the *graphic* stem ending — plain
  consonant, `ь`, velar (`г к х`), sibilant (`ж ч ш щ`), `ц`, vowel/`й`.
- Stress: patterns `a`–`f` plus primed variants (`b′ d′ f′ f″`).
- Verbs: **16 classes** plus irregulars.

The dictionary exists as machine-readable text at
[gramdict/zalizniak-2010](https://github.com/gramdict/zalizniak-2010). That is a
*primary* source; ours is Wiktionary's derivation of it. Worth considering for
Phase 4, where the difference between primary and derived data starts to matter.

## Zaliznyak, *Русское именное словоизменение* (1967)

The theoretical companion. Establishes the derivational architecture of Russian
nominal morphology and — the point that matters here — **identifies more than six
cases**, including the partitive, the second locative, and special adnumeral
forms. Estimates of the total run from eight to nine depending on the analyst.

## *Русская грамматика* (Academy Grammar, 1980, ed. Shvedova)

The comprehensive institutional description. Where to go for the boundaries of a
category (what counts as a short form, when a participle is possible) rather than
for a per-lemma index.

## Timberlake, *A Reference Grammar of Russian* (2004); Wade, *A Comprehensive Russian Grammar*

Descriptive references in English. Useful for prose statements of a rule; not a
source of per-lemma data.

## Jakobson (1948) / Townsend (1975): the one-stem system

The live architectural alternative, and we are **declining it deliberately**.

Jakobson observed that the shape of a Russian verb stem is predictable from the
following suffix, so both the infinitive and present stems can be derived from a
single underlying stem by truncation rules. Townsend's treatment gives 24
subclasses. For a *generator* this is attractive: it would shrink the lexicon's
principal-parts burden, because one stem plus a rule replaces two listed stems.

Three reasons not to adopt it:

1. **Our data is Zaliznyak-indexed.** Adopting Jakobson means discarding class
   codes we get for free and re-deriving 24 subclasses ourselves.
2. **It is contested, not settled.** The one-stem/two-stem argument ran through
   SEEJ and RLJ for decades without resolution; recent work frames the two as
   reconcilable under a usage-based model rather than one superseding the other.
3. **We already have the two-stem apparatus working**, with principal parts as
   the typed escape hatch for what the class cannot derive.

Recorded here so the question is closed with a reason rather than reopened by
whoever next reads a pedagogical grammar.

---

# Part 2 — The inflectional system, measured

## Nouns: stem classes

Zaliznyak's 8 declension types map onto our 7 `StemClass` variants. Counts by
Wiktionary's own labels:

| Class | Count | Ours |
|---|---:|---|
| hard-stem | 12 314 | `Hard` |
| velar-stem | 7 297 | `Velar` |
| i-stem | 3 583 | `I` |
| soft-stem | 1 452 | `Soft` |
| ц-stem | 1 340 | `Ts` |
| sibilant-stem | 899 | `Sibilant` |
| vowel-stem | 641 | `Vowel` |

Our classification is therefore not ad hoc — it is Zaliznyak's, with velar and
sibilant split out because in Ruthenian they are `k g h` and `zz sz cz szcz`, and
the spelling rules key on exactly that distinction.

## Nouns: accent patterns

**Derived from the data**, not quoted: for every pattern, where the stress
actually landed across ~285 000 attested forms. `S` = stem, `E` = ending.

| | nom | gen | dat | acc | ins | prp | NOM | GEN | DAT | ACC | INS | PRP | lemmas |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---:|
| **a** | S | S | S | S | S | S | S | S | S | S | S | S | 25 442 |
| **b** | E | E | E | E | E | E | E | E | E | E | E | E | 2 382 |
| **c** | S | S | S | S | S | S | E | E | E | E | E | E | 536 |
| **d** | E | E | E | E | E | E | S | S | S | S | S | S | 495 |
| **e** | S | S | S | S | S | S | S | E | E | S | E | E | 287 |
| **f** | E | E | E | E | E | E | S | E | E | S | E | E | 75 |

Three things this makes precise:

- **`e` and `f` differ only in the singular.** Both put the plural nominative and
  accusative on the stem and the plural obliques on the ending.
- **A published summary is wrong.** Wiktionary's noun stress appendix describes
  `f` as stem-stressed in the singular; it is ending-stressed. Measured, not
  argued.
- **Pattern `b` measures as mixed in the masculine nominative singular** because
  a null ending cannot bear stress and it retracts to the stem — `stól` /
  `stolá` / `stolóv`. That is a phonological consequence, not a seventh pattern.

Primed variants each move exactly *one* cell off the base pattern: `b′`
instrumental singular to the stem (`любовь`), `d′` and `f′` accusative singular
to the stem (`душа`, `рука`), `f″` instrumental singular and nominative/accusative
plural to the stem (`грудь`). Together: 72 nouns.

## Verbs: classes

| Class | Count | | Class | Count |
|---|---:|---|---|---:|
| 1 | 5 525 | | 8 | 132 |
| 4 | 3 273 | | 12 | 128 |
| 2 | 1 148 | | 11 | 104 |
| 3 | 752 | | 14 | 85 |
| 6 | 532 | | 13 | 66 |
| 5 | 354 | | 9 | 48 |
| 7 | 266 | | 15 | 41 |
| `irreg` | 252 | | 16 | 34 |
| | | | 10 | 33 |

Classes 1–6 are **11 584 of 12 773 = 90.7 %**. Aspect splits 5 517 imperfective /
4 902 perfective, plus 1 339 / 979 intransitive-marked and a handful of
impersonals. 3 182 verbs are reflexive.

## Adjectives

| Type | Count |
|---|---:|
| hard `-yj` | 6 669 |
| velar/sibilant (hard endings, `i`-spelling) | 2 356 |
| stressed `-oj` | 540 |
| other | 279 |
| **true soft `-nij`** | **155** |

4 571 of 9 999 have short forms; 5 428 do not. 2 248 declare a comparative in the
headword.

---

# Part 3 — The edge cases

Each: what it is, how big it is, what Ruthenian does now, and whether it is a
candidate for regularization. The last column is the input to the design
discussion, not a decision.

## 3.1 Marginal cases — Russian has more than six

The clearest instance of the engine being unable to express a distinction the
language makes. `Case` has six variants; the data has four more.

| Form | Lemmas | Example | Status |
|---|---:|---|---|
| partitive singular (второй родительный) | 206 | `сне́гу` "some snow" vs `сне́га` | **unrepresentable** |
| locative singular (второй предложный) | 199 | `на борту́`, `в лесу́` vs `о бо́рте` | **unrepresentable** |
| vocative | 40 | old `кня́же`, `ста́рче`; new `дя́дь`, `мама́ш` | **unrepresentable** |
| count form (счётная форма) | 31 | after numerals | **unrepresentable** |

These are lexically restricted — only certain nouns have them — so they belong in
the lexicon, not in a rule. This is why `год` → `v godú` scored as a miss: the
harness had nowhere to put it.

*Regularization candidate:* strong. A standardized language could drop all four
(fold partitive into genitive, locative-2 into prepositional) or keep them
regularly. Either way it must first be able to **represent** them.

## 3.2 Mobile stress

Patterns `c`–`f` (1 393 nouns) plus the primed variants (72). **Now
implemented**, from the derived table. The primed variants are not.

*Regularization candidate:* strong and cheap. Collapsing every noun onto pattern
`a` or `b` removes the entire mobile-stress system. `stress.fixed-stem` already
exists as a `RuleId` for exactly this.

## 3.3 Heteroclitic nouns

Nouns that switch declension between cases. The `-мя` neuters (`время`, `имя`,
`знамя`, `бремя`, `вымя`, `пламя`, `темя`, `племя`, `семя`, `стремя` — 10 core
lemmas, 24 including compounds) insert `-ен-` in the oblique cases: `время` →
`времени` → `времена`. Also `дитя` → `дитяти`, `мать`/`дочь` → `матери`/`дочери`,
`путь` (masculine with feminine endings).

Ruthenian: **not modelled**; they surface in the failure list as needing lexicon
support.

*Regularization candidate:* strong. ~15 lemmas, each memorized individually by
learners.

## 3.4 Fleeting vowels (беглые гласные)

A vowel appearing or vanishing before a zero ending: `okno` → `okon`, `kukla` →
`kukol`, `sbjerknizzka` → `sbjerknizzjek`, `sovjestnyj` → `sovjestjen`.

Ruthenian: **implemented for nouns** with a zero ending, keyed on the source's
`*` marker; **not for adjective short forms**, which is why masculine short forms
score 34.8 %.

*Regularization candidate:* medium. Removing them produces unpronounceable
clusters unless the phonotactics are relaxed too — this is a case where the
irregularity is doing phonological work.

## 3.5 Present-stem mutation (iotation)

Twenty distinct mutations measured; `ов → у` dominates at 675, six times the
next. Implemented: 14. Not implemented: `в → ∅` (41), `ев → у` (19), `ев → ю`
(11), `им → емл` (5), `ер → р` (3), `р → ер` (2).

The decisive finding: **mutation is conditioned on the class, not on the stem's
final consonant.** Of 1 977 class-1 verbs with a labial-final stem, *not one*
takes epenthesis. A rule keyed on the consonant would corrupt all of them.

*Regularization candidate:* medium. `iotation.uniform` exists as a `RuleId`.
Removing mutation entirely (`pisatj` → `pisaju`) is a large, very visible change.

## 3.6 Structural gaps vs lexical defectiveness

The distinction that most nearly broke the crate. Of the dump's `"-"` slots:

| Aspect | Verbs | Gap slots | Per verb |
|---|---:|---:|---:|
| perfective | 5 881 | 55 646 | 9.5 |
| imperfective | 6 856 | 9 517 | 1.4 |

Almost all are **structural**: a perfective verb has no present tense, no present
participles, no present gerund; an intransitive has no passive participle. These
are grammar and are *derived* in `verb::slot_exists`.

**Lexical** defectiveness is separate and tiny, and the source marks it with an
explicit override: `победить` carries `futr_1sg: "-"` because `*побежу` is
avoided.

*Regularization candidate:* the lexical gaps, yes — `gap.fill-defective-1sg`
exists for them. The structural ones, absolutely not: filling them would invent a
present tense for perfective verbs and destroy the aspect system.

## 3.7 Suppletion and irregular verbs

252 verbs carry `irreg`; the classic suppletives are `идти`/`шёл`, `быть`,
`есть`, `дать`, `хотеть`, `бежать`. Ruthenian returns `Err(Unsupported)` for
these — the lexicon must supply them.

*Regularization candidate:* strong in principle, small in count.
`suppletion.level` exists as a `RuleId` but is the most semantically disruptive
of the departures.

## 3.8 Indeclinables

1 193 nouns decline for nothing — `кофе`, `метро`, `пальто`, mostly loans.

*Regularization candidate:* strong, and interesting: a purist standard could
*decline* them (`metro`, `metra`, `metru`), which is regularization by
**addition** rather than removal.

## 3.9 Number defectiveness

460 plural-only nouns (`ножницы`, `часы`), 1 543 singular-only. Ruthenian carries
`NumberDefect` in the lexicon schema but does not act on it.

*Regularization candidate:* medium. `ножницы` having a singular is a semantic
question, not only a morphological one.

## 3.10 Animacy in the accusative

Masculine animate accusative = genitive; inanimate = nominative. Implemented, and
it works. Wiktionary marks animacy explicitly on a minority of records (mostly
animates), so the lexicon must carry it — it is not derivable.

*Regularization candidate:* weak. The syncretism is systematic and information-
bearing.

## 3.11 Numeral government

1 → nominative singular; 2–4 → genitive **singular**; 5+ → genitive plural; 11–14
→ genitive plural regardless of last digit. Implemented, returned as *structure*
(case + number) rather than a string.

*Regularization candidate:* strong. This is the subsystem most often named as
Russian's least defensible, and one uniform rule would replace it.

## 3.12 `ё` / `е`

`ё` is inherently stressed, and Wiktionary almost never marks it: **179 of 80 064
occurrences** carry U+0301, all in reduplicated intensives
(`чё́рный-пречё́рный`). Ruthenian normalizes `ё → е + U+0301`, transferring the
implicit stress — a naive substitution would destroy it.

*Regularization candidate:* already decided (normalize), and the decision is
sound.

## 3.13 Homographs

`писать` is two verbs — class `1a` and class `6c` — distinguished only by stress.
Our targeted fixture picked the wrong one. Composite keys (Phase 3) exist for
this.

*Regularization candidate:* n/a; a lexicon design problem, not a language one.

## 3.14 Soft vs hard adjective stems

True soft (`-ний`) is **155 of 9 999 = 1.6 %**. Velar and sibilant stems are
*hard* with an `i`-spelling (`русского`, never `*русскего`) — a distinction that
is easy to get backwards, and we did until it was measured.

*Regularization candidate:* strong. 1.6 % of adjectives carry an entire parallel
ending set.

## 3.15 Aspect

Not an irregularity but the organizing category of the verb: 5 517 imperfective,
4 902 perfective, paired by prefixation, suffixation and suppletion with no
general rule. The lexicon stores the partner as an `EntryKey`.

*Regularization candidate:* none. Removing aspect would not be a standardized
Russian; it would be a different language.

## 3.16 Short-form availability

4 571 adjectives have short forms, 5 428 do not, and which is lexical.

*Regularization candidate:* strong — give every adjective short forms, or none.

---

# Part 4 — Where this leaves `ruthenian-core`

**Implemented and principled:** the seven stem classes; all six accent patterns;
verb classes 1–6 (90.7 %); the two-stem model with typed principal parts;
structural-gap derivation; animacy syncretism; numeral government as structure;
14 mutations; fleeting vowels for nominal zero endings; adjective softness
derivation.

**Known missing**, in rough order of what they cost:

| Gap | Scale | Note |
|---|---|---|
| Verb classes 7–16 + `irreg` | 1 189 verbs (9.3 %) | returns `Unsupported`, never a wrong form |
| Marginal cases | ~450 lemmas | *unrepresentable* — worse than unimplemented |
| Heteroclitics | ~15 lemmas | high visibility, common words |
| Adjective short-form fleeting vowels | ~46 in sample at 34.8 % | |
| Six mutations | ~81 verbs | |
| Primed accent patterns | 72 nouns | |
| Past-passive-participle mutation (`+pжд`) | parsed, unapplied | |

**The shape of the standardization question.** Sorting the edge cases by whether
a standardized language could remove them:

- *Remove cleanly* — mobile stress, heteroclitics, indeclinables, numeral
  government, short-form availability, soft adjective stems, marginal cases,
  lexical defectiveness, suppletion.
- *Remove at a phonological price* — fleeting vowels, iotation.
- *Cannot remove without changing the language* — aspect, animacy syncretism,
  the two-stem verb, structural gaps.

That division, and what Ukrainian and Belarusian do with each, is the next
conversation.
