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
    pub fn new(address: Address, name: String) -> User {
        User { address, name }
    }

    pub fn rename(&mut self, new_name: String) -> Result<(), String> {
        self.name = new_name;
        DB::rename(self.address.clone(), self.name.clone())
    }
}
