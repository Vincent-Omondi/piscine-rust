fn convert(s: &str) -> u32 {
    let mut a = s.to_string();
    a.pop();
    let n = a.parse::<f32>().unwrap();
    (n * 1000.0) as u32
}

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut v: Vec<u32> = Vec::new();

    for token in s.split_whitespace() {
        if token.contains("k") {
            v.push(convert(token));
        } else {
            v.push(token.parse::<u32>().unwrap());
        }
    }
    Box::new(v)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}