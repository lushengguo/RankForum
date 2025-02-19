use crate::field::{Field, FilterOption};
use crate::post::{Comment, Post};
use crate::score::Score;
use crate::textual_integer::TextualInteger;
use crate::user::User;
use crate::Address;

pub trait Database {
    fn init(&self) -> Result<(), String>;
    fn upsert_user(&self, address: Address, name: String) -> Result<(), String>;
    fn select_user(&self, name: Option<String>, address: Option<Address>) -> Option<User>;
    fn select_score(&self, address: &str, field_address: &str) -> Score;
    fn select_all_fields(&self) -> Vec<Field>;
    fn select_comment(&self, address: &Address) -> Result<Comment, String>;
    fn upsert_comment(&self, comment: &Comment) -> Result<(), String>;
    fn select_post(&self, address: &str) -> Result<Post, String>;
    fn upsert_post(&self, post: &Post) -> Result<(), String>;
    fn insert_field(&self, field: &Field) -> Result<(), String>;
    fn select_field(&self, name: Option<String>, address: Option<Address>) -> Result<Field, String>;
    fn field_by_address(&self, comment_or_post_id: &Address) -> Option<Field>;
    fn filter_comments(&self, to: &Address, option: &FilterOption) -> Result<Vec<Comment>, String>;
    fn filter_posts(&self, to: &Address, option: &FilterOption) -> Result<Vec<Post>, String>;
    fn upvote(
        &self,
        from: &Address,
        to: &Address,
        voted_score: TextualInteger,
        field_address: &str,
    ) -> Result<(), String>;
    fn downvote(
        &self,
        from: &Address,
        to: &Address,
        voted_score: TextualInteger,
        field_address: &str,
    ) -> Result<(), String>;
}
