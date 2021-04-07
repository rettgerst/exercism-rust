pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|i| {
            factors.iter().any(|f| match f {
                0 => false,
                _ => i % f == 0,
            })
        })
        .sum()
}
