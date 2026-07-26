//! The productive morphology of **Ruthenian** as pure rules, plus the
//! grammatical vocabulary the rest of the workspace shares.
//!
//! ```
//! use ruthenian_core::{noun, Animacy, Case, Declension, Gender, NounClass, Number};
//!
//! let class = NounClass::hard(Declension::II);
//! let form = |case, number| {
//!     noun("dom", class, Gender::Masculine, Animacy::Inanimate, case, number).unwrap().text
//! };
//!
//! assert_eq!(form(Case::Gen, Number::Singular), "domogo");  // OF the house
//! assert_eq!(form(Case::Abl, Number::Singular), "doma");    // FROM the house
//! assert_eq!(form(Case::Nom, Number::Dual), "doma");        // two houses
//! ```
//!
//! # The specification is the ground truth
//!
//! The language is defined by [`docs/RUTHENIAN.md`], which is **normative**:
//! eight cases, three numbers, three declensions, six conjugation classes, three
//! past tenses. Where this crate's output disagrees with that document, this
//! crate is wrong — and `spec_paradigms_match` fails, because it reads the
//! document's own tables.
//!
//! Ruthenian is specified, not attested. There is no corpus of it and never will
//! be, so nothing here is measured against a natural language. Russian,
//! Ukrainian, Belarusian, Polish and OCS supply the cognates a *lemma* is
//! reconstructed from — that is `ruthenian-extract`'s job — and they supply no
//! expected outputs at all.
//!
//! # This is not Russian morphology
//!
//! Most of a Ruthenian paradigm is cells Russian does not have. Anything ported
//! from a Russian implementation will be silently wrong, and these are the
//! places it will happen:
//!
//! | | Ruthenian | Russian |
//! |---|---|---|
//! | cases | 8 — with the **ablative** and a productive **vocative** | 6 |
//! | numbers | 3 — the **dual** throughout, including verb agreement | 2 |
//! | declensions | 3, hard/soft | 8 |
//! | verb classes | 6 | 16 |
//! | past tenses | 3 — **aorist**, **imperfect**, perfect | 1 |
//! | 2nd palatalization | kept: `drug` → loc. `druzi` | lost (0 %) |
//! | stress | fixed, one position per word | 10 mobile patterns |
//! | aspect | **derived** from surface shape | lexical, stored in pairs |
//!
//! # Two jobs, one implementation
//!
//! This crate is both the **runtime fallback** for any lemma the generated
//! tables do not list, and the extractor's **predictor**: at table-generation
//! time any form these rules already produce is dropped, so the tables hold
//! exactly the residue.
//!
//! That duality is a hard contract. **Changing a rule here changes what counts
//! as irregular and requires regenerating the tables.**
//!
//! # No lexical data
//!
//! Nothing here knows about individual words, and nothing here knows that
//! Wiktionary exists. If a fact is about one lemma rather than a class of them,
//! it belongs in the lexicon and arrives as an argument.
//!
//! # Three outcomes, kept distinct
//!
//! | Result | Meaning |
//! |---|---|
//! | `Some(Prediction)` / `Ok(Some(_))` | the form |
//! | `None` / `Ok(None)` | **the cell does not exist** — a perfective verb has no present tense |
//! | `Err(Unsupported)` | the rules do not cover this input; never a wrong form |
//!
//! `None` never means "unimplemented". That is what `Err` is for, and keeping
//! them apart is what lets a caller trust a `None`.
//!
//! [`docs/RUTHENIAN.md`]: https://github.com/gold-silver-copper/ruthenian/blob/main/docs/RUTHENIAN.md

#![forbid(unsafe_code)]

pub mod adjective;
pub mod case_endings;
pub mod noun;
pub mod numeral;
pub mod paradigm;
pub mod phono;
pub mod pronoun;
pub mod trace;
pub mod types;
pub mod verb;

pub use adjective::adjective;
pub use noun::noun;
pub use numeral::{Government, government};
pub use paradigm::{AdjParadigm, NounParadigm, VerbParadigm, adj_forms, noun_forms, verb_forms};
pub use phono::Palatal;
pub use pronoun::{personal, reflexive};
pub use trace::{Prediction, Trace};
pub use types::{
    AdjClass, AdjForm, Animacy, Aspect, Case, Conjugation, Declension, Degree, Gender, NounClass,
    Number, ParticipleKind, Person, PersonNumber, PrincipalPartsRef, PronounStyle, Slot,
    StemHardness, Tense, VerbClass, VerbSlot, Voice,
};
pub use verb::{Derived, Resolved, Unsupported, VerbInfo, aspect_of, slot_exists, verb, verb_with};
