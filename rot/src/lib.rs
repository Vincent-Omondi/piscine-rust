pub fn rotate(input: &str, key: i8) -> String {
    // Normalize the key to be within 0-25 range
    let normalized_key = ((key % 26) + 26) % 26;
    
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            // Convert to 0-25 range, apply rotation, and convert back
            let rotated = (((c as u8 - base) as i8 + normalized_key) % 26) as u8 + base;
            rotated as char
        } else {
            // Non-alphabetic characters remain unchanged
            c
        }
    }).collect()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_rotate_basic() {
//         let input = "Hello, World!";
//         let key = 3;
//         let result = rotate(input, key);
//         assert_eq!(result, "Khoor, Zruog!");    
//     }

//     #[test]
//     fn test_rotate_wrap_around() {
//         let input = "Zebra";
//         let key = 1;
//         let result = rotate(input, key);                
//         assert_eq!(result, "Afcsb");
//     }

//     #[test]
//     fn test_rotate_non_alphabetic() {
//         let input = "Hello, World! 123";
//         let key = 5;
//         let result = rotate(input, key);
//         assert_eq!(result, "Mjqqt, Btwqi! 123");
//     }

//     #[test]
//     fn test_rotate_negative_key() {
//         let input = "Hello, World!";
//         let key = -3;   
//         let result = rotate(input, key);
//         assert_eq!(result, "Ebiil, Tloia!");
//     }

//     #[test]
//     fn test_rotate_large_key() {
//         let input = "Hello, World!";
//         let key = 26;
//         let result = rotate(input, key);
//         assert_eq!(result, "Hello, World!");
//     }

//     #[test] 
//     fn test_rotate_empty_string() {
//         let input = "";
//         let key = 3;
//         let result = rotate(input, key);
//         assert_eq!(result, "");
//     }   
// }
    
