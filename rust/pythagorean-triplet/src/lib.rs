use std::collections::HashSet;

/// Given the `sum`, return all possible Pythagorean triplets, which produce the said sum, or an
/// empty HashSet if there are no such triplets. Note that you are expected to return triplets in
/// [a, b, c] order, where a < b < c.
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let limit = sum as i32;
    let mut hmap: HashSet<[u32; 3]> = HashSet::new();
    let mut triplet: [u32; 3];
    let mut a: i32;
    let mut b: i32;
    let mut c: i32 = 0;

    let mut m: i32 = 2;
    while c < limit {
        for n in 1..m {
            let mut k: i32 = 1;
            a = k * (m.pow(2) - n.pow(2));
            b = k * 2 * m * n;
            c = k * (m.pow(2) + n.pow(2));

            let mut mul_sum: i32 = 0;
            while mul_sum < limit {
                mul_sum = k * (a + b + c);
                if mul_sum == limit {
                    triplet = [(k * a) as u32, (k * b) as u32, (k * c) as u32];
                    triplet.sort();
                    hmap.insert(triplet);
                }
                k += 1;
            }

            if c > limit {
                break;
            }
        }
        m += 1;
    }
    hmap
}
