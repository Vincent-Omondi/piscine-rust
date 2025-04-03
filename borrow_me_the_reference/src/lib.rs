pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    while i < s.len() {
        let current_char = s.chars().nth(i).unwrap();
        if current_char == '-' && i > 0 {
            s.remove(i);
            s.remove(i-1);
            i = i.saturating_sub(1); // Move back one position
        } else if current_char == '+' && i < s.len() -1 {
            s.remove(i);
            s.remove(i);
        } else {
            i+= 1; // Move to the next character
        }
    }
}

pub fn do_operations(v: &mut [String]) {
    for string in v.inter_mut() {
        if string.contains('+') {
            let parts: Vec<&str> = string.split('+').collect();
            let a: i32 = parts[0].parse().unwrap();
            let b: i32 = parts[1].parse().unwrap();
            *string = (a + b).to_string();
        } else if string.contains('-') {
            let parts: Vec<&str> = string.split('-').collect();
            let a: i32 = parts[0].parse().unwrap();
            let b: i32 = parts[1].parse().unwrap();
            *string = (a - b).to_string();
        }
    }
}