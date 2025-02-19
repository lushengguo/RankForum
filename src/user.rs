use crate::db::global_db;
use crate::Address;
use crate::db_trait::Database;

pub struct User {
    pub address: Address,
    pub name: String,
}

impl User {
    pub fn new(address: Address, name: String) -> User {
        User { address, name }
    }

    pub fn persist(&self) -> Result<(), String> {
        global_db().upsert_user(self.address.clone(), self.name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate_unique_address, generate_unique_name};

    #[test]
    fn test_user_persist() {
        // ok
        let user = User::new(generate_unique_address(), generate_unique_name());
        assert_eq!(user.persist(), Ok(()));
        let user2 = User::new(generate_unique_address(), generate_unique_name());
        assert_eq!(user2.persist(), Ok(()));

        // name/address already exists
        let user = User::new(user.address.clone(), user2.name.clone());
        assert!(user.persist().is_err());
    }
}
