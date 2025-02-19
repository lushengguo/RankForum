pub mod crypto;
pub mod db;
pub mod field;
pub mod post;
pub mod score;
pub mod service;
pub mod user;
pub mod textual_integer;
pub mod db_trait;
use uuid::Uuid;

pub type Address = String;

pub fn generate_unique_address() -> Address {
    Uuid::new_v4().to_string()
}

pub fn generate_unique_name() -> String {
    Uuid::new_v4().to_string()
}
