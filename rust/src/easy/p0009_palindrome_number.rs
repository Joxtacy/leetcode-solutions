pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        if x.len() == 1 {
            return true;
        }

        let mut start = 0;
        let mut end = x.len() - 1;

        let x = x.chars().collect::<Vec<char>>();
        while start < end {
            let a = x[start];
            let b = x[end];

            if a != b {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_positive_palindrome() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn test_negative_number() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!Solution::is_palindrome(123));
    }

    #[test]
    fn test_single_digit() {
        assert!(Solution::is_palindrome(7));
    }
}
