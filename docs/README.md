# Documentation map

Which document is authoritative for what. When two disagree, the one higher in
this list wins, and the lower one is the bug.

## The language

| Document | Authoritative for | Status |
|---|---|---|
| [`RUTHENIAN.md`](RUTHENIAN.md) | **The language itself.** Phonology, the eight cases, three numbers, three declensions, six conjugation classes, word formation, the closed classes, syntax, the lexicon's sourcing and borrowing rules. | **Normative.** Outranks every other document in the repository on any question about what Ruthenian is. |
| [`ORTHOGRAPHY.md`](ORTHOGRAPHY.md) | The Ruthenian alphabet and the Cyrillic↔Latin mapping. | Normative, subordinate to `RUTHENIAN.md` §2. |
| [`COMPARATIVE_GRAMMAR.md`](COMPARATIVE_GRAMMAR.md) | The comparative evidence the design rests on — PIE, Sanskrit, OCS, Russian, Ukrainian, Belarusian, Interslavic, one word class at a time. | Research. Explains *why* the spec chose what it chose; never overrides it. |

A claim about Ruthenian is settled by `RUTHENIAN.md`, not by measurement — there
is no Ruthenian corpus and there never will be (`INVARIANTS.md` I7). Where the
spec is silent, the fix is to amend the spec, not to infer an answer from a
source language.

## The source languages

[`sources/`](sources/) holds studies of the languages the lexicon draws cognates
from. **None of it is normative for Ruthenian.** These describe what each source
has, what it lost, and how confidently a Ruthenian form can be reconstructed from
it.

| Document | Language | Role |
|---|---|---|
| [`sources/RUSSIAN_GRAMMAR.md`](sources/RUSSIAN_GRAMMAR.md) | Russian | Largest inventory (419 283 lemmas); the Zaliznyak classification the extractor reads. Has lost yat, the dual, the ablative, the aorist, the second palatalization, and all but 40 vocatives. |

Ukrainian, Belarusian, Polish and Old Church Slavonic each warrant the same
treatment and do not have it yet. They are the sources for precisely what Russian
destroyed — the yat reflex `-i`, the second palatalization at 99 %, 25 180
vocatives, the nasal vowels, and the only attested dual (77 714 OCS forms).

Claims in this directory are **measured over the whole dump**, per
`INVARIANTS.md` I1 and I7.

## The software

| Document | Authoritative for |
|---|---|
| [`../DIRECTION.md`](../DIRECTION.md) | The boundaries between crates, the laws, the phase order, the stability contract. |
| [`../INVARIANTS.md`](../INVARIANTS.md) | Properties that must hold across the whole project, each with the command that falsifies it. |
| [`specs/<crate>.md`](specs/) | One crate each. Authoritative for that crate's API, guards and scope. |
| [`../LESSONS.md`](../LESSONS.md) | Why the laws exist — the failures they were paid for. |

Each spec is authoritative for its crate; `DIRECTION.md` is authoritative for
what lies between them; `RUTHENIAN.md` outranks both whenever the disagreement is
about the language rather than the code.

## Prompts and working notes

`PROMPT_*.md` and `PR2_FIX_PROMPT.md` at the repository root are task briefs, not
specifications. They record what was asked for at a point in time and go stale;
`PROMPT_SPEC_COMPLETION.md` in particular is the open-questions list feeding
`RUTHENIAN.md` §13. Never cite a prompt as authority for a design decision — cite
the document the prompt caused to change.
