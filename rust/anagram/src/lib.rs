use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    let lowercase_word = word.to_lowercase();
    let mut v = lowercase_word.chars().collect::<Vec<_>>();
    v.sort();

    for &p in possible_anagrams {
        let lowercase_p = p.to_lowercase();
        if lowercase_p == lowercase_word {
            continue;
        }
        let mut w = lowercase_p.chars().collect::<Vec<_>>();
        w.sort();
        if w == v {
            set.insert(p);
        }
    }
    set
}
