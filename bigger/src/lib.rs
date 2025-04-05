use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    // h.into_values().max().unwrap()
    let mut max = 0;
    for (_, value) in h {
        if value > max {
            max = value;
        }
    }
    max
}