pub mod db;
pub mod field;
pub mod post;
pub mod score;
pub mod user;
pub mod server;
use uuid::Uuid;

pub type Address = String;

pub fn generate_address() -> Address {
    Uuid::new_v4().to_string()
}