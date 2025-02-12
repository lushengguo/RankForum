use std::collections::{HashMap, HashSet};

use crate::db::global_db;
use crate::score::*;
use crate::textual_integer::TextualInteger;
use crate::{generate_address, Address};

use chrono::Utc;
use log::{error, warn};

#[derive(Debug, PartialEq)]
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
            address: generate_address(),
            field_address,
        }
    }

    pub fn persist(&self) -> Result<(), String> {
        global_db().insert_comment(self)
    }

    fn calculate_vote_score(&self, voter: &Address) -> TextualInteger {
        // this would not fail, if failed means db is corrupted or code bug
        let field = global_db().select_field(None, Some(self.to.clone())).unwrap();

        let voter_score = match global_db().select_score(&field.address, voter) {
            Ok(score) => score,
            Err(_) => {
                warn!("User {} not found in field {}", self.from, field.address);
                return TextualInteger::new("0");
            }
        };
        let voter_level = level(&voter_score.score);
        let self_level = level(&self.score);

        calculate_vote_score(self_level, voter_level)
    }

    pub fn upvote(&mut self, upvoter: &Address) {
        let vote_score = self.calculate_vote_score(upvoter);
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return;
        }
        self.score += vote_score;
        self.upvote += 1;
        global_db().insert_comment(self);
    }

    pub fn downvote(&mut self, downvoter: &Address) {
        let vote_score = self.calculate_vote_score(downvoter);
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return;
        }
        self.score -= vote_score;
        self.downvote += 1;
        global_db().insert_comment(self);
    }
}

type DirectCommentAddress = Address;
type InDirectCommentAddress = Address;

#[derive(Debug, PartialEq)]
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
    pub comments: HashMap<DirectCommentAddress, HashSet<InDirectCommentAddress>>,
}

impl Post {
    pub fn new(from: Address, field_address: Address, title: String, content: String) -> Post {
        Post {
            address: generate_address(),
            from: from.clone(),
            to: field_address,
            title,
            content,
            score: TextualInteger::new("0"),
            upvote: 0,
            downvote: 0,
            timestamp: Utc::now().timestamp(),
            comments: HashMap::new(),
        }
    }

    pub fn persist(&self) -> Result<(), String> {
        global_db().insert_post(self)
    }

    fn calculate_vote_score(&self, voter: &Address) -> TextualInteger {
        // this would not fail, if failed means db is corrupted or code bug
        let field = global_db().select_field(None, Some(self.address.clone())).unwrap();

        let voter_score = match global_db().select_score(&field.address, voter) {
            Ok(score) => score,
            Err(_) => {
                warn!("User {} not found in field {}", self.from, field.address);
                return TextualInteger::new("0");
            }
        };
        let voter_level = level(&voter_score.score);
        let self_level = level(&self.score);

        calculate_vote_score(self_level, voter_level)
    }

    pub fn upvote(&mut self, upvoter: &Address) {
        let vote_score = self.calculate_vote_score(upvoter);
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return;
        }
        self.score += vote_score;
        self.upvote += 1;
        global_db().insert_post(self);
    }

    pub fn downvote(&mut self, downvoter: &Address) {
        let vote_score = self.calculate_vote_score(downvoter);
        if vote_score == TextualInteger::new("0") {
            error!("Vote vote_score is 0, this should not happen");
            return;
        }
        self.score -= vote_score;
        self.downvote += 1;
        global_db().insert_post(self);
    }

    pub fn select_comment(&mut self, comment: &String, from: &Address) {
        let comment = Comment::new(from.clone(), self.address.clone(), comment.clone(), self.to.clone());
        self.comments
            .entry(comment.address.clone())
            .or_default()
            .insert(comment.address.clone());
        global_db().insert_post(self);
    }

    pub fn comment_on_comment(&mut self, comment: &String, from: &Address, to: &Address) {
        let comment = Comment::new(from.clone(), to.clone(), comment.clone(), self.to.clone());
        self.comments
            .entry(to.clone())
            .or_default()
            .insert(comment.address.clone());
        global_db().insert_post(self);
    }
}
