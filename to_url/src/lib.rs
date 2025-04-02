pub fn to_url(s: &str) -> String {
    // s.replace(" ", "%20")
    let mut url = String::new();
    for c in s.chars() {
        if c == ' ' {
            url.push_str("%20");
        } else {    
            url.push(c);
        }
    }
    url
}