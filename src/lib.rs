pub mod crypto;
pub mod db;
pub mod field;
pub mod post;
pub mod score;
pub mod service;
pub mod user;
pub mod textual_integer;
use uuid::Uuid;

pub type Address = String;

pub fn generate_address() -> Address {
    Uuid::new_v4().to_string()
}

pub fn generate_name() -> String {
    Uuid::new_v4().to_string()
}
