pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '-' if !result.is_empty() => {
                result.pop();
            }
            '+' => {
                let mut skip_count = 1;
                while i + 1 < chars.len() && chars[i + 1] == '+' {
                    skip_count += 1;
                    i += 1;
                }
                i += skip_count.min(chars.len() - i - 1);
            }
            ch => result.push(ch),
        }
        i += 1;
    }
    *s = result;
}

pub fn do_operations(strings: &mut [String]) {
    for s in strings.iter_mut() {
        if let Some(result) = evaluate_expression(s) {
            *s = result.to_string();
        }
    }
}

fn evaluate_expression(expr: &str) -> Option<i32> {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = None;

    for ch in expr.chars() {
        match ch {
            '0'..='9' => {
                if operator.is_none() {
                    num1.push(ch);
                } else {
                    num2.push(ch);
                }
            }
            '+' | '-' if operator.is_none() => {
                operator = Some(ch);
            }
            _ => {}
        }
    }

    let num1 = num1.parse::<i32>().ok()?;
    let num2 = num2.parse::<i32>().ok()?;

    match operator {
        Some('+') => Some(num1 + num2),
        Some('-') => Some(num1 - num2),
        _ => None,
    }
}
