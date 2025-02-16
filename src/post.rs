use std::collections::{HashMap, HashSet};

use crate::db::global_db;
use crate::field::FilterOption;
use crate::score::{self, calculate_vote_score};
use crate::textual_integer::TextualInteger;
use crate::{generate_unique_address, Address};

use chrono::Utc;
use log::{error, warn};
use rusqlite::fallible_iterator::Filter;

#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    pub address: Address,
    pub from: Address,
    pub to: Address,

    // score reflects value of this comment, the highest score comment will be shown first
    // and it's able to be negative
    pub score: TextualInteger,
    pub upvote: u64,
    pub downvote: u64,

    pub content: String,
    pub timestamp: i64,

    pub field_address: Address,

    pub comments: Vec<Comment>,
}

fn inner_calculate_vote_score(
    field_address: &str,
    from: &str,
    self_score: &TextualInteger,
) -> Result<TextualInteger, String> {
    let field = global_db().select_field(None, Some(field_address.to_string())).unwrap();
    let voter_score = global_db().select_score(&field.address, from);
    let voter_level = score::level(&voter_score.score);
    let self_level = score::level(&self_score);

    Ok(score::calculate_vote_score(self_level, voter_level))
}

impl Comment {
    pub fn new(from: Address, to: Address, content: String, field_address: Address) -> Comment {
        Comment {
            from,
            to,
            score: TextualInteger::new("0"),
            upvote: 0,
            downvote: 0,
            content,
            timestamp: Utc::now().timestamp(),
            address: generate_unique_address(),
            field_address,
            comments: vec![],
        }
    }

    pub fn from_db(address: Address) -> Result<Comment, String> {
        global_db().select_comment(&address)
    }

    pub fn persist(&self) -> Result<(), String> {
        global_db().upsert_comment(self)
    }

    fn calculate_vote_score(&self, voter: &Address) -> Result<TextualInteger, String> {
        let field = global_db().select_field(None, Some(self.field_address.clone()))?;
        inner_calculate_vote_score(&field.address, voter, &self.score)
    }

    pub fn upvote(&mut self, upvoter: &Address) -> Result<(), String> {
        let vote_score = self.calculate_vote_score(upvoter)?;
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return Err("Vote vote_score is 0".to_string());
        }
        self.score += vote_score;
        self.upvote += 1;
        global_db().upsert_comment(self)
    }

    pub fn downvote(&mut self, downvoter: &Address) -> Result<(), String> {
        let vote_score = self.calculate_vote_score(downvoter)?;
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return Err("Vote vote_score is 0".to_string());
        }
        self.score -= vote_score;
        self.downvote += 1;
        global_db().upsert_comment(self)
    }

    pub fn lazy_load_comments(&mut self, option: &FilterOption) -> Result<Vec<Comment>, String> {
        self.comments = global_db().filter_comments(&self.address, &option)?;
        Ok(self.comments.clone())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Post {
    pub address: Address,
    pub from: Address,
    pub to: Address,

    pub title: String,
    pub content: String,
    pub score: TextualInteger,
    pub upvote: u64,
    pub downvote: u64,
    pub timestamp: i64,

    // comments are lazy to load in memory
    // only queried comments will be loaded
    pub comments: Vec<Comment>,
}

impl Post {
    pub fn new(from: Address, field_address: Address, title: String, content: String) -> Post {
        Post {
            address: generate_unique_address(),
            from: from.clone(),
            to: field_address,
            title,
            content,
            score: TextualInteger::new("0"),
            upvote: 0,
            downvote: 0,
            timestamp: Utc::now().timestamp(),
            comments: Vec::new(),
        }
    }

    pub fn from_db(address: Address) -> Result<Post, String> {
        global_db().select_post(&address)
    }

    pub fn persist(&self) -> Result<(), String> {
        global_db().upsert_post(self)
    }

    fn calculate_vote_score(&self, voter: &Address) -> Result<TextualInteger, String> {
        let field = global_db().select_field(None, Some(self.to.clone())).unwrap();
        inner_calculate_vote_score(&field.address, voter, &self.score)
    }

    pub fn upvote(&mut self, upvoter: &Address) -> Result<(), String> {
        let vote_score = self.calculate_vote_score(upvoter)?;
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return Err("Vote vote_score is 0".to_string());
        }
        self.score += vote_score;
        self.upvote += 1;
        global_db().upsert_post(self)
    }

    pub fn downvote(&mut self, downvoter: &Address) -> Result<(), String> {
        let vote_score = self.calculate_vote_score(downvoter)?;
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return Err("Vote vote_score is 0".to_string());
        }
        self.score -= vote_score;
        self.downvote += 1;
        global_db().upsert_post(self)
    }

    pub fn lazy_load_comments(&mut self, option: &FilterOption) -> Result<Vec<Comment>, String> {
        self.comments = global_db().filter_comments(&self.address, &option)?;
        Ok(self.comments.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::Field;
    use crate::user::User;
    use crate::{generate_unique_address, generate_unique_name};

    fn new_persisted_field() -> Field {
        let field = Field::new(generate_unique_name(), generate_unique_address());
        assert_eq!(field.persist(), Ok(()));
        field
    }

    fn new_persisted_post(field_address: &str) -> Post {
        let post = Post::new(
            generate_unique_address(),
            field_address.to_string(),
            "test".to_string(),
            "test".to_string(),
        );
        assert_eq!(post.persist(), Ok(()));
        post
    }

    fn new_persisted_user() -> User {
        let user = User::new(generate_unique_address(), generate_unique_name());
        assert_eq!(user.persist(), Ok(()));
        user
    }

    #[test]
    fn test_comment_persist() {
        // field not exists
        let comment = Comment::new(
            generate_unique_address(),
            generate_unique_address(),
            "test".to_string(),
            generate_unique_address(),
        );
        assert!(comment.persist().is_err());

        // post not exists
        let field = new_persisted_field();
        let comment = Comment::new(
            generate_unique_address(),
            generate_unique_address(),
            "test".to_string(),
            field.address.clone(),
        );
        assert!(comment.persist().is_err());

        // post field didn't match comment field
        let field2 = new_persisted_field();
        let post = new_persisted_post(&field2.address);
        let comment = Comment::new(
            generate_unique_address(),
            post.address.clone(),
            "test".to_string(),
            field.address.clone(),
        );
        assert!(comment.persist().is_err());

        // ok
        let post = new_persisted_post(&field.address);
        let comment = Comment::new(
            generate_unique_address(),
            post.address.clone(),
            "test".to_string(),
            field.address.clone(),
        );
        assert_eq!(comment.persist(), Ok(()));
    }

    #[test]
    fn test_comment_from_db() {
        let field = new_persisted_field();
        let post = new_persisted_post(&field.address);
        let comment = Comment::new(
            generate_unique_address(),
            post.address.clone(),
            "test".to_string(),
            field.address.clone(),
        );
        assert_eq!(comment.persist(), Ok(()));

        let comment_from_db = Comment::from_db(comment.address.clone()).unwrap();
        assert_eq!(comment, comment_from_db);
    }

    #[test]
    fn test_comment_upvote() {
        let field = new_persisted_field();
        let post = new_persisted_post(&field.address);
        let mut comment = Comment::new(
            generate_unique_address(),
            post.address.clone(),
            "test".to_string(),
            field.address.clone(),
        );
        assert_eq!(comment.persist(), Ok(()));

        // user not exists
        assert!(comment.upvote(&generate_unique_address()).is_ok());
        assert_eq!(comment.score, TextualInteger::new("1"));

        // user exists
        let user = new_persisted_user();
        assert_eq!(comment.upvote(&user.address), Ok(()));
        assert_eq!(comment.score, TextualInteger::new("2"));
    }

    #[test]
    fn test_comment_downvote() {
        let field = new_persisted_field();
        let post = new_persisted_post(&field.address);
        let mut comment = Comment::new(
            generate_unique_address(),
            post.address.clone(),
            "test".to_string(),
            field.address.clone(),
        );
        assert_eq!(comment.persist(), Ok(()));

        // user not exists
        assert!(comment.downvote(&generate_unique_address()).is_ok());
        assert_eq!(comment.score, TextualInteger::new("-1"));

        // user exists
        let user = new_persisted_user();
        assert_eq!(comment.downvote(&user.address), Ok(()));
        assert_eq!(comment.score, TextualInteger::new("-2"));
    }

    #[test]
    fn test_post_persist() {
        // field not exists
        let post = Post::new(
            generate_unique_address(),
            generate_unique_address(),
            "test".to_string(),
            "test".to_string(),
        );
        assert!(post.persist().is_err());

        // ok
        let field = new_persisted_field();
        let post = Post::new(
            generate_unique_address(),
            field.address.clone(),
            "test".to_string(),
            "test".to_string(),
        );
        assert_eq!(post.persist(), Ok(()));
    }

    #[test]
    fn test_post_from_db() {
        let field = new_persisted_field();
        let post = new_persisted_post(&field.address);
        assert_eq!(Post::from_db(post.address.clone()).unwrap(), post);

        // not exists
        assert!(Post::from_db(generate_unique_address()).is_err());
    }

    #[test]
    fn test_post_upvote() {
        let field = new_persisted_field();
        let mut post = new_persisted_post(&field.address);

        // user not exists
        assert!(post.upvote(&generate_unique_address()).is_ok());
        assert_eq!(post.score, TextualInteger::new("1"));

        // user exists
        let user = new_persisted_user();
        assert_eq!(post.upvote(&user.address), Ok(()));
        assert_eq!(post.score, TextualInteger::new("2"));
    }

    #[test]
    fn test_post_downvote() {
        let field = new_persisted_field();
        let mut post = new_persisted_post(&field.address);

        // user not exists
        assert!(post.downvote(&generate_unique_address()).is_ok());
        assert_eq!(post.score, TextualInteger::new("-1"));

        // user exists
        let user = new_persisted_user();
        assert_eq!(post.downvote(&user.address), Ok(()));
        assert_eq!(post.score, TextualInteger::new("-2"));
    }

    use crate::field::{FilterOption, Ordering};

    fn make_comment(
        from: &Address,
        to: &Address,
        field: &Field,
        content: &str,
        timestamp: i64,
    ) -> Result<Comment, String> {
        let mut comment = Comment::new(from.clone(), to.clone(), content.to_string(), field.address.clone());
        comment.timestamp = timestamp;
        comment.persist()?;
        Ok(comment)
    }

    #[test]
    fn test_lazy_load_comments() {
        let field = new_persisted_field();
        let mut post = new_persisted_post(&field.address);

        let option = FilterOption {
            level: None,
            keyword: None,
            ordering: Ordering::ByTimestamp,
            ascending: true,
            max_results: 10,
        };
        assert_eq!(post.lazy_load_comments(&option), Ok(vec![]));

        let comment1 = make_comment(&generate_unique_address(), &post.address, &field, "test1", 1).unwrap();
        let comment2 = make_comment(&generate_unique_address(), &post.address, &field, "test2", 2).unwrap();
        let comment3 = make_comment(&generate_unique_address(), &comment2.address, &field, "test3", 3).unwrap();
        let comment4 = make_comment(&generate_unique_address(), &comment3.address, &field, "test4", 4).unwrap();

        let mut comments = post.lazy_load_comments(&option).unwrap();
        assert_eq!(comments.len(), 2);
        assert_eq!(comments, vec![comment1, comment2]);

        let mut comments2 = comments[1].lazy_load_comments(&option).unwrap();
        assert_eq!(comments2.len(), 1);
        assert_eq!(comments2, vec![comment3]);

        let comments3 = comments2[0].lazy_load_comments(&option).unwrap();
        assert_eq!(comments3.len(), 1);
        assert_eq!(comments3, vec![comment4]);
    }
}
