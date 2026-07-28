//! The declared alphabet: which characters exist, how they map, and which
//! strings are well-formed.
//!
//! The alphabet is not merely a character set. Three context rules are part of
//! the declaration, and they are what make the mapping bijective without the
//! reverse direction ever having to guess:
//!
//! * `ъ` is followed by `е ё ю я и` — the only environments in which Russian
//!   writes it. This is what lets `'` before `j`/`i` mean the hard sign while
//!   `'` anywhere else is a pure separator.
//! * `ь` follows a consonant, and `й` does not. Both are written `j`, so the
//!   reader tells them apart by what precedes; these rules make that decision
//!   correct rather than a heuristic.
//!
//! All three were validated against 41 462 lines of Russian prose before being
//! declared: 50 036 instances of `ь` with no vowel before any of them, 31 285
//! instances of `й` with no consonant before any of them, and every `ъ` followed
//! by `я`, `е` or `и`.

/// Combining acute accent. Ruthenian marks stress, and this is the mark.
pub const STRESS: char = '\u{0301}';

/// The separator, and the hard sign: one glyph, one rule — "the next character
/// starts a new letter".
pub const SEP: char = '\'';

/// What a letter does, for the context rules and the reader's `j` decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Vowel,
    Consonant,
    HardSign,
    SoftSign,
    /// `й` — a consonant phonologically, but it needs its own class because the
    /// reader's `j` decision and the `й`-after-consonant rule both key on it.
    ShortI,
}

/// One row of the mapping. `latin` is the lowercase spelling.
#[derive(Debug, Clone, Copy)]
pub struct Letter {
    pub lower: char,
    pub upper: char,
    pub latin: &'static str,
    pub class: Class,
}

macro_rules! letters {
    ($(($l:literal, $u:literal, $lat:literal, $c:ident)),* $(,)?) => {
        pub const LETTERS: &[Letter] = &[
            $(Letter { lower: $l, upper: $u, latin: $lat, class: Class::$c }),*
        ];
    };
}

letters![
    ('а', 'А', "a", Vowel),
    ('б', 'Б', "b", Consonant),
    ('в', 'В', "v", Consonant),
    ('г', 'Г', "g", Consonant),
    ('д', 'Д', "d", Consonant),
    ('е', 'Е', "je", Vowel),
    ('ж', 'Ж', "zz", Consonant),
    ('з', 'З', "z", Consonant),
    ('и', 'И', "i", Vowel),
    ('й', 'Й', "j", ShortI),
    ('к', 'К', "k", Consonant),
    ('л', 'Л', "l", Consonant),
    ('м', 'М', "m", Consonant),
    ('н', 'Н', "n", Consonant),
    ('о', 'О', "o", Vowel),
    ('п', 'П', "p", Consonant),
    ('р', 'Р', "r", Consonant),
    ('с', 'С', "s", Consonant),
    ('т', 'Т', "t", Consonant),
    ('у', 'У', "u", Vowel),
    ('ф', 'Ф', "f", Consonant),
    ('х', 'Х', "h", Consonant),
    ('ц', 'Ц', "c", Consonant),
    ('ч', 'Ч', "cz", Consonant),
    ('ш', 'Ш', "sz", Consonant),
    ('щ', 'Щ', "szcz", Consonant),
    ('ъ', 'Ъ', "'", HardSign),
    ('ы', 'Ы', "y", Vowel),
    ('ь', 'Ь', "j", SoftSign),
    ('э', 'Э', "e", Vowel),
    ('ю', 'Ю', "ju", Vowel),
    ('я', 'Я', "ja", Vowel),
];

/// Multi-character Ruthenian spellings, longest first. The reader is a greedy
/// longest match over this list, and that greed is the definition of how
/// Ruthenian is read — everything else is defined in terms of it.
pub const DIGRAPHS: &[(&str, char)] = &[
    ("szcz", 'щ'),
    ("sz", 'ш'),
    ("cz", 'ч'),
    ("zz", 'ж'),
    ("ja", 'я'),
    ("je", 'е'),
    ("ju", 'ю'),
];

/// The letters `ъ` may precede. Widened from the iotated vowels to include `и`
/// because the corpus attests `предъидешь`, `предъизбранным`, `предъизбрал` —
/// a rule derived from three real words rather than from tidiness.
/// `ъ` may only stand before these. `ё` is **not** among them: it is not in the
/// declared alphabet at all (see [`Unmapped::Yo`]).
pub const AFTER_HARD_SIGN: &[char] = &['е', 'ю', 'я', 'и', 'Е', 'Ю', 'Я', 'И'];

/// The hushing consonants `ж ш ч щ`, after which `е` is written `e` and never
/// `je`.
///
/// They are all outputs of palatalization — `*g > ž`, `*x > š`, `*k > č`,
/// `*skj > šč` — so each was inherently soft in Common Slavic, and `ж`/`ш` then
/// hardened in East Slavic while `ч`/`щ` stayed soft. Neither era gives any of
/// the four a hard/soft **contrast**, so the `j` has never marked anything after
/// them (`RUTHENIAN.md` §2.2, §3.8 rule 2).
///
/// The reverse reading is exact rather than a guess, on the same evidential
/// footing as the alphabet's other context rules: `ж ш ч щ` is followed by `э`
/// **zero** times in the 41 462-line corpus, so `e` after one of them can only
/// be `е`.
pub fn is_hushing(c: char) -> bool {
    matches!(c, 'ж' | 'ш' | 'ч' | 'щ' | 'Ж' | 'Ш' | 'Ч' | 'Щ')
}

pub fn find_cyrillic(c: char) -> Option<&'static Letter> {
    LETTERS.iter().find(|l| l.lower == c || l.upper == c)
}

/// Neutral characters pass through untouched in both directions: digits,
/// punctuation, whitespace. Deliberately excludes [`SEP`], the one non-letter
/// that carries meaning in Ruthenian, and control characters other than
/// whitespace — a BEL is not punctuation, and letting it through was a silent
/// passthrough of exactly the kind the declared alphabet exists to prevent.
/// Newline and tab stay neutral: they are control characters and they are also
/// ordinary text.
pub fn is_neutral(c: char) -> bool {
    c != SEP
        && c != STRESS
        && !c.is_alphabetic()
        && !is_combining(c)
        && (!c.is_control() || c.is_whitespace())
}

fn is_combining(c: char) -> bool {
    matches!(c as u32, 0x0300..=0x036F)
}

/// Why a character or a string position is not part of the declared alphabet.
/// Every rejection names one of these; nothing falls through silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unmapped {
    /// `ё`, which is **not in the declared alphabet**.
    ///
    /// It is not a vowel of its own: it is stressed `е` after the East Slavic
    /// `*e > o` shift, so `нёс` and `несу` are one root. The shift is
    /// conditioned entirely by stress, and `RUTHENIAN.md` §2.1 does not write
    /// stress — so spelling it would encode an alternation the language cannot
    /// see, and the stem would stop being invariant (§2.5).
    ///
    /// Russian's own orthography prints `е` for `ё` outside dictionaries, so
    /// normalizing `ё` to `е` before conversion is what a Russian text mostly
    /// does already. Declaring it out keeps the round-trip exact rather than
    /// silently lossy.
    Yo,
    /// `э` after `ж ш ч щ`. The hushing consonants take `е`, never `э`, so `e`
    /// after one of them reads back as `е` — see [`is_hushing`].
    HushingContext,
    /// ѣ ѳ і ѵ and friends — recognized only well enough to be refused.
    PreReform,
    /// Cyrillic, but not Russian: ґ є ї ў …
    ForeignCyrillic,
    /// Latin letters inside Cyrillic input. The reference silently transliterated
    /// these, turning `"cat дом"` into `"цат дом"`.
    LatinInCyrillic,
    /// A Cyrillic letter inside Ruthenian input.
    CyrillicInLatin,
    /// A Latin letter with no Ruthenian meaning: q, w, x.
    NotInAlphabet,
    Control,
    /// `'` in Cyrillic source. It is Ruthenian's separator, so it cannot also be
    /// literal text. The corpus contains none, so this costs nothing.
    Apostrophe,
    /// A combining mark other than the acute.
    ForeignMark,
    /// A stress mark not attached to a vowel.
    StrayStress,
    /// `ъ` outside the environments Russian writes it in (see [`AFTER_HARD_SIGN`]).
    HardSignContext,
    /// A hard sign whose case disagrees with the letter after it (`подЪезд`).
    /// `'` is caseless in Ruthenian, so the hard sign's case is recovered from
    /// its neighbour; requiring agreement is what makes that recovery exact
    /// instead of a guess. Real orthography never disagrees.
    HardSignCase,
    /// `ь` not after a consonant. Both `ь` and `й` are written `j`; this rule is
    /// half of what lets the reader tell them apart.
    SoftSignContext,
    /// `й` after a consonant — the other half.
    ShortIContext,
}

/// A position that is not well-formed, with the offset needed to point at it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlphabetError {
    pub offset: usize,
    pub found: char,
    pub kind: Unmapped,
}

impl core::fmt::Display for AlphabetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "byte {}: {:?} ({:?})",
            self.offset, self.found, self.kind
        )
    }
}

impl core::error::Error for AlphabetError {}

const PRE_REFORM: &[char] = &[
    'ѣ', 'Ѣ', 'ѳ', 'Ѳ', 'і', 'І', 'ѵ', 'Ѵ', 'ѕ', 'Ѕ', 'ѯ', 'Ѯ', 'ѱ', 'Ѱ', 'ѡ', 'Ѡ', 'ѫ', 'Ѫ', 'ѧ',
    'Ѧ', 'ꙗ', 'Ꙗ', 'ѥ', 'Ѥ',
];

pub fn classify_foreign(c: char) -> Unmapped {
    if PRE_REFORM.contains(&c) {
        Unmapped::PreReform
    } else if c.is_control() {
        Unmapped::Control
    } else if is_combining(c) {
        Unmapped::ForeignMark
    } else if matches!(c as u32, 0x0400..=0x04FF | 0x0500..=0x052F | 0xA640..=0xA69F) {
        Unmapped::ForeignCyrillic
    } else {
        Unmapped::LatinInCyrillic
    }
}

/// The Ruthenian side: every character a well-formed Ruthenian string may hold.
pub fn ruthenian_char_allowed(c: char) -> bool {
    if c == SEP || c == STRESS || is_neutral(c) {
        return true;
    }
    let lc = c.to_ascii_lowercase();
    c.is_ascii_alphabetic() && LETTERS.iter().any(|l| l.latin.contains(lc))
}
