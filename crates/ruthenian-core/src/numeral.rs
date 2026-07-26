//! Numerals, and the case and number a count imposes on what it counts
//! (`RUTHENIAN.md` §6).

use crate::types::{Case, Number};
use crate::variant::Trace;

/// What a count does to the noun it governs.
///
/// Returned as **structure, not a string**: a caller declining an agreeing
/// adjective needs the case and number the slot resolved to, and re-deriving the
/// rules locally is how consumers get it wrong. That is law 12, and the mistake
/// `interslavic` shipped `quantified_parts` to fix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Government {
    pub case: Case,
    pub number: Number,
    pub trace: Trace,
}

/// The government of a cardinal in a direct (nominative/accusative) slot.
///
/// **This is where the dual pays for itself.** Russian's genitive singular after
/// 2–4 is petrified dual agreement; with a real dual the rule is not simplified,
/// it disappears (§6.1):
///
/// | Numeral | Governs |
/// |---|---|
/// | `odin` | agrees, singular |
/// | **`dva`** | **the dual** |
/// | `tri`, `czjetyrje` | nominative plural |
/// | `pjatj` and above | genitive plural |
///
/// There is **no 11–14 exception and no last-digit rule**: five and above always
/// take the genitive plural. A compound numeral is governed by its **last word**.
///
/// ```
/// use ruthenian_core::{government, Case, Number};
///
/// assert_eq!(government(1).number, Number::Singular);
/// assert_eq!(government(2).number, Number::Dual);      // dva doma
/// assert_eq!(government(3).number, Number::Plural);
/// assert_eq!(government(3).case,   Case::Nom);         // tri domy
/// assert_eq!(government(5).case,   Case::Gen);         // pjatj domov
///
/// // no 11-14 exception: the Russian irregularity is simply absent
/// assert_eq!(government(12).case, Case::Gen);
/// assert_eq!(government(12).number, Number::Plural);
///
/// // a compound is governed by its last word
/// assert_eq!(government(22).number, Number::Dual);     // dvadcatj dva doma
/// assert_eq!(government(25).case, Case::Gen);          // dvadcatj pjatj domov
/// ```
pub fn government(n: u64) -> Government {
    // The last word of a compound is what governs, so only the final digit
    // matters — and unlike Russian there is no 11-14 window to carve out first.
    let last = n % 10;
    let teen = (11..=19).contains(&(n % 100));

    let (case, number, why) = match last {
        _ if teen => (
            Case::Gen,
            Number::Plural,
            "a teen ends in a word of its own and takes the genitive plural",
        ),
        1 => (
            Case::Nom,
            Number::Singular,
            "odin agrees with what it counts",
        ),
        2 => (
            Case::Nom,
            Number::Dual,
            "dva governs the dual — the inherited agreement, restored",
        ),
        3 | 4 => (
            Case::Nom,
            Number::Plural,
            "tri and czjetyrje govern the nominative plural",
        ),
        _ => (
            Case::Gen,
            Number::Plural,
            "five and above govern the genitive plural",
        ),
    };

    Government {
        case,
        number,
        trace: Trace::new(why),
    }
}
