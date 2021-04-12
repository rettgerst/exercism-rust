pub fn reply(message: &str) -> &str {
    if message.trim().len() == 0 {
        return "Fine. Be that way!";
    }
    if message.chars().any(|c| c.is_alphabetic()) && message.to_uppercase() == message {
        if message.chars().last().unwrap() == '?' {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    }

    if message.trim().chars().last().unwrap() == '?' {
        return "Sure.";
    }

    "Whatever."
}
