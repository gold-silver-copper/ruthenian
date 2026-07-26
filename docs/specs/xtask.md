# Spec: `xtask`

Phase 8, or grown incrementally alongside the phases it serves. Depends on
`ruthenian-extract`, `ruthenian-eval`.

## 1. Purpose

The workspace task runner: the three commands that regenerate, check and measure.
It is glue. It owns nothing, decides nothing, and contains no logic that belongs
in the crate it invokes.

The reason it exists as a crate rather than a shell script is that these commands
must run identically on a developer's machine and in CI, on every platform, with
the same argument handling and the same exit codes.

Wrong to put here: anything a human would want to call as a library, anything
with an invariant of its own, and — especially — any behaviour that the crate
being invoked should have had.

## 2. Public surface

```text
cargo xtask refresh-data --dump <path>   # extract → artifacts → generated tables
cargo xtask check-registry               # dump-free structural gate on committed tables
cargo xtask conformance                  # spec-driven measurement → eval/summary.json
```

**`refresh-data`** runs the extractor in release mode, writes the lexicon,
attested-forms and PHF artifacts, and prints the reject histogram and the
compression ratio. It is the only sanctioned way to change anything in
`crates/ruthenian/generated/`.

**`check-registry`** is the CI gate that does **not** need the 22 GiB dump, which
matters because CI will not have it. It verifies what can be verified from the
committed artifacts alone:

- every table key is well-formed and unique;
- every row has the right arity and no empty column;
- **no row merely duplicates the rule engine** — the layering check that catches
  "someone changed a rule and did not regenerate";
- the schema version and dump fingerprint are present and consistent across
  artifacts.

It cannot verify that a row's *value* is correct — that is the conformance run's
job against the spec, not
derivable without the dump. Say so in the command's own help text, the way
`english`'s xtask does, so nobody mistakes a green `check-registry` for a
correctness guarantee.

**`conformance`** runs the evaluator against the corpus extracted from
`docs/RUTHENIAN.md` and writes `eval/summary.json`, then regenerates the numbers
in the README from it. It also re-extracts the corpus and fails if the committed
artifact has drifted from the specification.
Running it is how a change acquires a number instead of an anecdote.

## 2a. Inputs and outputs

In: the dump path (`refresh-data`), the spec and the committed artifacts
(`conformance`), the committed artifacts
(`check-registry`), and nothing else — no config file, no network.

Out: `refresh-data` writes `crates/ruthenian/generated/` and the lexicon
artifacts; `conformance` writes `eval/summary.json` and the numbers it
regenerates in the README; `check-registry` writes nothing and
communicates entirely through its exit code and a report on stdout.

## 3. Data owned

Nothing.

## 4. Dependencies allowed

`ruthenian-extract`, `ruthenian-eval`, and a process runner. No morphology, no
orthography, no direct dump parsing.

## 5. Invariants

1. Every command is a delegation. If a behaviour is worth testing, it is in the
   crate being invoked, not here.
2. `check-registry` requires no dump and no network.
3. `refresh-data` is the only writer of `crates/ruthenian/generated/`.
4. Commands are deterministic and idempotent: running twice with no input change
   produces no diff.
5. Non-zero exit on any failure. No command reports success on a partial result.

## 6. Guards

| Name | Invariant | Failure witness | Status | Cost | Owner |
|---|---|---|---|---|---|
| `check_registry_offline` | Inv. 2 | Make it read the dump path; CI without the dump fails | required | seconds | crate |
| `registry_catches_stale_tables` | §2 layering | Change a `ruthenian-core` ending without regenerating; a table row becomes redundant and the gate fails | required | seconds | crate |
| `generated_single_writer` | Inv. 3 | Write to `generated/` from any other command or test | required | ms | workspace |
| `idempotent_refresh` | Inv. 4 | Introduce nondeterministic ordering in the extractor; the second run diffs | **scheduled** (two full passes) | minutes | crate |
| `xtask_has_no_logic` | Inv. 1 | Implement a transformation here instead of in `extract`/`eval`; the size/dependency check flags it | required | ms | crate |

Five guards. `idempotent_refresh` is scheduled rather than per-PR because it
costs two full extraction passes; it is marked as such rather than quietly
skipped.

## 7. Out of scope

- Being a build system. Cargo is the build system.
- CI configuration itself — the workflow calls these commands; it does not
  duplicate them.
- Publishing, tagging, or releasing. Those are deliberate human acts.
- Any command that only one person's workflow needs.

## 8. Done criteria

- Three commands implemented, each with help text stating what it does *not*
  verify.
- `check-registry` runs green in CI with no dump present.
- The five guards present, each demonstrated to fail under its witness.
- The README documents the regenerate → check → measure loop as the standard
  workflow for any rule change.

## 9. Closed decisions

- **`conformance` fails on a net-negative paired diff** — more slots broken than
  fixed blocks the change; everything else is reported without gating. A gate
  that always succeeds is telemetry wearing a gate's name, and the ecosystem has
  a documented example of exactly that. The comparison is against the committed
  baseline summary, and both stress variants are checked, so a segmental
  improvement cannot mask a stress regression.
- **`xtask` owns the developer workflows outright.** `ruth` does not mirror
  `extract` or `eval`, so `cargo xtask` is the single way to regenerate or
  measure.

- **`xtask fmt-docs` is added only if needed.** Generated docs
  (the README numbers) come out of the conformance run. A
  separate command is justified only once that run is slow enough that people
  start skipping it — the failure mode being a generated doc that drifts because
  regenerating it was inconvenient. Until then: one command, one path.

## 10. Open questions

None. Every question this spec opened is closed above.
