use crate::Address;
pub type TextxualInteger = String;

pub fn calculate_vote_score(target_level: u8, voter_level: u8) -> TextxualInteger {
    let target_minimal_score = minimal_score_of_level(target_level);
    let voter_minimal_score = minimal_score_of_level(voter_level);
    if voter_minimal_score > textual_integer_mul(&target_minimal_score, &10.to_string()) {
        return textual_integer_mul(&target_minimal_score, &"10".to_string());
    }
    voter_minimal_score
}

pub fn minimal_score_of_level(level: u8) -> TextxualInteger {
    textual_integer_pow("100", level.into())
}

pub fn level(score: &str) -> u8 {
    if score.starts_with('-') {
        return 1;
    }
    if score == "0" {
        return 0;
    }
    let mut current_score = score.to_string();
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
    pub score: TextxualInteger,
    pub upvote: u64,
    pub downvote: u64,
}

pub fn textual_integer_is_positive(value: &str) -> bool {
    !value.starts_with('-')
}

pub fn textual_integer_mul(value1: &str, value2: &str) -> TextxualInteger {
    let (negative1, value1) = if value1.starts_with('-') {
        (true, &value1[1..])
    } else {
        (false, value1)
    };

    let (negative2, value2) = if value2.starts_with('-') {
        (true, &value2[1..])
    } else {
        (false, value2)
    };

    let result = textual_integer_mul_positive(value1, value2);
    if negative1 ^ negative2 {
        format!("-{}", result)
    } else {
        result
    }
}

pub fn textual_integer_add(value1: &str, value2: &str) -> TextxualInteger {
    let (negative1, value1) = if value1.starts_with('-') {
        (true, &value1[1..])
    } else {
        (false, value1)
    };

    let (negative2, value2) = if value2.starts_with('-') {
        (true, &value2[1..])
    } else {
        (false, value2)
    };

    if negative1 && negative2 {
        return format!("-{}", textual_integer_add_positive(value1, value2));
    } else if negative1 {
        return textual_integer_sub(value2, value1);
    } else if negative2 {
        return textual_integer_sub(value1, value2);
    }

    textual_integer_add_positive(value1, value2)
}

pub fn textual_integer_add_positive(value1: &str, value2: &str) -> TextxualInteger {
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

pub fn textual_integer_is_smaller(value1: &str, value2: &str) -> bool {
    if value1.len() < value2.len() {
        return true;
    } else if value1.len() > value2.len() {
        return false;
    } else {
        return value1 < value2; // Lexicographical comparison when lengths are equal
    }
}

pub fn textual_integer_sub(value1: &str, value2: &str) -> TextxualInteger {
    let (negative1, value1) = if value1.starts_with('-') {
        (true, &value1[1..])
    } else {
        (false, value1)
    };

    let (negative2, value2) = if value2.starts_with('-') {
        (true, &value2[1..])
    } else {
        (false, value2)
    };

    if negative1 && negative2 {
        return textual_integer_sub(value2, value1);
    } else if negative1 {
        return format!("-{}", textual_integer_add_positive(value1, value2));
    } else if negative2 {
        return textual_integer_add_positive(value1, value2);
    }

    if textual_integer_is_smaller(value1, value2) {
        return format!("-{}", textual_integer_sub_positive(value2, value1));
    }

    textual_integer_sub_positive(value1, value2)
}

pub fn textual_integer_sub_positive(value1: &str, value2: &str) -> TextxualInteger {
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

    while result.ends_with('0') && result.len() > 1 {
        result.pop();
    }

    result.chars().rev().collect()
}

pub fn textual_integer_mul_positive(value1: &str, value2: &str) -> TextxualInteger {
    if value1 == "0" || value2 == "0" {
        return "0".to_string();
    }

    let chars1: Vec<char> = value1.chars().rev().collect();
    let chars2: Vec<char> = value2.chars().rev().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();
    let mut result_digits: Vec<u32> = vec![0; len1 + len2];

    for i in 0..len1 {
        for j in 0..len2 {
            let digit1 = chars1[i].to_digit(10).unwrap_or(0);
            let digit2 = chars2[j].to_digit(10).unwrap_or(0);
            let product = digit1 * digit2;
            result_digits[i + j] += product;
        }
    }

    let mut result = String::new();
    let mut carry = 0;
    for digit_sum in result_digits.iter_mut() {
        let current_sum = *digit_sum + carry;
        carry = current_sum / 10;
        result.push(std::char::from_digit(current_sum % 10, 10).unwrap());
    }

    if carry > 0 {
        result.push(std::char::from_digit(carry, 10).unwrap());
    }

    let mut result: String = result.chars().rev().collect();
    while result.starts_with('0') && result.len() > 1 {
        result.remove(0);
    }
    result
}

pub fn textual_integer_pow(base: &str, exponent: u32) -> TextxualInteger {
    if exponent == 0 {
        return "1".to_string();
    }
    if exponent == 1 {
        return base.to_string();
    }

    let mut res = "1".to_string();
    let mut current_base = base.to_string();
    let mut current_exponent = exponent;

    while current_exponent > 0 {
        if current_exponent % 2 == 1 {
            res = textual_integer_mul_positive(&res, &current_base);
        }
        current_base = textual_integer_mul_positive(&current_base, &current_base);
        current_exponent /= 2;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_vote_score() {
        // respect from people who are at the same level as you
        assert_eq!(super::calculate_vote_score(0, 0), "1");
        assert_eq!(super::calculate_vote_score(1, 1), "100");
        assert_eq!(super::calculate_vote_score(2, 2), "10000");

        // respect from people who are at a lower level than you
        assert_eq!(super::calculate_vote_score(1, 0), "1");
        assert_eq!(super::calculate_vote_score(2, 0), "1");
        assert_eq!(super::calculate_vote_score(2, 1), "100");

        // respect from people who are at a higher level than you
        // no matter how high the voter level is, the score will not exceed 10 * target_minimal_score
        assert_eq!(super::calculate_vote_score(0, 1), "10");
        assert_eq!(super::calculate_vote_score(0, 5), "10");
        assert_eq!(super::calculate_vote_score(1, 5), "1000");
    }

    #[test]
    fn test_textual_integer_add() {
        assert_eq!(super::textual_integer_add("0", "0"), "0");
        assert_eq!(super::textual_integer_add("1", "1"), "2");
        assert_eq!(super::textual_integer_add("9", "1"), "10");
        assert_eq!(super::textual_integer_add("99", "1"), "100");
        assert_eq!(super::textual_integer_add("999", "1"), "1000");
        assert_eq!(super::textual_integer_add("999", "2"), "1001");
        assert_eq!(super::textual_integer_add("999", "99"), "1098");
        assert_eq!(super::textual_integer_add("999", "999"), "1998");
        assert_eq!(super::textual_integer_add("999", "9999"), "10998");
        assert_eq!(
            super::textual_integer_add("1167167131617671654171616571", "5716516716714657161671671641"),
            "6883683848332328815843288212"
        );
        assert_eq!(super::textual_integer_add("-1", "1"), "0");
        assert_eq!(super::textual_integer_add("-1", "-1"), "-2");
    }

    #[test]
    fn test_textual_integer_sub() {
        assert_eq!(super::textual_integer_sub("0", "0"), "0");
        assert_eq!(super::textual_integer_sub("1", "1"), "0");
        assert_eq!(super::textual_integer_sub("10", "1"), "9");
        assert_eq!(super::textual_integer_sub("100", "1"), "99");
        assert_eq!(super::textual_integer_sub("1000", "1"), "999");
        assert_eq!(super::textual_integer_sub("1001", "2"), "999");
        assert_eq!(super::textual_integer_sub("1098", "99"), "999");
        assert_eq!(super::textual_integer_sub("1998", "999"), "999");
        assert_eq!(super::textual_integer_sub("10998", "9999"), "999");
        assert_eq!(
            super::textual_integer_sub("6883683848332328815843288212", "5716516716714657161671671641"),
            "1167167131617671654171616571"
        );

        assert_eq!(super::textual_integer_sub("0", "1"), "-1");
        assert_eq!(super::textual_integer_sub("-100", "-100"), "0");
        assert_eq!(super::textual_integer_sub("100", "-100"), "200");
        assert_eq!(super::textual_integer_sub("-100", "100"), "-200");
    }

    #[test]
    fn test_textual_integer_pow() {
        assert_eq!(super::textual_integer_pow("2", 0), "1");
        assert_eq!(super::textual_integer_pow("2", 1), "2");
        assert_eq!(super::textual_integer_pow("2", 2), "4");
        assert_eq!(super::textual_integer_pow("2", 3), "8");
        assert_eq!(super::textual_integer_pow("2", 10), "1024");
        assert_eq!(super::textual_integer_pow("10", 0), "1");
        assert_eq!(super::textual_integer_pow("10", 1), "10");
        assert_eq!(super::textual_integer_pow("10", 2), "100");
        assert_eq!(super::textual_integer_pow("10", 3), "1000");
        assert_eq!(super::textual_integer_pow("123", 2), "15129");
        assert_eq!(super::textual_integer_pow("0", 3), "0");
        assert_eq!(super::textual_integer_pow("1", 5), "1");
        assert_eq!(super::textual_integer_pow("99", 2), "9801");
    }

    #[test]
    fn test_textual_integer_mul() {
        assert_eq!(super::textual_integer_mul("0", "0"), "0");
        assert_eq!(super::textual_integer_mul("1", "0"), "0");
        assert_eq!(super::textual_integer_mul("0", "1"), "0");
        assert_eq!(super::textual_integer_mul("1", "1"), "1");
        assert_eq!(super::textual_integer_mul("2", "3"), "6");
        assert_eq!(super::textual_integer_mul("10", "10"), "100");
        assert_eq!(super::textual_integer_mul("12", "10"), "120");
        assert_eq!(super::textual_integer_mul("123", "456"), "56088");
        assert_eq!(super::textual_integer_mul("999", "999"), "998001");
        assert_eq!(
            super::textual_integer_mul("123456789", "987654321"),
            "121932631112635269"
        );
        assert_eq!(super::textual_integer_mul("-2", "3"), "-6");
        assert_eq!(super::textual_integer_mul("2", "-3"), "-6");
        assert_eq!(super::textual_integer_mul("-2", "-3"), "6");
        assert_eq!(super::textual_integer_mul("-100", "10"), "-1000");
        assert_eq!(super::textual_integer_mul("100", "-10"), "-1000");
        assert_eq!(super::textual_integer_mul("-100", "-10"), "1000");
        assert_eq!(super::textual_integer_mul("123", "-456"), "-56088");
        assert_eq!(super::textual_integer_mul("-123", "456"), "-56088");
        assert_eq!(super::textual_integer_mul("-123", "-456"), "56088");
    }

    #[test]
    fn test_level() {
        assert_eq!(super::level("0"), 0);
        assert_eq!(super::level("1"), 0);
        assert_eq!(super::level("99"), 0);
        assert_eq!(super::level("100"), 1);
        assert_eq!(super::level("101"), 1);
        assert_eq!(super::level("200"), 1);
        assert_eq!(super::level("9999"), 1);
        assert_eq!(super::level("10000"), 2);
        assert_eq!(super::level("10001"), 2);
        assert_eq!(super::level("12345"), 2);
        assert_eq!(super::level("999999"), 2);
        assert_eq!(super::level("1000000"), 3);
        assert_eq!(super::level("1000001"), 3);
        assert_eq!(super::level("1234567"), 3);
        assert_eq!(super::level("99999999"), 3);
        assert_eq!(super::level("100000000"), 4);
        assert_eq!(super::level("100000001"), 4);
        assert_eq!(super::level("123456789"), 4);
        assert_eq!(super::level("10000000000"), 5);
        assert_eq!(super::level("999999999999"), 5);
        assert_eq!(super::level("1000000000000"), 6);
        assert_eq!(super::level("123456789012345"), 7);

        assert_eq!(super::level("-1"), 1);
        assert_eq!(super::level("-99"), 1);
        assert_eq!(super::level("-100"), 1);
        assert_eq!(super::level("-1000000"), 1);
    }
}
