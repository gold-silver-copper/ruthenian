//! Numerals, and the case and number a count imposes on what it counts.

use crate::policy::{Prediction, Trace};
use crate::types::{Case, Number};

/// What a count does to the noun it governs.
///
/// Returned as **structure, not a string**: a caller declining an agreeing
/// adjective needs the case and number the slot resolved to, and re-deriving the
/// rules locally is how consumers get it wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Government {
    pub case: Case,
    pub number: Number,
}

/// The government of a cardinal in a direct (nominative/accusative) slot.
///
/// ```
/// use ruthenian_core::numeral::government;
/// use ruthenian_core::types::*;
/// assert_eq!(government(1).case, Case::Nom);
/// assert_eq!(government(3).number, Number::Singular);  // 2-4 take genitive singular
/// assert_eq!(government(5).number, Number::Plural);    // 5+ take genitive plural
/// ```
pub fn government(n: u64) -> Government {
    let last_two = n % 100;
    let last = n % 10;
    if (11..=14).contains(&last_two) {
        return Government {
            case: Case::Gen,
            number: Number::Plural,
        };
    }
    match last {
        1 => Government {
            case: Case::Nom,
            number: Number::Singular,
        },
        2..=4 => Government {
            case: Case::Gen,
            number: Number::Singular,
        },
        _ => Government {
            case: Case::Gen,
            number: Number::Plural,
        },
    }
}

/// Decline the small cardinals that inflect.
pub fn cardinal(n: u64, case: Case) -> Option<Prediction> {
    let forms: &[&str] = match n {
        1 => &["odin", "odnogo", "odnomu", "odin", "odnim", "odnom"],
        2 => &["dva", "dvuh", "dvum", "dva", "dvumja", "dvuh"],
        3 => &["tri", "trjoh", "trjom", "tri", "trjemja", "trjoh"],
        4 => &[
            "cztyrje",
            "cztyrjoh",
            "cztyrjom",
            "cztyrje",
            "cztyrjmja",
            "cztyrjoh",
        ],
        5 => &["pjatj", "pjati", "pjati", "pjatj", "pjatju", "pjati"],
        _ => return None,
    };
    let i = match case {
        Case::Nom => 0,
        Case::Gen => 1,
        Case::Dat => 2,
        Case::Acc => 3,
        Case::Ins => 4,
        Case::Loc => 5,
    };
    forms
        .get(i)
        .map(|f| Prediction::new(*f, Trace::new("cardinal numeral paradigm")))
}
