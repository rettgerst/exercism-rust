pub fn collatz(n: u64) -> Option<u64> {
    let mut active = n;

    let mut steps = 0;
    while active != 1 {
        if active % 2 == 0 {
            active /= 2;
        } else {
            active = (active * 3) + 1;
        }
        steps += 1;
    }

    Some(steps)
}
