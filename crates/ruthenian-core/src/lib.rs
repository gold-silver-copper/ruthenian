//! The inflectional morphology of Ruthenian: **give it a word and some grammar;
//! it gives you the form.**
//!
//! ```
//! use ruthenian_core::*;
//!
//! assert_eq!(noun("dom", Case::Genitive, Number::Singular), "domogo"); // OF the house
//! assert_eq!(noun("dom", Case::Ablative, Number::Singular), "doma");   // FROM the house
//! assert_eq!(noun("dom", Case::Nominative, Number::Dual), "doma");     // two houses
//! ```
//!
//! Everything a shape cannot predict is in the lemma itself (§2.1):
//!
//! ```
//! use ruthenian_core::*;
//!
//! // A capital first letter marks an animate noun.
//! assert_eq!(noun("Drug", Case::Accusative, Number::Singular), "druga");
//! assert_eq!(noun("drug", Case::Accusative, Number::Singular), "drug");
//!
//! // Output is always lowercase; sentence capitalisation is the caller's business.
//! assert_eq!(noun("Drug", Case::Nominative, Number::Singular), "drug");
//!
//! // A word-final `'` marks a lemma that is not what its ending predicts.
//! assert_eq!(noun("noczj'", Case::Genitive, Number::Singular), "noczi");
//! ```
//!
//! # What this crate is
//!
//! Everything is computed from rules. There is no dictionary here, no data
//! files, no lookup tables, no network, no I/O of any kind. A word the crate has
//! never seen inflects exactly as well as one it has.
//!
//! It is measured against **the specification**, `docs/RUTHENIAN.md`, because
//! there is no other authority: Ruthenian is specified, not attested, and no
//! corpus of it exists or ever will. Section references throughout are to that
//! document.
//!
//! # Totality
//!
//! **Every function is total.** No `Option`, no `Result`, no panic: any
//! combination of arguments the types permit returns a string. Where the
//! language has no form for a cell, the function returns a *declared*
//! substitute — see [`fallback`] for the complete list, which is the only place
//! a substitute may be introduced.

#![forbid(unsafe_code)]

pub mod adjective;
pub mod fallback;
pub mod grammar;
mod lemma;
pub mod noun;
pub mod numeral;
pub mod pronoun;
pub mod spelling;
pub mod verb;

pub use adjective::{Adjective, adjective, comparative, short_adjective, superlative};
pub use grammar::{Animacy, Case, Gender, Number, Person};
pub use noun::{Noun, noun};
pub use numeral::{numeral, ordinal};
pub use pronoun::{
    clitic_pronoun, clitic_reflexive, pronominal, pronoun, pronoun_paradigm, reflexive, relative,
    that, this, what, who,
};
pub use verb::{
    bytj, future_auxiliary, imperative, infinitive, l_participle, past_active_participle,
    past_gerund, past_passive_participle, present_active_participle, present_gerund,
    present_passive_participle, verb, verb_paradigm,
};
