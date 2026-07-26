//! Productive Russian morphology as pure rules, in Ruthenian orthography.
//!
//! ```
//! use ruthenian_core::{noun, AccentPattern, Animacy, Case, Gender, NounClass, Number, StemClass};
//!
//! let class = NounClass { stem: StemClass::Hard, accent: AccentPattern::A, reducible: false };
//! let form = noun("stól", class, Gender::Masculine, Animacy::Inanimate, Case::Gen, Number::Singular);
//! assert_eq!(form.unwrap().text, "stóla");
//! ```
//!
//! # Two jobs, one implementation
//!
//! This crate is both the **runtime fallback** for any lemma the generated
//! tables do not list, and the extractor's **predictor**: at table-generation
//! time any attested form these rules already produce is dropped, so the tables
//! hold exactly the exceptions.
//!
//! That duality is a hard contract. **Changing a rule here changes what counts
//! as irregular and requires regenerating the tables.**
//!
//! # No lexical data
//!
//! Nothing here knows about individual words. If a fact is about one lemma
//! rather than about a class of them, it belongs in the lexicon and arrives as a
//! [`types::PrincipalPartsRef`].
//!
//! # Three outcomes, kept distinct
//!
//! | Result | Meaning |
//! |---|---|
//! | `Some(Prediction)` / `Ok(Some(_))` | the form |
//! | `None` / `Ok(None)` | **the cell does not exist** — a perfective verb has no present tense |
//! | `Err(Unsupported)` | the rules do not cover this class; never a wrong form |
//!
//! `None` never means "unimplemented". That is what `Err` is for, and keeping
//! them apart is what lets a caller trust a `None`.
//!
//! # Coverage
//!
//! Verb classes 1–6 are implemented, which is **90.7 %** of Russian verb lemmas
//! carrying a class code (11 584 of 12 773); every other class returns
//! [`verb::Unsupported`]. **All six** noun accent patterns place stress, from a
//! table derived over 285 000 attested forms rather than quoted from a grammar.
//!
//! Accuracy on the random held-out sample — 5 747 attested cells drawn with a
//! fixed seed from the whole dump, no hand-picking:
//!
//! | | segmental | strict (with stress) |
//! |---|---:|---:|
//! | adjective | 96.4 % | 91.6 % |
//! | noun | 89.9 % | 83.4 % |
//! | verb | 89.2 % | 86.9 % |
//! | **all** | **90.5 %** | **86.5 %** |
//!
//! The targeted fixture scores lower by design: it is a regression net over the
//! hard tail, not a sample of the language (`INVARIANTS.md` I3).
//!
//! Every figure here is measured over the **entire** Wiktionary dump — see
//! `INVARIANTS.md`, which forbids sampling. `tools/measure.py` recomputes them.

#![forbid(unsafe_code)]

pub mod adjective;
pub mod class;
pub mod noun;
pub mod numeral;
pub mod phono;
pub mod policy;
pub mod pronoun;
pub mod types;
pub mod verb;

pub use adjective::adjective;
pub use class::{ClassParseError, Conjugation, ZaliznyakVerbClass};
pub use noun::noun;
pub use numeral::{Government, government};
pub use policy::{Policy, Prediction, RuleId, Trace};
pub use pronoun::third_person;
pub use types::{
    AccentPattern, AdjClass, AdjForm, Animacy, Aspect, Case, Gender, NounClass, Number,
    ParticipleKind, Person, PersonNumber, PrincipalPartsRef, PronounStyle, Slot, StemClass, Tense,
    VerbSlot, Voice,
};
pub use verb::{Resolved, Unsupported, VerbInfo, slot_exists, verb};
