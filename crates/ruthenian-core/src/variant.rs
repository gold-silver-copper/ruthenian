//! Optional features: the `RuleId` registry, [`Variant`], and the trace every
//! prediction carries.
//!
//! # The regularizations are not switchable
//!
//! Ruthenian's departures from its source languages — three declensions instead
//! of eight, six verb classes instead of sixteen, fixed stress, derived aspect,
//! no indeclinables — are not options. They are what the language *is*
//! (`RUTHENIAN.md` §1). There is no preset that turns them off, because the
//! thing on the other side of that switch would be Russian, and this crate does
//! not generate Russian.
//!
//! What [`Variant`] switches is the set of questions `RUTHENIAN.md` §13 still
//! calls **open**. Each is coherent on its own, and none may ship enabled while
//! the specification still lists it as undecided: the spec moves first, and the
//! code follows.

/// Stable identifier for one optional-feature rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleId(pub &'static str);

impl core::fmt::Display for RuleId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

/// One entry in the registry. `docs/VARIANTS.md` is generated from these, never
/// hand-written.
#[derive(Debug, Clone, Copy)]
pub struct RuleDef {
    pub id: RuleId,
    /// The `RUTHENIAN.md` §13 item this feature would settle.
    pub spec_item: u8,
    pub summary: &'static str,
    pub detail: &'static str,
}

/// Revive a distinct ablative plural (`RUTHENIAN.md` §13 item 2).
///
/// No attested language distinguishes it, and the standard variant follows:
/// ablative = dative in the plural. A maximally conservative reading could
/// revive PIE `*-ios`.
pub const ABL_PLURAL_DISTINCT: RuleId = RuleId("abl.plural-distinct");

/// A full/clitic pronoun opposition (`RUTHENIAN.md` §13 item 3).
///
/// OCS, Sanskrit and Interslavic all have one; Ruthenian does not. Restoring it
/// adds a [`crate::PronounStyle`] value, not a case.
pub const PRON_CLITIC_SERIES: RuleId = RuleId("pron.clitic-series");

/// The middle voice (`RUTHENIAN.md` §13 item 4).
///
/// Lost in all Slavic, its work done by `-sja`. The most radical available
/// conservatism, and the one with the largest surface.
pub const VOICE_MIDDLE: RuleId = RuleId("voice.middle");

/// The registry. Adding a feature means adding a row here; the generated
/// register reads this list.
///
/// The supine is deliberately **absent**: §7.10a specifies it completely, so it
/// is part of the standard language rather than an option. See
/// `docs/specs/ruthenian-core.md` §7 for the §13-item-7 discrepancy.
pub const RULES: &[RuleDef] = &[
    RuleDef {
        id: ABL_PLURAL_DISTINCT,
        spec_item: 2,
        summary: "a distinct ablative plural",
        detail: "The standard language syncretizes ablative with dative in the \
                 dual and plural, following every attested language. This rule \
                 revives a distinct plural ablative from PIE *-ios.",
    },
    RuleDef {
        id: PRON_CLITIC_SERIES,
        spec_item: 3,
        summary: "a full/clitic pronoun opposition",
        detail: "OCS, Sanskrit and Interslavic distinguish stressed full \
                 pronouns from unstressed clitics. Ruthenian has one series. \
                 This rule adds the second, as a PronounStyle rather than a case.",
    },
    RuleDef {
        id: VOICE_MIDDLE,
        spec_item: 4,
        summary: "the middle voice",
        detail: "Lost in all Slavic, where -sja does its work. Restoring it \
                 would be the most conservative single change available, and \
                 touches every verb paradigm.",
    },
];

/// Which optional features are active.
///
/// [`Variant::standard`] is the language exactly as `RUTHENIAN.md` specifies it.
/// It is the conformance baseline; if it drifts from that document, every later
/// conformance number becomes meaningless.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Variant {
    enabled: Vec<RuleId>,
}

impl Variant {
    /// The language as specified. No optional features.
    pub fn standard() -> Self {
        Self {
            enabled: Vec::new(),
        }
    }

    // NOTE: there is deliberately no `conservative()` or `maximal()` preset.
    //
    // Every rule here is off until the specification settles it, so such a
    // preset would today be identical to `standard()` — and a preset that
    // silently changes meaning is exactly what a consumer blesses into a test
    // and then depends on. Name the features you want:
    // `Variant::standard().with(VOICE_MIDDLE)`.

    pub fn with(mut self, rule: RuleId) -> Self {
        if !self.enabled.contains(&rule) {
            self.enabled.push(rule);
        }
        self
    }

    pub fn without(mut self, rule: RuleId) -> Self {
        self.enabled.retain(|r| *r != rule);
        self
    }

    pub fn has(&self, rule: RuleId) -> bool {
        self.enabled.contains(&rule)
    }

    pub fn active(&self) -> &[RuleId] {
        &self.enabled
    }

    pub fn is_standard(&self) -> bool {
        self.enabled.is_empty()
    }
}

/// Why a form looks the way it does: the rules that fired, in order.
///
/// Never empty. The evaluator attributes mismatches through it and the CLI
/// explains deviations with it; returning a bare string would force both to
/// re-derive what the engine already knew.
/// Deliberately **not** `Default`: a trace with no steps would let a prediction
/// arrive with no explanation, and every consumer downstream depends on there
/// being one. [`Trace::new`] is the only way to make one, so an empty trace is
/// unconstructible rather than merely discouraged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    steps: Vec<&'static str>,
    rules: Vec<RuleId>,
}

impl Trace {
    pub fn new(first: &'static str) -> Self {
        Self {
            steps: vec![first],
            rules: Vec::new(),
        }
    }

    pub fn then(mut self, step: &'static str) -> Self {
        self.steps.push(step);
        self
    }

    pub fn rule(mut self, rule: RuleId) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn steps(&self) -> &[&'static str] {
        &self.steps
    }

    /// The optional-feature rules that changed this form. Empty means the
    /// form is the standard language's.
    pub fn rules(&self) -> &[RuleId] {
        &self.rules
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
