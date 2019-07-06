pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    if let Some(exp) = s.checked_sub(1) {
        2u64.pow(exp)
    } else {
        panic!("Arithematic error");
    }
}

pub fn total() -> u64 { (1..=64).map(square).sum() }
