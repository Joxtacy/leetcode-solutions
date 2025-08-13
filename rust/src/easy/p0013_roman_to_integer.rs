use crate::utils::roman::Roman;

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
