pub fn first_subword(mut s: String) -> String {
    if s.is_empty() {
        return s;
    }
    
    let mut result = String::new();
    let mut chars = s.chars();
    
    // Handle the first character (special case for PascalCase)
    if let Some(first_char) = chars.next() {
        result.push(first_char);
    }
    
    // Process the rest of the characters
    while let Some(c) = chars.next() {
        // Stop at uppercase letter (for camelCase and PascalCase)
        if c.is_uppercase() {
            break;
        }
        // Stop at underscore (for snake_case)
        if c == '_' {
            break;
        }
        result.push(c);
    }
    
    result
}
