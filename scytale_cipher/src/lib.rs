/// Implements the scytale cipher (Spartan cipher)
/// 
/// The message is written across a strip wrapped around a cylinder
/// with the size parameter indicating the number of times (or columns) 
/// the strip wraps around the cylinder.
///
/// # Arguments
/// * `message` - The message to encode
/// * `size` - The number of columns (wraps around the cylinder)
///
/// # Returns
/// The encoded message
pub fn scytale_cipher(s: String, i: u32) -> String {
    if i as usize >= s.chars().count() || i == 1 {
        return s.to_string();
    }

    let width = (s.chars().count() as f64 / i as f64).ceil() as usize;
    let mut table = vec![vec![' '; width]; i as usize];

    for (pos, element) in s.chars().enumerate() {
        let col = pos % i as usize;
        let row = pos / i as usize;

        table[col][row] = element;
    }
    table
        .iter()
        .flatten()
        .collect::<String>()
        .trim_end()
        .to_string()
}