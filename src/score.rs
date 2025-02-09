use crate::Address;

pub fn calculate_vote_score(target_level: u8, voter_level: u8) -> i64 {
    let target_minimal_score = minimal_score_of_level(target_level);
    let voter_minimal_score = minimal_score_of_level(voter_level);
    if voter_minimal_score > target_minimal_score * 10 {
        return target_minimal_score * 10;
    }
    voter_minimal_score
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_vote_score() {
        // respect from people who are at the same level as you
        assert_eq!(super::calculate_vote_score(0, 0), 1);
        assert_eq!(super::calculate_vote_score(1, 1), 100);
        assert_eq!(super::calculate_vote_score(2, 2), 10000);

        // respect from people who are at a lower level than you
        assert_eq!(super::calculate_vote_score(1, 0), 1);
        assert_eq!(super::calculate_vote_score(2, 0), 1);
        assert_eq!(super::calculate_vote_score(2, 1), 100);

        // respect from people who are at a higher level than you
        // no matter how high the voter level is, the score will not exceed 10 * target_minimal_score
        assert_eq!(super::calculate_vote_score(0, 1), 10);
        assert_eq!(super::calculate_vote_score(0, 5), 1 * 10);
        assert_eq!(super::calculate_vote_score(1, 5), 100 * 10);
    }
}
