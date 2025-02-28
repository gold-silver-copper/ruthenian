use crate::phonetics::*;

pub struct RUTHUTILS {}

impl RUTHUTILS {
    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
    pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
        if let Some(last_index) = input.rfind(pattern) {
            let (before_last, _after_last) = input.split_at(last_index);
            format!("{}{}", before_last, replacement)
        } else {
            input.into()
        }
    }
    pub fn iotation_merge(root: &str, suffix: &str) -> String {
        if suffix.chars().nth(0) == Some('j') {
            for entry in J_MERGE_CHARS {
                if root.ends_with(entry) {
                    let new_root = match *entry {
                        "st" => RUTHUTILS::replace_last_occurence(root, entry, "szc"),
                        "zd" => RUTHUTILS::replace_last_occurence(root, entry, "zzdzz"),
                        "sk" => RUTHUTILS::replace_last_occurence(root, entry, "szcz"),
                        "zg" => RUTHUTILS::replace_last_occurence(root, entry, "zzzz"),
                        "s" => RUTHUTILS::replace_last_occurence(root, entry, "sz"),
                        //  "z" => RUTHUTILS::replace_last_occurence(root, entry, "zz"),
                        "t" => RUTHUTILS::replace_last_occurence(root, entry, "c"),
                        "d" => RUTHUTILS::replace_last_occurence(root, entry, "dzz"),
                        "k" => RUTHUTILS::replace_last_occurence(root, entry, "cz"),
                        "g" => RUTHUTILS::replace_last_occurence(root, entry, "zz"),
                        "h" => RUTHUTILS::replace_last_occurence(root, entry, "sz"),
                        _ => root.to_string(),
                    };
                    let new_suffix = suffix.get(1..).unwrap();
                    return format!("{new_root}{new_suffix}");
                }
            }

            format!("{root}{suffix}")
        } else {
            format!("{root}{suffix}")
        }
    }

    pub fn is_vowel(c: &char) -> bool {
        VOWELS.contains(c)
    }

    pub fn ends_with_soft_consonant(word: &str) -> bool {
        RUTHUTILS::is_soft_consonant(&RUTHUTILS::last_in_stringslice(word))
    }

    pub fn is_hard_consonant(c: &char) -> bool {
        HARD_CONSONANTS.contains(c)
    }

    pub fn is_soft_consonant(c: &char) -> bool {
        !RUTHUTILS::is_hard_consonant(c) && !RUTHUTILS::is_vowel(c)
    }
    pub fn last_in_stringslice(s: &str) -> char {
        s.to_string().pop().unwrap_or(' ')
    }
    pub fn is_consonant(c: &char) -> bool {
        !RUTHUTILS::is_vowel(c)
    }
    pub fn string_without_last_n(s: &str, n: i64) -> String {
        let mut stringik = s.to_string();
        for _ in 0..n {
            stringik.pop();
        }

        stringik
    }
}
