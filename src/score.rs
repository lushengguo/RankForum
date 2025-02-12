use serde_json::value;

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

type ScoreValue = String;
pub fn score_add(value1: &str, value2: &str) -> ScoreValue {
    let mut result = String::new();
    let mut carry = 0;

    let chars1: Vec<char> = value1.chars().rev().collect();
    let chars2: Vec<char> = value2.chars().rev().collect();
    let max_len = chars1.len().max(chars2.len());

    for i in 0..max_len {
        let digit1 = chars1.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);
        let digit2 = chars2.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);

        let sum = digit1 + digit2 + carry;
        carry = sum / 10;
        result.push(std::char::from_digit(sum % 10, 10).unwrap());
    }

    if carry > 0 {
        result.push(std::char::from_digit(carry, 10).unwrap());
    }

    result.chars().rev().collect()
}

pub fn socre_sub(value1: &str, value2: &str) -> ScoreValue {
    let mut result = String::new();
    let mut borrow = 0;

    let chars1: Vec<char> = value1.chars().rev().collect();
    let chars2: Vec<char> = value2.chars().rev().collect();
    let max_len = chars1.len().max(chars2.len());

    for i in 0..max_len {
        let digit1 = chars1.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);
        let digit2 = chars2.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);

        let diff = digit1 as i32 - digit2 as i32 - borrow;
        if diff < 0 {
            borrow = 1;
            result.push(std::char::from_digit((diff + 10) as u32, 10).unwrap());
        } else {
            borrow = 0;
            result.push(std::char::from_digit(diff as u32, 10).unwrap());
        }
    }

    result.chars().rev().collect()
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

    #[test]
    fn test_score_add() {
        assert_eq!(super::score_add("0", "0"), "0");
        assert_eq!(super::score_add("1", "1"), "2");
        assert_eq!(super::score_add("9", "1"), "10");
        assert_eq!(super::score_add("99", "1"), "100");
        assert_eq!(super::score_add("999", "1"), "1000");
        assert_eq!(super::score_add("999", "2"), "1001");
        assert_eq!(super::score_add("999", "99"), "1098");
        assert_eq!(super::score_add("999", "999"), "1998");
        assert_eq!(super::score_add("999", "9999"), "10998");
        assert_eq!(
            super::score_add("1167167131617671654171616571", "5716516716714657161671671641"),
            "6883683848332328815843288212"
        );
    }

    #[test]
    fn test_score_sub() {
        assert_eq!(super::socre_sub("0", "0"), "0");
        assert_eq!(super::socre_sub("1", "1"), "0");
        assert_eq!(super::socre_sub("10", "1"), "9");
        assert_eq!(super::socre_sub("100", "1"), "99");
        assert_eq!(super::socre_sub("1000", "1"), "999");
        assert_eq!(super::socre_sub("1001", "2"), "999");
        assert_eq!(super::socre_sub("1098", "99"), "999");
        assert_eq!(super::socre_sub("1998", "999"), "999");
        assert_eq!(super::socre_sub("10998", "9999"), "9999");
        assert_eq!(
            super::socre_sub("6883683848332328815843288212", "5716516716714657161671671641"),
            "1167167131617671654171616571"
        );
    }
}
