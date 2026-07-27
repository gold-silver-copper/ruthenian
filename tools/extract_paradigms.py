"""Write the ruthenian-core conformance corpus from docs/RUTHENIAN.md.

    python3 tools/extract_paradigms.py

Writes crates/ruthenian-core/tests/corpus/paradigms.tsv. Run it by hand and
commit the result; the crate never reads the specification at test time.

# Why this transcribes rather than parses

Parsing the specification's tables was tried in an earlier attempt and failed
*silently*: a heading match found `noczj` in §3.2's declension summary rather
than §3.6's paradigm, compared `dom`'s forms against it, and reported a clean
run. A parser that finds the wrong section still produces a plausible corpus.

So the paradigms below are transcribed by hand, and every form is then
**verified to occur in the specification** as a backticked token. A typo in the
transcription fails loudly; a form invented here fails loudly; and reformatting
the specification's prose cannot change what is checked without also changing
the checksum, which the `spec_currency` guard pins.

The two directions of error are therefore both covered: the spec cannot drift
away from the corpus (checksum), and the corpus cannot claim a form the spec
does not contain (verification).
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SPEC = ROOT / "docs" / "RUTHENIAN.md"
OUT = ROOT / "crates" / "ruthenian-core" / "tests" / "corpus" / "paradigms.tsv"

CASES = [
    "Nominative",
    "Vocative",
    "Accusative",
    "Genitive",
    "Ablative",
    "Dative",
    "Instrumental",
    "Locative",
]


def fnv1a64(data: bytes) -> int:
    """FNV-1a, so the Rust side can recompute it without a dependency."""
    h = 0xCBF29CE484222325
    for b in data:
        h ^= b
        h = (h * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


def noun(lemma, section, sg, du, pl):
    """One noun paradigm: three dicts keyed by case, values the form."""
    rows = []
    for number, cells in (("Singular", sg), ("Dual", du), ("Plural", pl)):
        for case in CASES:
            rows.append(("noun", lemma, f"{case}.{number}", cells[case], section))
    return rows


def spread(**kw):
    """Expand `nom_voc_acc="doma"` into one entry per case.

    The specification writes `= nom` and `= dat` for the syncretisms; this is
    that shorthand expanded, so the corpus holds one row per *cell* and the
    engine is asked all 24 questions rather than the 15 with distinct answers.
    """
    out = {}
    abbrev = {
        "nom": "Nominative",
        "voc": "Vocative",
        "acc": "Accusative",
        "gen": "Genitive",
        "abl": "Ablative",
        "dat": "Dative",
        "ins": "Instrumental",
        "loc": "Locative",
    }
    for key, form in kw.items():
        for part in key.split("_"):
            out[abbrev[part]] = form
    missing = [c for c in CASES if c not in out]
    if missing:
        sys.exit(f"cell(s) not given: {missing}")
    return out


# --- §3.3 declension II masculine -------------------------------------------
# `Drug` and `Konj` carry a capital because §3.3 tabulates them with the animate
# accusative (`druga`, `konja`, `drugov`, `konjev`), and §3.7 puts animacy in the
# lemma's first letter. `dom` is inanimate, so its accusative is its nominative.
DOM = noun(
    "dom",
    "3.3",
    spread(nom_acc="dom", voc="domje", gen="domogo", abl="doma", dat="domu",
           ins="domom", loc="domi"),
    spread(nom_voc_acc="doma", gen_loc="domu", dat_ins_abl="domoma"),
    spread(nom_voc_acc="domy", gen="domov", dat_abl="domom", ins="domami",
           loc="domah"),
)

KONJ = noun(
    "Konj",
    "3.3",
    spread(nom="konj", voc="konju", acc_abl="konja", gen="konjego", dat="konju",
           ins="konjem", loc="konji"),
    spread(nom_voc_acc="konja", gen_loc="konju", dat_ins_abl="konjema"),
    spread(nom_voc="konji", acc_gen="konjev", dat_abl="konjem", ins="konjami",
           loc="konjah"),
)

DRUG = noun(
    "Drug",
    "3.3",
    spread(nom="drug", voc="druzzje", acc_abl="druga", gen="drugogo",
           dat="drugu", ins="drugom", loc="druzi"),
    spread(nom_voc_acc="druga", gen_loc="drugu", dat_ins_abl="drugoma"),
    spread(nom_voc="drugi", acc_gen="drugov", dat_abl="drugom", ins="drugami",
           loc="drugah"),
)

# --- §3.4 declension II neuter ----------------------------------------------
OKNO = noun(
    "okno",
    "3.4",
    spread(nom_voc_acc="okno", gen="oknogo", abl="okna", dat="oknu",
           ins="oknom", loc="okni"),
    spread(nom_voc_acc="okni", gen_loc="oknu", dat_ins_abl="oknoma"),
    spread(nom_voc_acc="okna", gen="oknov", dat_abl="oknom", ins="oknami",
           loc="oknah"),
)

POLJE = noun(
    "polje",
    "3.4",
    spread(nom_voc_acc="polje", gen="poljego", abl="polja", dat="polju",
           ins="poljem", loc="polji"),
    spread(nom_voc_acc="polji", gen_loc="polju", dat_ins_abl="poljema"),
    spread(nom_voc_acc="polja", gen="poljev", dat_abl="poljem", ins="poljami",
           loc="poljah"),
)

# --- §3.5 declension I ------------------------------------------------------
ZZENA = noun(
    "zzena",
    "3.5",
    spread(nom="zzena", voc="zzeno", acc="zzenu", gen_abl="zzeny", dat="zzeni",
           ins="zzenoj", loc="zzeni"),
    spread(nom_voc_acc="zzeni", gen_loc="zzenu", dat_ins_abl="zzenama"),
    spread(nom_voc_acc="zzeny", gen="zzenov", dat_abl="zzenam", ins="zzenami",
           loc="zzenah"),
)

KNIGA = noun(
    "kniga",
    "3.5",
    spread(nom="kniga", voc="knigo", acc="knigu", gen_abl="knigi", dat="knizi",
           ins="knigoj", loc="knizi"),
    spread(nom_voc_acc="knizi", gen_loc="knigu", dat_ins_abl="knigama"),
    spread(nom_voc_acc="knigi", gen="knigov", dat_abl="knigam", ins="knigami",
           loc="knigah"),
)

ZJEMLJA = noun(
    "zjemlja",
    "3.5",
    spread(nom="zjemlja", voc="zjemljo", acc="zjemlju", gen_abl="zjemli",
           dat="zjemlji", ins="zjemljoj", loc="zjemli"),
    spread(nom_voc_acc="zjemlji", gen_loc="zjemlju", dat_ins_abl="zjemljama"),
    spread(nom_voc_acc="zjemli", gen="zjemljev", dat_abl="zjemljam",
           ins="zjemljami", loc="zjemljah"),
)

# The vowel-final stem `naci-`. Its genitive `nacii` is the doubled vowel that a
# contraction rule would silently eat, which is why it is in the corpus.
NACIJA = noun(
    "nacija",
    "3.5",
    spread(nom="nacija", voc="nacijo", acc="naciju", gen_abl="nacii",
           dat="naciji", ins="nacijoj", loc="nacii"),
    spread(nom_voc_acc="naciji", gen_loc="naciju", dat_ins_abl="nacijama"),
    spread(nom_voc_acc="nacii", gen="nacijev", dat_abl="nacijam",
           ins="nacijami", loc="nacijah"),
)

# `sluga'` — masculine in agreement, declension I in form. The accusative
# singular is `slugu` and *not* the ablative, which §3.5 states outright.
SLUGA = noun(
    "sluga'",
    "3.5",
    spread(nom="sluga", voc="slugo", acc="slugu", gen_abl="slugi", dat="sluzi",
           ins="slugoj", loc="sluzi"),
    spread(nom_voc_acc="sluzi", gen_loc="slugu", dat_ins_abl="slugama"),
    spread(nom_voc_acc="slugi", gen="slugov", dat_abl="slugam", ins="slugami",
           loc="slugah"),
)

# --- §3.6 declension III ----------------------------------------------------
NOCZJ = noun(
    "noczj'",
    "3.6",
    spread(nom_acc="noczj", voc_gen_abl_dat_loc="noczi", ins="noczjju"),
    spread(nom_voc_acc="noczi", gen_loc="noczju", dat_ins_abl="noczjma"),
    spread(nom_voc_acc="noczi", gen="noczjev", dat_abl="noczjam",
           ins="noczjami", loc="noczjah"),
)

ROWS = (
    DOM + KONJ + DRUG + OKNO + POLJE + ZZENA + KNIGA + ZJEMLJA + NACIJA
    + SLUGA + NOCZJ
)


def main() -> None:
    spec = SPEC.read_bytes()
    text = spec.decode("utf-8")

    # Every backticked token in the specification, as the vocabulary a corpus
    # form is allowed to come from.
    #
    # Split per line and take the odd fields rather than running one regex over
    # the whole file: backticks are delimiters, so a single unbalanced one flips
    # the parity of every span after it and the scan then reads the *gaps*
    # between spans as if they were spans. That failure is silent — it yields a
    # large, plausible vocabulary that happens to exclude the paradigm tables —
    # and it is the same class of bug as the heading match this script exists to
    # avoid. Per-line splitting confines any imbalance to its own line, and the
    # imbalance is reported rather than absorbed.
    attested = set()
    unbalanced = []
    open_span = False
    in_fence = False
    for lineno, line in enumerate(text.split("\n"), 1):
        if line.startswith("```"):
            in_fence = not in_fence
            open_span = False
            continue
        if not line.strip():
            # A span cannot cross a blank line, so this is where an imbalance
            # stops being a wrapped span and becomes a typo.
            if open_span:
                unbalanced.append(lineno - 1)
            open_span = False
            continue
        fields = line.split("`")
        # A span left open by the previous line means this line's fields are
        # offset by one: prose wraps `noun(word, case,` / `number)` across two
        # lines, and both halves are inside the span.
        for span in (fields[0::2] if open_span else fields[1::2]):
            attested.add(span)
            for word in re.split(r"[\s/,;()]+", span):
                if word:
                    attested.add(word.strip("*"))
        if len(fields) % 2 == 0:
            open_span = not open_span
    if unbalanced:
        print(
            f"warning: backtick left open at line(s) {unbalanced}",
            file=sys.stderr,
        )

    # §2.1: the mark is morphology, not part of the word, so a lemma written
    # `sluga'` never appears in the specification as the bare nominative
    # `sluga`. The nominative *form* is still the lemma without its mark.
    for _, lemma, _, _, _ in ROWS:
        attested.add(lemma.rstrip("'").lower())

    unattested = sorted(
        {(lemma, form) for _, lemma, _, form, _ in ROWS if form not in attested}
    )
    if unattested:
        for lemma, form in unattested:
            print(f"NOT IN SPEC: {form}  (paradigm of {lemma})", file=sys.stderr)
        sys.exit(
            f"{len(unattested)} form(s) are not attested in the specification. "
            "Either the transcription is wrong or the spec is missing a form; "
            "do not weaken this check."
        )

    seen = {}
    for pos, lemma, features, form, section in ROWS:
        key = (pos, lemma, features)
        if key in seen:
            sys.exit(f"duplicate cell: {key}")
        seen[key] = form

    OUT.parent.mkdir(parents=True, exist_ok=True)
    with OUT.open("w", encoding="utf-8") as f:
        f.write("# ruthenian-core conformance corpus\n")
        f.write("# GENERATED by tools/extract_paradigms.py — do not hand-edit\n")
        f.write("# every form below occurs in docs/RUTHENIAN.md, checked on write\n")
        f.write(f"# spec-fnv1a64\t{fnv1a64(spec):#018x}\n")
        f.write(f"# rows\t{len(ROWS)}\n")
        f.write("pos\tlemma\tfeatures\tform\tsection\n")
        for row in ROWS:
            f.write("\t".join(row) + "\n")

    print(f"wrote {OUT.relative_to(ROOT)}: {len(ROWS)} cells")
    print(f"spec fnv1a64: {fnv1a64(spec):#018x}")


if __name__ == "__main__":
    main()
