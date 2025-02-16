use crate::db::global_db;
use crate::post::Post;
use crate::Address;

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub address: String,
}

pub struct FilterOption {
    pub level: Option<u8>,
    pub keyword: Option<String>,
    pub ascending_by_timestamp: bool,
    pub ascending_by_absolute_score: bool,
    pub max_results: u32,
}

impl Field {
    pub fn persist(&self) -> Result<(), String> {
        global_db().insert_field(self)
    }

    pub fn new(name: String, address: Address) -> Field {
        Field { name, address }
    }

    pub fn filter_posts(&self, mut option: FilterOption) -> Vec<Post> {
        if option.max_results > 100 {
            option.max_results = 100;
        }
        global_db().filter_posts(&self.name, &option)
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

    // #[test]
    // fn test_field_filter_posts() {
    //     let field = Field::new("test".to_string(), "test".to_string());
    //     let option = FilterOption {
    //         level: None,
    //         keyword: None,
    //         ascending_by_timestamp: false,
    //         ascending_by_absolute_score: false,
    //         max_results: 10,
    //     };
    //     assert_eq!(field.filter_posts(option).len(), 0);
    // }
}
