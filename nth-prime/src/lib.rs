fn is_prime(x: u32) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }

    true
}

// this is a naive implementation but I'm here to learn rust, not math
pub fn nth(n: u32) -> u32 {
    // short-circuit these two cases because if n < 2
    // we will never find 2 or 3 by skipping even numbers
    if n == 0 {
        return 2;
    } else if n == 1 {
        return 3;
    }

    let mut primes_found = 2;
    let mut i = 5;
    loop {
        if is_prime(i) {
            primes_found += 1;
            if primes_found == n + 1 {
                return i;
            }
        }
        i += 2;
    }
}
