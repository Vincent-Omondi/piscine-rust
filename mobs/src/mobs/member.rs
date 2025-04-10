#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub age: u8,
    pub role: Role,
}

#[derive(Debug, Clone)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Member {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
            role: Role::Associate,
        }
    }
} 