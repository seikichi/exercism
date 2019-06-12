use std::io::Write;

pub fn raindrops(n: u32) -> String {
    let mut bytes = vec![];
    if n % 3 == 0 {
        write!(&mut bytes, "Pling").unwrap();
    }
    if n % 5 == 0 {
        write!(&mut bytes, "Plang").unwrap();
    }
    if n % 7 == 0 {
        write!(&mut bytes, "Plong").unwrap();
    }
    if bytes.is_empty() {
        write!(&mut bytes, "{}", n).unwrap();
    }
    String::from_utf8(bytes).unwrap()
}
