pub const PREFIXES: &[&str] = &[
    "", "v", "u", "s", "o", "za", "vo", "vy", "vo", "so", "po", "od", "ob", "ne", "na", "iz", "do",
    "voz", "raz", "prje", "pro", "pri", "pre", "pod", "nad", "odpo", "izpo", "razpro", "prjedpo",
];

pub const IRREGULAR_STEMS: &[(&str, &str)] = &[
    ("britj", "brije"),
    ("bitj", "bije"),
    ("bratj", "bere"),
    ("idti", "ide"),
    ("odjetj", "odjene "),
    ("jestj", "je "),
    //  ("spatj", "spl"),
];

pub fn irregular_present_stem(infinitive: &str) -> String {
    for (inf, third) in IRREGULAR_STEMS {
        for prefix in PREFIXES {
            let combined = format!("{}{}", prefix, inf);
            if combined == infinitive {
                return format!("{}{}", prefix, third);
            }
        }
    }
    "".into()
}
