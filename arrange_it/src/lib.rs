pub fn arrange_phrase(phrase: &str) -> String {
    // Create a vector to store (position, word) pairs
    let mut words: Vec<(usize, &str)> = Vec::new();
    
    // Process each word in the phrase
    for word in phrase.split_whitespace() {
        // Find the position number in each word
        let mut position = 0;
        for c in word.chars() {
            if c.is_digit(10) {
                position = c.to_digit(10).unwrap() as usize;
                break;
            }
        }
        
        // Add the (position, word) pair to our vector
        words.push((position, word));
    }
    
    // Sort the words by position
    words.sort_by_key(|&(pos, _)| pos);
    
    // Create a vector of words without the numbers
    let result: Vec<String> = words
        .into_iter()
        .map(|(_, word)| {
            // Filter out digits from each word
            word.chars()
                .filter(|&c| !c.is_digit(10))
                .collect::<String>()
        })
        .collect();
    
    // Join the words with spaces
    result.join(" ")
}