pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    // Original value, exponential function, natural logarithm of absolute value
    let exp = (c as f64).exp();
    let ln = if c == 0 {
        f64::NEG_INFINITY
    } else {
        (c as f64).abs().ln()
    };
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    // Original string, and exponential function of each number in the string
    let exp_values = a
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap_or(0.0).exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    
    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    // Original vector, and natural logarithm of each absolute value
    let ln_values = b
        .iter()
        .map(|&x| {
            if x == 0 {
                f64::NEG_INFINITY
            } else {
                (x as f64).abs().ln()
            }
        })
        .collect();
    
    (b, ln_values)
} 