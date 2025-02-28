use crate::case_endings::*;
use crate::grammar::*;
use crate::known_nouns::*;
use crate::utils::*;

//NOUN STUFF
impl RUTH {
    pub fn decline_noun(word: &str, case: &Case, number: &Number) -> Noun {
        let word = word.to_lowercase();
        let gender = RUTH::guess_gender(&word);
        let word_is_animate = RUTH::noun_is_animate(&word);
        let word_stem_is_soft = RUTH::stem_of_noun_is_soft(&word);
        let word_stem = RUTH::get_noun_stem(&word, number);

        let endings = match gender {
            Gender::Masculine => {
                if word_is_animate {
                    if word_stem_is_soft {
                        &ANIMATE_SOFT_ENDINGS
                    } else {
                        &ANIMATE_HARD_ENDINGS
                    }
                } else {
                    if word_stem_is_soft {
                        &INANIMATE_SOFT_ENDINGS
                    } else {
                        &INANIMATE_HARD_ENDINGS
                    }
                }
            }
            Gender::Feminine => {
                if word_stem_is_soft {
                    &FEMININE_SOFT_ENDINGS
                } else {
                    &FEMININE_HARD_ENDINGS
                }
            }
            Gender::Neuter => {
                if word_stem_is_soft {
                    &NEUTER_SOFT_ENDINGS
                } else {
                    &NEUTER_HARD_ENDINGS
                }
            }
        };

        let ending = endings.ending(case, number);
        let merged = format!("{}{}", word_stem, ending);
        return (merged, gender.clone());
    }
    pub fn noun_is_animate(word: &str) -> bool {
        ANIMATE_MASCULINE_NOUNS.contains(&word)
    }

    pub fn guess_gender(word: &str) -> Gender {
        if ANIMATE_MASCULINE_NOUNS.contains(&word) || INANIMATE_MASCULINE_NOUNS.contains(&word) {
            return Gender::Masculine;
        } else if FEMININE_NOUNS.contains(&word) {
            return Gender::Feminine;
        } else if NEUTER_NOUNS.contains(&word) {
            return Gender::Neuter;
        }
        let last_one = RUTHUTILS::last_n_chars(word, 1);

        if (last_one == "a") || (last_one == "i") {
            return Gender::Feminine;
        } else if (last_one == "o") || (last_one == "e") {
            return Gender::Neuter;
        } else {
            return Gender::Masculine;
        }
    }

    pub fn get_noun_stem(word: &str, number: &Number) -> String {
        if word.ends_with("anin") && number == &Number::Plural {
            return RUTHUTILS::string_without_last_n(word, 2);
        }
        if word.ends_with("anina") && number == &Number::Plural {
            return RUTHUTILS::string_without_last_n(word, 3);
        }

        if RUTHUTILS::is_vowel(&RUTHUTILS::last_in_stringslice(word)) {
            return RUTHUTILS::string_without_last_n(word, 1);
        } else {
            return String::from(word);
        }
    }
    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        RUTHUTILS::ends_with_soft_consonant(&RUTH::get_noun_stem(word, &Number::Singular))
    }
}
