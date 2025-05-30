pub fn is_empty(v: &str) -> bool {
    // v.is_empty()
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())    
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    // v.find(pat)
    for (i, c) in v.chars().enumerate() {
        if c == pat {
            return i;
        }
    }
    usize::MAX
}