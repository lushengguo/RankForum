use crate::Address;

pub fn calculate_vote_score(poster_level: u8, voter_level: u8) -> i64 {
    let poster_level_score = 100_u64.pow(poster_level as u32) as i64;
    let voter_level_score = 100_u64.pow(voter_level as u32) as i64;
    if voter_level_score > poster_level_score * 10 {
        return poster_level_score * 10;
    }
    voter_level_score
}

pub fn minimal_score_of_level(level: u8) -> i64 {
    100_u64.pow(level as u32) as i64
}

pub struct Score {
    pub address: Address,
    pub field_address: Address,
    pub score: i64,
    pub upvote: u64,
    pub downvote: u64,
}
