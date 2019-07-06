pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut num = n;
    let mut idx = 2;
    loop {
        if num % idx == 0 {
            primes.push(idx);
            num = num/idx;
        } else {
            idx += 1;
        }

        if num == 1 {
            break;
        }
    }
    primes
}
