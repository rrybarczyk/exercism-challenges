pub fn verse(n: i32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else if n ==2 {
        "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
    } else {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();

    for i in (end..=start).rev() {
        song.push_str(&verse(i));
        if i != end {
            song.push_str("\n");
        }
    }
    song
}
