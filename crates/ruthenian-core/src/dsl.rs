//! The notation the grammar is written in.
//!
//! Phonological rules here are declared in SPE rewrite notation, `A → B / C _ D`
//! — "A becomes B when C precedes and D follows" — which has been the standard
//! way to write such rules since *The Sound Pattern of English* (1968). The
//! macros below let the Rust source *be* that notation, so a rule reads the way
//! §3.8 states it, and the two-level tools of computational morphology (lexc,
//! twolc) would recognize the shape.
//!
//! Two macros, two shapes:
//!
//! - [`rewrites!`] — ordered rules at the **seam**, rewriting an ending's start
//!   conditioned on the stem's final letter: `"y" => "i" / [k g h] _ ;`
//! - [`letters!`] — unordered letter maps at the **stem edge**: `k => cz`,
//!   the palatalizations and the present-stem mutations.
//!
//! The declarations are const data, not generated code. One interpreter in
//! `spelling.rs` applies a [`Rewrite`] list in order; making the order a fact
//! about a list — rather than about the sequence of `if` blocks in a function —
//! is most of the point, because §3.8's rules 1 and 2 are order-dependent and
//! an ordering buried in control flow is invisible until it bites.

/// One rewrite at the stem/ending seam.
///
/// Declared as `FROM => TO / [CLASS] _ ;` — or `/ [CLASS] _ V ;` when the rule
/// applies only where a vowel follows the rewritten letter, which is how a
/// glide `j` is told apart from a soft-sign `j` (§3.8 rule 2).
#[derive(Debug, Clone, Copy)]
pub struct Rewrite {
    /// What the ending must start with.
    pub from: &'static str,
    /// What replaces it.
    pub to: &'static str,
    /// The stem-final letters that trigger the rule — matched as **letters**,
    /// digraph-aware, never as raw characters.
    pub after: &'static [&'static str],
    /// Restrict the rule to `from` immediately before a vowel.
    pub only_before_vowel: bool,
}

/// Declare an ordered list of seam rewrites in SPE notation.
///
/// ```text
/// rewrites! {
///     pub const SEAM = [
///         "j" => ""  / [zz sz cz szcz] _ V ;
///         "y" => "i" / [k g h zz sz cz szcz] _ ;
///     ];
/// }
/// ```
///
/// **The list order is the application order.** A rule whose output feeds a
/// later rule must precede it, and swapping two dependent rules is a change in
/// the language — which is why the order lives in a declaration the diff shows,
/// with a witness in the doctests of `spelling.rs`.
macro_rules! rewrites {
    (
        $(#[$meta:meta])*
        pub const $name:ident = [
            $( $from:literal => $to:literal / [ $($class:ident)+ ] _ $($v:ident)? ; )+
        ];
    ) => {
        $(#[$meta])*
        pub const $name: &[$crate::dsl::Rewrite] = &[
            $(
                $crate::dsl::Rewrite {
                    from: $from,
                    to: $to,
                    after: &[ $( stringify!($class) ),+ ],
                    only_before_vowel: $crate::dsl::rewrites!(@vowel $($v)?),
                }
            ),+
        ];
    };
    (@vowel V) => { true };
    (@vowel) => { false };
    (@vowel $other:ident) => {
        compile_error!("after `_` only `V` (before a vowel) is meaningful")
    };
}

/// Declare letter maps — `k => cz` pairs applied to a stem's final letter.
///
/// ```text
/// letters! {
///     /// §2.4 — the first palatalization.
///     pub const FIRST = [ k => cz, g => zz, h => sz, c => cz ];
/// }
/// ```
///
/// A map is **not** ordered the way [`rewrites!`] is — at most one pair can
/// match a given stem — except where one source is a suffix of another, in
/// which case the longer must come first (`ov` before `v` in the mutations).
macro_rules! letters {
    (
        $(
            $(#[$meta:meta])*
            pub const $name:ident = [ $( $from:ident => $to:ident ),+ $(,)? ];
        )+
    ) => {
        $(
            $(#[$meta])*
            pub const $name: &[(&str, &str)] = &[
                $( (stringify!($from), stringify!($to)) ),+
            ];
        )+
    };
}

pub(crate) use letters;
pub(crate) use rewrites;

/// One declension's ending table: a cell per (case, number) the resolver can
/// reach, `None` where it cannot.
///
/// A `None` cell is a **claim** — "resolution never sends this case here" — and
/// the `paradigm_totality` guard is what makes the claim checked rather than
/// hoped: it walks every grammatical combination through the public API and
/// fails if any resolves onto a missing cell.
pub struct Paradigm {
    pub sg: &'static [(crate::grammar::Case, Cell)],
    pub du: &'static [(crate::grammar::Case, Cell)],
    pub pl: &'static [(crate::grammar::Case, Cell)],
}

/// An ending and the palatalization it triggers, or `None` for an unreachable
/// cell.
pub type Cell = Option<(&'static str, crate::spelling::Palatal)>;

impl Paradigm {
    /// The cell for a **resolved** case — callers run §3.1/§3.7 syncretism
    /// resolution first, so the table holds only distinct forms.
    pub(crate) fn cell(
        &self,
        case: crate::grammar::Case,
        number: crate::grammar::Number,
    ) -> Option<(&'static str, crate::spelling::Palatal)> {
        use crate::grammar::Number;
        let column = match number {
            Number::Singular => self.sg,
            Number::Dual => self.du,
            Number::Plural => self.pl,
        };
        column
            .iter()
            .find(|(c, _)| *c == case)
            .and_then(|(_, cell)| *cell)
    }
}

/// Declare ending tables in the spec's own orientation — a row per case, a
/// column per number, exactly as §3.3–§3.6 print them.
///
/// ```text
/// paradigm! {
///     /// §3.3 — declension II masculine.
///     pub const II_MASCULINE = [
///         //   singular       dual     plural
///         nom: "",             "a",    "y";
///         voc: (First "je"),   -,      -;
///         ...
///     ];
/// }
/// ```
///
/// A cell is an ending literal, `(Palatalization "ending")` where the ending
/// triggers one, or `-` where case resolution never reaches the cell.
macro_rules! paradigm {
    (
        $(
            $(#[$meta:meta])*
            pub const $name:ident = [
                $( $case:ident : $sg:tt , $du:tt , $pl:tt ; )+
            ];
        )+
    ) => {
        $(
            $(#[$meta])*
            pub(crate) const $name: $crate::dsl::Paradigm = $crate::dsl::Paradigm {
                sg: &[ $( ($crate::dsl::paradigm!(@case $case), $crate::dsl::paradigm!(@cell $sg)) ),+ ],
                du: &[ $( ($crate::dsl::paradigm!(@case $case), $crate::dsl::paradigm!(@cell $du)) ),+ ],
                pl: &[ $( ($crate::dsl::paradigm!(@case $case), $crate::dsl::paradigm!(@cell $pl)) ),+ ],
            };
        )+
    };
    (@case nom) => { $crate::grammar::Case::Nominative };
    (@case voc) => { $crate::grammar::Case::Vocative };
    (@case acc) => { $crate::grammar::Case::Accusative };
    (@case gen) => { $crate::grammar::Case::Genitive };
    (@case abl) => { $crate::grammar::Case::Ablative };
    (@case dat) => { $crate::grammar::Case::Dative };
    (@case ins) => { $crate::grammar::Case::Instrumental };
    (@case loc) => { $crate::grammar::Case::Locative };
    (@cell -) => { None };
    (@cell $end:literal) => { Some(($end, $crate::spelling::Palatal::None)) };
    (@cell ( $pal:ident $end:literal )) => {
        Some(($end, $crate::spelling::Palatal::$pal))
    };
}

pub(crate) use paradigm;

/// Find `key`'s row and take column `col`. `None` is a missing row or a `-`
/// cell — for every table a guard proves the reachable queries never see it.
pub(crate) fn lookup<K: PartialEq, const N: usize>(
    rows: &'static [(K, [Cell; N])],
    key: K,
    col: usize,
) -> Option<(&'static str, crate::spelling::Palatal)> {
    rows.iter()
        .find(|(k, _)| *k == key)
        .and_then(|(_, cells)| cells[col])
}

/// Declare a general ending table — any row key, any number of columns.
///
/// [`paradigm!`] is the noun-shaped special case; this is for the shapes that
/// are not case-by-number: the long adjective (a case row, five agreement
/// columns) and the non-past verb (a person/number row, one column per
/// conjugation).
///
/// ```text
/// table! {
///     pub const NON_PAST: [(Person, Number); 2] = [
///         //                              1st       2nd
///         (Person::Second, Number::Singular) => "jeszj", "iszj";
///     ];
/// }
/// ```
macro_rules! table {
    (
        $(#[$meta:meta])*
        pub const $name:ident : [$key:ty ; $n:tt] = [
            $( $k:expr => $( $cell:tt ),+ ; )+
        ];
    ) => {
        $(#[$meta])*
        pub(crate) const $name: &[($key, [$crate::dsl::Cell; $n])] = &[
            $( ($k, [ $( $crate::dsl::table!(@cell $cell) ),+ ]) ),+
        ];
    };
    (@cell -) => { None };
    (@cell $end:literal) => { Some(($end, $crate::spelling::Palatal::None)) };
    (@cell ( $pal:ident $end:literal )) => {
        Some(($end, $crate::spelling::Palatal::$pal))
    };
}

pub(crate) use table;
