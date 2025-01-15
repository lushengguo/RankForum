pub fn calculate_vote_impact(poster_level: u8, voter_level: u8) -> i64 {
    let poster_level_score = (100 as u64).pow(poster_level as u32) as i64;
    let voter_level_score = (100 as u64).pow(voter_level as u32) as i64;
    if voter_level_score > poster_level_score * 10 {
        return poster_level_score * 10;
    }
    return voter_level_score;
}
