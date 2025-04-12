
pub fn number_logic(num: u32) -> bool {
    // Convert number to string to count digits
    let num_str = num.to_string();
    let digits_count = num_str.len() as u32;
    
    // Calculate sum of each digit raised to power of number of digits
    let sum: u32 = num_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|digit| digit.pow(digits_count))
        .sum();
    
    // Return true if the sum equals the original number
    sum == num
}
