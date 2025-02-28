use crate::grammar::*;
use crate::irregular_verbs::*;
use crate::utils::*;
use crate::verb_endings::*;

//VERB STUFF
impl RUTH {
    pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
        let infinitive_stem = RUTH::get_infinitive_stem(infinitive);
        let irregular = irregular_present_stem(infinitive);

        if irregular != "" {
            if irregular.ends_with("me") {
                return (
                    RUTHUTILS::replace_last_occurence(&irregular, "me", "m"),
                    Conjugation::First,
                );
            }
            if irregular.ends_with("ne") {
                return (
                    RUTHUTILS::replace_last_occurence(&irregular, "ne", "n"),
                    Conjugation::First,
                );
            }

            if irregular.ends_with("je") {
                return (
                    RUTHUTILS::replace_last_occurence(&irregular, "je", "j"),
                    Conjugation::First,
                );
            }

            if irregular.ends_with("e") {
                return (
                    RUTHUTILS::replace_last_occurence(&irregular, "e", ""),
                    Conjugation::First,
                );
            }
            if irregular.ends_with("i") {
                return (
                    RUTHUTILS::replace_last_occurence(&irregular, "i", ""),
                    Conjugation::Second,
                );
            }
        }

        if RUTHUTILS::is_consonant(&RUTHUTILS::last_in_stringslice(&infinitive_stem)) {
            (infinitive_stem, Conjugation::First)
        } else if infinitive.ends_with("ovatj") {
            (infinitive.replace("ovatj", "uj"), Conjugation::First)
        } else if infinitive.ends_with("evatj") {
            (infinitive.replace("evatj", "uj"), Conjugation::First)
        } else if infinitive.ends_with("nutj") {
            (infinitive.replace("nutj", "n"), Conjugation::First)
        } else if infinitive.ends_with("atj") {
            (
                RUTHUTILS::replace_last_occurence(infinitive, "atj", "aj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("etj") {
            (
                RUTHUTILS::replace_last_occurence(infinitive, "etj", "ej"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("entj") {
            (
                RUTHUTILS::replace_last_occurence(infinitive, "entj", "n"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("ytj") {
            (
                RUTHUTILS::replace_last_occurence(infinitive, "ytj", "yj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("utj") {
            (
                RUTHUTILS::replace_last_occurence(infinitive, "utj", "uj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("itj") {
            (
                RUTHUTILS::replace_last_occurence(infinitive, "itj", ""),
                Conjugation::Second,
            )
        } else if infinitive.ends_with("jetj") {
            (
                RUTHUTILS::replace_last_occurence(infinitive, "jetj", ""),
                Conjugation::Second,
            )
        } else {
            panic!("IMPROPER PERSENT TENSE STEM: {}", infinitive);
        }
    }
    pub fn get_infinitive_stem(word: &str) -> String {
        RUTHUTILS::string_without_last_n(word, 2)
    }

    pub fn conjugate_verb(
        word: &str,
        person: &Person,
        number: &Number,
        gender: &Gender,
        tense: &Tense,
    ) -> Verb {
        let word = word.to_lowercase();
        let (present_stem, conjugation) = RUTH::get_present_tense_stem(&word);
        let infinitive_stem = RUTH::get_infinitive_stem(&word);

        let endings = match conjugation {
            Conjugation::First => &FIRST_CONJUGATION,
            Conjugation::Second => &SECOND_CONJUGATION,
        };

        match tense {
            Tense::Present => {
                let ending = endings.ending(person, number);
                let merged = RUTHUTILS::iotation_merge(&present_stem, ending);
                merged
            }

            _ => panic!("TENSE NOT IMPLEMENTED"),
        }
    }
    pub fn l_participle(word: &str, gender: &Gender, number: &Number) -> Verb {
        if word == "idti" {
            match number {
                Number::Singular => String::from("szli"),
                Number::Plural => match gender {
                    Gender::Masculine => String::from("szol"),
                    Gender::Feminine => String::from("szla"),
                    Gender::Neuter => String::from("szlo"),
                },
            }
        } else {
            let infinitive_stem = RUTH::get_infinitive_stem(word);
            match number {
                Number::Plural => {
                    format!("{}{}", infinitive_stem, "li")
                }
                Number::Singular => match gender {
                    Gender::Masculine => {
                        format!("{}{}", infinitive_stem, "l")
                    }
                    Gender::Feminine => {
                        format!("{}{}", infinitive_stem, "la")
                    }
                    Gender::Neuter => {
                        format!("{}{}", infinitive_stem, "lo")
                    }
                },
            }
        }
    }
}
