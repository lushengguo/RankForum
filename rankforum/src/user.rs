use crate::db::*;
use crate::generate_address;
use crate::Address;

pub fn level(score: i64) -> u8 {
    (score as f64).log(100.0).floor() as u8
}

pub struct User {
    pub id: Address,
    pub username: String,
    pub score: i64,
}

impl User {
    pub fn new(username: String) -> User {
        User {
            id: generate_address(),
            username,
            score: 0,
        }
    }

    pub fn upvote(&mut self, score: i64) {
        self.score += score;
    }

    pub fn downvote(&mut self, score: i64) {
        if self.score > score {
            self.score -= score;
        } else {
            self.score = 0;
        }
    }

    pub fn level(&self) -> u8 {
        (self.score as f64).log(100.0).floor() as u8
    }
}
