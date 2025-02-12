use crate::db::global_db;
use crate::Address;

pub struct User {
    pub address: Address,
    pub name: String,
}

impl User {
    pub fn new(address: Address, name: String) -> User {
        User { address, name }
    }

    pub fn rename_user(&mut self, new_name: String) -> Result<(), String> {
        self.name = new_name;
        global_db().rename_user(self.address.clone(), self.name.clone())
    }
}
