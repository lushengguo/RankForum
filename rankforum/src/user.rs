use crate::db::*;
use crate::generate_address;
use crate::Address;

pub fn level(score: i64) -> u8 {
    (score as f64).log(100.0).floor() as u8
}

pub struct User {
    pub id: Address,
    pub username: String,
}

impl User {
    pub fn new(username: String) -> User {
        User {
            id: generate_address(),
            username,
        }
    }
}
