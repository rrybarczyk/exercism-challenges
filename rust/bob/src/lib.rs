fn is_uppercase(message: &str) -> bool {
    // dbg!(message.as_bytes().iter().max().unwrap() < &97);
    message.as_bytes().iter().max().unwrap() < &97
}

fn contains_letters(message: &str) -> bool {
    message.as_bytes().iter().max().unwrap() >= &65
}

pub fn reply(message: &str) -> &str {
    let response: &str;

    let message = message.trim();

    if message.is_empty() {
        response = "Fine. Be that way!";
    } else {
        let message = message.trim();
        if !contains_letters(message) && message.ends_with("?") {
            response = "Sure.";
        } else if !contains_letters(message) {
            response = "Whatever.";
        } else if message.ends_with(".") || (message.ends_with("!") && !is_uppercase(message)) {
            response = "Whatever.";
        } else if message.ends_with("?")  && is_uppercase(message) {
            response = "Calm down, I know what I'm doing!";
        } else if is_uppercase(message) {
            response = "Whoa, chill out!";
        } else if message.ends_with("?")  || message.ends_with("!") && !is_uppercase(message) {
            response = "Sure.";
        } else {
            response = "Whatever.";
        }
    }
    response
}
