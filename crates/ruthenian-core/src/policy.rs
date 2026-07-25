//! Regularization: the `RuleId` registry, `Policy`, and the trace every
//! prediction carries.
//!
//! Every departure from standard Russian lives here, next to the rule it
//! modifies, identified by a [`RuleId`] and switched by [`Policy`]. There is no
//! separate "standard Ruthenian" code path: a second generation pipeline would
//! diverge from this one silently, and nobody would know which the published
//! numbers described.
//!
//! Every rule is **off in both presets** until phase 6 prices it, and is
//! reachable only through `Policy::attested().with(rule)`.

/// Stable identifier for one regularization rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleId(pub &'static str);

impl core::fmt::Display for RuleId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.0)
    }
}

/// One entry in the registry. `docs/REGULARIZATION.md` is generated from these,
/// never hand-written.
#[derive(Debug, Clone, Copy)]
pub struct RuleDef {
    pub id: RuleId,
    pub summary: &'static str,
    pub detail: &'static str,
}

/// Fill the first-person singular **future** of a defective perfective verb.
///
/// Targets exactly the lexical gap that the source marks with an explicit
/// override argument — `победить` carries `futr_1sg: "-"`, because `*побежу` is
/// avoided. It must **never** touch a present-tense slot: a perfective verb has
/// no present tense at all, and filling those would invent one and destroy the
/// aspect distinction. Structural gaps are derived, not filled.
pub const GAP_FILL_DEFECTIVE_1SG: RuleId = RuleId("gap.fill-defective-1sg");

/// Apply a class's present-stem mutation uniformly, including to the lemmas that
/// escape it in standard Russian.
pub const IOTATION_UNIFORM: RuleId = RuleId("iotation.uniform");

/// Replace mobile stress with fixed stem stress.
pub const STRESS_FIXED_STEM: RuleId = RuleId("stress.fixed-stem");

/// The registry. Adding a rule means adding a row here; the generated register
/// reads this list.
pub const RULES: &[RuleDef] = &[
    RuleDef {
        id: GAP_FILL_DEFECTIVE_1SG,
        summary: "fill the 1sg future of defective perfectives",
        detail: "Standard Russian avoids a first-person singular future for a \
                 small set of perfective verbs (pobjeditj). The slot is \
                 grammatically available; only usage declines to fill it. This \
                 rule generates the regular form. It never touches present-tense \
                 slots, which are structurally absent for perfectives.",
    },
    RuleDef {
        id: IOTATION_UNIFORM,
        summary: "apply the class mutation to lemmas that escape it",
        detail: "Some lemmas resist the present-stem mutation their class \
                 otherwise imposes. This rule levels them onto the class.",
    },
    RuleDef {
        id: STRESS_FIXED_STEM,
        summary: "fixed stem stress instead of mobile stress",
        detail: "Accent patterns b-f move stress across the paradigm. This rule \
                 collapses them onto pattern a.",
    },
];

/// Which regularizations are active.
///
/// `attested()` reproduces standard Russian and is the evaluator's baseline; if
/// it drifts, every later accuracy number becomes meaningless.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Policy {
    enabled: Vec<RuleId>,
}

impl Policy {
    /// Standard Russian. No departures.
    pub fn attested() -> Self {
        Self {
            enabled: Vec::new(),
        }
    }

    /// Every departure that has been priced by the evaluator — which, until
    /// phase 6 runs, is none. Deliberately identical to [`Policy::attested`] for
    /// now: no rule ships enabled before its impact is measured.
    pub fn regularized() -> Self {
        Self::attested()
    }

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

    /// The regularization rules that changed this form. Empty means the form is
    /// what the attested rules produce.
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
