/// Checks if a number is prime and returns it
fn is_prime(num: u32) -> Option<u32> {
    if num == 2 {
        return Some(num);
    } else if num <= 1 || num % 2 == 0 || num % 3 == 0 {
        return None;
    }
    let mut i = 5;
    while (i * i) <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return None;
        }
        i += 1;
    }
    Some(num)
}

pub fn nth(n: u32) -> u32 {
    if n == 0 { return 2; }
    if n == 1 { return 3; }

    let mut tmp = 1;
    let mut prime = 2;
    let mut prime_count = 0;

    while prime_count < n {
        if let Some(x) = is_prime(tmp) {
            prime_count += 1;
            prime = x;
        };
        tmp += 1;
    }
    prime
}
