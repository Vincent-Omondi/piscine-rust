use std::cmp;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    
    if source.is_empty() {
        return target.len();
    }
    
    if target.is_empty() {
        return source.len();
    }
    
    // Initialize a matrix with dimensions (source_len+1) x (target_len+1)
    let mut dp = vec![vec![0; target_chars.len() + 1]; source_chars.len() + 1];
    
    // Initialize the first row and column
    for i in 0..=source_chars.len() {
        dp[i][0] = i;
    }
    
    for j in 0..=target_chars.len() {
        dp[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=source_chars.len() {
        for j in 1..=target_chars.len() {
            let cost = if source_chars[i-1] == target_chars[j-1] { 0 } else { 1 };
            
            dp[i][j] = cmp::min(
                cmp::min(
                    dp[i-1][j] + 1,      // deletion
                    dp[i][j-1] + 1       // insertion
                ),
                dp[i-1][j-1] + cost     // substitution
            );
        }
    }
    
    dp[source_chars.len()][target_chars.len()]
}
