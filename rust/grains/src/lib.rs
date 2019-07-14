pub fn total() -> u64 {
    (1..=64).map(square).sum()
}

pub fn square(s: u32) -> u64 {
    // calculate b^n (b = 2, n = s - 1)
    //
    // https://mitpress.mit.edu/sites/default/files/sicp/full-text/book/book-Z-H-11.html#%_sec_1.2.4
    if s < 1 || 64 < s {
        panic!("Square must be between 1 and 64");
    }

    // a*b^n is unchanged
    let mut a = 1;
    let mut b = 2;
    let mut n = s - 1;

    // b^n = (b^2)^(n/2) if b %2 == 0
    // b^n = b*b^(n-1) if b %2 == 1
    while n != 0 {
        if n % 2 == 0 {
            b *= b;
            n /= 2;
        } else {
            a *= b;
            n -= 1;
        }
    }
    a
}
