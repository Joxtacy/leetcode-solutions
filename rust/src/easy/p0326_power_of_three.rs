pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            false
        } else {
            let mut x = n;
            while x % 3 == 0 {
                x /= 3;
            }
            x == 1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_power_of_three() {
        assert!(Solution::is_power_of_three(1));
        assert!(Solution::is_power_of_three(3));
        assert!(Solution::is_power_of_three(9));
        assert!(Solution::is_power_of_three(27));
        assert!(Solution::is_power_of_three(81));
        assert!(Solution::is_power_of_three(243));
    }

    #[test]
    fn test_not_power_of_three() {
        assert!(!Solution::is_power_of_three(0));
        assert!(!Solution::is_power_of_three(2));
        assert!(!Solution::is_power_of_three(4));
        assert!(!Solution::is_power_of_three(10));
        assert!(!Solution::is_power_of_three(-3));
        assert!(!Solution::is_power_of_three(45));
    }
}
