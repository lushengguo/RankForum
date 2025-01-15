use crate::field::FilterOption;
use crate::post::*;
use crate::Address;

pub struct DB {}

impl DB {
    pub fn new() -> Self {
        Self {}
    }

    pub fn score(field: &String, id: &Address) -> Option<i64> {
        None
    }

    pub fn update_score(field: &String, id: &Address) {}

    pub fn comment(id: &Address) -> Option<Comment> {
        None
    }

    pub fn update_comment(comment: &Comment) {}

    pub fn post(id: Address) -> Option<Post> {
        None
    }

    pub fn field(comment_or_post_id: &Address) -> Option<String> {
        None
    }

    pub fn update_post(post: &Post) {}

    pub fn filter_posts(field: &String, option: &FilterOption) -> Vec<Post> {
        // filter posts by level, returns max 100 posts
        vec![]
    }
}
