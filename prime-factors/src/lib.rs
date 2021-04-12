pub fn factors(n: u64) -> Vec<u64> {
    println!("finding prime factors of {}", n);
    let mut prime_factors = vec![];

    let mut remainder = n;

    let mut divisor = 2;

    while remainder > 1 {
        while remainder % divisor == 0 {
            prime_factors.push(divisor);
            remainder /= divisor;
        }

        divisor += 1;
    }

    prime_factors
}
