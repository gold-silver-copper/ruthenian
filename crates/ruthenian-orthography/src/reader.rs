//! The reader: greedy longest match over the digraph table.
//!
//! This module is the single source of truth for how a Ruthenian string is
//! *read*. The writer is defined in terms of it — it emits a separator exactly
//! where running this reader over its own output would not reproduce the input —
//! so the round-trip property holds by construction rather than by patching
//! cases as they are discovered.

use crate::alphabet::{
    Class, DIGRAPHS, Letter, SEP, STRESS, find_cyrillic, is_hushing, is_neutral,
};

/// One unit of a read Ruthenian string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grapheme {
    Letter {
        cyr: char,
        upper: bool,
        stress: bool,
    },
    /// A pure separator: it says "the next character starts a new letter" and
    /// contributes nothing to the Cyrillic output.
    Separator,
    /// Digits, punctuation, whitespace — passed through untouched.
    Neutral(char),
}

fn starts_with_fold(hay: &str, needle: &str) -> bool {
    let mut h = hay.chars();
    for n in needle.chars() {
        match h.next() {
            Some(c) if c.to_ascii_lowercase() == n => {}
            _ => return false,
        }
    }
    true
}

fn single(c: char) -> Option<&'static Letter> {
    let lc = c.to_ascii_lowercase();
    crate::alphabet::LETTERS
        .iter()
        .find(|l| l.latin.len() == 1 && l.latin.starts_with(lc) && l.class != Class::SoftSign)
}

/// Read a Ruthenian string into graphemes.
///
/// `before` is the class of the letter preceding this string, which matters
/// because a bare `j` is `ь` after a consonant and `й` otherwise. Callers
/// reading a whole string pass `None`; the writer passes real context when it
/// checks a two-letter window.
pub fn tokenize(s: &str, before: Option<Class>) -> Vec<Grapheme> {
    let mut out = Vec::new();
    let mut prev_class = before;
    // The previous Cyrillic letter, for the hushing rule. `before` gives a class
    // and not a letter, so a probe that begins mid-word cannot use this — which
    // is right: the writer only ever asks about a two-letter window it supplies
    // in full.
    let mut prev_cyr: Option<char> = None;
    let mut i = 0;

    while i < s.len() {
        let rest = &s[i..];
        let c = match rest.chars().next() {
            Some(c) => c,
            None => break,
        };

        if is_neutral(c) {
            out.push(Grapheme::Neutral(c));
            prev_class = None; // a word boundary resets the j-context
            prev_cyr = None;
            i += c.len_utf8();
            continue;
        }

        // `e` after `ж ш ч щ` is `е`, not `э`. The alphabet declares that no
        // hushing consonant is followed by `э`, so this is exact.
        if (c == 'e' || c == 'E') && prev_cyr.is_some_and(is_hushing) {
            let upper = c.is_uppercase();
            let (cyr, class) = cased('е', upper);
            i += c.len_utf8();
            let stress = eat_stress(s, &mut i);
            out.push(Grapheme::Letter { cyr, upper, stress });
            prev_class = Some(class);
            prev_cyr = Some(cyr);
            continue;
        }

        if c == SEP {
            // One glyph, one rule. `'` is the hard sign exactly where Russian
            // writes one — before an iotated vowel (`j…`) or `и` (`i`) — and a
            // pure separator everywhere else. A separator is never *needed*
            // before `j` or `i`, because no digraph contains either in a
            // non-initial position, so the two readings can never collide.
            let next = rest[c.len_utf8()..].chars().next();
            let is_hard_sign = matches!(next, Some('j' | 'J' | 'i' | 'I'));
            if is_hard_sign {
                // `'` is caseless; the hard sign takes the case of the letter it
                // precedes. The alphabet requires the two to agree, so this is
                // exact rather than a guess.
                let upper = next.is_some_and(char::is_uppercase);
                let cyr = if upper { 'Ъ' } else { 'ъ' };
                out.push(Grapheme::Letter {
                    cyr,
                    upper,
                    stress: false,
                });
                prev_class = Some(Class::HardSign);
                prev_cyr = Some(cyr);
            } else {
                out.push(Grapheme::Separator);
                // A separator does not reset the j-context: in `batalj'on` the
                // `j` was already decided, and in `maj'ami` the following `a`
                // is a fresh letter, not a continuation.
            }
            i += c.len_utf8();
            continue;
        }

        // Greedy longest match, case-insensitive.
        let mut matched = None;
        for (lat, cyr) in DIGRAPHS {
            if starts_with_fold(rest, lat) {
                matched = Some((*lat, *cyr));
                break;
            }
        }

        if let Some((lat, cyr)) = matched {
            let upper = c.is_uppercase();
            let (cyr, class) = cased(cyr, upper);
            i += lat.len();
            let stress = eat_stress(s, &mut i);
            out.push(Grapheme::Letter { cyr, upper, stress });
            prev_class = Some(class);
            prev_cyr = Some(cyr);
            continue;
        }

        if c == 'j' || c == 'J' {
            // The one decision the reference guessed at. Here it is a rule: the
            // alphabet declares that `ь` only follows a consonant and `й` never
            // does, so the preceding class settles it.
            let soft = matches!(prev_class, Some(Class::Consonant));
            let base = if soft { 'ь' } else { 'й' };
            let upper = c.is_uppercase();
            let (cyr, class) = cased(base, upper);
            i += c.len_utf8();
            let stress = eat_stress(s, &mut i);
            out.push(Grapheme::Letter { cyr, upper, stress });
            prev_class = Some(class);
            prev_cyr = Some(cyr);
            continue;
        }

        if let Some(l) = single(c) {
            let upper = c.is_uppercase();
            let (cyr, class) = cased(l.lower, upper);
            i += c.len_utf8();
            let stress = eat_stress(s, &mut i);
            out.push(Grapheme::Letter { cyr, upper, stress });
            prev_class = Some(class);
            prev_cyr = Some(cyr);
            continue;
        }

        // Unreadable input. `Ruthenian::parse` rejects it before we get here;
        // reached only through the internal window checks, where passing it
        // through unchanged keeps the comparison honest.
        out.push(Grapheme::Neutral(c));
        prev_class = None;
        i += c.len_utf8();
    }

    out
}

fn eat_stress(s: &str, i: &mut usize) -> bool {
    if s[*i..].starts_with(STRESS) {
        *i += STRESS.len_utf8();
        true
    } else {
        false
    }
}

/// Resolve a table letter to its cased form. `lower` always comes from the
/// mapping table, so the lookup cannot miss — but it returns a fallback rather
/// than panicking, because "cannot happen" is not a reason to leave a panic on a
/// path reachable from public input.
fn cased(lower: char, upper: bool) -> (char, Class) {
    match find_cyrillic(lower) {
        Some(l) => (if upper { l.upper } else { l.lower }, l.class),
        None => (lower, Class::Consonant),
    }
}

/// Render read graphemes back to a Cyrillic string.
pub fn to_string(graphemes: &[Grapheme]) -> String {
    let mut s = String::new();
    for g in graphemes {
        match g {
            Grapheme::Letter { cyr, stress, .. } => {
                s.push(*cyr);
                if *stress {
                    s.push(STRESS);
                }
            }
            Grapheme::Separator => {}
            Grapheme::Neutral(c) => s.push(*c),
        }
    }
    s
}
