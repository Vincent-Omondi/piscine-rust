pub fn num_to_ordinal(x: u32) -> String {
    let last_digit = x % 10;
    let last_two_digits = x % 100;

    if last_two_digits >= 11 && last_two_digits <= 13 {
        format!("{}th", x)
    } else {
        match last_digit {
            1 => format!("{}st", x),
            2 => format!("{}nd", x),
            3 => format!("{}rd", x),
            _ => format!("{}th", x),
        }
    }
}