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
    // Disambiguation patterns (s, c, z intersections)
    ("z'z", "зз"),
    ("c'z", "цз"),
    ("s'z", "сз"),
    ("z'sz", "зш"),
    ("s'sz", "сш"),
    ("c'sz", "цш"),
    ("z'cz", "зч"),
    ("s'cz", "сч"),
    ("c'cz", "цч"),
    ("z'zz", "зж"),
    ("s'zz", "сж"),
    ("c'zz", "цж"),
    ("zz'zz", "жж"),
    ("sz'zz", "шж"),
    ("cz'zz", "чж"),
    ("c'z'z", "цзз"), // optional deeper cases
    ("s'z'z", "сзз"),
    ("i'j", "ий"),
    ("I'j", "Ий"),
];

// Russian to Ruthenian mapping
pub const RUSSIAN_RUTHENIAN: &[(&str, &str)] = &[
    // Reverse disambiguations
    ("зз", "z'z"),
    ("цз", "c'z"),
    ("сз", "s'z"),
    ("зш", "z'sz"),
    ("сш", "s'sz"),
    ("цш", "c'sz"),
    ("зч", "z'cz"),
    ("сч", "s'cz"),
    ("цч", "c'cz"),
    ("зж", "z'zz"),
    ("сж", "s'zz"),
    ("цж", "c'zz"),
    ("жж", "zz'zz"),
    ("шж", "sz'zz"),
    ("чж", "cz'zz"),
    ("ийа", "i'ja"),
    ("ийе", "i'je"),
    ("ийо", "i'jo"),
    ("ийу", "i'ju"),
    ("ийи", "i'ji"),
    ("ийы", "i'jy"),
    // Uppercase
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
pub const RUSSIAN_VOWELS: &[&str] = &[
    "А", "Э", "И", "О", "У", "Ы", "Я", "Е", "Ё", "Ю", "а", "э", "и", "о", "у", "ы", "я", "е", "ё",
    "ю",
];

// Russian consonants
pub const RUSSIAN_CONSONANTS: &[&str] = &[
    "Б", "Ц", "Ч", "Д", "Ф", "Г", "Х", "Й", "К", "Л", "М", "Н", "П", "Р", "С", "Ш", "Щ", "Т", "В",
    "З", "Ж", "б", "ц", "ч", "д", "ф", "г", "х", "й", "к", "л", "м", "н", "п", "р", "с", "ш", "щ",
    "т", "в", "з", "ж",
];

// Ruthenian vowels
pub const RUTHENIAN_VOWELS: &[&str] = &[
    "A", "E", "I", "O", "U", "Y", "JA", "JE", "JO", "JU", "a", "e", "i", "o", "u", "y", "ja", "je",
    "jo", "ju",
];

// Ruthenian consonants
pub const RUTHENIAN_CONSONANTS: &[&str] = &[
    "B", "C", "CZ", "D", "F", "G", "H", "J", "K", "L", "M", "N", "P", "R", "S", "SZ", "SZCZ", "T",
    "V", "Z", "ZZ", "b", "c", "cz", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s",
    "sz", "szcz", "t", "v", "z", "zz",
];

pub fn russian_to_ruthenian(input: &str) -> String {
    let mut result = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let mut matched = false;

        // Try matching progressively longer sequences (5, 4, 3, 2, 1 chars)
        for len in (1..=5).rev() {
            if i + len <= bytes.len() {
                // Get a string slice of the input
                if let Ok(substr) = std::str::from_utf8(&bytes[i..i + len]) {
                    // Try to match in the lookup table
                    for (russian, ruthenian) in RUSSIAN_RUTHENIAN {
                        if *russian == substr {
                            result.push_str(ruthenian);
                            i += len;
                            matched = true;
                            break;
                        }
                    }

                    if matched {
                        break;
                    }
                }
            }
        }

        // If no match found, just copy the character
        if !matched {
            if let Ok(ch_str) = std::str::from_utf8(&bytes[i..i + 1]) {
                result.push_str(ch_str);
                i += 1;
            } else {
                // Skip invalid UTF-8
                i += 1;
            }
        }
    }

    result
}
pub fn ruthenian_to_russian(input: &str) -> String {
    let mut result = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let mut matched = false;

        // Try matching progressively longer sequences (5, 4, 3, 2, 1 chars)
        for len in (1..=5).rev() {
            if i + len <= bytes.len() {
                // Get a string slice of the input
                if let Ok(substr) = std::str::from_utf8(&bytes[i..i + len]) {
                    // Special handling for 'j' or 'J' when it's a single character
                    if len == 1 && (substr == "j" || substr == "J") {
                        // Check if previous character in result is a consonant
                        if let Some(last_char) = result.chars().last() {
                            if RUTHUTILS::is_consonant(&last_char.to_string()) {
                                // This is a soft sign
                                result.push(if substr == "J" { 'Ь' } else { 'ь' });
                                i += len;
                                matched = true;
                                break;
                            } else {
                                // This is a soft sign
                                result.push(if substr == "J" { 'Й' } else { 'й' });
                                i += len;
                                matched = true;
                                break;
                            }
                        }
                    }

                    // Try to match in the lookup table
                    for (ruthenian, russian) in RUTHENIAN_RUSSIAN {
                        if *ruthenian == substr {
                            result.push_str(russian);
                            i += len;
                            matched = true;
                            break;
                        }
                    }

                    if matched {
                        break;
                    }
                }
            }
        }

        // If no match found, just copy the character
        if !matched {
            if let Ok(ch_str) = std::str::from_utf8(&bytes[i..i + 1]) {
                result.push_str(ch_str);
                i += 1;
            } else {
                // Skip invalid UTF-8
                i += 1;
            }
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
