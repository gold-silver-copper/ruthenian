//! §3.8's automatic spelling adjustments and §2.4's three palatalizations.
//!
//! **One module, used by every part of speech.** A second copy of a seam rule
//! means it is in the wrong place.
//!
//! These are what let Ruthenian have three declensions where Russian has eight:
//! the velar, sibilant, `c` and vowel stem-classes are not separate paradigms
//! but the same endings under the rules below (§3.2, §3.8).
//!
//! Nothing here reads or places stress. §2.1 makes stress real, lexical and
//! **unwritten**, so no rule may condition on it — see §3.8's note on why rule 2
//! dropped its stress clause.

/// `k g h` — the velar stems.
pub const VELARS: &[&str] = &["k", "g", "h"];

/// `ж ш ч щ` in Ruthenian — the hushing consonants.
pub const SIBILANTS: &[&str] = &["zz", "sz", "cz", "szcz"];

/// The labials that take an epenthetic `l` in the present stem (§7.11).
pub const LABIALS: &[&str] = &["p", "b", "v", "m"];

/// The present-stem mutations of §7.11.
///
/// **Additive: the consonant stays and the palatal reflex is added after it.**
/// The labial rules were always additive — `p` → `plj` keeps the `p` — and this
/// is that one operation applied to every stem-final consonant. It keeps the
/// root legible (`vidzzu` shows `vid-`, where Russian's `вижу` does not), and it
/// separates two homograph pairs the replacive version created: `voditj` and
/// `vozitj` both gave `vozzu`, and `letjetj` "fly" collided with `leczitj` "heal"
/// at `leczu`.
///
/// Applied **by class**, never by stem shape: a class-1 verb with a
/// labial-final stem takes no mutation at all, verified across 1 977 Russian
/// verbs without exception (`COMPARATIVE_GRAMMAR.md`, Method). A rule keyed on
/// "ends in a labial" would corrupt all 1 977, which is why
/// [`mutate_present_stem`] is only ever called for the classes that mutate.
///
/// `z` → `z'zz` carries the separator because `zzz` reads as `zz` + `z` rather
/// than `z` + `zz`. It is the only output that needs one.
///
/// `ov` → `u` is class 2's stem formation rather than iotation (§7.3), and it
/// replaces rather than adds. It is listed first because `njegodov` ends in `v`
/// and would otherwise take the labial rule, coming out `njegodovlj`.
///
/// §7.11's `st` → `szcz` and `sk` → `szcz` are **gone**: additively the general
/// rule applies to the cluster's last consonant and the `s` is left alone, so
/// `krjest` → `krjestcz` needs no rule of its own.
pub const MUTATIONS: &[(&str, &str)] = &[
    ("ov", "u"),
    ("t", "tcz"),
    ("d", "dzz"),
    ("s", "ssz"),
    ("z", "z'zz"),
    ("k", "kcz"),
    ("g", "gzz"),
    ("h", "hsz"),
    ("p", "plj"),
    ("b", "blj"),
    ("v", "vlj"),
    ("m", "mlj"),
];

/// Which palatalization an ending triggers (§2.4).
///
/// Ruthenian keeps all three. Russian levelled the second away entirely (0 %);
/// Ukrainian keeps it at 99 % and OCS at 66 %. It is not decoration: it is what
/// distinguishes the locative `druzi` from the vocative `druzzje` in the
/// consonant, while `-i` against `-je` distinguishes them in the vowel. A
/// morphophonology ported from a Russian implementation will not have it, and
/// every velar stem is then silently wrong in two cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Palatal {
    /// No change.
    None,
    /// Before front vowels and `j` — the vocative `-je`, the present stem, the
    /// comparative. `k`→`cz`, `g`→`zz`, `h`→`sz`, and **`c`→`cz`**.
    First,
    /// Before yat-derived `-i` — the locative singular, the feminine dative
    /// singular, the neuter and feminine dual. `k`→`c`, `g`→`z`, `h`→`s`.
    Second,
}

/// Apply a palatalization to a stem's final consonant. A stem with no eligible
/// consonant is returned unchanged, which is why this is safe to call
/// unconditionally.
///
/// ```
/// use ruthenian_core::spelling::{palatalize, Palatal};
/// // first: the vocative `drug` -> `druzzje`
/// assert_eq!(palatalize("drug", Palatal::First), "druzz");
/// // first also applies to `c`, which governs the whole -jec class (§2.4):
/// // `otjec` -> `otjecze`
/// assert_eq!(palatalize("otjec", Palatal::First), "otjecz");
/// // second: the locative `drug` -> `druzi`, and `kniga` -> `knizi`
/// assert_eq!(palatalize("drug", Palatal::Second), "druz");
/// assert_eq!(palatalize("knig", Palatal::Second), "kniz");
/// // the two are distinct, which is the whole point
/// assert_ne!(
///     palatalize("drug", Palatal::First),
///     palatalize("drug", Palatal::Second)
/// );
/// // non-velar stems are untouched
/// assert_eq!(palatalize("zzen", Palatal::Second), "zzen");
/// ```
pub fn palatalize(stem: &str, which: Palatal) -> String {
    let table: &[(&str, &str)] = match which {
        Palatal::None => return stem.to_string(),
        // `c` is itself the output of an earlier palatalization and reverts
        // before a front vowel, exactly as OCS `otьcь` -> `otьče` (§2.4).
        Palatal::First => &[("k", "cz"), ("g", "zz"), ("h", "sz"), ("c", "cz")],
        // No `dz`: OCS had it as the second-palatalization output of `g`, and
        // Ruthenian gives `z` instead, following East Slavic (§2.4).
        Palatal::Second => &[("k", "c"), ("g", "z"), ("h", "s")],
    };
    for (from, to) in table {
        if ends_with_letter(stem, from) {
            let cut = stem.len() - from.len();
            return format!("{}{}", &stem[..cut], to);
        }
    }
    stem.to_string()
}

/// Apply the present-stem mutation of §7.11 to a stem.
///
/// Only call this for classes that mutate (§7.3: class 4 in the 1sg, class 6
/// throughout). Keying on the stem's final consonant alone is the error this
/// crate must not make.
///
/// ```
/// use ruthenian_core::spelling::mutate_present_stem;
/// // Additive: the root stays visible.
/// assert_eq!(mutate_present_stem("vid"), "vidzz");   // vidjetj -> vidzzu
/// assert_eq!(mutate_present_stem("let"), "letcz");   // letjetj -> letczu
/// assert_eq!(mutate_present_stem("pis"), "pissz");   // pisatj' -> pisszu
/// assert_eq!(mutate_present_stem("ljub"), "ljublj"); // ljubitj -> ljublju
///
/// // The two homographs the replacive version created are now distinct.
/// assert_ne!(mutate_present_stem("vod"), mutate_present_stem("voz"));
/// assert_eq!(mutate_present_stem("voz"), "voz'zz");  // the separator: zzz
/// assert_eq!(mutate_present_stem("lecz"), "lecz");   // leczitj -> leczu
///
/// // Class 2's stem formation replaces rather than adds.
/// assert_eq!(mutate_present_stem("njegodov"), "njegodu");
///
/// // An already-mutated stem is not mutated twice.
/// assert_eq!(mutate_present_stem("vidzz"), "vidzz");
/// assert_eq!(mutate_present_stem("pissz"), "pissz");
/// ```
pub fn mutate_present_stem(stem: &str) -> String {
    for (from, to) in MUTATIONS {
        if ends_with_letter(stem, from) {
            let cut = stem.len() - from.len();
            return format!("{}{}", &stem[..cut], to);
        }
    }
    stem.to_string()
}

/// True when the stem ends in a velar (`k g h`).
fn ends_velar(stem: &str) -> bool {
    VELARS.iter().any(|v| ends_with_letter(stem, v))
}

/// True when the stem ends in a hushing consonant (`zz sz cz szcz`).
fn ends_sibilant(stem: &str) -> bool {
    SIBILANTS.iter().any(|v| ends_with_letter(stem, v))
}

/// True when the stem ends in `c` — the `ц` stems, and not `cz`.
fn ends_ts(stem: &str) -> bool {
    ends_with_letter(stem, "c")
}

/// `ends_with` that does not mistake the tail of a digraph for a letter: `sz`
/// ends in the letter `sz`, not in the letter `z`.
///
/// This is the single most bug-prone operation in the crate, because every
/// Ruthenian hushing consonant is a digraph whose second character is also a
/// letter in its own right.
fn ends_with_letter(stem: &str, letter: &str) -> bool {
    if !stem.ends_with(letter) {
        return false;
    }
    let before = &stem[..stem.len() - letter.len()];
    // `z` is the tail of *three* digraphs — `zz`, `sz` and `cz` — and `cz` is the
    // one that is easy to forget, because `zz` and `sz` come to mind as a pair.
    // Omitting it makes `lecz` (from `leczitj` "heal") match the `z` mutation and
    // come out `lecz'zz`. `szcz` is covered by the same `c` test.
    //
    // The other letters need no exclusion: no Ruthenian digraph ends in `t`, `d`,
    // `s`, `h`, `k`, `g` or a labial, except `sz` for `s`.
    match letter {
        "z" => !(before.ends_with('z') || before.ends_with('s') || before.ends_with('c')),
        "s" => !stem.ends_with("sz"),
        "c" => !stem.ends_with("cz"),
        _ => true,
    }
}

/// The spelling adjustments of §3.8 that apply at a stem/ending seam.
///
/// Three rules, in order:
///
/// 1. after `k g h` and `zz sz cz szcz`, `y` is written `i` (`knigi`, not
///    `*knigy`);
/// 2. after `zz sz cz szcz c`, an ending's `o` is written `je`;
/// 3. (rule 2a) the vocative `-je` is written `-e` after `cz` and `szcz` — §2.2
///    gives these two no hard/soft distinction, so the glide has nothing to
///    mark. `zz` and `sz` do have soft values and keep it.
///
/// Rule 2a is doubly narrow, and both bounds are load-bearing. It stops at the
/// line §2.2 draws, which is what keeps `otjecze` and `druzzje` both right — a
/// wider consonant set makes the second `druzze`, contradicting §3.3, §3.8 and
/// §3.1. And it applies to the vocative ending alone, because everywhere else a
/// leading `j` is rule 3's soft sign rather than a glide: a wider ending set
/// makes §3.6's `noczjju` into `noczju` and its `noczjev` into `noczev`.
///
/// Rule 2 no longer conditions on stress, because §2.1 never writes it — see
/// §3.8's note.
///
/// ```
/// use ruthenian_core::spelling::spell_ending;
/// // rule 1
/// assert_eq!(spell_ending("knig", "y"), "i");        // knigi
/// assert_eq!(spell_ending("dom", "y"), "y");         // domy
/// // rule 2a, at the §2.2 line: `cz` drops the glide, `zz` keeps it
/// assert_eq!(spell_ending("otjecz", "je"), "e");     // otjecze
/// assert_eq!(spell_ending("druzz", "je"), "je");     // druzzje
/// // rule 2
/// assert_eq!(spell_ending("nozz", "om"), "jem");     // nozzjem
/// assert_eq!(spell_ending("otjec", "om"), "jem");    // otjecjem
/// // a plain stem is untouched
/// assert_eq!(spell_ending("dom", "om"), "om");       // domom
/// ```
pub fn spell_ending(stem: &str, ending: &str) -> String {
    let mut out = ending.to_string();
    if (ends_velar(stem) || ends_sibilant(stem)) && out.starts_with('y') {
        out.replace_range(0..1, "i");
    }
    if (ends_sibilant(stem) || ends_ts(stem)) && out.starts_with('o') {
        out.replace_range(0..1, "je");
    }
    // Rule 2a, and it is deliberately about the vocative `-je` alone rather
    // than about any `j`-initial ending. Everywhere else a leading `j` is the
    // *soft sign* that rule 3 assigns to the ending — `konj` + `-jem`, and
    // declension III's `-jju`, which §3.6 says keeps the sign and takes the
    // ending both. Stripping those gives `noczju` for `noczjju` and `noczev`
    // for `noczjev`.
    if ending == "je" && ends_inherently_palatal(stem) {
        out = "e".to_string();
    }
    out
}

/// True when the stem ends in `cz` or `szcz` — the two consonants §2.2 gives no
/// hard/soft distinction, and so the two that reject an ending's glide.
///
/// `zz` and `sz` are deliberately excluded: they have soft values, and
/// `druzzje` depends on it.
fn ends_inherently_palatal(stem: &str) -> bool {
    stem.ends_with("cz") || stem.ends_with("szcz")
}

/// Join a stem and an ending, applying §3.8's seam rules.
///
/// This is the only way a form is ever built, so no caller can bypass the
/// spelling rules by concatenating.
///
/// ```
/// use ruthenian_core::spelling::join;
/// assert_eq!(join("dom", "ogo"), "domogo");
/// assert_eq!(join("knig", "y"), "knigi");     // rule 1
/// assert_eq!(join("otjecz", "je"), "otjecze"); // rule 2a
/// ```
pub fn join(stem: &str, ending: &str) -> String {
    format!("{stem}{}", spell_ending(stem, ending))
}
