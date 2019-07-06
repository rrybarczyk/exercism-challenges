pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (1..=n).sum();
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).fold(0, |acc, x| acc + x.pow(2))
}

pub fn difference(n: u32) -> u32 {
    if let Some(x) = square_of_sum(n).checked_sub(sum_of_squares(n)) {
        x
    } else {
        panic!("Something went wrong");
    }
}
