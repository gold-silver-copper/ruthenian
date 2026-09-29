use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Range;

use crate::compile::{Atom, CompiledRule, Matcher, Pattern, Quantifier, ReplacementPart};
use crate::{Application, Direction};

/// A validated, executable sequence of sound laws.
#[derive(Debug, Clone)]
pub struct Program {
    rules: Vec<CompiledRule>,
}

impl Program {
    pub(crate) fn new(rules: Vec<CompiledRule>) -> Self {
        Self { rules }
    }

    /// The number of ordered rules in this program.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Iterate over stable rule names in declaration order.
    pub fn rule_names(&self) -> impl ExactSizeIterator<Item = &str> {
        self.rules.iter().map(|rule| rule.name.as_str())
    }

    /// Compose two compiled programs, applying `self` before `next`.
    ///
    /// Stable rule names must remain unique so traces and named ranges are
    /// unambiguous.
    pub fn then(&self, next: &Program) -> Result<Program, ProgramCompositionError> {
        let mut names: HashSet<&str> = self.rule_names().collect();
        if let Some(duplicate) = next.rule_names().find(|name| !names.insert(name)) {
            return Err(ProgramCompositionError {
                duplicate_rule: duplicate.to_owned(),
            });
        }
        let mut rules = Vec::with_capacity(self.rules.len() + next.rules.len());
        rules.extend(self.rules.iter().cloned());
        rules.extend(next.rules.iter().cloned());
        Ok(Program::new(rules))
    }

    /// Apply every rule once, in declaration order.
    pub fn apply(&self, input: &str) -> String {
        apply_rules(&self.rules, input)
    }

    /// Apply every rule with a deterministic limit on matcher search work.
    ///
    /// One work unit is charged for each recursive pattern state and repeated
    /// matcher state examined. This is an algorithmic guard, not wall-clock
    /// timing, so a given program and input fail reproducibly.
    pub fn apply_with_limit(
        &self,
        input: &str,
        max_steps: usize,
    ) -> Result<String, MatchLimitError> {
        let mut budget = WorkBudget::limited(max_steps);
        apply_rules_with_budget(&self.rules, input, &mut budget)
    }

    /// Apply an end-exclusive range of rules by declaration index.
    pub fn apply_range(&self, input: &str, range: Range<usize>) -> Result<String, RuleRangeError> {
        if range.start > range.end || range.end > self.rules.len() {
            return Err(RuleRangeError::OutOfBounds {
                start: range.start,
                end: range.end,
                rule_count: self.rules.len(),
            });
        }
        Ok(apply_rules(&self.rules[range], input))
    }

    /// Apply the inclusive range from `first` through `last` by rule name.
    pub fn apply_named_range(
        &self,
        input: &str,
        first: &str,
        last: &str,
    ) -> Result<String, RuleRangeError> {
        let start = self.rule_index(first)?;
        let end = self.rule_index(last)?;
        if start > end {
            return Err(RuleRangeError::Reversed {
                first: first.to_owned(),
                last: last.to_owned(),
            });
        }
        Ok(apply_rules(&self.rules[start..=end], input))
    }

    /// Apply rules from the beginning through `last`, inclusively.
    pub fn apply_through(&self, input: &str, last: &str) -> Result<String, RuleRangeError> {
        let end = self.rule_index(last)?;
        Ok(apply_rules(&self.rules[..=end], input))
    }

    /// Apply rules from `first` through the end of the program.
    pub fn apply_from(&self, input: &str, first: &str) -> Result<String, RuleRangeError> {
        let start = self.rule_index(first)?;
        Ok(apply_rules(&self.rules[start..], input))
    }

    /// Apply every rule while recording only the rules that changed the word.
    pub fn apply_with_trace(&self, input: &str) -> (String, Vec<TraceStep>) {
        let mut word = input.to_owned();
        let mut trace = Vec::new();
        let mut budget = WorkBudget::unlimited();

        for rule in &self.rules {
            let next = apply_rule(rule, &word, &mut budget);
            if next != word {
                trace.push(TraceStep {
                    rule: rule.name.clone(),
                    before: word,
                    after: next.clone(),
                });
                word = next;
            }
        }

        (word, trace)
    }

    /// Reapply the complete ordered program until it stops changing the word.
    ///
    /// Cycles are detected, and `max_passes` is a hard guard against programs
    /// that keep producing novel forms forever.
    pub fn apply_until_stable(
        &self,
        input: &str,
        max_passes: usize,
    ) -> Result<String, IterationError> {
        let mut seen = HashMap::new();
        let mut word = input.to_owned();
        seen.insert(word.clone(), 0usize);

        for pass in 1..=max_passes {
            let next = self.apply(&word);
            if next == word {
                return Ok(word);
            }
            if let Some(first_seen) = seen.insert(next.clone(), pass) {
                return Err(IterationError::Cycle {
                    form: next,
                    first_seen,
                    repeated_at: pass,
                });
            }
            word = next;
        }

        Err(IterationError::Limit {
            max_passes,
            last_form: word,
        })
    }

    fn rule_index(&self, name: &str) -> Result<usize, RuleRangeError> {
        self.rules
            .iter()
            .position(|rule| rule.name == name)
            .ok_or_else(|| RuleRangeError::UnknownRule(name.to_owned()))
    }
}

fn apply_rules(rules: &[CompiledRule], input: &str) -> String {
    let mut budget = WorkBudget::unlimited();
    apply_rules_with_budget(rules, input, &mut budget)
        .expect("an unlimited matcher budget cannot be exhausted")
}

fn apply_rules_with_budget(
    rules: &[CompiledRule],
    input: &str,
    budget: &mut WorkBudget,
) -> Result<String, MatchLimitError> {
    let mut word = input.to_owned();
    for rule in rules {
        word = apply_rule(rule, &word, budget);
        if budget.exhausted {
            return Err(MatchLimitError {
                max_steps: budget.max_steps,
                rule: rule.name.clone(),
            });
        }
    }
    Ok(word)
}

/// One changed intermediate form from [`Program::apply_with_trace`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceStep {
    /// The declarative rule name.
    pub rule: String,
    /// The word immediately before this rule.
    pub before: String,
    /// The word immediately after this rule.
    pub after: String,
}

/// Failure to reach a fixed point in [`Program::apply_until_stable`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IterationError {
    /// A form already seen during this run appeared again.
    Cycle {
        /// The repeated form.
        form: String,
        /// The zero-based pass where the form first appeared.
        first_seen: usize,
        /// The pass where it appeared again.
        repeated_at: usize,
    },
    /// The program was still changing after the requested number of passes.
    Limit {
        /// The configured pass limit.
        max_passes: usize,
        /// The last form produced before stopping.
        last_form: String,
    },
}

impl fmt::Display for IterationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cycle {
                form,
                first_seen,
                repeated_at,
            } => write!(
                f,
                "sound-law iteration cycled on `{form}` (passes {first_seen} and {repeated_at})"
            ),
            Self::Limit {
                max_passes,
                last_form,
            } => write!(
                f,
                "sound-law iteration did not stabilize in {max_passes} passes; last form was `{last_form}`"
            ),
        }
    }
}

impl std::error::Error for IterationError {}

/// An invalid index or name passed to a partial-execution API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleRangeError {
    /// No compiled rule has this name.
    UnknownRule(String),
    /// The named first rule occurs after the named last rule.
    Reversed {
        /// Requested first rule.
        first: String,
        /// Requested last rule.
        last: String,
    },
    /// An index range was reversed or extended beyond the program.
    OutOfBounds {
        /// Inclusive start index.
        start: usize,
        /// Exclusive end index.
        end: usize,
        /// Number of compiled rules.
        rule_count: usize,
    },
}

impl fmt::Display for RuleRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRule(rule) => write!(f, "unknown sound-law rule `{rule}`"),
            Self::Reversed { first, last } => write!(
                f,
                "sound-law rule range is reversed: `{first}` occurs after `{last}`"
            ),
            Self::OutOfBounds {
                start,
                end,
                rule_count,
            } => write!(
                f,
                "sound-law rule range {start}..{end} is invalid for {rule_count} rules"
            ),
        }
    }
}

impl std::error::Error for RuleRangeError {}

/// Two compiled programs cannot be composed without ambiguous rule names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramCompositionError {
    duplicate_rule: String,
}

impl ProgramCompositionError {
    /// The rule name present in both programs.
    pub fn duplicate_rule(&self) -> &str {
        &self.duplicate_rule
    }
}

impl fmt::Display for ProgramCompositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cannot compose sound-law programs with duplicate rule `{}`",
            self.duplicate_rule
        )
    }
}

impl std::error::Error for ProgramCompositionError {}

/// Matcher search exceeded the caller's deterministic work budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchLimitError {
    max_steps: usize,
    rule: String,
}

impl MatchLimitError {
    /// The configured maximum number of matcher work units.
    pub fn max_steps(&self) -> usize {
        self.max_steps
    }

    /// The rule being evaluated when the budget was exhausted.
    pub fn rule(&self) -> &str {
        &self.rule
    }
}

impl fmt::Display for MatchLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sound law `{}` exceeded its matcher work limit of {} steps",
            self.rule, self.max_steps
        )
    }
}

impl std::error::Error for MatchLimitError {}

#[derive(Debug)]
struct WorkBudget {
    remaining: Option<usize>,
    max_steps: usize,
    exhausted: bool,
}

impl WorkBudget {
    fn unlimited() -> Self {
        Self {
            remaining: None,
            max_steps: usize::MAX,
            exhausted: false,
        }
    }

    fn limited(max_steps: usize) -> Self {
        Self {
            remaining: Some(max_steps),
            max_steps,
            exhausted: false,
        }
    }

    fn spend(&mut self) -> bool {
        match self.remaining {
            None => true,
            Some(0) => {
                self.exhausted = true;
                false
            }
            Some(remaining) => {
                self.remaining = Some(remaining - 1);
                true
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct MatchState {
    position: usize,
    captures: Vec<Option<Span>>,
}

#[derive(Debug, Clone)]
struct FoundMatch {
    start: usize,
    end: usize,
    captures: Vec<Option<Span>>,
}

fn apply_rule(rule: &CompiledRule, input: &str, budget: &mut WorkBudget) -> String {
    if rule.target.atoms.is_empty() {
        return apply_insertion_rule(rule, input, budget);
    }

    let matches = select_matches(rule, input, budget);
    if matches.is_empty() {
        return input.to_owned();
    }

    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    for found in matches {
        output.push_str(&input[cursor..found.start]);
        render_replacement(rule, input, &found.captures, &mut output);
        cursor = found.end;
    }
    output.push_str(&input[cursor..]);
    output
}

fn apply_insertion_rule(rule: &CompiledRule, input: &str, budget: &mut WorkBudget) -> String {
    let mut insertions = Vec::new();
    let mut boundaries = boundaries_from(input, 0);
    if rule.direction == Direction::RightToLeft {
        boundaries.reverse();
    }
    for boundary in boundaries {
        if budget.exhausted {
            break;
        }
        if let Some(captures) = insertion_captures(rule, input, boundary, budget) {
            insertions.push((boundary, captures));
            if rule.application == Application::Once {
                break;
            }
        }
    }
    if insertions.is_empty() {
        return input.to_owned();
    }
    insertions.sort_by_key(|(boundary, _)| *boundary);

    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    let mut changed = false;

    // Unlike consuming matches, an insertion has no natural "next byte".
    // Enumerating the original boundaries exactly once gives it simultaneous
    // semantics and prevents emitted text from matching this rule recursively.
    for (boundary, captures) in insertions {
        output.push_str(&input[cursor..boundary]);
        let before_replacement = output.len();
        render_replacement(rule, input, &captures, &mut output);
        changed |= output.len() != before_replacement;
        cursor = boundary;
    }

    if !changed {
        return input.to_owned();
    }
    output.push_str(&input[cursor..]);
    output
}

fn insertion_captures(
    rule: &CompiledRule,
    input: &str,
    boundary: usize,
    budget: &mut WorkBudget,
) -> Option<Vec<Option<Span>>> {
    if rule.target.anchored_at_start && boundary != 0 {
        return None;
    }
    if rule.target.anchored_at_end && boundary != input.len() {
        return None;
    }

    let empty_captures = vec![None; rule.capture_count];
    for left_state in left_context_matches(&rule.left, input, boundary, &empty_captures, budget) {
        for right_state in
            match_pattern_from(&rule.right, input, boundary, &left_state.captures, budget)
        {
            if !is_excluded(
                rule,
                input,
                boundary,
                boundary,
                &right_state.captures,
                budget,
            ) {
                return Some(right_state.captures);
            }
        }
    }
    None
}

fn select_matches(rule: &CompiledRule, input: &str, budget: &mut WorkBudget) -> Vec<FoundMatch> {
    let mut selected = Vec::new();
    match rule.direction {
        Direction::LeftToRight => {
            let mut cursor = 0;
            while let Some(found) = find_next_match(rule, input, cursor, budget) {
                cursor = found.end;
                selected.push(found);
                if rule.application == Application::Once {
                    break;
                }
            }
        }
        Direction::RightToLeft => {
            let mut edge = input.len();
            while let Some(found) = find_previous_match(rule, input, edge, budget) {
                edge = found.start;
                selected.push(found);
                if rule.application == Application::Once {
                    break;
                }
            }
            selected.sort_by_key(|found| found.start);
        }
    }
    selected
}

fn find_next_match(
    rule: &CompiledRule,
    input: &str,
    search_from: usize,
    budget: &mut WorkBudget,
) -> Option<FoundMatch> {
    for start in boundaries_from(input, search_from) {
        if budget.exhausted {
            break;
        }
        if rule.target.anchored_at_start && start != 0 {
            break;
        }
        if let Some(found) = match_at(rule, input, start, None, budget) {
            return Some(found);
        }
    }
    None
}

fn find_previous_match(
    rule: &CompiledRule,
    input: &str,
    through: usize,
    budget: &mut WorkBudget,
) -> Option<FoundMatch> {
    for start in boundaries_through(input, through).into_iter().rev() {
        if budget.exhausted {
            break;
        }
        if rule.target.anchored_at_start && start != 0 {
            continue;
        }
        if let Some(found) = match_at(rule, input, start, Some(through), budget) {
            return Some(found);
        }
    }
    None
}

fn match_at(
    rule: &CompiledRule,
    input: &str,
    start: usize,
    must_end_by: Option<usize>,
    budget: &mut WorkBudget,
) -> Option<FoundMatch> {
    let empty_captures = vec![None; rule.capture_count];
    for left_state in left_context_matches(&rule.left, input, start, &empty_captures, budget) {
        if budget.exhausted {
            break;
        }
        for target_state in
            match_pattern_from(&rule.target, input, start, &left_state.captures, budget)
        {
            // Nullable pieces are useful inside real patterns, but consuming
            // rules must make progress. Pure insertion has a separate path.
            if target_state.position == start
                || must_end_by.is_some_and(|edge| target_state.position > edge)
            {
                continue;
            }
            for right_state in match_pattern_from(
                &rule.right,
                input,
                target_state.position,
                &target_state.captures,
                budget,
            ) {
                if !is_excluded(
                    rule,
                    input,
                    start,
                    target_state.position,
                    &right_state.captures,
                    budget,
                ) {
                    return Some(FoundMatch {
                        start,
                        end: target_state.position,
                        captures: right_state.captures,
                    });
                }
            }
        }
    }
    None
}

fn is_excluded(
    rule: &CompiledRule,
    input: &str,
    target_start: usize,
    target_end: usize,
    captures: &[Option<Span>],
    budget: &mut WorkBudget,
) -> bool {
    let (Some(left), Some(right)) = (&rule.excluded_left, &rule.excluded_right) else {
        return false;
    };

    left_context_matches(left, input, target_start, captures, budget)
        .into_iter()
        .any(|left_state| {
            !match_pattern_from(right, input, target_end, &left_state.captures, budget).is_empty()
        })
}

fn left_context_matches(
    pattern: &Pattern,
    input: &str,
    target_start: usize,
    captures: &[Option<Span>],
    budget: &mut WorkBudget,
) -> Vec<MatchState> {
    if pattern.atoms.is_empty() && !pattern.anchored_at_start && !pattern.anchored_at_end {
        return vec![MatchState {
            position: target_start,
            captures: captures.to_vec(),
        }];
    }

    let starts: Vec<usize> = if pattern.anchored_at_start {
        vec![0]
    } else {
        boundaries_through(input, target_start)
    };
    let mut matches = Vec::new();
    for start in starts {
        if budget.exhausted {
            break;
        }
        matches.extend(
            match_pattern_from(pattern, input, start, captures, budget)
                .into_iter()
                .filter(|state| state.position == target_start),
        );
    }
    matches
}

fn match_pattern_from(
    pattern: &Pattern,
    input: &str,
    start: usize,
    captures: &[Option<Span>],
    budget: &mut WorkBudget,
) -> Vec<MatchState> {
    if pattern.anchored_at_start && start != 0 {
        return Vec::new();
    }
    let mut matches = Vec::new();
    match_atoms(pattern, 0, input, start, captures, &mut matches, budget);
    matches
}

fn match_atoms(
    pattern: &Pattern,
    atom_index: usize,
    input: &str,
    position: usize,
    captures: &[Option<Span>],
    matches: &mut Vec<MatchState>,
    budget: &mut WorkBudget,
) {
    if !budget.spend() {
        return;
    }
    if atom_index == pattern.atoms.len() {
        if !pattern.anchored_at_end || position == input.len() {
            matches.push(MatchState {
                position,
                captures: captures.to_vec(),
            });
        }
        return;
    }

    let atom = &pattern.atoms[atom_index];
    for end in repetition_ends(atom, input, position, captures, budget) {
        let mut next_captures = captures.to_vec();
        if let Some(capture) = atom.capture {
            next_captures[capture] = Some(Span {
                start: position,
                end,
            });
        }
        match_atoms(
            pattern,
            atom_index + 1,
            input,
            end,
            &next_captures,
            matches,
            budget,
        );
    }
}

fn repetition_ends(
    atom: &Atom,
    input: &str,
    start: usize,
    captures: &[Option<Span>],
    budget: &mut WorkBudget,
) -> Vec<usize> {
    let (minimum, maximum) = match atom.quantifier {
        Quantifier::One => (1, Some(1)),
        Quantifier::Optional => (0, Some(1)),
        Quantifier::ZeroOrMore => (0, None),
        Quantifier::OneOrMore => (1, None),
    };

    let mut candidates = Vec::new();
    let mut frontier = vec![(0usize, start)];
    let mut visited = HashSet::from([(0usize, start)]);

    while let Some((count, position)) = frontier.pop() {
        if !budget.spend() {
            break;
        }
        if count >= minimum && maximum.is_none_or(|limit| count <= limit) {
            candidates.push((count, position));
        }
        if maximum.is_some_and(|limit| count >= limit) {
            continue;
        }

        for end in unit_ends(&atom.matcher, input, position, captures) {
            // Quantified matchers must make progress. A one-shot empty
            // backreference remains valid, but cannot make `*` loop forever.
            if end == position && maximum.is_none() {
                continue;
            }
            let next = (count + 1, end);
            if visited.insert(next) {
                frontier.push(next);
            }
        }
    }

    // Greedy means greatest consumed span first. Count breaks ties for class
    // inventories whose members admit more than one segmentation.
    candidates.sort_by(|(left_count, left_end), (right_count, right_end)| {
        right_end
            .cmp(left_end)
            .then_with(|| right_count.cmp(left_count))
    });
    candidates.into_iter().map(|(_, end)| end).collect()
}

fn unit_ends(
    matcher: &Matcher,
    input: &str,
    position: usize,
    captures: &[Option<Span>],
) -> Vec<usize> {
    let rest = &input[position..];
    match matcher {
        Matcher::Literal(literal) => rest
            .starts_with(literal)
            .then_some(vec![position + literal.len()])
            .unwrap_or_default(),
        Matcher::Class(members) => members
            .iter()
            .filter(|member| rest.starts_with(member.as_str()))
            .map(|member| position + member.len())
            .collect(),
        Matcher::NotClass(members) => {
            if members
                .iter()
                .any(|member| rest.starts_with(member.as_str()))
            {
                Vec::new()
            } else {
                rest.chars()
                    .next()
                    .map(|ch| vec![position + ch.len_utf8()])
                    .unwrap_or_default()
            }
        }
        Matcher::Any => rest
            .chars()
            .next()
            .map(|ch| vec![position + ch.len_utf8()])
            .unwrap_or_default(),
        Matcher::WordBoundary => is_word_boundary(input, position)
            .then_some(vec![position])
            .unwrap_or_default(),
        Matcher::Backreference(capture) => {
            let Some(span) = captures[*capture] else {
                return Vec::new();
            };
            let captured = &input[span.start..span.end];
            rest.starts_with(captured)
                .then_some(vec![position + captured.len()])
                .unwrap_or_default()
        }
    }
}

fn is_word_boundary(input: &str, position: usize) -> bool {
    let previous = input[..position].chars().next_back();
    let next = input[position..].chars().next();
    match (previous, next) {
        (None, _) | (_, None) => true,
        (Some(previous), Some(next)) => previous.is_whitespace() != next.is_whitespace(),
    }
}

fn render_replacement(
    rule: &CompiledRule,
    input: &str,
    captures: &[Option<Span>],
    output: &mut String,
) {
    for part in &rule.replacement {
        match part {
            ReplacementPart::Literal(literal) => output.push_str(literal),
            ReplacementPart::Capture(capture) => {
                let span = captures[*capture]
                    .expect("compiled replacement captures are set by every successful match");
                output.push_str(&input[span.start..span.end]);
            }
            ReplacementPart::MappedCapture { capture, entries } => {
                let span = captures[*capture]
                    .expect("compiled replacement captures are set by every successful match");
                let captured = &input[span.start..span.end];
                let (_, mapped) = entries
                    .iter()
                    .find(|(source, _)| source == captured)
                    .expect("map coverage is checked while compiling the capture");
                output.push_str(mapped);
            }
        }
    }
}

fn boundaries_from(input: &str, from: usize) -> Vec<usize> {
    input
        .char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(input.len()))
        .filter(|index| *index >= from)
        .collect()
}

fn boundaries_through(input: &str, through: usize) -> Vec<usize> {
    input
        .char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(input.len()))
        .take_while(|index| *index <= through)
        .collect()
}
