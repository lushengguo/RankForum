use crate::db::*;
use crate::generate_address;
use crate::Address;

pub fn level(score: i64) -> u8 {
    (score as f64).log(100.0).floor() as u8
}

pub struct User {
    pub address: Address,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> User {
        User {
            address: generate_address(),
            name,
        }
    }
}
