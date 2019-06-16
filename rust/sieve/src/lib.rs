pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let n = (upper_bound + 1) as usize;
    let mut primes = Vec::with_capacity(n);
    for i in 0..n {
        primes.push(i as u64);
    }
    primes[1] = 0;

    for i in (2..).take_while(|&i| i * i < n) {
        if primes[i] > 0 {
            for j in ((i * i)..n).step_by(i) {
                primes[j] = 0;
            }
        }
    }

    primes.retain(|&i| i > 0);
    primes
}
