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

use std::iter::Peekable;
use std::str::Chars;

// Helper macro for simple character mappings
macro_rules! push_mapping {
    ($result:expr, $($ch:literal => $s:literal),+ $(,)?) => {
        match ch {
            $($ch => $result.push_str($s),)+
            _ => unreachable!(),
        }
    };
}

// Helper function for lookahead patterns (з/ж combinations)
fn handle_z_lookahead(result: &mut String, chars: &mut Peekable<Chars>, base: &str) {
    result.push_str(base);

    match chars.peek() {
        Some('з') => {
            result.push_str("'z");
            chars.next();
        }

        Some('З') => {
            result.push_str("'Z");
            chars.next();
        }
        Some('Ж') => {
            result.push_str("'Zz");
            chars.next();
        }
        Some('ж') => {
            result.push_str("'zz");
            chars.next();
        }
        _ => (),
    }
}

pub fn russian_to_ruthenian(input: &str) -> String {
    let mut chars = input.chars().peekable();
    let mut result = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            // Simple uppercase mappings
            'А' => result.push('A'),
            'Б' => result.push('B'),
            'В' => result.push('V'),
            'Г' => result.push('G'),
            'Д' => result.push('D'),
            'И' => result.push('I'),
            'К' => result.push('K'),
            'Л' => result.push('L'),
            'М' => result.push('M'),
            'Н' => result.push('N'),
            'О' => result.push('O'),
            'П' => result.push('P'),
            'Р' => result.push('R'),
            'Т' => result.push('T'),
            'У' => result.push('U'),
            'Ф' => result.push('F'),
            'Х' => result.push('H'),
            'Ы' => result.push('Y'),
            'Э' => result.push('E'),
            'Й' | 'Ь' => result.push('J'),
            'ъ' => result.push('\''),

            'Ъ' => result.push_str("''"),
            // Multi-character uppercase
            'Е' => result.push_str("Je"),
            'Ё' => result.push_str("Jo"),
            'Ч' => result.push_str("Cz"),
            'Ш' => result.push_str("Sz"),
            'Щ' => result.push_str("Szcz"),
            'Ю' => result.push_str("Ju"),
            'Я' => result.push_str("Ja"),

            // Uppercase with lookahead
            'Ж' => handle_z_lookahead(&mut result, &mut chars, "Zz"),
            'З' => handle_z_lookahead(&mut result, &mut chars, "Z"),
            'С' => handle_z_lookahead(&mut result, &mut chars, "S"),

            'Ц' => handle_z_lookahead(&mut result, &mut chars, "C"),
            // Simple lowercase mappings
            'а' => result.push('a'),
            'б' => result.push('b'),
            'в' => result.push('v'),
            'г' => result.push('g'),
            'д' => result.push('d'),
            'и' => result.push('i'),
            'к' => result.push('k'),
            'л' => result.push('l'),
            'м' => result.push('m'),
            'н' => result.push('n'),
            'о' => result.push('o'),
            'п' => result.push('p'),
            'р' => result.push('r'),
            'т' => result.push('t'),
            'у' => result.push('u'),
            'ф' => result.push('f'),
            'х' => result.push('h'),
            'ы' => result.push('y'),
            'э' => result.push('e'),
            'ь' => result.push('j'),

            // Multi-character lowercase
            'е' => result.push_str("je"),
            'ё' => result.push_str("jo"),
            'ч' => result.push_str("cz"),
            'ш' => result.push_str("sz"),
            'щ' => result.push_str("szcz"),
            'ю' => result.push_str("ju"),
            'я' => result.push_str("ja"),

            // Lowercase with lookahead
            'з' => handle_z_lookahead(&mut result, &mut chars, "z"),

            'ж' => handle_z_lookahead(&mut result, &mut chars, "zz"),
            'с' => handle_z_lookahead(&mut result, &mut chars, "s"),
            'ц' => handle_z_lookahead(&mut result, &mut chars, "c"),

            'й' => {
                if matches!(chars.peek(), Some(next) if RUSSIAN_VOWELS.contains(next)) {
                    result.push_str("j'");
                } else {
                    result.push('j');
                }
            }

            // Preserve non-Cyrillic chars
            other => result.push(other),
        }
    }

    result
}

pub fn ruthenian_to_russian(input: &str) -> String {
    let mut chars = input.chars().peekable();
    let mut result = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            'j' | 'J' => {
                let is_upper = ch == 'J';

                // Check for j' -> й
                if matches!(chars.peek(), Some('\'')) {
                    result.push(if is_upper { 'Й' } else { 'й' });
                    chars.next();
                    continue;
                }

                // Add soft sign after consonants if needed
                if let Some(last) = RUTHUTILS::last_char(&result) {
                    if RUSSIAN_CONSONANTS.contains(&last) {
                        let needs_soft_sign = match chars.peek() {
                            Some(&next) => {
                                !matches!(next, 'a' | 'A' | 'e' | 'E' | 'o' | 'O' | 'u' | 'U')
                            }
                            None => true,
                        };

                        if needs_soft_sign {
                            result.push(if is_upper { 'Ь' } else { 'ь' });
                            continue;
                        }
                    }
                }

                // Handle j + vowel combinations
                if let Some(&next) = chars.peek() {
                    let converted = match next {
                        'a' | 'A' => Some(if is_upper { 'Я' } else { 'я' }),
                        'e' | 'E' => Some(if is_upper { 'Е' } else { 'е' }),
                        'o' | 'O' => Some(if is_upper { 'Ё' } else { 'ё' }),
                        'u' | 'U' => Some(if is_upper { 'Ю' } else { 'ю' }),
                        _ => None,
                    };

                    if let Some(cyrillic) = converted {
                        result.push(cyrillic);
                        chars.next();
                    } else {
                        result.push(if is_upper { 'Й' } else { 'й' });
                    }
                } else {
                    result.push(if is_upper { 'Й' } else { 'й' });
                }
            }

            // Simple consonant mappings (using pattern matching for case)
            'b' | 'B' => result.push(if ch == 'B' { 'Б' } else { 'б' }),
            'v' | 'V' => result.push(if ch == 'V' { 'В' } else { 'в' }),
            'g' | 'G' => result.push(if ch == 'G' { 'Г' } else { 'г' }),
            'd' | 'D' => result.push(if ch == 'D' { 'Д' } else { 'д' }),
            'k' | 'K' => result.push(if ch == 'K' { 'К' } else { 'к' }),
            'l' | 'L' => result.push(if ch == 'L' { 'Л' } else { 'л' }),
            'm' | 'M' => result.push(if ch == 'M' { 'М' } else { 'м' }),
            'n' | 'N' => result.push(if ch == 'N' { 'Н' } else { 'н' }),
            'p' | 'P' => result.push(if ch == 'P' { 'П' } else { 'п' }),
            'r' | 'R' => result.push(if ch == 'R' { 'Р' } else { 'р' }),
            't' | 'T' => result.push(if ch == 'T' { 'Т' } else { 'т' }),
            'f' | 'F' => result.push(if ch == 'F' { 'Ф' } else { 'ф' }),
            'h' | 'H' => result.push(if ch == 'H' { 'Х' } else { 'х' }),

            'c' | 'C' => {
                let is_upper = ch == 'C';
                if matches!(chars.peek(), Some('z' | 'Z')) {
                    result.push(if is_upper { 'Ч' } else { 'ч' });
                    chars.next();
                } else {
                    result.push(if is_upper { 'Ц' } else { 'ц' });
                }
            }

            's' | 'S' => {
                let is_upper = ch == 'S';
                if matches!(chars.peek(), Some('z' | 'Z')) {
                    chars.next();
                    // Check for szcz
                    if matches!(chars.peek(), Some('c' | 'C')) {
                        let third = *chars.peek().unwrap();
                        chars.next();
                        if matches!(chars.peek(), Some('z' | 'Z')) {
                            result.push(if is_upper { 'Щ' } else { 'щ' });
                            chars.next();
                        } else {
                            // sz + c (not szcz)
                            result.push(if is_upper { 'Ш' } else { 'ш' });
                            result.push(if third.is_uppercase() { 'Ц' } else { 'ц' });
                        }
                    } else {
                        result.push(if is_upper { 'Ш' } else { 'ш' });
                    }
                } else {
                    result.push(if is_upper { 'С' } else { 'с' });
                }
            }

            'z' | 'Z' => {
                let is_upper = ch == 'Z';
                if matches!(chars.peek(), Some('z' | 'Z')) {
                    result.push(if is_upper { 'Ж' } else { 'ж' });
                    chars.next();
                } else {
                    result.push(if is_upper { 'З' } else { 'з' });
                }
            }

            // Simple vowel mappings
            'a' | 'A' => result.push(if ch == 'A' { 'А' } else { 'а' }),
            'e' | 'E' => result.push(if ch == 'E' { 'Э' } else { 'э' }),
            'i' | 'I' => result.push(if ch == 'I' { 'И' } else { 'и' }),
            'o' | 'O' => result.push(if ch == 'O' { 'О' } else { 'о' }),
            'u' | 'U' => result.push(if ch == 'U' { 'У' } else { 'у' }),
            'y' | 'Y' => result.push(if ch == 'Y' { 'Ы' } else { 'ы' }),

            '\'' => {
                if chars.peek() == Some(&'\'') {
                    result.push_str("Ъ");
                    chars.next();
                } else if !matches!(chars.peek(), Some(next) if RUTHENIAN_CONSONANTS.contains(next))
                {
                    result.push('ъ');
                }
            }

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
