pub fn stars(n: u32) -> String {
    let mut result = String::new();
    let pow = u32::pow(2, n);
    for _ in 0..pow {
        result.push('*');
    }
    result
}
