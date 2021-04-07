use core::panic;

pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64");
    }

    match s {
        1 => 1,
        2 => 2,
        _ => 2_u64.pow(s - 1),
    }
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
