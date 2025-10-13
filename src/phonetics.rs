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
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'R', 'S', 'T', 'V', 'Z', 'b', 'c',
    'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'z',
];

/// Context-aware transliteration from Russian → Ruthenian
pub fn russian_to_ruthenian(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Try to match the longest possible substring (8 down to 1)
        let mut matched = false;
        for l in (1..=8).rev() {
            if i + l <= len {
                let segment: String = chars[i..i + l].iter().collect();
                if let Some(&(_, ruth)) = RUSSIAN_RUTHENIAN.iter().find(|&&(k, _)| k == segment) {
                    result.push_str(ruth);
                    i += l; // Skip all matched characters
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            // If no rule matched, fallback to copying char as-is
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Context-aware transliteration from Ruthenian → Russian
pub fn ruthenian_to_russian(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let curr = chars[i];

        // --- Context-sensitive handling for `'j'` ---
        if curr == 'j' || curr == 'J' {
            // Look at the previous character in the input
            let prev_is_consonant = if i > 0 {
                RUTHUTILS::is_consonant(&chars[i - 1])
            } else {
                false
            };

            if prev_is_consonant {
                // After consonant: soft sign
                result.push(if curr == 'J' { 'Ь' } else { 'ь' });
            } else {
                // At start or after vowel: short i
                result.push(if curr == 'J' { 'Й' } else { 'й' });
            }
            i += 1;
            continue;
        }

        // --- Try to match longest substring in table ---
        let mut matched = false;
        for l in (1..=8).rev() {
            if i + l <= len {
                let segment: String = chars[i..i + l].iter().collect();
                if let Some(&(_, rus)) = RUTHENIAN_RUSSIAN.iter().find(|&&(k, _)| k == segment) {
                    result.push_str(rus);
                    i += l; // Skip all matched characters
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            result.push(curr);
            i += 1;
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
