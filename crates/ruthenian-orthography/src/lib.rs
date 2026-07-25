//! Bijective Cyrillic↔Latin orthography for Ruthenian — Latin-script Russian.
//!
//! ```
//! use ruthenian_orthography::{Cyrillic, to_latin, to_cyrillic};
//!
//! let c = Cyrillic::parse("подъезд").unwrap();
//! let r = to_latin(&c);
//! assert_eq!(r.as_str(), "pod'jezd");
//! assert_eq!(to_cyrillic(&r).as_str(), "подъезд");
//! ```
//!
//! # The contract
//!
//! `to_cyrillic(to_latin(s)) == s` for every [`Cyrillic`] value. This holds *by
//! construction*: [`reader`] defines how Ruthenian is read, and the writer emits
//! a separator exactly where re-reading its own output would diverge.
//!
//! # The declared alphabet
//!
//! The contract is claimed only over the declared alphabet, so there is no entry
//! point that silently accepts arbitrary text. [`Cyrillic::parse`] returns an
//! [`AlphabetError`] with a byte offset and a reason; [`to_latin_mixed`] is the
//! lenient path and reports what it skipped.
//!
//! The alphabet includes three *context* rules, not just a character set. They
//! are what let the reverse direction decide without guessing, and each was
//! validated against 41 462 lines of Russian prose:
//!
//! * `ъ` is followed by `е ё ю я и` — so `'` before `j`/`i` is the hard sign and
//!   `'` elsewhere is a pure separator;
//! * `ь` follows a consonant and `й` does not — both are written `j`, and this
//!   is how the reader tells them apart.
//!
//! # Stress
//!
//! Ruthenian marks stress with a combining acute, carried in both directions and
//! attached to the same vowel (`писа́ть` ↔ `pisátj`). A stressed and an
//! unstressed spelling are **different strings**; nothing here normalizes one
//! into the other.

#![forbid(unsafe_code)]

pub mod alphabet;
pub mod reader;
mod writer;

pub use alphabet::{AlphabetError, STRESS, Unmapped};
pub use reader::Grapheme;

use alphabet::{
    AFTER_HARD_SIGN, Class, SEP, classify_foreign, find_cyrillic, is_neutral,
    ruthenian_char_allowed,
};

/// A string verified to be well-formed Cyrillic under the declared alphabet.
///
/// Its existence makes "an unmapped character reached the mapper"
/// unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cyrillic(String);

/// A string verified to be well-formed Ruthenian.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ruthenian(String);

impl Cyrillic {
    /// Validate a Cyrillic string, including the three context rules.
    ///
    /// ```
    /// use ruthenian_orthography::{Cyrillic, Unmapped};
    /// assert!(Cyrillic::parse("вода").is_ok());
    ///
    /// // Latin inside Cyrillic is refused rather than silently transliterated.
    /// let e = Cyrillic::parse("cat дом").unwrap_err();
    /// assert_eq!(e.kind, Unmapped::LatinInCyrillic);
    /// assert_eq!(e.offset, 0);
    ///
    /// // Pre-reform letters are recognized well enough to be named.
    /// assert_eq!(Cyrillic::parse("мѣсто").unwrap_err().kind, Unmapped::PreReform);
    /// ```
    pub fn parse(s: &str) -> Result<Self, AlphabetError> {
        let mut prev: Option<Class> = None;
        let mut prev_char: Option<char> = None;
        for (offset, c) in s.char_indices() {
            if c == STRESS {
                match prev {
                    Some(Class::Vowel) => {}
                    _ => {
                        return Err(AlphabetError {
                            offset,
                            found: c,
                            kind: Unmapped::StrayStress,
                        });
                    }
                }
                continue;
            }
            if c == SEP {
                return Err(AlphabetError {
                    offset,
                    found: c,
                    kind: Unmapped::Apostrophe,
                });
            }
            if is_neutral(c) {
                // A hard sign must be followed by a letter, so a word boundary
                // right after one is just as ill-formed as a wrong letter. The
                // property guard found this hole: `2ъ2` slipped through, because
                // the check below only ran when the next character was a letter.
                if matches!(prev, Some(Class::HardSign)) {
                    return Err(AlphabetError {
                        offset,
                        found: prev_char.unwrap_or('ъ'),
                        kind: Unmapped::HardSignContext,
                    });
                }
                prev = None;
                prev_char = None;
                continue;
            }
            let Some(l) = find_cyrillic(c) else {
                return Err(AlphabetError {
                    offset,
                    found: c,
                    kind: classify_foreign(c),
                });
            };

            match l.class {
                Class::SoftSign if !matches!(prev, Some(Class::Consonant)) => {
                    return Err(AlphabetError {
                        offset,
                        found: c,
                        kind: Unmapped::SoftSignContext,
                    });
                }
                Class::ShortI if matches!(prev, Some(Class::Consonant)) => {
                    return Err(AlphabetError {
                        offset,
                        found: c,
                        kind: Unmapped::ShortIContext,
                    });
                }
                _ => {}
            }

            // The hard sign constrains what follows it, so it is checked one
            // character late.
            if matches!(prev, Some(Class::HardSign)) {
                if !AFTER_HARD_SIGN.contains(&c) {
                    return Err(AlphabetError {
                        offset,
                        found: prev_char.unwrap_or('ъ'),
                        kind: Unmapped::HardSignContext,
                    });
                }
                // `'` is caseless, so the hard sign's case is recovered from the
                // following letter. Requiring them to agree makes that exact.
                let hard_upper = prev_char == Some('Ъ');
                if hard_upper != c.is_uppercase() {
                    return Err(AlphabetError {
                        offset,
                        found: prev_char.unwrap_or('ъ'),
                        kind: Unmapped::HardSignCase,
                    });
                }
            }

            prev = Some(l.class);
            prev_char = Some(c);
        }

        if matches!(prev, Some(Class::HardSign)) {
            return Err(AlphabetError {
                offset: s.len(),
                found: prev_char.unwrap_or('ъ'),
                kind: Unmapped::HardSignContext,
            });
        }

        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Ruthenian {
    /// Validate a Ruthenian string.
    ///
    /// ```
    /// use ruthenian_orthography::{Ruthenian, Unmapped};
    /// assert!(Ruthenian::parse("pod'jezd").is_ok());
    /// assert_eq!(Ruthenian::parse("quiz").unwrap_err().kind, Unmapped::NotInAlphabet);
    /// assert_eq!(Ruthenian::parse("дом").unwrap_err().kind, Unmapped::CyrillicInLatin);
    /// ```
    pub fn parse(s: &str) -> Result<Self, AlphabetError> {
        for (offset, c) in s.char_indices() {
            if ruthenian_char_allowed(c) {
                continue;
            }
            let kind = if find_cyrillic(c).is_some() || (c as u32) >= 0x0400 {
                Unmapped::CyrillicInLatin
            } else if c.is_control() {
                Unmapped::Control
            } else {
                Unmapped::NotInAlphabet
            };
            return Err(AlphabetError {
                offset,
                found: c,
                kind,
            });
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Cyrillic → Ruthenian.
///
/// ```
/// use ruthenian_orthography::{Cyrillic, to_latin};
/// let latin = |s: &str| to_latin(&Cyrillic::parse(s).unwrap()).as_str().to_string();
///
/// assert_eq!(latin("Щука"), "Szczuka");
/// assert_eq!(latin("ЩУКА"), "SZCZUKA");   // the reference produced "SzczUKA"
/// assert_eq!(latin("Ийон"), "Ij'on");     // И + й + о, not И + ё
/// assert_eq!(latin("Иён"), "Ijon");
/// assert_eq!(latin("шчи"), "sz'czi");     // ш + ч, not щ
/// assert_eq!(latin("щи"), "szczi");
/// assert_eq!(latin("батальон"), "batalj'on");
/// ```
pub fn to_latin(s: &Cyrillic) -> Ruthenian {
    Ruthenian(writer::write(&s.0))
}

/// Ruthenian → Cyrillic.
///
/// ```
/// use ruthenian_orthography::{Ruthenian, to_cyrillic};
/// let cyr = |s: &str| to_cyrillic(&Ruthenian::parse(s).unwrap()).as_str().to_string();
///
/// assert_eq!(cyr("pod'jezd"), "подъезд");
/// assert_eq!(cyr("podjezd"), "подезд");   // the pair stays distinct
/// assert_eq!(cyr("s'zadi"), "сзади");
/// ```
pub fn to_cyrillic(s: &Ruthenian) -> Cyrillic {
    Cyrillic(reader::to_string(&reader::tokenize(&s.0, None)))
}

/// A run of input that [`to_latin_mixed`] left alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedSpan {
    pub start: usize,
    pub end: usize,
    pub kind: Unmapped,
}

/// The lenient path: transliterate maximal runs of declared-alphabet characters,
/// leave everything else byte-identical, and report what was skipped.
///
/// The return type is a plain `String`, not [`Ruthenian`], and deliberately so:
/// mixed output contains text this crate makes no claims about, so it does not
/// participate in the round-trip contract. Use [`to_latin`] when you need that
/// guarantee.
///
/// ```
/// use ruthenian_orthography::to_latin_mixed;
/// let (out, skipped) = to_latin_mixed("cat дом");
/// assert_eq!(out, "cat dom");
/// assert_eq!(skipped.len(), 1);      // the Latin run is reported, not converted
/// assert_eq!((skipped[0].start, skipped[0].end), (0, 3));
/// ```
pub fn to_latin_mixed(s: &str) -> (String, Vec<SkippedSpan>) {
    let mut out = String::with_capacity(s.len() * 2);
    let mut skipped: Vec<SkippedSpan> = Vec::new();
    let mut buf = String::new();
    let flush = |buf: &mut String, out: &mut String| {
        if !buf.is_empty() {
            out.push_str(&writer::write(buf));
            buf.clear();
        }
    };

    for (offset, c) in s.char_indices() {
        if find_cyrillic(c).is_some() || c == STRESS {
            buf.push(c);
            continue;
        }
        flush(&mut buf, &mut out);
        if is_neutral(c) {
            out.push(c);
            continue;
        }
        let kind = classify_foreign(c);
        out.push(c);
        match skipped.last_mut() {
            Some(last) if last.end == offset && last.kind == kind => {
                last.end = offset + c.len_utf8();
            }
            _ => skipped.push(SkippedSpan {
                start: offset,
                end: offset + c.len_utf8(),
                kind,
            }),
        }
    }
    flush(&mut buf, &mut out);
    (out, skipped)
}

/// The declared inventory, queryable.
pub struct Alphabet;

impl Alphabet {
    /// Is this character part of the declared alphabet, on either side?
    pub fn contains(c: char) -> bool {
        c == STRESS
            || c == SEP
            || is_neutral(c)
            || find_cyrillic(c).is_some()
            || ruthenian_char_allowed(c)
    }

    /// The multi-character spellings, longest first — the order that defines
    /// greedy reading.
    pub fn digraphs() -> &'static [(&'static str, char)] {
        alphabet::DIGRAPHS
    }

    /// Every Cyrillic letter in the alphabet.
    pub fn letters() -> impl Iterator<Item = char> {
        alphabet::LETTERS.iter().map(|l| l.lower)
    }
}
