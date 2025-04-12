use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let alphabet: HashSet<char> = s
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    alphabet.len() == 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pangram() {
        assert!(is_pangram("the quick brown fox jumps over the lazy dog!"));
        assert!(!is_pangram("this is not a pangram!"));
    }
}
