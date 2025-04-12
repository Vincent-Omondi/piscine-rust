pub fn score(s: &str) -> u64 {
    let mut total = 0;

    for c in s.chars() {
        match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => total += 1,
            'D' | 'G' => total += 2,
            'B' | 'C' | 'M' | 'P' => total += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => total += 4,
            'K' => total += 5,
            'J' | 'X' => total += 8,
            'Q' | 'Z' => total += 10,
            _ => total += 0,
        }
    }
    
    total
}