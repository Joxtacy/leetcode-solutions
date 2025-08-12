use std::ops::{Add, Sub};

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut curr_roman = Roman::I;
        s.chars().rev().fold(0, |acc, c| {
            let r: Roman = c.try_into().unwrap();
            if r < curr_roman {
                acc - r
            } else if r > curr_roman {
                curr_roman = r;
                acc + r
            } else {
                acc + r
            }
        })
    }
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
#[repr(u16)]
enum Roman {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl TryFrom<char> for Roman {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'I' | 'i' => Ok(Roman::I),
            'V' | 'v' => Ok(Roman::V),
            'X' | 'x' => Ok(Roman::X),
            'L' | 'l' => Ok(Roman::L),
            'C' | 'c' => Ok(Roman::C),
            'D' | 'd' => Ok(Roman::D),
            'M' | 'm' => Ok(Roman::M),
            _ => Err(format!("'{c}' is not a valid roman numeral")),
        }
    }
}

// Roman + i32
impl Add<i32> for Roman {
    type Output = i32;

    fn add(self, other: i32) -> i32 {
        self as i32 + other
    }
}

// i32 + Roman
impl Add<Roman> for i32 {
    type Output = i32;

    fn add(self, other: Roman) -> i32 {
        self + other as i32
    }
}

// Roman - i32
impl Sub<i32> for Roman {
    type Output = i32;

    fn sub(self, other: i32) -> i32 {
        self as i32 - other
    }
}

// i32 - Roman
impl Sub<Roman> for i32 {
    type Output = i32;

    fn sub(self, other: Roman) -> i32 {
        self - other as i32
    }
}

impl From<Roman> for i32 {
    fn from(r: Roman) -> i32 {
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_roman_to_int_iii() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_roman_to_int_iv() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn test_roman_to_int_ix() {
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn test_roman_to_int_lviii() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_roman_to_int_mcmxciv() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn test_roman_to_int_xl() {
        assert_eq!(Solution::roman_to_int("XL".to_string()), 40);
    }

    #[test]
    fn test_roman_to_int_xc() {
        assert_eq!(Solution::roman_to_int("XC".to_string()), 90);
    }

    #[test]
    fn test_roman_to_int_cd() {
        assert_eq!(Solution::roman_to_int("CD".to_string()), 400);
    }

    #[test]
    fn test_roman_to_int_cm() {
        assert_eq!(Solution::roman_to_int("CM".to_string()), 900);
    }

    #[test]
    fn test_roman_to_int_mmxxiii() {
        assert_eq!(Solution::roman_to_int("MMXXIII".to_string()), 2023);
    }
}
