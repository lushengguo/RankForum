use crate::textual_integer::TextualInteger;
use crate::Address;

pub fn calculate_vote_score(target_level: u8, voter_level: u8) -> TextualInteger {
    if voter_level > target_level {
        return minimal_score_of_level(target_level) * TextualInteger::new("10");
    }
    minimal_score_of_level(voter_level)
}

pub fn minimal_score_of_level(level: u8) -> TextualInteger {
    TextualInteger::new("100").pow(level.into())
}

pub fn level(score: &TextualInteger) -> u8 {
    if score.to_string().starts_with('-') {
        return 1;
    }
    if score.to_string() == "0" {
        return 0;
    }
    let mut current_score = score.to_string().clone();
    let mut level: u8 = 0;
    loop {
        if current_score == "0" {
            break;
        }
        if current_score.len() <= 2 {
            if current_score != "0" {
                current_score = "0".to_string();
                level += 1;
            } else {
                break;
            }
        } else {
            current_score = current_score[..current_score.len() - 2].to_string();
            level += 1;
        }
    }
    level - 1
}

pub struct Score {
    pub address: Address,
    pub field_address: Address,
    pub score: TextualInteger,
    pub upvote: u64,
    pub downvote: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_vote_score() {
        // respect from people who are at the same level as you
        assert_eq!(calculate_vote_score(0, 0), TextualInteger::new("1"));
        assert_eq!(calculate_vote_score(1, 1), TextualInteger::new("100"));
        assert_eq!(calculate_vote_score(2, 2), TextualInteger::new("10000"));

        // respect from people who are at a lower level than you
        assert_eq!(calculate_vote_score(1, 0), TextualInteger::new("1"));
        assert_eq!(calculate_vote_score(2, 0), TextualInteger::new("1"));
        assert_eq!(calculate_vote_score(2, 1), TextualInteger::new("100"));

        // respect from people who are at a higher level than you
        // no matter how high the voter level is, the score will not exceed 10 * target_minimal_score
        assert_eq!(calculate_vote_score(0, 1), TextualInteger::new("10"));
        assert_eq!(calculate_vote_score(0, 5), TextualInteger::new("10"));
        assert_eq!(calculate_vote_score(1, 5), TextualInteger::new("1000"));
    }

    #[test]
    fn test_level() {
        assert_eq!(level(&TextualInteger::new("0")), 0);
        assert_eq!(level(&TextualInteger::new("1")), 0);
        assert_eq!(level(&TextualInteger::new("99")), 0);
        assert_eq!(level(&TextualInteger::new("100")), 1);
        assert_eq!(level(&TextualInteger::new("101")), 1);
        assert_eq!(level(&TextualInteger::new("200")), 1);
        assert_eq!(level(&TextualInteger::new("9999")), 1);
        assert_eq!(level(&TextualInteger::new("10000")), 2);
        assert_eq!(level(&TextualInteger::new("10001")), 2);
        assert_eq!(level(&TextualInteger::new("12345")), 2);
        assert_eq!(level(&TextualInteger::new("999999")), 2);
        assert_eq!(level(&TextualInteger::new("1000000")), 3);
        assert_eq!(level(&TextualInteger::new("1000001")), 3);
        assert_eq!(level(&TextualInteger::new("1234567")), 3);
        assert_eq!(level(&TextualInteger::new("99999999")), 3);
        assert_eq!(level(&TextualInteger::new("100000000")), 4);
        assert_eq!(level(&TextualInteger::new("100000001")), 4);
        assert_eq!(level(&TextualInteger::new("123456789")), 4);
        assert_eq!(level(&TextualInteger::new("10000000000")), 5);
        assert_eq!(level(&TextualInteger::new("999999999999")), 5);
        assert_eq!(level(&TextualInteger::new("1000000000000")), 6);
        assert_eq!(level(&TextualInteger::new("123456789012345")), 7);

        assert_eq!(level(&TextualInteger::new("-1")), 1);
        assert_eq!(level(&TextualInteger::new("-99")), 1);
        assert_eq!(level(&TextualInteger::new("-100")), 1);
        assert_eq!(level(&TextualInteger::new("-1000000")), 1);
    }
}
