use crate::utils::*;

// Ruthenian to Russian mapping
pub const RUTHENIAN_RUSSIAN: &[(&str, &str)] = &[
    // Uppercase
    ("Szcz", "Щ"),
    ("Sz", "Ш"),
    ("Cz", "Ч"),
    ("Zz", "Ж"),
    ("Ja", "Я"),
    ("Je", "Е"),
    ("Jo", "Ё"),
    ("Ju", "Ю"),
    ("A", "А"),
    ("B", "Б"),
    ("C", "Ц"),
    ("D", "Д"),
    ("E", "Э"),
    ("F", "Ф"),
    ("G", "Г"),
    ("H", "Х"),
    ("I", "И"),
    ("J", "Й"),
    ("K", "К"),
    ("L", "Л"),
    ("M", "М"),
    ("N", "Н"),
    ("O", "О"),
    ("P", "П"),
    ("R", "Р"),
    ("S", "С"),
    ("T", "Т"),
    ("U", "У"),
    ("V", "В"),
    ("Y", "Ы"),
    ("Z", "З"),
    ("'", "ъ"),
    // Lowercase core mappings
    ("szcz", "щ"),
    ("sz", "ш"),
    ("cz", "ч"),
    ("zz", "ж"),
    ("ja", "я"),
    ("je", "е"),
    ("jo", "ё"),
    ("ju", "ю"),
    ("a", "а"),
    ("b", "б"),
    ("c", "ц"),
    ("d", "д"),
    ("e", "э"),
    ("f", "ф"),
    ("g", "г"),
    ("h", "х"),
    ("i", "и"),
    ("j", "й"),
    ("k", "к"),
    ("l", "л"),
    ("m", "м"),
    ("n", "н"),
    ("o", "о"),
    ("p", "п"),
    ("r", "р"),
    ("s", "с"),
    ("t", "т"),
    ("u", "у"),
    ("v", "в"),
    ("y", "ы"),
    ("z", "з"),
    ("'", "ъ"),
];

// Russian to Ruthenian mapping
pub const RUSSIAN_RUTHENIAN: &[(&str, &str)] = &[
    ("Щ", "Szcz"),
    ("Ш", "Sz"),
    ("Ч", "Cz"),
    ("Ж", "Zz"),
    ("Я", "Ja"),
    ("Е", "Je"),
    ("Ё", "Jo"),
    ("Ю", "Ju"),
    ("А", "A"),
    ("Б", "B"),
    ("Ц", "C"),
    ("Д", "D"),
    ("Э", "E"),
    ("Ф", "F"),
    ("Г", "G"),
    ("Х", "H"),
    ("И", "I"),
    ("Й", "J"),
    ("Ь", "J"),
    ("К", "K"),
    ("Л", "L"),
    ("М", "M"),
    ("Н", "N"),
    ("О", "O"),
    ("П", "P"),
    ("Р", "R"),
    ("С", "S"),
    ("Т", "T"),
    ("У", "U"),
    ("В", "V"),
    ("Ы", "Y"),
    ("З", "Z"),
    ("Ъ", "'"),
    // Lowercase
    ("щ", "szcz"),
    ("ш", "sz"),
    ("ч", "cz"),
    ("ж", "zz"),
    ("я", "ja"),
    ("е", "je"),
    ("ё", "jo"),
    ("ю", "ju"),
    ("а", "a"),
    ("б", "b"),
    ("ц", "c"),
    ("д", "d"),
    ("э", "e"),
    ("ф", "f"),
    ("г", "g"),
    ("х", "h"),
    ("и", "i"),
    ("й", "j"),
    ("ь", "j"),
    ("к", "k"),
    ("л", "l"),
    ("м", "m"),
    ("н", "n"),
    ("о", "o"),
    ("п", "p"),
    ("р", "r"),
    ("с", "s"),
    ("т", "t"),
    ("у", "u"),
    ("в", "v"),
    ("ы", "y"),
    ("з", "z"),
    ("ъ", "'"),
];

// Russian vowels
pub const RUSSIAN_VOWELS: &[char] = &[
    'А', 'Э', 'И', 'О', 'У', 'Ы', 'Я', 'Е', 'Ё', 'Ю', 'а', 'э', 'и', 'о', 'у', 'ы', 'я', 'е', 'ё',
    'ю',
];

// Russian consonants
pub const RUSSIAN_CONSONANTS: &[char] = &[
    'Б', 'Ц', 'Ч', 'Д', 'Ф', 'Г', 'Х', 'Й', 'К', 'Л', 'М', 'Н', 'П', 'Р', 'С', 'Ш', 'Щ', 'Т', 'В',
    'З', 'Ж', 'б', 'ц', 'ч', 'д', 'ф', 'г', 'х', 'й', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'ш', 'щ',
    'т', 'в', 'з', 'ж',
];

// Ruthenian vowels (no clusters like "JA" etc.)
pub const RUTHENIAN_VOWELS: &[char] = &['A', 'E', 'I', 'O', 'U', 'Y', 'a', 'e', 'i', 'o', 'u', 'y'];

// Ruthenian consonants (no clusters like "SZ", "CZ", "ZZ", etc.)
pub const RUTHENIAN_CONSONANTS: &[char] = &[
    'B', 'C', 'D', 'F', 'G', 'H', 'K', 'L', 'M', 'N', 'P', 'R', 'S', 'T', 'V', 'Z', 'b', 'c', 'd',
    'f', 'g', 'h', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'z',
];

/// Context-aware transliteration from Russian → Ruthenian
pub fn russian_to_ruthenian(input: &str) -> String {
    let mut chars = input.chars().peekable();
    let mut result = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            // Uppercase
            'А' => result.push_str("A"),
            'Б' => result.push_str("B"),
            'В' => result.push_str("V"),
            'Г' => result.push_str("G"),
            'Д' => result.push_str("D"),
            'Е' => result.push_str("Je"),
            'Ё' => result.push_str("Jo"),
            'Ж' => result.push_str("Zz"),
            'З' => result.push_str("Z"),
            'И' => result.push_str("I"),
            'Й' | 'Ь' => result.push_str("J"),
            'К' => result.push_str("K"),
            'Л' => result.push_str("L"),
            'М' => result.push_str("M"),
            'Н' => result.push_str("N"),
            'О' => result.push_str("O"),
            'П' => result.push_str("P"),
            'Р' => result.push_str("R"),
            'С' => result.push_str("S"),
            'Т' => result.push_str("T"),
            'У' => result.push_str("U"),
            'Ф' => result.push_str("F"),
            'Х' => result.push_str("H"),
            'Ц' => result.push_str("C"),
            'Ч' => result.push_str("Cz"),
            'Ш' => result.push_str("Sz"),
            'Щ' => result.push_str("Szcz"),
            'Ы' => result.push_str("Y"),
            'Э' => result.push_str("E"),
            'Ю' => result.push_str("Ju"),
            'Я' => result.push_str("Ja"),
            'Ъ' => result.push_str("'"),
            // Lowercase
            'а' => result.push_str("a"),
            'б' => result.push_str("b"),
            'в' => result.push_str("v"),
            'г' => result.push_str("g"),
            'д' => result.push_str("d"),
            'е' => result.push_str("je"),
            'ё' => result.push_str("jo"),
            'ж' => result.push_str("zz"),
            'з' => match chars.peek() {
                Some('з') => {
                    result.push_str("z'z");
                    chars.next();
                }
                _ => result.push_str("z"),
            },

            'и' => result.push_str("i"),
            'й' | 'ь' => result.push_str("j"),
            'к' => result.push_str("k"),
            'л' => result.push_str("l"),
            'м' => result.push_str("m"),
            'н' => result.push_str("n"),
            'о' => result.push_str("o"),
            'п' => result.push_str("p"),
            'р' => result.push_str("r"),

            'с' => match chars.peek() {
                Some('з') => {
                    result.push_str("s'z");
                    chars.next();
                }
                _ => result.push_str("s"),
            },

            'т' => result.push_str("t"),
            'у' => result.push_str("u"),
            'ф' => result.push_str("f"),
            'х' => result.push_str("h"),
            'ц' => result.push_str("c"),
            'ч' => result.push_str("cz"),
            'ш' => result.push_str("sz"),
            'щ' => result.push_str("szcz"),
            'ы' => result.push_str("y"),
            'э' => result.push_str("e"),
            'ю' => result.push_str("ju"),
            'я' => result.push_str("ja"),
            'ъ' => result.push_str("'"),
            // Preserve non-Cyrillic chars
            other => result.push(other),
        }
    }

    result
}

/// Context-aware transliteration from Ruthenian → Russian
pub fn ruthenian_to_russian(input: &str) -> String {
    let mut chars = input.chars().peekable();
    let mut result = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            // --- Handle 'j' prefix vowels ---
            'j' | 'J' => {
                let mut soft_sign_added = false;
                if let Some(charik) = RUTHUTILS::last_char(&result) {
                    if RUSSIAN_CONSONANTS.contains(&charik) {
                        if let Some(&next) = chars.peek() {
                            // Only add soft sign if next char is NOT a vowel (meaning standalone j)
                            if !matches!(
                                next,
                                'a' | 'A'
                                    | 'e'
                                    | 'E'
                                    | 'o'
                                    | 'O'
                                    | 'u'
                                    | 'U'
                                    | 'i'
                                    | 'I'
                                    | 'y'
                                    | 'Y'
                            ) {
                                result.push('ь');
                                soft_sign_added = true;
                            }
                        }
                    }
                }

                if soft_sign_added {
                    continue;
                }
                if let Some(&next) = chars.peek() {
                    let is_upper = ch == 'J';

                    match next {
                        'a' | 'A' => {
                            result.push(if is_upper { 'Я' } else { 'я' });
                            chars.next();
                        }
                        'e' | 'E' => {
                            result.push(if is_upper { 'Е' } else { 'е' });
                            chars.next();
                        }
                        'o' | 'O' => {
                            result.push(if is_upper { 'Ё' } else { 'ё' });
                            chars.next();
                        }
                        'u' | 'U' => {
                            result.push(if is_upper { 'Ю' } else { 'ю' });
                            chars.next();
                        }
                        'i' | 'I' => {
                            result.push(if is_upper { 'И' } else { 'и' });
                            chars.next();
                        }
                        'y' | 'Y' => {
                            result.push(if is_upper { 'Ы' } else { 'ы' });
                            chars.next();
                        }
                        _ => {
                            // Standalone J/j
                            result.push(if is_upper { 'Й' } else { 'й' });
                        }
                    }
                } else {
                    result.push(if ch == 'J' { 'Й' } else { 'й' });
                }
            }

            // --- Core consonant transliteration ---
            'b' => result.push('б'),
            'B' => result.push('Б'),
            'v' => result.push('в'),
            'V' => result.push('В'),
            'g' => result.push('г'),
            'G' => result.push('Г'),
            'd' => result.push('д'),
            'D' => result.push('Д'),
            'k' => result.push('к'),
            'K' => result.push('К'),
            'l' => result.push('л'),
            'L' => result.push('Л'),
            'm' => result.push('м'),
            'M' => result.push('М'),
            'n' => result.push('н'),
            'N' => result.push('Н'),
            'p' => result.push('п'),
            'P' => result.push('П'),
            'r' => result.push('р'),
            'R' => result.push('Р'),
            't' => result.push('т'),
            'T' => result.push('Т'),
            'f' => result.push('ф'),
            'F' => result.push('Ф'),
            'h' => result.push('х'),
            'H' => result.push('Х'),

            // --- Special Ruthenian multi-letter sounds ---
            'c' | 'C' => {
                if let Some(&next) = chars.peek() {
                    match next {
                        'z' | 'Z' => {
                            result.push(if ch.is_uppercase() { 'Ч' } else { 'ч' });
                            chars.next();
                        }
                        _ => {
                            result.push(if ch.is_uppercase() { 'Ц' } else { 'ц' });
                        }
                    }
                } else {
                    result.push(if ch.is_uppercase() { 'Ц' } else { 'ц' });
                }
            }
            's' | 'S' => {
                if let Some(&next) = chars.peek() {
                    match next {
                        'z' | 'Z' => {
                            chars.next(); // consume 'z'
                            if let Some(&third) = chars.peek() {
                                match third {
                                    'c' | 'C' => {
                                        chars.next(); // consume 'c'
                                        if let Some(&fourth) = chars.peek() {
                                            match fourth {
                                                'z' | 'Z' => {
                                                    // szcz -> Щ
                                                    result.push(if ch.is_uppercase() {
                                                        'Щ'
                                                    } else {
                                                        'щ'
                                                    });
                                                    chars.next(); // consume second 'z'
                                                }
                                                _ => {
                                                    // sz + c (not szcz)
                                                    result.push(if ch.is_uppercase() {
                                                        'Ш'
                                                    } else {
                                                        'ш'
                                                    });
                                                    result.push(if third.is_uppercase() {
                                                        'Ц'
                                                    } else {
                                                        'ц'
                                                    });
                                                }
                                            }
                                        } else {
                                            // sz + c at end
                                            result.push(if ch.is_uppercase() {
                                                'Ш'
                                            } else {
                                                'ш'
                                            });
                                            result.push(if third.is_uppercase() {
                                                'Ц'
                                            } else {
                                                'ц'
                                            });
                                        }
                                    }
                                    _ => {
                                        // sz (not szcz)
                                        result.push(if ch.is_uppercase() { 'Ш' } else { 'ш' });
                                    }
                                }
                            } else {
                                // sz at end
                                result.push(if ch.is_uppercase() { 'Ш' } else { 'ш' });
                            }
                        }
                        _ => {
                            result.push(if ch.is_uppercase() { 'С' } else { 'с' });
                        }
                    }
                } else {
                    result.push(if ch.is_uppercase() { 'С' } else { 'с' });
                }
            }
            'z' | 'Z' => {
                if let Some(&next) = chars.peek() {
                    if next == 'z' || next == 'Z' {
                        result.push(if ch.is_uppercase() { 'Ж' } else { 'ж' });
                        chars.next();
                    } else {
                        result.push(if ch.is_uppercase() { 'З' } else { 'з' });
                    }
                } else {
                    result.push(if ch.is_uppercase() { 'З' } else { 'з' });
                }
            }

            // --- Simple vowels ---
            'a' => result.push('а'),
            'A' => result.push('А'),
            'e' => result.push('э'),
            'E' => result.push('Э'),
            'i' => result.push('и'),
            'I' => result.push('И'),
            'o' => result.push('о'),
            'O' => result.push('О'),
            'u' => result.push('у'),
            'U' => result.push('У'),
            'y' => result.push('ы'),
            'Y' => result.push('Ы'),

            // --- Apostrophe or separator ---
            '\'' => match chars.peek() {
                Some(next) => {
                    if !RUTHENIAN_CONSONANTS.contains(next) {
                        result.push('ъ')
                    }
                }

                _ => result.push('ъ'),
            },

            // --- Default: preserve punctuation, spaces, etc. ---
            _ => result.push(ch),
        }
    }

    result
}

pub fn test_roundtrip_from_file(filename: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut line_num = 0;
    let mut failed_lines = Vec::new();

    for line in reader.lines() {
        line_num += 1;
        let original = line?;

        // Skip empty lines
        if original.trim().is_empty() {
            continue;
        }

        let ruthenian = russian_to_ruthenian(&original);
        let back_to_russian = ruthenian_to_russian(&ruthenian);

        if original != back_to_russian {
            failed_lines.push((
                line_num,
                original.clone(),
                ruthenian.clone(),
                back_to_russian.clone(),
            ));
            println!("❌ Line {}: Round-trip failed!", line_num);
            println!("   Original:  {}", original);
            println!("   Ruthenian: {}", ruthenian);
            println!("   Back:      {}", back_to_russian);
            println!();
            panic!();
        } else {
            println!("✓ Line {}: OK", line_num);
        }
    }

    println!("\n=== Summary ===");
    println!("Total lines tested: {}", line_num);
    println!("Failed lines: {}", failed_lines.len());

    if failed_lines.is_empty() {
        println!("✅ All lines passed! The transliteration is one-to-one.");
    } else {
        println!("❌ Some lines failed the round-trip test.");
        println!("\nFailed lines:");
        for (num, orig, rut, back) in failed_lines {
            println!("  Line {}: '{}' -> '{}' -> '{}'", num, orig, rut, back);
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_from_russian() {
        // Test individual uppercase letters
        let russian_upper = "АБЦЧДЭФГХИЙКЛМНОПРСТУВЫЗЖЪЯЕЁЮ";
        let ruthenian = russian_to_ruthenian(russian_upper);
        let back_to_russian = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            russian_upper, back_to_russian,
            "Roundtrip failed for uppercase: {} -> {} -> {}",
            russian_upper, ruthenian, back_to_russian
        );

        // Test individual lowercase letters
        let russian_lower = "абцчдэфгхийклмнопрстувызжъяеёю";
        let ruthenian = russian_to_ruthenian(russian_lower);
        let back_to_russian = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            russian_lower, back_to_russian,
            "Roundtrip failed for lowercase: {} -> {} -> {}",
            russian_lower, ruthenian, back_to_russian
        );

        // Test common Russian words
        let test_words = vec![
            "Привет",
            "Москва",
            "Щука",
            "Борщ",
            "Хорошо",
            "Спасибо",
            "Здравствуйте",
            "Россия",
            "Жизнь",
            "Ёжик",
        ];

        for word in test_words {
            let ruthenian = russian_to_ruthenian(word);
            let back_to_russian = ruthenian_to_russian(&ruthenian);
            assert_eq!(
                word, back_to_russian,
                "Roundtrip failed for word '{}': -> '{}' -> '{}'",
                word, ruthenian, back_to_russian
            );
        }

        // Test sentences with mixed case
        let sentence = "Привет, мир! Как дела?";
        let ruthenian = russian_to_ruthenian(sentence);
        let back_to_russian = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            sentence, back_to_russian,
            "Roundtrip failed for sentence: {} -> {} -> {}",
            sentence, ruthenian, back_to_russian
        );

        // Test soft and hard signs
        let signs_test = "Объявление и мальчик";
        let ruthenian = russian_to_ruthenian(signs_test);
        let back_to_russian = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            signs_test, back_to_russian,
            "Roundtrip failed for signs: {} -> {} -> {}",
            signs_test, ruthenian, back_to_russian
        );
    }

    #[test]
    fn test_roundtrip_from_ruthenian() {
        // Test individual uppercase letters
        let ruthenian_upper = "ABCCzDEFGHIJKLMNOPRSSTUVYZSzSzczZz'JaJeJoJu";
        let russian = ruthenian_to_russian(ruthenian_upper);
        let back_to_ruthenian = russian_to_ruthenian(&russian);
        assert_eq!(
            ruthenian_upper, back_to_ruthenian,
            "Roundtrip failed for uppercase: {} -> {} -> {}",
            ruthenian_upper, russian, back_to_ruthenian
        );

        // Test individual lowercase letters
        let ruthenian_lower = "abcczdeffghijklmnoprstuvyzszszczz'jajejoju";
        let russian = ruthenian_to_russian(ruthenian_lower);
        let back_to_ruthenian = russian_to_ruthenian(&russian);
        assert_eq!(
            ruthenian_lower, back_to_ruthenian,
            "Roundtrip failed for lowercase: {} -> {} -> {}",
            ruthenian_lower, russian, back_to_ruthenian
        );

        // Test common Ruthenian transliterated words
        let test_words = vec![
            "Pryvjet",
            "Moskva",
            "Szczuka",
            "Borszcz",
            "Horoszo",
            "Spasybo",
            "Zdravstvujte",
            "Rossyja",
            "Zzyzn",
            "Jozzyk",
        ];

        for word in test_words {
            let russian = ruthenian_to_russian(word);
            let back_to_ruthenian = russian_to_ruthenian(&russian);
            assert_eq!(
                word, back_to_ruthenian,
                "Roundtrip failed for word '{}': -> '{}' -> '{}'",
                word, russian, back_to_ruthenian
            );
        }

        // Test mixed case sentences
        let sentence = "Pryvjet, myr! Kak djela?";
        let russian = ruthenian_to_russian(sentence);
        let back_to_ruthenian = russian_to_ruthenian(&russian);
        assert_eq!(
            sentence, back_to_ruthenian,
            "Roundtrip failed for sentence: {} -> {} -> {}",
            sentence, russian, back_to_ruthenian
        );
    }

    #[test]
    fn test_roundtrip_preserves_non_alphabet_characters() {
        // Test with punctuation and numbers
        let text = "Тест 123, привет! Как дела? (Хорошо)";
        let ruthenian = russian_to_ruthenian(text);
        let back = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            text, back,
            "Roundtrip failed with special chars: {} -> {} -> {}",
            text, ruthenian, back
        );

        // Test with spaces and newlines
        let multiline = "Первая строка\nВторая строка\n\tТретья строка";
        let ruthenian = russian_to_ruthenian(multiline);
        let back = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            multiline, back,
            "Roundtrip failed with whitespace: {} -> {} -> {}",
            multiline, ruthenian, back
        );
    }

    #[test]
    fn test_roundtrip_special_characters() {
        // Test ъ and ь specifically
        let hard_sign = "Объект";
        let ruthenian = russian_to_ruthenian(hard_sign);
        let back = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            hard_sign, back,
            "Roundtrip failed for hard sign: {} -> {} -> {}",
            hard_sign, ruthenian, back
        );

        let soft_sign = "Мальчик";
        let ruthenian = russian_to_ruthenian(soft_sign);
        let back = ruthenian_to_russian(&ruthenian);
        assert_eq!(
            soft_sign, back,
            "Roundtrip failed for soft sign: {} -> {} -> {}",
            soft_sign, ruthenian, back
        );
    }
}
