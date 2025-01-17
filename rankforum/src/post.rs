use std::collections::{HashMap, HashSet};

use crate::db::*;
use crate::field::Field;
use crate::score::calculate_vote_impact;
use crate::user::level;
use crate::{generate_address, Address};

use chrono::Utc;
use log::{error, warn};

pub struct Comment {
    pub address: Address,
    pub from: Address,
    pub to: Address,

    // score reflects value of this comment, the highest score comment will be shown first
    // and it's able to be negative
    pub score: i64,
    pub upvote: u64,
    pub downvote: u64,

    pub content: String,
    pub timestamp: i64,
}

impl Comment {
    pub fn new(from: Address, to: Address, content: String) -> Comment {
        let comment = Comment {
            from,
            to,
            score: 0,
            upvote: 0,
            downvote: 0,
            content,
            timestamp: Utc::now().timestamp(),
            address: generate_address(),
        };

        DB::update_comment(&comment);
        return comment;
    }

    fn calculate_vote_impact(&self, voter: &Address) -> i64 {
        // this would not fail, if failed means db is corrupted or code bug
        let field = DB::field_by_address(&self.to).unwrap();

        let voter_score = match DB::score(&field.address, &voter) {
            Some(score) => score,
            None => {
                warn!("User {} not found in field {}", self.from, field.address);
                return 0;
            }
        };
        let voter_level = level(voter_score);
        let self_level = level(self.score);

        calculate_vote_impact(self_level, voter_level)
    }

    pub fn upvote(&mut self, upvoter: &Address) {
        let impact = self.calculate_vote_impact(upvoter);
        if impact == 0 {
            error!("Vote impact is 0, this should not happen");
            return;
        }
        self.score += impact;
        self.upvote += 1;
        DB::update_comment(&self);
    }

    pub fn downvote(&mut self, downvoter: &Address) {
        let impact = self.calculate_vote_impact(downvoter);
        if impact == 0 {
            error!("Vote impact is 0, this should not happen");
            return;
        }
        self.score -= impact;
        self.downvote += 1;
        DB::update_comment(&self);
    }
}

type DirectCommentAddress = Address;
type InDirectCommentAddress = Address;
pub struct Post {
    pub address: Address,
    pub from: Address,
    pub to: Address,

    pub title: String,
    pub content: String,
    pub score: i64,
    pub upvote: u64,
    pub downvote: u64,
    pub timestamp: i64,

    // comments are lazy to load in memory
    // only queried comments will be loaded
    pub comments: HashMap<DirectCommentAddress, HashSet<InDirectCommentAddress>>,
}

impl Post {
    pub fn new(from: Address, field_address: Address, title: String, content: String) -> Post {
        let post = Post {
            address: generate_address(),
            from: from.clone(),
            to: field_address,
            title,
            content,
            score: 0,
            upvote: 0,
            downvote: 0,
            timestamp: Utc::now().timestamp(),
            comments: HashMap::new(),
        };

        DB::update_post(&post);
        post
    }

    fn calculate_vote_impact(&self, voter: &Address) -> i64 {
        // this would not fail, if failed means db is corrupted or code bug
        let field = DB::field_by_address(&self.address).unwrap();

        let voter_score = match DB::score(&field.address, &voter) {
            Some(score) => score,
            None => {
                warn!("User {} not found in field {}", self.from, field.address);
                return 0;
            }
        };
        let voter_level = level(voter_score);
        let self_level = level(self.score);

        calculate_vote_impact(self_level, voter_level)
    }

    pub fn upvote(&mut self, upvoter: &Address) {
        let impact = self.calculate_vote_impact(upvoter);
        if impact == 0 {
            error!("Vote impact is 0, this should not happen");
            return;
        }
        self.score += impact;
        self.upvote += 1;
        DB::update_post(&self);
    }

    pub fn downvote(&mut self, downvoter: &Address) {
        let impact = self.calculate_vote_impact(downvoter);
        if impact == 0 {
            error!("Vote impact is 0, this should not happen");
            return;
        }
        self.score -= impact;
        self.downvote += 1;
        DB::update_post(&self);
    }

    pub fn comment(&mut self, comment: &String, from: &Address) {
        let comment = Comment::new(from.clone(), self.address.clone(), comment.clone());
        self.comments
            .entry(comment.address.clone())
            .or_insert(HashSet::new())
            .insert(comment.address.clone());
        DB::update_post(&self);
    }

    pub fn comment_on_comment(&mut self, comment: &String, from: &Address, to: &Address) {
        let comment = Comment::new(from.clone(), to.clone(), comment.clone());
        self.comments
            .entry(to.clone())
            .or_insert(HashSet::new())
            .insert(comment.address.clone());
        DB::update_post(&self);
    }
}
