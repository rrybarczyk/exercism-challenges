pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

// pub fn reverse(input: &str) -> String {
//     let mut rev_vec: Vec<char> = Vec::new();
//     for i in input.chars().rev() {
//         rev_vec.push(i);
//     }
//     rev_vec.iter().collect()
// }

