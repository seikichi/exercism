use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    for (s1, s2) in list.iter().zip(list.iter().skip(1)) {
        writeln!(&mut result, "For want of a {} the {} was lost.", s1, s2).unwrap();
    }
    write!(&mut result, "And all for the want of a {}.", list[0]).unwrap();

    result
}
