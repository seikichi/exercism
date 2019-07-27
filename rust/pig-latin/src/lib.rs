#![feature(slice_patterns)]

pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(String::from)
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: String) -> String {
    let word = word.chars().collect::<Vec<_>>();
    let to_s = |v: &[char]| -> String { v.iter().collect() };

    match &word[..] {
        // Rule 1
        [v, rest..] if is_v(*v) => format!("{}{}ay", v, to_s(rest)),
        ['x', 'r', rest..] => format!("xr{}ay", to_s(rest)),
        ['y', 't', rest..] => format!("yt{}ay", to_s(rest)),
        // Rule 3
        ['q', 'u', rest..] => format!("{}quay", to_s(rest)),
        [c, 'q', 'u', rest..] if is_c(*c) => format!("{}{}quay", to_s(rest), c),
        // Rule 4
        [c1, c2, c3, 'y', rest..] if is_c(*c1) && is_c(*c2) && is_c(*c3) => {
            format!("y{}{}{}{}ay", to_s(rest), c1, c2, c3)
        }
        [c1, c2, 'y', rest..] if is_c(*c1) && is_c(*c2) => format!("y{}{}{}ay", to_s(rest), c1, c2),
        [c, 'y', rest..] if is_c(*c) => format!("y{}{}ay", to_s(rest), c),
        // Rule 2
        [c1, c2, c3, rest..] if is_c(*c1) && is_c(*c2) && is_c(*c3) => {
            format!("{}{}{}{}ay", to_s(rest), c1, c2, c3)
        }
        [c1, c2, rest..] if is_c(*c1) && is_c(*c2) => format!("{}{}{}ay", to_s(rest), c1, c2),
        [c, rest..] if is_c(*c) => format!("{}{}ay", to_s(rest), c),
        // Other
        _ => word.iter().collect(),
    }
}

fn is_v(c: char) -> bool {
    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => true,
        _ => false,
    }
}

fn is_c(c: char) -> bool {
    !is_v(c)
}
