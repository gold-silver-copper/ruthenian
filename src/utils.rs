use crate::phonetics::*;

pub struct RUTHUTILS {}

impl RUTHUTILS {
    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
    pub fn pair_match(word: &str, listik: &[(&str, &str)]) -> Option<String> {
        listik
            .iter()
            .find(|(sing, _)| *sing == word)
            .map(|(_, plur)| plur.to_string())
    }

    pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
        if let Some(last_index) = input.rfind(pattern) {
            let (before_last, _after_last) = input.split_at(last_index);
            format!("{}{}", before_last, replacement)
        } else {
            input.into()
        }
    }
    pub fn iter_replace_last(word: &str, pairs: &[(&str, &str)]) -> Option<String> {
        for (sing, plur) in pairs {
            if word.ends_with(sing) {
                return Some(RUTHUTILS::replace_last_occurence(word, sing, plur));
            }
        }
        None
    }
    pub fn is_vowel(ch: &char) -> bool {
        RUTHENIAN_VOWELS.contains(&ch) || RUSSIAN_VOWELS.contains(&ch)
    }
    pub fn last_char(s: &str) -> Option<char> {
        s.chars().rev().next()
    }

    pub fn is_consonant(ch: &char) -> bool {
        !RUTHUTILS::is_vowel(ch)
    }

    pub fn starts_with_uppercase(word: &str) -> bool {
        word.chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
    }
    pub fn string_without_last_n(s: &str, n: i64) -> String {
        let mut stringik = s.to_string();
        for _ in 0..n {
            stringik.pop();
        }

        stringik
    }
}
