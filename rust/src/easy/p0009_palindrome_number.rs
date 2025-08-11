pub struct Solution;

impl Solution {
    /// Checks if the given integer is a palindrome.
    ///
    /// A palindrome is a number that reads the same backward as forward.
    /// Negative numbers are not considered palindromes.
    /// Numbers ending with 0 (except 0 itself) are not palindromes.
    ///
    /// # Arguments
    ///
    /// * `x` - The integer to check.
    ///
    /// # Returns
    ///
    /// * `true` if `x` is a palindrome, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Solution::is_palindrome(121), true);
    /// assert_eq!(Solution::is_palindrome(-121), false);
    /// ```
    pub fn is_palindrome(x: i32) -> bool {
        // Negative numbers can't be palindromes because of the sign switching place to the back.
        if x < 0 {
            return false;
        }

        // Single digit numbers are always palindromes.
        if x / 10 == 0 {
            return true;
        }

        // Any number ending with `0` can't be a palindrome.
        if x % 10 == 0 {
            return false;
        }

        let mut rev = 0;
        let mut num = x;

        // Reverse the number.
        while num != 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }

        x == rev
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_palindrome_positive() {
        assert!(Solution::is_palindrome(1221));
        assert!(Solution::is_palindrome(1));
        assert!(Solution::is_palindrome(0));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!Solution::is_palindrome(123));
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn test_negative() {
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(-1));
    }
}
