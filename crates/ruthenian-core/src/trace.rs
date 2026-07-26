//! [`Prediction`] and [`Trace`]: a generated form, and why it looks that way.
//!
//! # There is no configuration here
//!
//! This engine is a function: `(stem, class, slot) -> form`. It has no policy,
//! no variant, no feature flags, and nothing that changes an answer for a fixed
//! input. The language is fixed by `RUTHENIAN.md`, so when the language changes
//! the specification changes and this crate changes with it — that is a source
//! edit, not a runtime switch.
//!
//! `RUTHENIAN.md` §13's open questions (the ablative plural, clitic pronouns,
//! the middle voice) are language-design decisions awaiting an answer, not
//! options a caller picks between. Modelling them as runtime configuration would
//! presuppose Ruthenian ships as several simultaneous dialects, which nothing
//! calls for, and would put a dead branch in every rule until it did.
//!
//! The trace remains, because provenance is not configuration: a caller is
//! entitled to know *which rule* produced a form, and that is true of a single
//! fixed grammar just as much as of a configurable one.

/// Why a form looks the way it does: the rules that fired, in order.
///
/// Never empty. The evaluator attributes mismatches through it and the CLI's
/// `--show-derivation` explains a form with it; returning a bare string would
/// force both to re-derive what the engine already knew.
/// Deliberately **not** `Default`: a trace with no steps would let a prediction
/// arrive with no explanation, and every consumer downstream depends on there
/// being one. [`Trace::new`] is the only way to make one, so an empty trace is
/// unconstructible rather than merely discouraged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    steps: Vec<&'static str>,
}

impl Trace {
    pub fn new(first: &'static str) -> Self {
        Self { steps: vec![first] }
    }

    pub fn then(mut self, step: &'static str) -> Self {
        self.steps.push(step);
        self
    }

    pub fn steps(&self) -> &[&'static str] {
        &self.steps
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// A generated form and the reasoning behind it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prediction {
    pub text: String,
    pub trace: Trace,
}

impl Prediction {
    pub fn new(text: impl Into<String>, trace: Trace) -> Self {
        Self {
            text: text.into(),
            trace,
        }
    }
}
