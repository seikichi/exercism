use std::fmt::Write;

pub fn raindrops(n: u32) -> String {
    let mut results = String::new();
    if n % 3 == 0 {
        write!(&mut results, "Pling").unwrap();
    }
    if n % 5 == 0 {
        write!(&mut results, "Plang").unwrap();
    }
    if n % 7 == 0 {
        write!(&mut results, "Plong").unwrap();
    }
    if results.is_empty() {
        write!(&mut results, "{}", n).unwrap();
    }
    results
}
