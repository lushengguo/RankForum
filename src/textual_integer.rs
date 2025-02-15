use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextualInteger {
    value: String,
}

impl TextualInteger {
    pub fn new(value: &str) -> Self {
        TextualInteger {
            value: value.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    pub fn is_positive(&self) -> bool {
        !self.value.starts_with('-')
    }

    pub fn pow(&self, exponent: u32) -> Self {
        if exponent == 0 {
            return TextualInteger::new("1");
        }
        if exponent == 1 {
            return self.clone();
        }

        let mut res = TextualInteger::new("1");
        let mut current_base = self.clone();
        let mut current_exponent = exponent;

        while current_exponent > 0 {
            if current_exponent % 2 == 1 {
                res = res.mul_positive(&current_base);
            }
            current_base = current_base.mul_positive(&current_base);
            current_exponent /= 2;
        }
        res
    }

    fn mul_positive(&self, other: &Self) -> Self {
        if self.value == "0" || other.value == "0" {
            return TextualInteger::new("0");
        }

        let chars1: Vec<char> = self.value.chars().rev().collect();
        let chars2: Vec<char> = other.value.chars().rev().collect();
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

        let mut result_str = String::new();
        let mut carry = 0;
        for digit_sum in result_digits.iter_mut() {
            let current_sum = *digit_sum + carry;
            carry = current_sum / 10;
            result_str.push(std::char::from_digit(current_sum % 10, 10).unwrap());
        }

        if carry > 0 {
            result_str.push(std::char::from_digit(carry, 10).unwrap());
        }

        let mut result: String = result_str.chars().rev().collect();
        while result.starts_with('0') && result.len() > 1 {
            result.remove(0);
        }
        TextualInteger::new(&result)
    }

    fn add_positive(&self, other: &Self) -> Self {
        let mut result_str = String::new();
        let mut carry = 0;

        let chars1: Vec<char> = self.value.chars().rev().collect();
        let chars2: Vec<char> = other.value.chars().rev().collect();
        let max_len = chars1.len().max(chars2.len());

        for i in 0..max_len {
            let digit1 = chars1.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);
            let digit2 = chars2.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);

            let sum = digit1 + digit2 + carry;
            carry = sum / 10;
            result_str.push(std::char::from_digit(sum % 10, 10).unwrap());
        }

        if carry > 0 {
            result_str.push(std::char::from_digit(carry, 10).unwrap());
        }

        TextualInteger::new(&result_str.chars().rev().collect::<String>())
    }

    fn sub_positive(&self, other: &Self) -> Self {
        let mut result_str = String::new();
        let mut borrow = 0;

        let chars1: Vec<char> = self.value.chars().rev().collect();
        let chars2: Vec<char> = other.value.chars().rev().collect();
        let max_len = chars1.len().max(chars2.len());

        for i in 0..max_len {
            let digit1 = chars1.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);
            let digit2 = chars2.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);

            let diff = digit1 as i32 - digit2 as i32 - borrow;
            if diff < 0 {
                borrow = 1;
                result_str.push(std::char::from_digit((diff + 10) as u32, 10).unwrap());
            } else {
                borrow = 0;
                result_str.push(std::char::from_digit(diff as u32, 10).unwrap());
            }
        }

        while result_str.ends_with('0') && result_str.len() > 1 {
            result_str.pop();
        }

        TextualInteger::new(&result_str.chars().rev().collect::<String>())
    }

    pub fn is_smaller(&self, other: &Self) -> bool {
        if self.value.len() < other.value.len() {
            return true;
        } else if self.value.len() > other.value.len() {
            return false;
        } else {
            return self.value < other.value;
        }
    }
}

impl Add for TextualInteger {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (negative1, value1) = if self.value.starts_with('-') {
            (true, &self.value[1..])
        } else {
            (false, &self.value[..])
        };

        let (negative2, value2) = if other.value.starts_with('-') {
            (true, &other.value[1..])
        } else {
            (false, &other.value[..])
        };

        if negative1 && negative2 {
            return TextualInteger::new(&format!(
                "-{}",
                TextualInteger::new(value1)
                    .add_positive(&TextualInteger::new(value2))
                    .value
            ));
        } else if negative1 {
            return TextualInteger::new(&TextualInteger::new(value2).sub(TextualInteger::new(value1)).value);
        } else if negative2 {
            return TextualInteger::new(&TextualInteger::new(value1).sub(TextualInteger::new(value2)).value);
        }

        TextualInteger::new(
            &TextualInteger::new(value1)
                .add_positive(&TextualInteger::new(value2))
                .value,
        )
    }
}

impl AddAssign for TextualInteger {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for TextualInteger {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (negative1, value1) = if self.value.starts_with('-') {
            (true, &self.value[1..])
        } else {
            (false, &self.value[..])
        };

        let (negative2, value2) = if other.value.starts_with('-') {
            (true, &other.value[1..])
        } else {
            (false, &other.value[..])
        };

        if negative1 && negative2 {
            return TextualInteger::new(&TextualInteger::new(value2).sub(TextualInteger::new(value1)).value);
        } else if negative1 {
            return TextualInteger::new(&format!(
                "-{}",
                TextualInteger::new(value1)
                    .add_positive(&TextualInteger::new(value2))
                    .value
            ));
        } else if negative2 {
            return TextualInteger::new(
                &TextualInteger::new(value1)
                    .add_positive(&TextualInteger::new(value2))
                    .value,
            );
        }

        if TextualInteger::new(value1).is_smaller(&TextualInteger::new(value2)) {
            return TextualInteger::new(&format!(
                "-{}",
                TextualInteger::new(value2)
                    .sub_positive(&TextualInteger::new(value1))
                    .value
            ));
        }

        TextualInteger::new(
            &TextualInteger::new(value1)
                .sub_positive(&TextualInteger::new(value2))
                .value,
        )
    }
}

impl SubAssign for TextualInteger {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl Mul for TextualInteger {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (negative1, value1) = if self.value.starts_with('-') {
            (true, &self.value[1..])
        } else {
            (false, &self.value[..])
        };

        let (negative2, value2) = if other.value.starts_with('-') {
            (true, &other.value[1..])
        } else {
            (false, &other.value[..])
        };

        let result = self.mul_positive(&other);
        if negative1 ^ negative2 {
            TextualInteger::new(&format!("-{}", result.value))
        } else {
            result
        }
    }
}

impl MulAssign for TextualInteger {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

pub type TextualIntegerType = TextualInteger;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textual_integer_add() {
        assert_eq!(
            TextualInteger::new("0") + TextualInteger::new("0"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("1") + TextualInteger::new("1"),
            TextualInteger::new("2")
        );
        assert_eq!(
            TextualInteger::new("9") + TextualInteger::new("1"),
            TextualInteger::new("10")
        );
        assert_eq!(
            TextualInteger::new("99") + TextualInteger::new("1"),
            TextualInteger::new("100")
        );
        assert_eq!(
            TextualInteger::new("999") + TextualInteger::new("1"),
            TextualInteger::new("1000")
        );
        assert_eq!(
            TextualInteger::new("999") + TextualInteger::new("2"),
            TextualInteger::new("1001")
        );
        assert_eq!(
            TextualInteger::new("999") + TextualInteger::new("99"),
            TextualInteger::new("1098")
        );
        assert_eq!(
            TextualInteger::new("999") + TextualInteger::new("999"),
            TextualInteger::new("1998")
        );
        assert_eq!(
            TextualInteger::new("999") + TextualInteger::new("9999"),
            TextualInteger::new("10998")
        );
        assert_eq!(
            TextualInteger::new("1167167131617671654171616571") + TextualInteger::new("5716516716714657161671671641"),
            TextualInteger::new("6883683848332328815843288212")
        );
        assert_eq!(
            TextualInteger::new("-1") + TextualInteger::new("1"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("-1") + TextualInteger::new("-1"),
            TextualInteger::new("-2")
        );
    }

    #[test]
    fn test_textual_integer_sub() {
        assert_eq!(
            TextualInteger::new("0") - TextualInteger::new("0"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("1") - TextualInteger::new("1"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("10") - TextualInteger::new("1"),
            TextualInteger::new("9")
        );
        assert_eq!(
            TextualInteger::new("100") - TextualInteger::new("1"),
            TextualInteger::new("99")
        );
        assert_eq!(
            TextualInteger::new("1000") - TextualInteger::new("1"),
            TextualInteger::new("999")
        );
        assert_eq!(
            TextualInteger::new("1001") - TextualInteger::new("2"),
            TextualInteger::new("999")
        );
        assert_eq!(
            TextualInteger::new("1098") - TextualInteger::new("99"),
            TextualInteger::new("999")
        );
        assert_eq!(
            TextualInteger::new("1998") - TextualInteger::new("999"),
            TextualInteger::new("999")
        );
        assert_eq!(
            TextualInteger::new("10998") - TextualInteger::new("9999"),
            TextualInteger::new("999")
        );
        assert_eq!(
            TextualInteger::new("6883683848332328815843288212") - TextualInteger::new("5716516716714657161671671641"),
            TextualInteger::new("1167167131617671654171616571")
        );

        assert_eq!(
            TextualInteger::new("0") - TextualInteger::new("1"),
            TextualInteger::new("-1")
        );
        assert_eq!(
            TextualInteger::new("-100") - TextualInteger::new("-100"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("100") - TextualInteger::new("-100"),
            TextualInteger::new("200")
        );
        assert_eq!(
            TextualInteger::new("-100") - TextualInteger::new("100"),
            TextualInteger::new("-200")
        );
    }

    #[test]
    fn test_textual_integer_pow() {
        assert_eq!(TextualInteger::new("2").pow(0), TextualInteger::new("1"));
        assert_eq!(TextualInteger::new("2").pow(1), TextualInteger::new("2"));
        assert_eq!(TextualInteger::new("2").pow(2), TextualInteger::new("4"));
        assert_eq!(TextualInteger::new("2").pow(3), TextualInteger::new("8"));
        assert_eq!(TextualInteger::new("2").pow(10), TextualInteger::new("1024"));
        assert_eq!(TextualInteger::new("10").pow(0), TextualInteger::new("1"));
        assert_eq!(TextualInteger::new("10").pow(1), TextualInteger::new("10"));
        assert_eq!(TextualInteger::new("10").pow(2), TextualInteger::new("100"));
        assert_eq!(TextualInteger::new("10").pow(3), TextualInteger::new("1000"));
        assert_eq!(TextualInteger::new("123").pow(2), TextualInteger::new("15129"));
        assert_eq!(TextualInteger::new("0").pow(3), TextualInteger::new("0"));
        assert_eq!(TextualInteger::new("1").pow(5), TextualInteger::new("1"));
        assert_eq!(TextualInteger::new("99").pow(2), TextualInteger::new("9801"));
    }

    #[test]
    fn test_textual_integer_mul() {
        assert_eq!(
            TextualInteger::new("0") * TextualInteger::new("0"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("1") * TextualInteger::new("0"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("0") * TextualInteger::new("1"),
            TextualInteger::new("0")
        );
        assert_eq!(
            TextualInteger::new("1") * TextualInteger::new("1"),
            TextualInteger::new("1")
        );
        assert_eq!(
            TextualInteger::new("2") * TextualInteger::new("3"),
            TextualInteger::new("6")
        );
        assert_eq!(
            TextualInteger::new("10") * TextualInteger::new("10"),
            TextualInteger::new("100")
        );
        assert_eq!(
            TextualInteger::new("12") * TextualInteger::new("10"),
            TextualInteger::new("120")
        );
        assert_eq!(
            TextualInteger::new("123") * TextualInteger::new("456"),
            TextualInteger::new("56088")
        );
        assert_eq!(
            TextualInteger::new("999") * TextualInteger::new("999"),
            TextualInteger::new("998001")
        );
        assert_eq!(
            TextualInteger::new("123456789") * TextualInteger::new("987654321"),
            TextualInteger::new("121932631112635269")
        );
        assert_eq!(
            TextualInteger::new("-2") * TextualInteger::new("3"),
            TextualInteger::new("-6")
        );
        assert_eq!(
            TextualInteger::new("2") * TextualInteger::new("-3"),
            TextualInteger::new("-6")
        );
        assert_eq!(
            TextualInteger::new("-2") * TextualInteger::new("-3"),
            TextualInteger::new("6")
        );
        assert_eq!(
            TextualInteger::new("-100") * TextualInteger::new("10"),
            TextualInteger::new("-1000")
        );
        assert_eq!(
            TextualInteger::new("100") * TextualInteger::new("-10"),
            TextualInteger::new("-1000")
        );
        assert_eq!(
            TextualInteger::new("-100") * TextualInteger::new("-10"),
            TextualInteger::new("1000")
        );
        assert_eq!(
            TextualInteger::new("123") * TextualInteger::new("-456"),
            TextualInteger::new("-56088")
        );
        assert_eq!(
            TextualInteger::new("-123") * TextualInteger::new("456"),
            TextualInteger::new("-56088")
        );
        assert_eq!(
            TextualInteger::new("-123") * TextualInteger::new("-456"),
            TextualInteger::new("56088")
        );
    }
}
