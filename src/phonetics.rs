// Ruthenian to Russian mapping
pub const RUTHENIAN_RUSSIAN: &[(&str, &str)] = &[
    ("A", "А"),
    ("B", "Б"),
    ("C", "Ц"),
    ("CZ", "Ч"),
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
    ("SZ", "Ш"),
    ("SZCZ", "Щ"),
    ("T", "Т"),
    ("U", "У"),
    ("V", "В"),
    ("Y", "Ы"),
    ("Z", "З"),
    ("ZZ", "Ж"),
    ("'", "Ъ"),
    ("JA", "Я"),
    ("JE", "Е"),
    ("JO", "Ё"),
    ("JU", "Ю"),
    // Lowercase
    ("a", "а"),
    ("b", "б"),
    ("c", "ц"),
    ("cz", "ч"),
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
    ("sz", "ш"),
    ("szcz", "щ"),
    ("t", "т"),
    ("u", "у"),
    ("v", "в"),
    ("y", "ы"),
    ("z", "з"),
    ("zz", "ж"),
    ("ja", "я"),
    ("je", "е"),
    ("jo", "ё"),
    ("ju", "ю"),
];

// Russian to Ruthenian mapping
pub const RUSSIAN_RUTHENIAN: &[(&str, &str)] = &[
    ("А", "A"),
    ("Б", "B"),
    ("Ц", "C"),
    ("Ч", "Cz"),
    ("Д", "D"),
    ("Э", "E"),
    ("Ф", "F"),
    ("Г", "G"),
    ("Х", "H"),
    ("И", "I"),
    ("Й", "J"),
    ("К", "K"),
    ("Л", "L"),
    ("М", "M"),
    ("Н", "N"),
    ("О", "O"),
    ("П", "P"),
    ("Р", "R"),
    ("С", "S"),
    ("Ш", "Sz"),
    ("Щ", "Szcz"),
    ("Т", "T"),
    ("У", "U"),
    ("В", "V"),
    ("Ы", "Y"),
    ("З", "Z"),
    ("Ж", "Zz"),
    ("Ъ", "'"),
    ("Ь", "J"),
    ("Я", "Ja"),
    ("Е", "Je"),
    ("Ё", "Jo"),
    ("Ю", "Ju"),
    // Lowercase
    ("а", "a"),
    ("б", "b"),
    ("ц", "c"),
    ("ч", "cz"),
    ("д", "d"),
    ("э", "e"),
    ("ф", "f"),
    ("г", "g"),
    ("х", "h"),
    ("и", "i"),
    ("й", "j"),
    ("к", "k"),
    ("л", "l"),
    ("м", "m"),
    ("н", "n"),
    ("о", "o"),
    ("п", "p"),
    ("р", "r"),
    ("с", "s"),
    ("ш", "sz"),
    ("щ", "szcz"),
    ("т", "t"),
    ("у", "u"),
    ("в", "v"),
    ("ы", "y"),
    ("з", "z"),
    ("ж", "zz"),
    ("ь", "j"),
    ("я", "ja"),
    ("е", "je"),
    ("ё", "jo"),
    ("ю", "ju"),
];

pub fn russian_to_ruthenian(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        let mut matched = false;
        
        // Try to match multi-character Cyrillic sequences first
        // (Currently Russian doesn't have multi-char sequences, but structure is here for consistency)
        let s = c.to_string();
        
        // Look up single character
        for (russian, ruthenian) in RUSSIAN_RUTHENIAN {
            if *russian == s {
                result.push_str(ruthenian);
                matched = true;
                break;
            }
        }
        
        if !matched {
            result.push(c);
        }
    }
    
    result
}

pub fn ruthenian_to_russian(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
    
    while i < chars.len() {
        let mut matched = false;
        
        // Try to match longer sequences first
        for len in (1..=5).rev() {
            if i + len <= chars.len() {
                let substr: String = chars[i..i+len].iter().collect();
                
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
        
        if !matched {
            result.push(chars[i]);
            i += 1;
        }
    }
    
    result
}
