//! The declared substitutes, and the only place one may be introduced.
//!
//! Law 4: **every function is total, and every fallback is declared.** Where the
//! language has no form for a cell, the function returns a *named* substitute
//! from this module — never an undocumented guess, and never nothing.
//!
//! Adding a fallback means adding a function here, a row to `DIRECTION.md`'s
//! totality table, and a test that exercises it. The `every_fallback_exercised`
//! guard fails if the last of those is missing, so a substitute cannot be
//! introduced quietly.
//!
//! # The complete list
//!
//! | Call | Returns | Why |
//! |---|---|---|
//! | `reflexive(Nominative)` | `sjebja` | §5.2 gives the reflexive no nominative: it cannot be a subject. The citation form stands in. |
//! | `imperative(w, First \| Third, Singular)` | the present indicative | §7.10 builds these periphrastically; this is the form the particle attaches to. |
//! | `pronoun(_, _, _, Vocative)` | the nominative | §5.1's table has no vocative row; §3.1's convention is that the nominative is used. |
//! | `clitic_pronoun` outside acc/dat | the full form | §5.1a gives clitics for the accusative and dative only. |
//! | anything, on an unreadable lemma | [`UNREADABLE`] | See below. |
//!
//! # The unreadable lemma
//!
//! A lemma that `ruthenian-orthography` cannot parse — `""`, `"'"`, `"дом"`,
//! `"x'y'z"` — has no stem to inflect, and the signatures return `String`. Every
//! function returns [`UNREADABLE`] for such input, identically, so a caller
//! cannot mistake one part of speech's failure for another's output.
//!
//! It is deliberately **not** a plausible Ruthenian word: a caller that ignores
//! it gets something conspicuous rather than something that reads like a form.

/// What every function returns for a lemma the orthography cannot read.
///
/// Not a word: `?` is outside the Ruthenian alphabet entirely (§2.1), so this
/// can never collide with a real form.
pub const UNREADABLE: &str = "?";

/// True when a form is the unreadable-lemma substitute rather than a word.
///
/// ```
/// use ruthenian_core::{noun, Case, Number};
/// use ruthenian_core::fallback::is_unreadable;
///
/// assert!(is_unreadable(&noun("", Case::Nominative, Number::Singular)));
/// assert!(is_unreadable(&noun("дом", Case::Nominative, Number::Singular)));
/// assert!(!is_unreadable(&noun("dom", Case::Nominative, Number::Singular)));
/// ```
pub fn is_unreadable(form: &str) -> bool {
    form == UNREADABLE
}
