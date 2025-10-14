use crate::*;

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
    fn test_ukrainian_roundtrip() {
        let test_words = vec![
            "Привіт",
            "Київ",
            "Україна",
            "Борщ",
            "Добре",
            "Дякую",
            "Здоровенькі були",
            "Життя",
            "Їжак",
            "Щастя",
        ];

        for word in test_words {
            let ruthenian = ukrainian_to_ruthenian(word);
            let back_to_ukrainian = ruthenian_to_ukrainian(&ruthenian);
            assert_eq!(
                word, back_to_ukrainian,
                "Roundtrip failed for word '{}': -> '{}' -> '{}'",
                word, ruthenian, back_to_ukrainian
            );
        }
    }

    #[test]
    fn test_ukrainian_special_letters() {
        // Test Ukrainian-specific letters
        let text = "Ґанок їсть їжу в Україні";
        let ruthenian = ukrainian_to_ruthenian(text);
        let back = ruthenian_to_ukrainian(&ruthenian);
        assert_eq!(text, back, "Failed: {} -> {} -> {}", text, ruthenian, back);
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
