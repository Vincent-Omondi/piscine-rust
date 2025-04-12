pub fn talking(text: &str) -> &str {
    // Trim the input to handle cases where there might be only whitespace
    let trimmed = text.trim();
    
    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_question = trimmed.ends_with("?");
    let is_yelling = trimmed.chars().any(|c| c.is_ascii_alphabetic()) && 
                     trimmed.chars().filter(|c| c.is_ascii_alphabetic())
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

