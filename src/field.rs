
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
    // load from db to memory as cache
    pub fn save_to_db(&self) {
        global_db().insert_field(self);
    }

    // create new field instance
    pub fn new(name: String, address: Address) -> Field {
        // save to db
        Field { name, address }
    }

    pub fn filter_posts(&self, mut option: FilterOption) -> Vec<Post> {
        if option.max_results > 100 {
            option.max_results = 100;
        }
        global_db().filter_posts(&self.name, &option)
    }
}
