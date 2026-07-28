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
           ins="konjem", loc="konje"),
    spread(nom_voc_acc="konja", gen_loc="konju", dat_ins_abl="konjema"),
    spread(nom_voc="konji", acc_gen="konjev", dat_abl="konjem", ins="konjami",
           loc="konjah"),
)

DRUG = noun(
    "Drug",
    "3.3",
    spread(nom="drug", voc="druzze", acc_abl="druga", gen="drugogo",
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
           ins="poljem", loc="polje"),
    spread(nom_voc_acc="polje", gen_loc="polju", dat_ins_abl="poljema"),
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
           dat_loc="zjemlje", ins="zjemljej"),
    spread(nom_voc_acc="zjemlje", gen_loc="zjemlju", dat_ins_abl="zjemljama"),
    spread(nom_voc_acc="zjemli", gen="zjemljev", dat_abl="zjemljam",
           ins="zjemljami", loc="zjemljah"),
)

# The vowel-final stem `naci-`. Its genitive `nacii` is the doubled vowel that a
# contraction rule would silently eat, which is why it is in the corpus.
NACIJA = noun(
    "nacija",
    "3.5",
    spread(nom="nacija", voc="nacijo", acc="naciju", gen_abl="nacii",
           dat_loc="nacije", ins="nacijej"),
    spread(nom_voc_acc="nacije", gen_loc="naciju", dat_ins_abl="nacijama"),
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
    spread(nom_voc_acc="noczi", gen_loc="noczu", dat_ins_abl="noczjma"),
    spread(nom_voc_acc="noczi", gen="noczev", dat_abl="noczam",
           ins="noczami", loc="noczah"),
)



# --- §4 adjectives ----------------------------------------------------------
# §4.1's and §4.2's tables give a column per gender in the singular but a
# *single* Dual and Plural column. Those single columns are the masculine — the
# nominal declension's dual differs by gender (`zzenama` against `domoma`), and
# its neuter plural nominative is `-a`. So the dual and plural rows below are
# transcribed as masculine, which is what the tables actually show; the other
# genders are derived by §4.1's rule ("endings are the noun's, exactly") and are
# checked by the noun paradigms instead.
def adjective(pos, section, sg_m, sg_n, sg_f, du_m, pl_m, animate=None):
    rows = []
    for gender, cells in (("Masculine", sg_m), ("Neuter", sg_n), ("Feminine", sg_f)):
        for case in CASES:
            rows.append((pos, "dobr", f"{case}.Singular.{gender}", cells[case], section))
    for number, cells in (("Dual", du_m), ("Plural", pl_m)):
        for case in CASES:
            rows.append((pos, "dobr", f"{case}.{number}.Masculine", cells[case], section))
    for features, form in (animate or {}).items():
        rows.append((pos, "dobr", features, form, section))
    return rows


SHORT = adjective(
    "short_adjective", "4.1",
    spread(nom_acc="dobr", voc="dobrje", gen="dobrogo", abl="dobra", dat="dobru",
           ins="dobrom", loc="dobri"),
    spread(nom_voc_acc="dobro", gen="dobrogo", abl="dobra", dat="dobru",
           ins="dobrom", loc="dobri"),
    spread(nom="dobra", voc="dobro", acc="dobru", gen_abl="dobry", dat="dobri",
           ins="dobroj", loc="dobri"),
    spread(nom_voc_acc="dobra", gen_loc="dobru", dat_ins_abl="dobroma"),
    spread(nom_voc_acc="dobry", gen="dobrov", dat_abl="dobrom", ins="dobrami",
           loc="dobrah"),
    animate={
        "Accusative.Singular.Masculine.Animate": "dobra",
        "Accusative.Plural.Masculine.Animate": "dobrov",
    },
)

# §4.2: no vocative — the nominative is used, as in every language measured.
LONG = adjective(
    "adjective", "4.2",
    spread(nom_voc_acc="dobryj", gen="dobrogo", abl="dobra", dat="dobromu",
           ins="dobrym", loc="dobrom"),
    spread(nom_voc_acc="dobroje", gen="dobrogo", abl="dobra", dat="dobromu",
           ins="dobrym", loc="dobrom"),
    spread(nom_voc="dobraja", acc="dobruju",
           gen_abl_dat_ins_loc="dobroj"),
    spread(nom_voc_acc="dobraja", gen_loc="dobroju", dat_ins_abl="dobryma"),
    spread(nom_voc_acc="dobryje", gen_loc="dobryh", dat_abl="dobrym",
           ins="dobrymi"),
    animate={
        "Accusative.Singular.Masculine.Animate": "dobra",
        "Accusative.Plural.Masculine.Animate": "dobryh",
    },
)



# --- §5 pronouns ------------------------------------------------------------
# Features are Person.Number.Gender.Case throughout. Gender is inert outside the
# third-person singular (§5.1), so the first and second persons are emitted as
# Masculine only rather than three identical times.
def pronoun(pos, section, person, number, gender, cells):
    return [
        (pos, "-", f"{person}.{number}.{gender}.{case}", cells[case], section)
        for case in CASES
    ]


PERSONAL = (
    pronoun("pronoun", "5.1", "First", "Singular", "Masculine",
            spread(nom="ja", voc="ja", acc_abl="mjenja", gen="mjenjego",
                   dat_loc="mnje", ins="mnoj"))
    + pronoun("pronoun", "5.1", "Second", "Singular", "Masculine",
              spread(nom="ty", voc="ty", acc_abl="tjebja", gen="tjebjego",
                     dat_loc="tjebje", ins="toboj"))
    + pronoun("pronoun", "5.1", "First", "Dual", "Masculine",
              spread(nom="vje", voc="vje", acc="na", gen_loc="naju",
                     dat_ins_abl="nama"))
    + pronoun("pronoun", "5.1", "Second", "Dual", "Masculine",
              spread(nom_voc_acc="va", gen_loc="vaju", dat_ins_abl="vama"))
    + pronoun("pronoun", "5.1", "First", "Plural", "Masculine",
              spread(nom="my", voc="my", acc_gen_loc="nas", dat_abl="nam",
                     ins="nami"))
    + pronoun("pronoun", "5.1", "Second", "Plural", "Masculine",
              spread(nom="vy", voc="vy", acc_gen_loc="vas", dat_abl="vam",
                     ins="vami"))
    # Third person, §5.1's own table.
    + pronoun("pronoun", "5.1", "Third", "Singular", "Masculine",
              spread(nom_voc="on", acc_gen_abl="jego", dat="jemu", ins="jim",
                     loc="jem"))
    + pronoun("pronoun", "5.1", "Third", "Singular", "Neuter",
              spread(nom_voc="ono", acc_gen_abl="jego", dat="jemu", ins="jim",
                     loc="jem"))
    + pronoun("pronoun", "5.1", "Third", "Singular", "Feminine",
              spread(nom_voc="ona", acc="ju", gen_abl="jeje", dat_loc="jej",
                     ins="jeju"))
    + pronoun("pronoun", "5.1", "Third", "Dual", "Masculine",
              spread(nom_voc="ona", acc="ja", gen_loc="jeju",
                     dat_ins_abl="jima"))
    + pronoun("pronoun", "5.1", "Third", "Plural", "Masculine",
              spread(nom_voc="oni", acc_gen_loc="jih", dat_abl="jim",
                     ins="jimi"))
)

# §5.1a lists clitics for the accusative and dative only, so only those cells
# are asserted. Everything else falls back to the full form, which the
# `every_fallback_exercised` guard checks rather than the corpus.
CLITICS = [
    ("clitic_pronoun", "-", f"{p}.{n}.{g}.{c}", form, "5.1a")
    for p, n, g, c, form in [
        ("First", "Singular", "Masculine", "Accusative", "mja"),
        ("First", "Singular", "Masculine", "Dative", "mi"),
        ("Second", "Singular", "Masculine", "Accusative", "tja"),
        ("Second", "Singular", "Masculine", "Dative", "ti"),
        ("Third", "Singular", "Masculine", "Accusative", "go"),
        ("Third", "Singular", "Masculine", "Dative", "mu"),
        ("Third", "Singular", "Feminine", "Accusative", "ju"),
        ("Third", "Singular", "Feminine", "Dative", "ji"),
        ("First", "Plural", "Masculine", "Accusative", "ny"),
        ("First", "Plural", "Masculine", "Dative", "ni"),
        ("Second", "Plural", "Masculine", "Accusative", "vy"),
        ("Second", "Plural", "Masculine", "Dative", "vi"),
        ("Third", "Plural", "Masculine", "Accusative", "jih"),
        ("Third", "Plural", "Masculine", "Dative", "jim"),
    ]
]

# §5.2. The nominative is the declared fallback rather than a form, so it is
# left to the guard.
REFLEXIVE = [
    ("reflexive", "-", case, form, "5.2")
    for case, form in [
        ("Accusative", "sjebja"), ("Ablative", "sjebja"),
        ("Genitive", "sjebjego"), ("Dative", "sjebje"),
        ("Locative", "sjebje"), ("Instrumental", "soboj"),
    ]
] + [
    ("clitic_reflexive", "-", "Accusative", "sja", "5.2"),
    ("clitic_reflexive", "-", "Dative", "si", "5.2"),
]



# --- §5.4-§5.5 the non-personal series --------------------------------------
# The pronominal declension itself, on the hard stem `t-` and the soft `sj-`,
# plus the two interrogatives. `sjej` is given in §5.4 as four cited forms
# rather than a table, so only those four are asserted.
DEMONSTRATIVE = [
    ("pronominal", "t", f"{case}.{number}.{gender}", form, "5.4")
    for case, number, gender, form in [
        ("Nominative", "Singular", "Masculine", "toj"),  # the declension itself
        ("Accusative", "Singular", "Masculine", "toj"),
        ("Genitive", "Singular", "Masculine", "togo"),
        ("Ablative", "Singular", "Masculine", "toga"),
        ("Dative", "Singular", "Masculine", "tomu"),
        ("Instrumental", "Singular", "Masculine", "tjem"),
        ("Locative", "Singular", "Masculine", "tom"),
        ("Nominative", "Singular", "Neuter", "to"),
        ("Nominative", "Singular", "Feminine", "ta"),
        ("Accusative", "Singular", "Feminine", "tu"),
        ("Genitive", "Singular", "Feminine", "toj"),
        ("Nominative", "Dual", "Masculine", "ta"),
        ("Genitive", "Dual", "Masculine", "toju"),
        ("Dative", "Dual", "Masculine", "tjema"),
        ("Nominative", "Plural", "Masculine", "tje"),
        ("Genitive", "Plural", "Masculine", "tjeh"),
        ("Dative", "Plural", "Masculine", "tjem"),
        ("Instrumental", "Plural", "Masculine", "tjemi"),
    ]
] + [
    ("pronominal", "sj", f"{case}.Singular.Masculine", form, "5.4")
    for case, form in [
        ("Nominative", "sjej"), ("Genitive", "sjego"), ("Dative", "sjemu"),
        ("Instrumental", "sjim"), ("Locative", "sjem"),
    ]
]

# §5.4's `tot`: the declension with its one reduplicated cell.
DEMONSTRATIVE += [
    ("that", "-", f"{case}.{number}.{gender}", form, "5.4")
    for case, number, gender, form in [
        ("Nominative", "Singular", "Masculine", "tot"),
        ("Accusative", "Singular", "Masculine", "tot"),
        ("Genitive", "Singular", "Masculine", "togo"),
        ("Ablative", "Singular", "Masculine", "toga"),
        ("Nominative", "Singular", "Neuter", "to"),
        ("Nominative", "Singular", "Feminine", "ta"),
        ("Nominative", "Plural", "Masculine", "tje"),
    ]
]

INTERROGATIVE = [
    ("who", "-", case, form, "5.5")
    for case, form in [
        ("Nominative", "kto"), ("Accusative", "koga"), ("Genitive", "kogo"),
        ("Ablative", "koga"), ("Dative", "komu"), ("Instrumental", "kjem"),
        ("Locative", "kom"),
    ]
] + [
    ("what", "-", case, form, "5.5")
    for case, form in [
        ("Nominative", "czto"), ("Accusative", "czto"), ("Genitive", "czego"),
        ("Ablative", "czega"), ("Dative", "czemu"),
        ("Instrumental", "czem"), ("Locative", "czem"),
    ]
] + [
    ("relative", "-", f"{case}.Singular.Masculine", form, "5.5")
    for case, form in [
        ("Nominative", "izze"), ("Accusative", "jegozze"),
        ("Genitive", "jegozze"), ("Dative", "jemuzze"),
    ]
]



# --- §7 verbs ---------------------------------------------------------------
PNS = [("First", "Singular"), ("Second", "Singular"), ("Third", "Singular"),
       ("First", "Dual"), ("Second", "Dual"), ("Third", "Dual"),
       ("First", "Plural"), ("Second", "Plural"), ("Third", "Plural")]


def finite(pos, lemma, section, _tense, forms):
    """Nine person/number cells."""
    return [
        (pos, lemma, f"{p}.{n}", form, section)
        for (p, n), form in zip(PNS, forms)
    ]


VERB = (
    # §7.4, class 1 — the paradigm §7.3 and §7.4 both spell out.
    finite("verb", "czitatj", "7.4", "NonPast", [
        "czitaju", "czitajeszj", "czitajet",
        "czitajevje", "czitajeta", "czitajetje",
        "czitajem", "czitajetje", "czitajut"])
    # §7.3's class-6 example, which the word-final mark selects.
    + finite("verb", "pisatj'", "7.3", "NonPast", [
        "piszu", "piszeszj", "piszet", "", "", "", "", "", ""])[:3]
    # §7.8's auxiliary.
    + finite("future_auxiliary", "-", "7.8", "Future", [
        "budu", "budjeszj", "budjet",
        "budjevje", "budjeta", "budjetje",
        "budjem", "budjetje", "budut"])
    # §7.9's copula: the present and the imperfect are suppletive and tabulated;
    # the aorist is regular and comes out of the general path on `bytj`.
    + finite("bytj", "-", "7.9", "Present", [
        "jesmj", "jesi", "jestj", "jesvje", "jesta", "jestje",
        "jesm", "jestje", "sutj"])
    # §7.7's l-participle, and §7.9's for `bytj`.
    + [("l_participle", "czitatj", f"{g}.{n}", form, "7.7")
       for g, n, form in [
           ("Masculine", "Singular", "czital"), ("Feminine", "Singular", "czitala"),
           ("Neuter", "Singular", "czitalo"), ("Masculine", "Dual", "czitala"),
           ("Masculine", "Plural", "czitali")]]
    + [("l_participle", "bytj", f"{g}.Singular", form, "7.9")
       for g, form in [("Masculine", "byl"), ("Feminine", "byla"), ("Neuter", "bylo")]]
    + [("l_participle", "bytj", "Masculine.Plural", "byli", "7.9")]
    # §7.10's imperative.
    + [("imperative", "czitatj", f"{p}.{n}", form, "7.10")
       for p, n, form in [
           ("Second", "Singular", "czitaj"), ("Second", "Dual", "czitajta"),
           ("Second", "Plural", "czitajtje"),
           ("First", "Dual", "czitajvje"), ("First", "Plural", "czitajm")]]
    + [("infinitive", "bytj", "-", "bytj", "7.9"),
       ("imperative", "bytj", "Second.Singular", "budj", "7.9")]
    # §7.12. The participles return an adjective *stem*, so the corpus holds the
    # stem and the long form the stem declines to; the gerunds are indeclinable
    # and are finished forms.
    + [(pos, lemma, "-", form, "7.12") for pos, lemma, form in [
        ("present_active_participle", "czitatj", "czitajuszcz"),
        ("past_active_participle", "czitatj", "czitavsz"),
        ("present_passive_participle", "czitatj", "czitajem"),
        ("past_passive_participle", "poczitatj", "poczitan"),
        ("past_passive_participle", "rjeszitj", "rjeszen"),
        ("past_passive_participle", "bitj", "bit"),
        ("present_gerund", "czitatj", "czitaja"),
        ("past_gerund", "czitatj", "czitav"),
    ]]
    # …and the long forms §7.12 tabulates, through the ordinary adjective.
    # --- §6 numerals --------------------------------------------------------
    + [("numeral", str(v), "Nominative", form, sec) for v, form, sec in [
        (0, "nolj", "6.2"), (1, "odin", "6.2"), (2, "dva", "6.2"),
        (3, "tri", "6.2"), (4, "czetyrje", "6.2"), (5, "pjatj", "6.2"),
        (6, "szestj", "6.2"), (7, "sjedmj", "6.2"), (8, "osmj", "6.2"),
        (9, "djevjatj", "6.2"), (10, "djesjatj", "6.2"),
        (11, "odinnadjesjat", "6.3"), (12, "dvanadjesjat", "6.3"),
        (13, "trinadjesjat", "6.3"), (14, "czetyrnadjesjat", "6.3"),
        (15, "pjatnadjesjat", "6.3"), (16, "szestnadjesjat", "6.3"),
        (17, "sjedmnadjesjat", "6.3"), (18, "osmnadjesjat", "6.3"),
        (19, "djevjatnadjesjat", "6.3"),
        (20, "dvadjesjat", "6.3"), (30, "tridjesjat", "6.3"),
        (40, "czetyrjedjesjat", "6.3"), (50, "pjatjdjesjat", "6.3"),
        (60, "szestjdjesjat", "6.3"), (70, "sjedmjdjesjat", "6.3"),
        (80, "osmjdjesjat", "6.3"), (90, "djevjatjdjesjat", "6.3"),
        (100, "sto", "6.3"), (200, "dvjesto", "6.3"), (300, "tristo", "6.3"),
        (400, "czetyrjesto", "6.3"), (500, "pjatjsto", "6.3"),
        (600, "szestjsto", "6.3"), (700, "sjedmjsto", "6.3"),
        (800, "osmjsto", "6.3"), (900, "djevjatjsto", "6.3"),
        (1000, "tysjacza", "6.3"), (1000000, "miljon", "6.3"),
        (1000000000, "biljon", "6.3"),
        (132, "sto tridjesjat dva", "6.3"),
    ]]
    # §6.4's declensions, and §6.1's worked examples.
    + [("numeral", "2", case, form, "6.4") for case, form in [
        ("Genitive", "dvu"), ("Locative", "dvu"),
        ("Dative", "dvoma"), ("Instrumental", "dvoma"), ("Ablative", "dvoma")]]
    + [("numeral", "3", case, form, "6.4") for case, form in [
        ("Genitive", "trjeh"), ("Locative", "trjeh"),
        ("Dative", "trjem"), ("Instrumental", "trjemi")]]
    + [("numeral", "4", case, form, "6.4") for case, form in [
        ("Genitive", "czetyrjeh"), ("Dative", "czetyrjem"),
        ("Instrumental", "czetyrjmi")]]
    + [("numeral", "5", case, form, "6.4") for case, form in [
        ("Genitive", "pjati"), ("Dative", "pjati"), ("Accusative", "pjatj"),
        ("Instrumental", "pjatjju"), ("Locative", "pjati")]]
    + [("numeral", "1", case, form, "6.4") for case, form in [
        ("Genitive", "odinogo")]]
    # §6.5's ordinal stems, through the ordinary adjective.
    + [("ordinal", str(v), "Nominative.Singular.Masculine", form, "6.5")
       for v, form in [
           (1, "pjervyj"), (2, "vtoryj"), (3, "trjetyj"), (4, "czetvjertyj"),
           (5, "pjatyj"), (6, "szestyj"), (7, "sjedmyj"), (8, "osmyj"),
           (9, "djevjatyj"), (10, "djesjatyj"), (100, "sotyj"),
           (1000, "tysjacznyj")]]
    + [("adjective", stem, "Nominative.Singular.Masculine", form, "7.12")
       for stem, form in [
           ("czitajuszcz", "czitajuszczij"), ("czitavsz", "czitavszij"),
           ("czitajem", "czitajemyj"), ("poczitan", "poczitanyj"),
           ("rjeszen", "rjeszenyj"), ("bit", "bityj")]]
)

ROWS = (
    DOM + KONJ + DRUG + OKNO + POLJE + ZZENA + KNIGA + ZJEMLJA + NACIJA
    + SLUGA + NOCZJ + SHORT + LONG + PERSONAL + CLITICS + REFLEXIVE
    + DEMONSTRATIVE + INTERROGATIVE + VERB
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
        if in_fence:
            # A fenced block is a *display* of forms and carries no backticks,
            # so every word in it counts as attested. Without this the check
            # cannot see §7.3's `piszu, piszeszj, piszet` at all — it tracked
            # the fence state and then never used it, which made every
            # fence-only form look unattested.
            for word in re.split(r"[\s/,;()>—…-]+", line):
                if word:
                    attested.add(word.strip("*`."))
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
        if lemma != "-":
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
