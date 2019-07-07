fn get_num_digits(num: u32) -> u32 {
    let mut num_digits = 1;
    let mut denom = 10;
    let mut quotient = num/denom;

    while quotient > 0 {
        num_digits += 1;
        denom *= 10;
        quotient = num / denom;
    }
    num_digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 { return true; }

    let num_digits = get_num_digits(num);

    let mut armstrong: Vec<u32> = Vec::new();
    let mut denom: u32;
    let mut n = num;
    let mut i = 1;
    let mut b: u32;

    while armstrong.len() < (num_digits as usize) {
        denom = 10_u32.pow(num_digits - i);
        b = n / denom;
        armstrong.push(b.pow(num_digits));
        if let Some(x) = n.checked_sub(denom * b) {
            n = x;
        } else {
            panic!("Arithmatic error");
        }
        i += 1;
    }
    let my_sum: u32 = armstrong.iter().sum();
    my_sum == num
}
