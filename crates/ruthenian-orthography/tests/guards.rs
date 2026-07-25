//! The eleven guards from `docs/specs/ruthenian-orthography.md` §9.
//!
//! Every one names the minimal mutation that must make it fail. A guard that
//! survives its own witness is stale and must be fixed or deleted.

use ruthenian_orthography::{
    Alphabet, Cyrillic, Grapheme, Ruthenian, STRESS, Unmapped, to_cyrillic, to_latin,
    to_latin_mixed,
};

fn roundtrip(s: &str) -> Result<String, String> {
    let c = Cyrillic::parse(s).map_err(|e| format!("parse: {e}"))?;
    let latin = to_latin(&c);
    Ruthenian::parse(latin.as_str())
        .map_err(|e| format!("latin {:?} invalid: {e}", latin.as_str()))?;
    Ok(to_cyrillic(&latin).as_str().to_string())
}

fn assert_roundtrip(s: &str) {
    match roundtrip(s) {
        Ok(back) => assert_eq!(back, s, "round-trip changed {s:?}"),
        Err(e) => panic!("round-trip failed for {s:?}: {e}"),
    }
}

/// Letters that may legally follow each class, so the exhaustive tests build
/// well-formed strings rather than testing the validator by accident.
fn legal(seq: &[char]) -> bool {
    Cyrillic::parse(&seq.iter().collect::<String>()).is_ok()
}

/// The Russian alphabet, written out independently of the crate's own table.
///
/// Mutation testing caught this: the exhaustive guards originally iterated
/// `Alphabet::letters()`, which is derived from the mapping table, so deleting a
/// row from the table merely stopped the guard from testing that letter. A guard
/// must not source its expectations from the thing it is checking.
const RUSSIAN: [char; 33] = [
    'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с',
    'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 'я',
];

// --------------------------------------------------------------------------
// 1. roundtrip_exhaustive_singles
//    Witness: remove one row from the mapping table.
// --------------------------------------------------------------------------
#[test]
fn roundtrip_exhaustive_singles() {
    // Every letter of the independently-written alphabet must be mapped. A row
    // missing from the crate's table shows up here as a parse failure, not as a
    // silently smaller test.
    for &c in &RUSSIAN {
        assert!(
            Alphabet::contains(c),
            "{c:?} is a Russian letter but is not in the alphabet"
        );
    }
    let mut checked = 0;
    for &c in &RUSSIAN {
        for s in [c.to_string(), c.to_uppercase().to_string()] {
            let chars: Vec<char> = s.chars().collect();
            if !legal(&chars) {
                // ъ and ь cannot stand alone; they are covered by the pair and
                // triple guards in their legal environments.
                assert!(
                    matches!(c, 'ъ' | 'ь'),
                    "{c:?} should be well-formed on its own"
                );
                continue;
            }
            assert_roundtrip(&s);
            checked += 1;
        }
    }
    assert_eq!(checked, 62, "expected 31 letters x 2 cases");
}

// --------------------------------------------------------------------------
// 2. roundtrip_exhaustive_pairs
//    Witness: delete the separator insertion for j+vowel — `ij'on` regresses.
// --------------------------------------------------------------------------
#[test]
fn roundtrip_exhaustive_pairs() {
    let letters: Vec<char> = Alphabet::letters().collect();
    let mut checked = 0;
    for &a in &letters {
        for &b in &letters {
            if !legal(&[a, b]) {
                continue;
            }
            assert_roundtrip(&format!("{a}{b}"));
            checked += 1;
        }
    }
    assert!(checked > 800, "only {checked} pairs checked");
}

// --------------------------------------------------------------------------
// 3. roundtrip_exhaustive_triples
//    Witness: make the digraph list unordered so `sz` shadows `szcz`.
//    This is also what proves the writer's local window is wide enough.
// --------------------------------------------------------------------------
#[test]
fn roundtrip_exhaustive_triples() {
    let letters: Vec<char> = Alphabet::letters().collect();
    let mut checked = 0;
    for &a in &letters {
        for &b in &letters {
            for &c in &letters {
                if !legal(&[a, b, c]) {
                    continue;
                }
                assert_roundtrip(&format!("{a}{b}{c}"));
                checked += 1;
            }
        }
    }
    assert!(checked > 20_000, "only {checked} triples checked");
}

// --------------------------------------------------------------------------
// 4. roundtrip_corpus
//    Witness: reintroduce the reference's catch-all passthrough arm.
// --------------------------------------------------------------------------
#[test]
fn roundtrip_corpus_sample() {
    let raw = include_str!("corpus/sample.tsv");
    let mut lines = 0;
    let mut failures = Vec::new();
    for row in raw.lines() {
        let Some((no, text)) = row.split_once('\t') else {
            continue;
        };
        lines += 1;
        match roundtrip(text) {
            Ok(back) if back == text => {}
            Ok(_) => failures.push(no.to_string()),
            Err(e) => failures.push(format!("{no} ({e})")),
        }
    }
    assert!(lines > 1500, "fixture shrank to {lines} lines");
    assert!(
        failures.is_empty(),
        "{} of {lines} fixture lines failed: {:?}",
        failures.len(),
        &failures[..failures.len().min(10)]
    );
}

/// The three lines the reference implementation fails, pinned individually so a
/// regression names the defect rather than a line count.
#[test]
fn roundtrip_reference_failure_lines() {
    let raw = include_str!("corpus/sample.tsv");
    for target in ["12695", "13444", "31725"] {
        let row = raw
            .lines()
            .find(|l| l.starts_with(&format!("{target}\t")))
            .unwrap_or_else(|| panic!("line {target} missing from the fixture"));
        let text = row.split_once('\t').unwrap().1;
        assert_roundtrip(text);
    }
}

/// The full corpus. Reports loudly when it does not run rather than passing
/// silently — a test that skips on a missing file is a guard that does not run.
#[test]
#[ignore = "needs RUTHENIAN_CORPUS=<path to biblija_ru.txt>"]
fn roundtrip_corpus_full() {
    let Ok(path) = std::env::var("RUTHENIAN_CORPUS") else {
        panic!("RUTHENIAN_CORPUS is unset — the full-corpus guard did NOT run");
    };
    let text = std::fs::read_to_string(&path).expect("corpus readable");
    let mut total = 0;
    let mut nonempty = 0;
    let mut failures = Vec::new();
    for (i, line) in text.lines().enumerate() {
        total += 1;
        if line.trim().is_empty() {
            continue;
        }
        nonempty += 1;
        match roundtrip(line) {
            Ok(back) if back == line => {}
            Ok(_) => failures.push(i + 1),
            Err(e) => {
                eprintln!("line {}: {e}", i + 1);
                failures.push(i + 1);
            }
        }
    }
    println!(
        "corpus: {total} lines, {nonempty} non-empty, {} failures",
        failures.len()
    );
    assert!(failures.is_empty(), "failing lines: {failures:?}");
}

// --------------------------------------------------------------------------
// 5. reference_defect_witnesses
//    Witness: revert any one fix — each input below is pinned with the
//    corrected output.
// --------------------------------------------------------------------------
#[test]
fn reference_defect_witnesses() {
    let latin = |s: &str| to_latin(&Cyrillic::parse(s).unwrap()).as_str().to_string();

    // D1: й + vowel collided with the iotified vowel.
    assert_eq!(latin("Ийон"), "Ij'on");
    assert_eq!(latin("Иён"), "Ijon");
    assert_eq!(latin("Йод"), "J'od");
    assert_eq!(latin("ёд"), "jod");
    for w in ["Ийон", "Иён", "Йод", "ёд", "майор", "батальон"] {
        assert_roundtrip(w);
    }

    // D2: шч collided with щ.
    assert_eq!(latin("шчи"), "sz'czi");
    assert_eq!(latin("щи"), "szczi");
    assert_roundtrip("шчи");
    assert_roundtrip("щи");

    // D3: Latin input was consumed as Ruthenian ("cat дом" -> "цат дом").
    assert_eq!(
        Cyrillic::parse("cat дом").unwrap_err().kind,
        Unmapped::LatinInCyrillic
    );
    let (out, skipped) = to_latin_mixed("cat дом");
    assert_eq!(out, "cat dom");
    assert_eq!(skipped.len(), 1);
    assert_eq!((skipped[0].start, skipped[0].end), (0, 3));

    // D4: no case layer — the reference produced "SzczUKA".
    assert_eq!(latin("ЩУКА"), "SZCZUKA");
    assert_eq!(latin("Щука"), "Szczuka");
    assert_eq!(latin("ЩуКа"), "SzczuKa");
    for w in ["ЩУКА", "Щука", "ЩуКа", "СЗАДИ"] {
        assert_roundtrip(w);
    }

    // D5: unmapped characters passed through as raw Cyrillic inside Latin.
    assert_eq!(
        Cyrillic::parse("мѣсто").unwrap_err().kind,
        Unmapped::PreReform
    );
    assert_eq!(
        Cyrillic::parse("ѳита").unwrap_err().kind,
        Unmapped::PreReform
    );
    assert_eq!(
        Cyrillic::parse("ґанок").unwrap_err().kind,
        Unmapped::ForeignCyrillic
    );

    // D6: `'` is one glyph with one rule; `Ъ` is never `''`.
    assert_eq!(latin("подъезд"), "pod'jezd");
    assert_eq!(latin("подезд"), "podjezd");
    assert_eq!(latin("ПРЕДЪИДЕШЬ"), "PRJED'IDJESZJ");
    assert!(!latin("подъезд").contains("''"));

    // Behaviour that already worked and must not regress.
    assert_eq!(latin("сзади"), "s'zadi");
    assert_eq!(latin("изжить"), "iz'zzitj");
    assert_eq!(latin("СЗАДИ"), "S'ZADI");
    for w in ["сзади", "изжить", "подъезд", "подезд", "предъидешь"]
    {
        assert_roundtrip(w);
    }
}

// --------------------------------------------------------------------------
// 6. alphabet_totality
//    Witness: add a character to the mapping without adding it to
//    `Alphabet::contains`.
// --------------------------------------------------------------------------
#[test]
fn alphabet_totality() {
    // Every mapped letter is in the alphabet, in both cases.
    for c in Alphabet::letters() {
        assert!(
            Alphabet::contains(c),
            "{c:?} maps but is not in the alphabet"
        );
        for u in c.to_uppercase() {
            assert!(
                Alphabet::contains(u),
                "{u:?} maps but is not in the alphabet"
            );
        }
    }
    // Every character either parses or yields a typed error — never a silent
    // passthrough, and never a panic.
    for c in [
        'ѣ', 'ѳ', 'ґ', 'є', 'ї', 'q', 'w', 'x', '\u{0300}', '\u{7}', '\'',
    ] {
        let s = c.to_string();
        match Cyrillic::parse(&s) {
            Ok(_) => panic!("{c:?} parsed as Cyrillic but is not in the alphabet"),
            Err(e) => assert_eq!(e.found, c),
        }
    }
    // Neutral characters pass through untouched.
    let neutral = "0123456789 ,.!?()[]-:;\"_=\n";
    let c = Cyrillic::parse(neutral).unwrap();
    assert_eq!(to_latin(&c).as_str(), neutral);
    assert_eq!(to_cyrillic(&to_latin(&c)).as_str(), neutral);
}

/// The three context rules are declared, not guessed. Each is checked with the
/// error it must produce.
#[test]
fn context_rules_declared() {
    assert_eq!(
        Cyrillic::parse("съз").unwrap_err().kind,
        Unmapped::HardSignContext
    );
    assert_eq!(
        Cyrillic::parse("аь").unwrap_err().kind,
        Unmapped::SoftSignContext
    );
    assert_eq!(
        Cyrillic::parse("нй").unwrap_err().kind,
        Unmapped::ShortIContext
    );
    assert_eq!(
        Cyrillic::parse("подЪезд").unwrap_err().kind,
        Unmapped::HardSignCase
    );
    // …and the environments that are legal.
    for w in ["съезд", "предъидешь", "конь", "мой", "статьи"] {
        assert!(Cyrillic::parse(w).is_ok(), "{w} should be well-formed");
        assert_roundtrip(w);
    }
}

// --------------------------------------------------------------------------
// 7. case_restoration
//    Witness: map before folding instead of after.
// --------------------------------------------------------------------------
#[test]
fn case_restoration() {
    let latin = |s: &str| to_latin(&Cyrillic::parse(s).unwrap()).as_str().to_string();
    // An all-caps token contains no lowercase letter.
    for w in ["ЩУКА", "СЗАДИ", "ЖЖЁТ", "ПРЕДЪИДЕШЬ"] {
        let out = latin(w);
        assert!(
            !out.chars().any(|c| c.is_lowercase()),
            "{w} -> {out} leaked lowercase"
        );
        assert_roundtrip(w);
    }
    assert_eq!(latin("Щука"), "Szczuka");
    assert_eq!(latin("щука"), "szczuka");
}

// --------------------------------------------------------------------------
// 8. stress_preserved
//    Witness: strip combining marks during normalization.
// --------------------------------------------------------------------------
#[test]
fn stress_preserved() {
    let latin = |s: &str| to_latin(&Cyrillic::parse(s).unwrap()).as_str().to_string();
    // Written decomposed on both sides: the dump marks stress with a combining
    // acute, and nothing here composes it into a precomposed codepoint.
    assert_eq!(latin("писа\u{301}ть"), "pisa\u{301}tj");
    assert_eq!(latin("кле\u{301}в"), "klje\u{301}v");
    for w in [
        "писа\u{301}ть",
        "кле\u{301}в",
        "недоплати\u{301}ть",
        "ко\u{301}рюшка",
        "воды\u{301}",
    ] {
        assert_roundtrip(w);
        assert!(latin(w).contains(STRESS), "{w} lost its stress mark");
    }
    // The mark attaches to the same vowel it came from.
    let out = latin("писа\u{301}ть");
    let at = out.find(STRESS).unwrap();
    assert_eq!(out[..at].chars().next_back(), Some('a'));
}

// --------------------------------------------------------------------------
// 9. stress_is_distinguishing
//    Witness: normalize `pisátj` to `pisatj` anywhere in the pipeline.
// --------------------------------------------------------------------------
#[test]
fn stress_is_distinguishing() {
    let a = to_latin(&Cyrillic::parse("писать").unwrap());
    let b = to_latin(&Cyrillic::parse("писа́ть").unwrap());
    assert_ne!(a.as_str(), b.as_str(), "stress was normalized away");
    assert_eq!(to_cyrillic(&a).as_str(), "писать");
    assert_eq!(to_cyrillic(&b).as_str(), "писа́ть");
    // A stray mark is refused rather than silently dropped.
    assert_eq!(
        Cyrillic::parse("т\u{0301}ы").unwrap_err().kind,
        Unmapped::StrayStress
    );

    // The mark must be carried *by the vowel*, not merely survive as a stray
    // character. Mutation testing caught this: making the reader ignore the mark
    // still round-tripped, because the unconsumed mark fell through the
    // passthrough arm and landed back in the output by accident.
    let toks = ruthenian_orthography::reader::tokenize(b.as_str(), None);
    assert!(
        !toks
            .iter()
            .any(|g| matches!(g, Grapheme::Neutral(c) if *c == STRESS)),
        "the stress mark passed through as a neutral character"
    );
    let stressed: Vec<char> = toks
        .iter()
        .filter_map(|g| match g {
            Grapheme::Letter {
                cyr, stress: true, ..
            } => Some(*cyr),
            _ => None,
        })
        .collect();
    assert_eq!(stressed, vec!['а'], "the stress must attach to а in писа́ть");
}

// --------------------------------------------------------------------------
// 10. no_dependencies
//     Witness: add any `[dependencies]` entry.
// --------------------------------------------------------------------------
#[test]
fn no_dependencies() {
    let manifest = include_str!("../Cargo.toml");
    let deps = manifest
        .split("[dependencies]")
        .nth(1)
        .expect("a [dependencies] section must exist, even if empty");
    let entries: Vec<&str> = deps
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with('['))
        .collect();
    assert!(
        entries.is_empty(),
        "this crate must have zero dependencies, found: {entries:?}"
    );
    // Dev-dependencies are permitted only for property testing; none are used.
    assert!(
        !manifest.contains("[dev-dependencies]"),
        "no dev-dependencies are needed; the property guard is hand-rolled"
    );
}

// --------------------------------------------------------------------------
// 11. property_roundtrip
//     Witness: any of the above. Random well-formed strings, deterministic seed
//     so a failure is reproducible without a dependency.
// --------------------------------------------------------------------------
#[test]
fn property_roundtrip() {
    let letters: Vec<char> = Alphabet::letters().collect();
    let neutral: Vec<char> = " ,.-0123456789".chars().collect();
    let mut state: u64 = 0x5DEECE66D;
    let mut next = move || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (state >> 33) as usize
    };

    let mut checked = 0;
    for _ in 0..20_000 {
        let len = 1 + next() % 12;
        let mut s = String::new();
        for _ in 0..len {
            let r = next() % 100;
            if r < 8 {
                s.push(neutral[next() % neutral.len()]);
            } else {
                let c = letters[next() % letters.len()];
                if next() % 4 == 0 {
                    s.extend(c.to_uppercase());
                } else {
                    s.push(c);
                }
                if next() % 20 == 0 && "аеёиоуыэюя".contains(c) {
                    s.push(STRESS);
                }
            }
        }
        if Cyrillic::parse(&s).is_ok() {
            assert_roundtrip(&s);
            checked += 1;
        }
    }
    assert!(
        checked > 2000,
        "only {checked} random strings were well-formed"
    );
}

// --------------------------------------------------------------------------
// Writer invariants that back the separator rule.
// --------------------------------------------------------------------------
#[test]
fn separators_are_minimal_and_never_doubled() {
    let letters: Vec<char> = Alphabet::letters().collect();
    for &a in &letters {
        for &b in &letters {
            let s: String = [a, b].iter().collect();
            if Cyrillic::parse(&s).is_err() {
                continue;
            }
            let out = to_latin(&Cyrillic::parse(&s).unwrap());
            let text = out.as_str();
            assert!(
                !text.contains("''"),
                "{s} produced a doubled separator: {text}"
            );
            // No gratuitous separator: removing it must actually break the read.
            if let Some(pos) = text.find('\'') {
                let mut without = String::from(&text[..pos]);
                without.push_str(&text[pos + 1..]);
                if let Ok(r) = Ruthenian::parse(&without) {
                    assert_ne!(
                        to_cyrillic(&r).as_str(),
                        s,
                        "{s} -> {text}: the separator was unnecessary"
                    );
                }
            }
        }
    }
}

#[test]
fn reader_is_the_definition() {
    // tokenize is public because the writer is defined in terms of it; check it
    // reports what the contract says it reports.
    let r = Ruthenian::parse("pod'jezd").unwrap();
    let toks = ruthenian_orthography::reader::tokenize(r.as_str(), None);
    let hard = toks
        .iter()
        .filter(|g| matches!(g, Grapheme::Letter { cyr: 'ъ', .. }))
        .count();
    assert_eq!(
        hard, 1,
        "the hard sign must read as a letter, not a separator"
    );

    let r = Ruthenian::parse("s'zadi").unwrap();
    let toks = ruthenian_orthography::reader::tokenize(r.as_str(), None);
    assert!(
        toks.iter().any(|g| matches!(g, Grapheme::Separator)),
        "a pure separator must read as Separator, not as a letter"
    );
}
