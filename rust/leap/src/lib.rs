pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 400 == 0 {
            return true; // if year is div 400 -> true
        } else if year % 100 == 0 {
            return false; // if year is div 100 -> false
        }
        true // year is div by 4 -> true
    } else {
        false
    }
}
