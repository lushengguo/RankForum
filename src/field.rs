use crate::db::default_global_db;
use crate::post::Post;
use crate::Address;
use crate::db_trait::Database;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Field {
    pub name: String,
    pub address: String,
}

#[derive(Debug, PartialEq)]
pub enum Ordering {
    ByTimestamp,
    ByScore,
    ByUpVote,
    ByDownVote,
    ByUpvoteSubDownVote,
}

pub struct FilterOption {
    pub level: Option<u8>,
    pub keyword: Option<String>,
    pub ordering: Ordering,
    pub ascending: bool,
    pub max_results: u32,
}

impl Field {
    pub fn persist(&self) -> Result<(), String> {
        default_global_db().insert_field(self)
    }

    pub fn new(name: String, address: Address) -> Field {
        Field { name, address }
    }

    pub fn filter_posts(&self, option: FilterOption) -> Result<Vec<Post>, String> {
        default_global_db().filter_posts(&self.name, &option)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{generate_unique_address, generate_unique_name};

    #[test]
    fn test_field_persist() {
        let field = Field::new(generate_unique_name(), generate_unique_address());
        assert_eq!(field.persist(), Ok(()));

        let field = Field::new(field.name.clone(), field.address.clone());
        assert!(field.persist().is_err());
    }
}
