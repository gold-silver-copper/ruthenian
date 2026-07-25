//! Morphophonology: the present-stem mutations, the spelling rules, and stress
//! placement.
//!
//! **One module, used by every part of speech.** A second copy of a seam rule
//! means it is in the wrong place — `interslavic-phrase` paid for that lesson
//! with a rewrite.
//!
//! The mutation table below was measured, not recalled, over **every** Russian
//! verb in the dump. Two things it teaches that a grammar book states less
//! sharply: `ov` → `u` is the commonest mutation by a factor of six (the
//! `-ovatj` class), and mutation is **conditioned on the conjugation class, not
//! on the stem's final consonant** — of 1 977 class-1 verbs with a labial-final
//! stem, **every single one** takes no epenthesis, because the theme vowel
//! intervenes. A rule keyed on "ends in a labial" would corrupt all 1 977;
//! [`mutate_present_stem`] is therefore only ever called for the classes that
//! mutate.

pub const STRESS: char = '\u{0301}';

/// Plain vowels. The iotated letters are digraphs whose vowel is the second
/// character, so this list is what stress attaches to.
pub const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'y'];

/// `k g h` — the velar stems.
pub const VELARS: &[&str] = &["k", "g", "h"];

/// `ж ш ч щ` in Ruthenian.
pub const SIBILANTS: &[&str] = &["zz", "sz", "cz", "szcz"];

/// The labials that take an epenthetic `l` before the first-person ending.
pub const LABIALS: &[&str] = &["b", "p", "v", "f", "m"];

/// The measured present-stem mutations, longest match first.
///
/// | Cyrillic | here | count | example |
/// |---|---|---:|---|
/// | ов → у | `ov` → `u` | 675 | мульчировать/мульчирую |
/// | д → ж | `d` → `zz` | 112 | щадить/щажу |
/// | т → ч | `t` → `cz` | 60 | лететь/лечу |
/// | с → ш | `s` → `sz` | 56 | писать/пишу |
/// | з → ж | `z` → `zz` | 40 | возить/вожу |
/// | п → пл | `p` → `plj` | 38 | спать/сплю |
/// | в → вл | `v` → `vlj` | 27 | готовить/готовлю |
/// | б → бл | `b` → `blj` | 25 | любить/люблю |
/// | ст → щ | `st` → `szcz` | 24 | крестить/крещу |
/// | м → мл | `m` → `mlj` | 19 | кормить/кормлю |
/// | к → ч | `k` → `cz` | 9 | плакать/плачу |
/// | ск → щ | `sk` → `szcz` | 6 | искать/ищу |
/// | х → ш | `h` → `sz` | 6 | махать/машу |
/// | т → щ | `t` → `szcz` | 5 | трепетать/трепещу |
///
/// Not yet implemented, and each a known miss: `в → ∅` (давать/даю, 41),
/// `ев → у` (бичевать/бичую, 19), `ев → ю` (блевать/блюю, 11),
/// `им → емл` (внимать/внемлю, 5), `ер → р` (тереть/тру, 3),
/// `р → ер` (брать/беру, 2).
///
/// `d` → `zz` and `z` → `zz` collide, exactly as they do in Russian
/// (`voditj`/`voziti` both give `vozzu`). That is a real homograph, not a bug.
pub const MUTATIONS: &[(&str, &str)] = &[
    ("szcz", "szcz"),
    ("st", "szcz"),
    ("sk", "szcz"),
    ("ov", "u"),
    ("cz", "cz"),
    ("sz", "sz"),
    ("zz", "zz"),
    ("s", "sz"),
    ("t", "cz"),
    ("d", "zz"),
    ("z", "zz"),
    ("h", "sz"),
    ("k", "cz"),
    ("g", "zz"),
];

/// Apply the present-stem mutation to a stem.
///
/// Only call this for classes that mutate. Keying on the stem's final consonant
/// alone is the error this crate must not make.
///
/// ```
/// use ruthenian_core::phono::mutate_present_stem;
/// assert_eq!(mutate_present_stem("pis"), "pisz");     // писать -> пишу
/// assert_eq!(mutate_present_stem("vod"), "vozz");     // водить -> вожу
/// assert_eq!(mutate_present_stem("ljub"), "ljublj");  // любить -> люблю
/// assert_eq!(mutate_present_stem("negodov"), "negodu"); // негодовать -> негодую
/// ```
pub fn mutate_present_stem(stem: &str) -> String {
    let bare = stem;
    // Longest and most specific first. `ov` -> `u` must be tried before the
    // labial rule, or `negodov` ends in `v`, takes epenthesis, and comes out
    // `negodovlj` instead of `negodu`. This is the class-conditioning trap in
    // miniature: a shorter, more general pattern must never pre-empt a longer
    // one that names the actual environment.
    for (from, to) in MUTATIONS {
        if bare.ends_with(from) {
            let cut = bare.len() - from.len();
            return format!("{}{}", &bare[..cut], to);
        }
    }
    for lab in LABIALS {
        if bare.ends_with(lab) {
            return format!("{bare}lj");
        }
    }
    bare.to_string()
}

/// True when the stem ends in a velar (`k g h`).
pub fn ends_velar(stem: &str) -> bool {
    VELARS.iter().any(|v| ends_with_letter(stem, v))
}

/// True when the stem ends in a sibilant (`zz sz cz szcz`).
pub fn ends_sibilant(stem: &str) -> bool {
    SIBILANTS.iter().any(|v| ends_with_letter(stem, v))
}

/// True when the stem ends in `c` (the `ц` stems).
pub fn ends_ts(stem: &str) -> bool {
    ends_with_letter(stem, "c") && !ends_with_letter(stem, "cz")
}

/// `ends_with` that does not mistake the tail of a digraph for a letter: `sz`
/// ends in `sz`, not in `z`.
fn ends_with_letter(stem: &str, letter: &str) -> bool {
    if !stem.ends_with(letter) {
        return false;
    }
    let cut = stem.len() - letter.len();
    let before = &stem[..cut];
    // `z` must not match the `z` of `zz`/`sz`, nor `c` the `c` of `cz`.
    !(letter == "z" && (before.ends_with('z') || before.ends_with('s')))
        && !(letter == "c" && before.ends_with('s'))
}

/// The spelling rules that apply at a stem/ending seam.
///
/// Two of them, and the second is conditional on stress:
///
/// * after a velar or sibilant, `y` is written `i` (`knigi`, not `*knigy`);
/// * after a sibilant or `c`, **unstressed** `o` is written `je` — Ruthenian
///   writes Russian `е` as `je`, so a bare `e` would spell `э`.
///
/// `ending_stressed` is the caller's business because only it knows the accent
/// pattern: `nozzóm` keeps its `o` because the ending bears the stress, while
/// `továriszczjem` takes `je` because it does not. Deciding this from the string
/// alone is impossible — at this point no stress has been placed yet.
///
/// ```
/// use ruthenian_core::phono::spell_after_stem;
/// assert_eq!(spell_after_stem("knig", "y", false), "i");        // книги
/// assert_eq!(spell_after_stem("stol", "y", false), "y");        // столы
/// assert_eq!(spell_after_stem("gribnic", "oj", false), "jej");  // грибницей
/// assert_eq!(spell_after_stem("nozz", "om", true), "om");       // ножом
/// ```
pub fn spell_after_stem(stem: &str, ending: &str, ending_stressed: bool) -> String {
    let mut out = ending.to_string();
    if (ends_velar(stem) || ends_sibilant(stem)) && out.starts_with('y') {
        out.replace_range(0..1, "i");
    }
    if (ends_sibilant(stem) || ends_ts(stem)) && out.starts_with('o') && !ending_stressed {
        out.replace_range(0..1, "je");
    }
    out
}

/// Remove every stress mark.
pub fn unstress(s: &str) -> String {
    s.chars().filter(|c| *c != STRESS).collect()
}

/// Does this string carry a stress mark?
pub fn is_stressed(s: &str) -> bool {
    s.contains(STRESS)
}

/// Place stress on the last vowel of `s`, replacing any existing mark.
pub fn stress_last_vowel(s: &str) -> String {
    place_stress(&unstress(s), |n| n.saturating_sub(1))
}

/// Place stress on the first vowel of `s`.
pub fn stress_first_vowel(s: &str) -> String {
    place_stress(&unstress(s), |_| 0)
}

fn place_stress(s: &str, pick: impl Fn(usize) -> usize) -> String {
    let positions: Vec<usize> = s
        .char_indices()
        .filter(|(_, c)| VOWELS.contains(c))
        .map(|(i, c)| i + c.len_utf8())
        .collect();
    if positions.is_empty() {
        return s.to_string();
    }
    let idx = pick(positions.len()).min(positions.len() - 1);
    let at = positions[idx];
    let mut out = String::with_capacity(s.len() + STRESS.len_utf8());
    out.push_str(&s[..at]);
    out.push(STRESS);
    out.push_str(&s[at..]);
    out
}

/// Count the vowels in a string, ignoring stress marks.
pub fn vowel_count(s: &str) -> usize {
    s.chars().filter(|c| VOWELS.contains(c)).count()
}

/// Which vowel carries the stress, counted in vowels rather than bytes.
///
/// Morphology has to strip and append around a stress mark that sits *after* the
/// vowel it belongs to, so `strip_suffix('i')` fails on a stressed `i`. Working
/// segmentally and re-placing the stress by index avoids that whole class of bug.
///
/// ```
/// use ruthenian_core::phono::stressed_index;
/// assert_eq!(stressed_index("govori\u{301}tj"), Some(2));
/// assert_eq!(stressed_index("citatj"), None);
/// ```
pub fn stressed_index(s: &str) -> Option<usize> {
    let mut n = 0;
    let mut prev_vowel = false;
    for c in s.chars() {
        if c == STRESS {
            return Some(if prev_vowel { n - 1 } else { n });
        }
        if VOWELS.contains(&c) {
            n += 1;
            prev_vowel = true;
        } else {
            prev_vowel = false;
        }
    }
    None
}

/// Place the stress on the `idx`-th vowel, replacing any existing mark. Out of
/// range leaves the string unstressed rather than guessing.
pub fn apply_stress_at(s: &str, idx: usize) -> String {
    let bare = unstress(s);
    let positions: Vec<usize> = bare
        .char_indices()
        .filter(|(_, c)| VOWELS.contains(c))
        .map(|(i, c)| i + c.len_utf8())
        .collect();
    let Some(at) = positions.get(idx) else {
        return bare;
    };
    let mut out = String::with_capacity(bare.len() + STRESS.len_utf8());
    out.push_str(&bare[..*at]);
    out.push(STRESS);
    out.push_str(&bare[*at..]);
    out
}

/// Insert a fleeting vowel into a final consonant cluster before a zero ending.
///
/// Russian nouns and adjectives with a reducible stem grow a vowel where a zero
/// ending would otherwise leave an unpronounceable cluster: `okno` → `okon`,
/// `kukla` → `kukol`, `sbjerknizzka` → `sbjerknizzjek`, `sovjestnyj` →
/// `sovjestjen`.
///
/// Which vowel is decided by what precedes the insertion point: `je` after a
/// sibilant or a soft consonant, `o` otherwise.
///
/// ```
/// use ruthenian_core::phono::insert_fleeting_vowel;
/// assert_eq!(insert_fleeting_vowel("okn"), "okon");
/// assert_eq!(insert_fleeting_vowel("kukl"), "kukol");
/// assert_eq!(insert_fleeting_vowel("sbjerknizzk"), "sbjerknizzjek");
/// // nothing to break up
/// assert_eq!(insert_fleeting_vowel("stol"), "stol");
/// ```
pub fn insert_fleeting_vowel(stem: &str) -> String {
    let bare: Vec<char> = stem.chars().collect();
    if bare.len() < 2 {
        return stem.to_string();
    }
    // Find the final consonant, then check that something consonantal precedes
    // it; a vowel there means there is no cluster to break up.
    let last = bare.len() - 1;
    if VOWELS.contains(&bare[last]) || bare[last] == STRESS {
        return stem.to_string();
    }
    let mut before = last;
    while before > 0 && bare[before - 1] == STRESS {
        before -= 1;
    }
    if before == 0 || VOWELS.contains(&bare[before - 1]) {
        return stem.to_string();
    }
    let head: String = bare[..before].iter().collect();
    let tail: String = bare[before..].iter().collect();
    // `j` here is a soft sign, which the fleeting vowel replaces rather than
    // follows: `pjesnj` -> `pjesjen`.
    let (head, vowel) = if let Some(h) = head.strip_suffix('j') {
        (h.to_string(), "je")
    } else if ends_sibilant(&head) || ends_ts(&head) {
        (head, "je")
    } else {
        (head, "o")
    };
    format!("{head}{vowel}{tail}")
}
