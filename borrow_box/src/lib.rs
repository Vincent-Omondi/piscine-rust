#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    // create the box
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    // read from the box using the reference `&`
    // return only the player that as the bigger score
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            self.p1.clone()
        } else if self.p1.1 < self.p2.1 {
            self.p2.clone()
        } else {
            (String::from("Same score! tied"), self.p2.1)
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.p1.1 + self.p2.1 < self.nb_games
            && self.p1.1 * 2 <= self.nb_games
            && self.p2.1 * 2 <= self.nb_games
        {
            if self.p1.0 == user_name {
                self.p1.1 += 1;
            } else if self.p2.0 == user_name {
                self.p2.1 += 1;
            }
        }
    }

    pub fn delete(self) -> String {
        String::from(format!("game deleted: id -> {:?}", self.id))
    }
}