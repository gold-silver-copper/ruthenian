//! The writer: naive mapping plus a separator wherever re-reading would diverge.
//!
//! The separator decision is *local*. The longest digraph is four characters and
//! no digraph contains `j`, `i` or `'` in a non-initial position, so whether a
//! boundary needs a separator is decided entirely by the two letters meeting at
//! it, plus the class of the letter before them (which the reader needs for its
//! `j` decision). The exhaustive triples guard is what proves the window is big
//! enough — if a triple ever fails, the window is too small, not the rule.

use crate::alphabet::{Class, Letter, SEP, STRESS, find_cyrillic, is_hushing};
use crate::reader::{Grapheme, tokenize};

#[derive(Clone, Copy)]
struct Unit {
    letter: &'static Letter,
    upper: bool,
    stress: bool,
}

enum Item {
    Letter(Unit),
    Neutral(char),
}

/// Map a validated Cyrillic string to Ruthenian.
pub fn write(cyrillic: &str) -> String {
    let items = split(cyrillic);
    let mut out = String::with_capacity(cyrillic.len() * 2);

    // Case is a token-level property: an all-uppercase token gets uppercase
    // digraphs (`SZCZUKA`), anything else gets Title-case ones (`Szczuka`).
    // Decoding is per unit — a unit whose first character is uppercase came from
    // an uppercase letter — and the two rules agree in every case, which is what
    // lets mixed case round-trip without a special path.
    let mut idx = 0;
    while idx < items.len() {
        match &items[idx] {
            Item::Neutral(c) => {
                out.push(*c);
                idx += 1;
            }
            Item::Letter(_) => {
                let start = idx;
                while matches!(items.get(idx), Some(Item::Letter(_))) {
                    idx += 1;
                }
                write_token(&items[start..idx], &mut out);
            }
        }
    }
    out
}

fn write_token(items: &[Item], out: &mut String) {
    let units: Vec<Unit> = items
        .iter()
        .filter_map(|i| match i {
            Item::Letter(u) => Some(*u),
            Item::Neutral(_) => None,
        })
        .collect();

    let all_upper = units.len() >= 2 && units.iter().all(|u| u.upper);

    for (n, u) in units.iter().enumerate() {
        if n > 0 {
            let prev = units[n - 1];
            let before = if n >= 2 {
                Some(units[n - 2].letter.class)
            } else {
                None
            };
            let next = units.get(n + 1).map(|u| u.letter);
            if !reads_back(prev.letter, u.letter, next, before) {
                out.push(SEP);
            }
        }
        let after_hushing = n > 0 && is_hushing(units[n - 1].letter.lower);
        push_spelling(*u, all_upper, after_hushing, out);
    }
}

fn push_spelling(u: Unit, all_upper: bool, after_hushing: bool, out: &mut String) {
    let lat = spelling(u.letter, after_hushing);
    if !u.upper {
        out.push_str(lat);
    } else if all_upper {
        for c in lat.chars() {
            out.push(c.to_ascii_uppercase());
        }
    } else {
        let mut cs = lat.chars();
        if let Some(first) = cs.next() {
            out.push(first.to_ascii_uppercase());
            out.push_str(cs.as_str());
        }
    }
    if u.stress {
        out.push(STRESS);
    }
}

/// Would the reader recover exactly these two letters from their naive
/// concatenation?
///
/// The window reaches one letter in each direction, and both are load-bearing:
///
/// * `before` — the class of the letter preceding `prev`, which the reader needs
///   to decide whether a bare `j` is `ь` or `й`;
/// * `next` — **only when `cur` is the hard sign**, because `'` is read as the
///   hard sign or as a pure separator according to what *follows* it. Without
///   it, `подъезд` emits a spurious separator and comes out `pod''jezd`.
///
/// The lookahead is restricted to the hard sign deliberately. Including `next`
/// unconditionally assumes no separator will be inserted between `cur` and
/// `next` — which is precisely what the following step has yet to decide — and
/// that assumption is wrong: it makes `Ийон` come out `I'j'on`, separating и
/// from й because the probe `ijo` reads its tail as `ё`.
///
/// Restricting it is sound. The hard sign is the only unit whose reading depends
/// on its right context, and a separator is never inserted immediately after
/// one: `ъ` is only ever followed by `е ё ю я и`, whose spellings begin with `j`
/// or `i`, and no digraph contains either in a non-initial position. Only the
/// first two letters are asserted on; `next` is context, not a claim.
/// How a letter is spelled, given whether a hushing consonant precedes it.
///
/// `е` is `je` everywhere except after `ж ш ч щ`, where the `j` would mark a
/// softness contrast none of the four has (`RUTHENIAN.md` §2.2, §3.8 rule 2).
/// The reader inverts it exactly, because the alphabet declares that no hushing
/// consonant is followed by `э`.
fn spelling(l: &Letter, after_hushing: bool) -> &'static str {
    match after_hushing && l.lower == 'е' {
        true => "e",
        false => l.latin,
    }
}

fn reads_back(prev: &Letter, cur: &Letter, next: Option<&Letter>, before: Option<Class>) -> bool {
    let mut probe = String::with_capacity(12);
    probe.push_str(prev.latin);
    probe.push_str(spelling(cur, is_hushing(prev.lower)));
    if cur.class == Class::HardSign
        && let Some(n) = next
    {
        probe.push_str(n.latin);
    }
    let toks = tokenize(&probe, before);
    let letters: Vec<char> = toks
        .iter()
        .filter_map(|g| match g {
            Grapheme::Letter { cyr, .. } => Some(*cyr),
            _ => None,
        })
        .collect();
    letters.len() >= 2 && letters[0] == prev.lower && letters[1] == cur.lower
}

fn split(s: &str) -> Vec<Item> {
    let mut items = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match find_cyrillic(c) {
            Some(l) => {
                let stress = chars.peek() == Some(&STRESS);
                if stress {
                    chars.next();
                }
                items.push(Item::Letter(Unit {
                    letter: l,
                    upper: c == l.upper,
                    stress,
                }));
            }
            None => items.push(Item::Neutral(c)),
        }
    }
    items
}
