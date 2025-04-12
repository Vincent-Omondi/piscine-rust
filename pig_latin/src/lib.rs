pub fn pig_latin(text: &str) -> String {
    // Return early if empty string
    if text.is_empty() {
        return String::new();
    }
    
    let mut chars = text.chars();
    let first_char = chars.next().unwrap().to_ascii_lowercase();
    
    // Check if the word starts with a vowel
    if is_vowel(first_char) {
        // If starts with vowel, just add "ay"
        return format!("{}ay", text);
    }
    
    // Convert to a vector of chars for easier manipulation
    let chars: Vec<char> = text.chars().collect();
    
    // Find the position of the first vowel
    let mut first_vowel_pos = 1;
    while first_vowel_pos < chars.len() && !is_vowel(chars[first_vowel_pos]) {
        // Check for the 'qu' special case
        if first_vowel_pos > 0 && chars[first_vowel_pos - 1] == 'q' && chars[first_vowel_pos] == 'u' {
            first_vowel_pos += 1;
            break;
        }
        first_vowel_pos += 1;
    }
    
    // If no vowel was found or we're at the end, just add "ay"
    if first_vowel_pos >= chars.len() {
        return format!("{}ay", text);
    }
    
    // Create the Pig Latin word
    let mut result = String::new();
    
    // Add the part after the consonant cluster
    for i in first_vowel_pos..chars.len() {
        result.push(chars[i]);
    }
    
    // Add the consonant cluster at the end
    for i in 0..first_vowel_pos {
        result.push(chars[i]);
    }
    
    // Add "ay" at the end
    result.push_str("ay");
    
    result
}

// Helper function to check if a character is a vowel
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
