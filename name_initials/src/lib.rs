pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter().map(|name| {
        name.split_whitespace()
            .map(|word| word.chars().next().unwrap())
            .map(|c| format!("{}.", c))
            .collect::<Vec<String>>()
            .join(" ")
    }).collect()    
}
