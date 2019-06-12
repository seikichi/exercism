pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    for i in (3..).step_by(2) {
        if (primes.len() - 1) >= n as usize {
            break;
        }
        let is_prime = primes
            .iter()
            .take_while(|&p| p * p <= i)
            .all(|&p| i % p != 0);
        if is_prime {
            primes.push(i);
        }
    }
    primes[n as usize]
}
