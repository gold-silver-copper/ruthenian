use crate::case_endings::*;
use crate::grammar::*;
// ADJECTIVE STUFF
impl RUTH {
    pub fn decline_adj(
        word: &str,
        case: &Case,
        number: &Number,
        gender: &Gender,
        animate: bool,
    ) -> Adjective {
        let word = word.to_lowercase();
        let stem_is_soft = RUTH::stem_of_adj_is_soft(&word);
        let adj_stem = RUTH::get_adj_stem(&word);

        let endings = match gender {
            Gender::Masculine => {
                if animate {
                    if stem_is_soft {
                        &ADJ_ANIMATE_SOFT_ENDINGS
                    } else {
                        &ADJ_ANIMATE_HARD_ENDINGS
                    }
                } else {
                    if stem_is_soft {
                        &ADJ_INANIMATE_SOFT_ENDINGS
                    } else {
                        &ADJ_INANIMATE_HARD_ENDINGS
                    }
                }
            }
            Gender::Feminine => {
                if stem_is_soft {
                    &ADJ_FEMININE_SOFT_ENDINGS
                } else {
                    &ADJ_FEMININE_HARD_ENDINGS
                }
            }
            Gender::Neuter => {
                if stem_is_soft {
                    &ADJ_NEUTER_SOFT_ENDINGS
                } else {
                    &ADJ_NEUTER_HARD_ENDINGS
                }
            }
        };
        let ending = endings.ending(case, number);
        let merged = format!("{}{}", adj_stem, ending);
        return merged;
    }

    pub fn stem_of_adj_is_soft(word: &str) -> bool {
        if word.ends_with("ij") {
            true
        } else {
            false
        }
    }
    pub fn get_adj_stem(word: &str) -> String {
        let mut adj_stem = word.to_string();
        adj_stem.pop();
        adj_stem.pop();
        adj_stem
    }
}
