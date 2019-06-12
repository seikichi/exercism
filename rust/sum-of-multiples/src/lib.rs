pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![false; limit as usize];
    for f in factors.iter().filter(|&f| *f > 0) {
        for m in (*f..limit).step_by(*f as usize) {
            multiples[m as usize] = true;
        }
    }

    multiples
        .iter()
        .enumerate()
        .filter(|&(_, m)| *m)
        .map(|(i, _)| i as u32)
        .sum()
}
