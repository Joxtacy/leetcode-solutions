pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        recursion(n)
    }
}

fn recursion(n: i32) -> bool {
    if n <= 0 {
        false
    } else if n == 1 {
        true
    } else if n % 3 != 0 {
        false
    } else {
        recursion(n / 3)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use super::recursion;

    #[test]
    fn test_power_of_three_true() {
        assert!(recursion(1));
        assert!(recursion(3));
        assert!(recursion(9));
        assert!(recursion(27));
        assert!(recursion(81));
    }

    #[test]
    fn test_power_of_three_false() {
        assert!(!recursion(0));
        assert!(!recursion(2));
        assert!(!recursion(10));
        assert!(!recursion(-3));
        assert!(!recursion(45));
    }

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
