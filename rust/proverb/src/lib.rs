pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.len() > 1 {
        proverb = list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .collect();
    }

    if list.len() >= 1 {
       proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    }

    proverb
}
