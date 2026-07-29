//! §3.8's automatic spelling adjustments and §2.4's three palatalizations.
//!
//! **One module, used by every part of speech.** A second copy of a seam rule
//! means it is in the wrong place.
//!
//! These are what let Ruthenian have three declensions where Russian has eight:
//! the velar, sibilant, `c` and vowel stem-classes are not separate paradigms
//! but the same endings under the rules below (§3.2, §3.8).
//!
//! Rules are declared in the SPE notation of `dsl.rs` and applied by the two
//! interpreters below ([`spell_ending`] for the seam, [`palatalize`] and
//! [`mutate_present_stem`] for the stem edge). The declarations are the
//! normative statement of the rules; the interpreters hold no linguistics.
//!
//! Nothing here reads or places stress. §2.1 makes stress real, lexical and
//! **unwritten**, so no rule may condition on it — see §3.8's note on why rule 2
//! dropped its stress clause.

crate::dsl::letters! {
    /// The present-stem mutations of §7.11.
    ///
    /// **A stop keeps its place before its reflex; a fricative merges with its own.**
    ///
    /// | | |
    /// |---|---|
    /// | stops `t d k g`, and the labials | additive — `letcz`, `vidzz`, `iskcz`, `ljublj` |
    /// | fricatives `s z h` | replacive — `pisz`, `vozz`, `masz` |
    ///
    /// The split is phonetic rather than arbitrary. A stop stays audible in front of
    /// its reflex, so writing it costs nothing and keeps the root legible: `vidzz`
    /// [vidʒ] shows `vid-` where Russian's `вижу` does not. Two fricatives in
    /// sequence do not survive: `s` + `sz` would be [sʃ], which is no Slavic sound
    /// and collapses to [ʃː], so only the reflex is written.
    ///
    /// The labial rules were always additive — `p` → `plj` keeps the `p` — so this
    /// is the rule they already followed, extended to the other stops.
    ///
    /// Two homographs the fully replacive version created are gone: `voditj` and
    /// `vozitj` both gave `vozzu`, and `letjetj` "fly" collided with `leczitj` "heal"
    /// at `leczu`. Both pairs separate here, and precisely because `d`/`t` are stops
    /// while `z` is a fricative — `vodzzu` against `vozzu`, `letczu` against `leczu`.
    ///
    /// No output needs the separator `'`: `tcz`, `dzz`, `kcz` and `gzz` are each
    /// unambiguous under the greedy reader, and the one form that did need it —
    /// additive `z` → `z'zz` — is not taken.
    ///
    /// Applied **by class**, never by stem shape: a class-1 verb with a
    /// labial-final stem takes no mutation at all, verified across 1 977 Russian
    /// verbs without exception (`COMPARATIVE_GRAMMAR.md`, Method). A rule keyed on
    /// "ends in a labial" would corrupt all 1 977, which is why
    /// [`mutate_present_stem`] is only ever called for the classes that mutate.
    ///
    /// `ov` → `u` is class 2's stem formation rather than iotation (§7.3), and it
    /// replaces. It is listed first because `njegodov` ends in `v` and would
    /// otherwise take the labial rule, coming out `njegodovlj`.
    ///
    /// §7.11's `st` → `szcz` and `sk` → `szcz` are **gone**: the general rule applies
    /// to the cluster's last consonant, both of which are stops, so `krjest` →
    /// `krjestcz` and `isk` → `iskcz` need no rules of their own.
    pub const MUTATIONS = [
        ov => u,
        // stops — additive
        t => tcz, d => dzz, k => kcz, g => gzz,
        p => plj, b => blj, v => vlj, m => mlj,
        // fricatives — replacive
        s => sz, z => zz, h => sz,
    ];
}

/// Which palatalization an ending triggers (§2.4).
///
/// Ruthenian keeps all three. Russian levelled the second away entirely (0 %);
/// Ukrainian keeps it at 99 % and OCS at 66 %. It is not decoration: it is what
/// distinguishes the locative `druzi` from the vocative `druzze` in the
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

crate::dsl::letters! {
    /// §2.4 — the **first** palatalization, before front vowels and `j`.
    ///
    /// `c` is here because it is itself the output of an earlier
    /// palatalization and reverts before a front vowel, exactly as OCS
    /// `otьcь` -> `otьče`; it governs the whole `-jec` class.
    pub const FIRST_PALATALIZATION = [ k => cz, g => zz, h => sz, c => cz ];

    /// §2.4 — the **second** palatalization, before a yat-derived `-i`.
    ///
    /// No `dz`: OCS had it as the output of `g`, and Ruthenian gives `z`
    /// instead, following East Slavic.
    pub const SECOND_PALATALIZATION = [ k => c, g => z, h => s ];
}

/// Apply a palatalization to a stem's final consonant. A stem with no eligible
/// consonant is returned unchanged, which is why this is safe to call
/// unconditionally.
///
/// ```
/// use ruthenian_core::spelling::{palatalize, Palatal};
/// // first: the vocative `drug` -> `druzze`
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
    let table = match which {
        Palatal::None => return stem.to_string(),
        Palatal::First => FIRST_PALATALIZATION,
        Palatal::Second => SECOND_PALATALIZATION,
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
/// // A stop is kept, so the root stays visible.
/// assert_eq!(mutate_present_stem("vid"), "vidzz");   // vidjetj -> vidzzu
/// assert_eq!(mutate_present_stem("let"), "letcz");   // letjetj -> letczu
/// assert_eq!(mutate_present_stem("isk"), "iskcz");   // iskatj' -> iskczu
/// assert_eq!(mutate_present_stem("ljub"), "ljublj"); // ljubitj -> ljublju
///
/// // A fricative merges with its reflex: [s] + [sz] is no Slavic sound.
/// assert_eq!(mutate_present_stem("pis"), "pisz");    // pisatj' -> piszesz
/// assert_eq!(mutate_present_stem("voz"), "vozz");    // vozitj  -> vozzu
/// assert_eq!(mutate_present_stem("mah"), "masz");    // mahatj' -> maszu
///
/// // Both homographs of the fully replacive version stay separate, because
/// // d and t are stops where z is a fricative.
/// assert_ne!(mutate_present_stem("vod"), mutate_present_stem("voz"));
/// assert_eq!(mutate_present_stem("lecz"), "lecz");   // leczitj -> leczu
///
/// // Class 2's stem formation replaces rather than adds.
/// assert_eq!(mutate_present_stem("njegodov"), "njegodu");
///
/// // An already-mutated stem is not mutated twice.
/// assert_eq!(mutate_present_stem("vidzz"), "vidzz");
/// assert_eq!(mutate_present_stem("pisz"), "pisz");
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

/// Apply [`SEAM`] — §3.8's seam rules, in the list's order. This function is
/// an interpreter and holds no linguistics; the rules are the declaration.
///
/// Rule 2 covers the vocative (`otjecze`, `druzze`), the present endings
/// (`piszesz`, §7.3) and the `-jem`/`-jego` series, which after these stems is
/// simply `-om` and `-ogo`.
///
/// **There used to be a third rule and its deletion is why this one is simple.**
/// It wrote an ending's `o` as `je` after these stems, conditioned on the ending
/// being unstressed — unimplementable, since §2.1 never writes stress, and
/// phonologically wrong besides: `nozzjem` claims a palatalized `zz`, and §2.2
/// has no such consonant. Without it the endings are invariant and `nozzom`,
/// `otjecom` come out as Russian's `ножом`, `отцом`.
///
/// ```
/// use ruthenian_core::spelling::spell_ending;
/// // rule 1
/// assert_eq!(spell_ending("knig", "y"), "i");        // knigi
/// assert_eq!(spell_ending("dom", "y"), "y");         // domy
/// // rule 2, across all four hard-or-inherently-palatal consonants
/// assert_eq!(spell_ending("otjecz", "je"), "e");     // otjecze
/// assert_eq!(spell_ending("druzz", "je"), "e");      // druzze
/// assert_eq!(spell_ending("pisz", "jesz"), "esz");  // piszesz
/// // the deleted rule's cells: the ending is simply invariant now
/// assert_eq!(spell_ending("nozz", "om"), "om");      // nozzom
/// assert_eq!(spell_ending("otjec", "ogo"), "ogo");   // otjecogo
/// assert_eq!(spell_ending("dom", "om"), "om");       // domom
/// // and rule 2 does not touch a soft sign: §3.6 keeps both
/// assert_eq!(spell_ending("nocz", "jju"), "jju");    // noczjju
/// // rule 3b
/// assert_eq!(spell_ending("czitaj", "jesz"), "esz");  // czitajesz
/// ```
pub fn spell_ending(stem: &str, ending: &str) -> String {
    let mut out = ending.to_string();
    for rule in SEAM {
        if !out.starts_with(rule.from) {
            continue;
        }
        if !rule.after.iter().any(|l| ends_with_letter(stem, l)) {
            continue;
        }
        if rule.only_before_vowel {
            let next = out[rule.from.len()..].chars().next();
            if !matches!(next, Some(v) if VOWELS.contains(&v)) {
                continue;
            }
        }
        out.replace_range(0..rule.from.len(), rule.to);
    }
    out
}

crate::dsl::rewrites! {
    /// §3.8's seam rules, **in application order** — the list is the ordering.
    ///
    /// Rule 2 precedes rule 1 because its output feeds it: §3.2's soft series
    /// contains `-jy`, and a soft hushing stem reaches rule 1 only once the
    /// glide is gone — `nozzj` + `-jy` is `nozz` + `-y` is `nozzi`, the plural
    /// the hard `nozz` has. In the other order it stays `nozzy`, which rule 1
    /// forbids outright. No other pair of rules interacts.
    ///
    /// The `V` on rule 2 is what tells a glide from a soft sign: a `j` not
    /// before a vowel is §3.8 rule 3's soft sign and stays, so declension III
    /// keeps `noczj`, `noczjju`, `noczjma` while `nozz` + `-je` loses its glide.
    ///
    /// Rule 3b closes the list: a stem-final `j` and an ending-initial `j` are
    /// written once. A class-1 present stem ends in `j` by construction (§7.3),
    /// so this is every form of every such verb — `czitaj` + `-jesz` is
    /// `czitajesz`.
    pub const SEAM = [
        // rule 2 — a glide is unwritable after a hushing consonant
        "j" => ""  / [zz sz cz szcz] _ V ;
        // rule 1 — `y` is written `i` after a velar or hushing consonant
        "y" => "i" / [k g h zz sz cz szcz] _ ;
        // rule 3b — `jj` across the seam is written once
        "j" => ""  / [j] _ ;
    ];
}

/// The plain vowels (§2.3).
const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

/// Join a stem and an ending, applying §3.8's seam rules.
///
/// This is the only way a form is ever built, so no caller can bypass the
/// spelling rules by concatenating.
///
/// ```
/// use ruthenian_core::spelling::join;
/// assert_eq!(join("dom", "ogo"), "domogo");
/// assert_eq!(join("knig", "y"), "knigi");     // rule 1
/// assert_eq!(join("otjecz", "je"), "otjecze"); // rule 2
/// // Rule 2 before rule 1: the glide comes off, and *then* the `y` it uncovers
/// // is respelled. The other order leaves `nozzy`, which rule 1 forbids.
/// assert_eq!(join("nozz", "y"), "nozzi");
/// assert_eq!(join("nozz", "jy"), "nozzi");
/// assert_eq!(join("sj", "im"), "sjim");       // the stem is invariant (§2.5)
/// assert_eq!(join("kon", "ji"), "konji");
/// assert_eq!(join("czitaj", "jesz"), "czitajesz"); // rule 3b: jj is one ja
/// ```
pub fn join(stem: &str, ending: &str) -> String {
    format!("{stem}{}", spell_ending(stem, ending))
}
