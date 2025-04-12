pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        return "Just say something!";
    }

    let is_question = text.ends_with("?");
    let is_yelling = text.chars().any(|c| c.is_ascii_alphabetic()) && 
                     text.chars().filter(|c| c.is_ascii_alphabetic())
                         .all(|c| c.is_uppercase());

    if is_question {
        if is_yelling {
            return "Quiet, I am thinking!";
        }
        return "Sure.";
    }

    if is_yelling {
        return "There is no need to yell, calm down!";
    }

    "Interesting"
}

