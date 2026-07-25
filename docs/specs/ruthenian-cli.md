# Spec: `ruthenian-cli`

Phase 7. Depends on `ruthenian`, `ruthenian-orthography`.

## 1. Purpose

The `ruth` binary — the product. Everything the library can do, reachable from a
terminal, in both scripts, with machine-readable output.

It is a **thin adapter**. It parses arguments, calls the facade, and formats the
answer. It contains no morphology, no orthography rules, no lookup logic, and no
special cases for particular words. If `ruth` needs behaviour the facade does not
provide, the change goes in the facade.

This constraint is not tidiness. The CLI and the evaluator must consume the same
`Form` from the same call, or the published accuracy number stops describing the
shipped tool — which is the first and largest failure mode this project is
designed to avoid.

## 2. Public surface

```text
ruth translit <text>                    # Cyrillic → Ruthenian
ruth translit --to-cyrillic <text>      # the other direction
ruth lookup <lemma>                     # metadata: pos, gender, animacy, class,
                                        #   accent, aspect + partner, provenance
ruth decline <lemma> [--case gen] [--number pl]
ruth conjugate <lemma> [--person 1] [--number sg] [--tense present]
ruth paradigm <lemma>                   # the full table, aligned — every slot
ruth principal-parts <lemma>            # the forms the class does not predict
ruth stats                              # lexicon counts by pos, class, rule impact
```

**`ruth` is the user-facing tool and nothing else.** Regeneration and measurement
live in `cargo xtask refresh-data` / `cargo xtask accuracy` and are deliberately
*not* mirrored here: the shipped binary contains nothing a user cannot use, and
there is exactly one way to invoke a developer workflow. A contributor learns
`xtask` from the README's regenerate → check → measure loop.

`paradigm` prints the **complete** table by default. Producing every form from a
lemma is the product's whole purpose, so it is not put behind a flag; the short
view has its own subcommand instead.

Global flags:

- `--json` on **every** subcommand. Same information as the human output, no
  more, no less.
- `--policy attested|regularized` on every generating subcommand; `attested` is
  the default for v1.
- `--show-deviations` prints the `RuleId` and trace for any form that is not
  plain attested Russian.
- `--script cyrillic|ruthenian` for output; input script is auto-detected.

## 3. Behaviour that is contractual

- **Both scripts accepted everywhere.** `ruth decline voda` and `ruth decline
  вода` do the same thing. Detection is by alphabet, and an ambiguous or
  mixed-script argument produces an error naming the offending offset — never a
  silent guess. (The reference orthography's `"cat дом"` → `"цат дом"` failure is
  exactly what this prevents.)
- **A guess is visibly a guess.** For an out-of-vocabulary lemma the rules still
  answer, and the output says so — a marker in human output, an `origin` field in
  JSON. A user must never have to wonder whether a form was looked up or derived.
- **Gaps are shown as gaps.** A defective slot prints as an em dash with a note,
  not as a blank or a fabricated form. Under `--policy regularized` it prints the
  filled form tagged with `gap.fill-1sg`.
- **Exit codes mean something**: `0` success; `1` no such lemma; `2` bad usage;
  `3` internal error. `--json` still emits a structured error body on failure.
- **No panics.** Any input, including invalid UTF-8 arguments and empty strings,
  produces a diagnostic.

## 3a. Inputs and outputs

In: command-line arguments and, for `translit`, stdin when no text argument is
given. No config file, no environment variables that change linguistic output,
no network.

Out: stdout for results, stderr for diagnostics, an exit code from §3. `--json`
switches stdout to a single JSON document per invocation. **Nothing is ever
written to disk** — `ruth` has no developer commands, so it has no reason to
write anything.

Sense data reaches the binary through `include_bytes!` at build time
(`ruthenian-lexicon.md` §2a), so `ruth` stays a single self-contained executable
with no data file to locate at runtime.

## 4. Data owned

Nothing. Argument parsing and output formatting only.

## 5. Dependencies allowed

`ruthenian`, `ruthenian-orthography`, `clap` (derive), and a JSON serializer.
Nothing that touches morphology.

## 6. Invariants

1. No subcommand computes a form. Every form comes from a facade call.
2. `--json` output for a given invocation is a lossless encoding of the human
   output.
3. Every generating subcommand honours `--policy`, and the active policy appears
   in the output.
4. Script auto-detection never silently transliterates mixed-script input.
5. Every form printed carries its origin, in both output modes.
6. No panic on any input.

## 7. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `cli_has_no_morphology` | Inv. 1 | Add an ending-manipulating function to the CLI crate; the dependency/API check flags it, and the differential test against the facade diverges | required | seconds | crate |
| `json_parity` | Inv. 2 | Print a note in human output that has no JSON field | required | seconds | crate |
| `policy_echoed` | Inv. 3 | Accept `--policy` and ignore it; output under both policies is byte-identical for a rule-affected lemma | required | ms | crate |
| `mixed_script_rejected` | Inv. 4 | Feed `"cat дом"`; must error with an offset, not transliterate | required | ms | crate |
| `origin_always_shown` | Inv. 5 | Print a rule-derived form with no marker | required | ms | crate |
| `no_panic_fuzz` | Inv. 6 | Empty string, lone combining mark, 10 kB argument, invalid UTF-8 | required | seconds | crate |
| `exit_codes` | §3 | Return `0` for an unknown lemma | required | ms | crate |
| `golden_invocations` | The documented examples work | Change output formatting; the pinned transcripts diff | required | ms | crate |
| `colour_carries_no_information` | §10 — piped output loses nothing but styling | Convey a gap or a regularized origin by colour alone; the piped transcript no longer distinguishes it | required | ms | crate |

Eight guards.

## 8. Out of scope

- Interactive mode, REPL, shell completions (v1).
- Text processing: no files of sentences, no batch translation, no tokenization.
  `ruth` works on lemmas and single strings.
- Surface → lemma analysis. `ruth lookup` matches citation forms.
- Any output format beyond human and JSON.
- Being a library. Nothing may depend on this crate.

## 9. Done criteria

- Every subcommand in §2 implemented with `--json`, `--policy` and
  `--show-deviations` where applicable.
- `--help` text for every subcommand, with a worked example.
- The eight guards present, each demonstrated to fail under its witness.
- A transcript in the README showing the four commands that matter most —
  `translit`, `paradigm`, `principal-parts`, `lookup` — with real output, not
  invented output.
- No panic paths; `#![forbid(unsafe_code)]`.

## 10. Closed decisions

- **`paradigm` prints the full table by default**; `principal-parts` is its own
  subcommand. Neither view hides behind a flag.
- **No developer subcommands.** `extract` and `eval` live only in `cargo xtask`.
- **Formatting: always aligned, colour only on a TTY.** Paradigm tables are
  column-aligned unconditionally. Colour is emitted only when stdout is a
  terminal, and **never carries information that is not also in the text** — so
  a piped or redirected run loses styling and nothing else. Origin markers and
  gap indicators are therefore textual first, coloured second.

## 11. Open questions

None. Every question this spec opened is closed above.
