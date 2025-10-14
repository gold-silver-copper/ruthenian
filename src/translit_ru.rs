use crate::phonetics::*;
use crate::utils::*;
use std::iter::Peekable;
use std::str::Chars;

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
pub fn test_roundtrip_from_file_ru(filename: &str) -> std::io::Result<()> {
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
