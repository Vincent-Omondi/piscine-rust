pub fn pig_latin(text: &str) -> String {
    // Return early if empty string
    if text.is_empty() {
        return String::new();
    }
    
    let chars: Vec<char> = text.chars().collect();
    
    // Check if the word starts with a vowel
    if is_vowel(chars[0]) {
        // If starts with vowel, just add "ay"
        return format!("{}ay", text);
    }
    
    let mut result = String::new();
    
    // Find the index where to split the word
    let split_index = find_split_index(&chars);
    
    // Build the result
    // First part: everything after the split index
    for i in split_index..chars.len() {
        result.push(chars[i]);
    }
    
    // Second part: everything before the split index
    for i in 0..split_index {
        result.push(chars[i]);
    }
    
    // Add "ay" at the end
    result.push_str("ay");
    
    result
}

// Helper function to find where to split the word
fn find_split_index(chars: &[char]) -> usize {
    // Special case handling for words starting with 'qu'
    if chars.len() >= 2 && chars[0].to_ascii_lowercase() == 'q' && chars[1].to_ascii_lowercase() == 'u' {
        return 1; // For "queen", split after "q" so "u" becomes start of the new word
    }
    
    // Check for special case: consonant + "qu"
    if chars.len() >= 3 && !is_vowel(chars[0]) && 
       chars[1].to_ascii_lowercase() == 'q' && chars[2].to_ascii_lowercase() == 'u' {
        // For word like "square", split after "squ" (index 3)
        return 3;
    }
    
    // Normal case: find the first vowel
    for i in 1..chars.len() {
        if is_vowel(chars[i]) {
            return i;
        }
    }
    
    // If no vowel found, return the length of the word
    chars.len()
}

// Helper function to check if a character is a vowel
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
