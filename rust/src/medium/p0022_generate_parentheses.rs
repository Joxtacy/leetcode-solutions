pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let current = String::new();
        backtrack(&mut result, current, 0, 0, n);
        result
    }
}

fn backtrack(result: &mut Vec<String>, current: String, open: i32, closed: i32, max: i32) {
    if current.len() as i32 == max * 2 {
        result.push(current);
        return;
    }

    if open < max {
        let mut new_current = current.clone();
        new_current.push('(');
        backtrack(result, new_current, open + 1, closed, max);
    }
    if closed < open {
        let mut new_current = current.clone();
        new_current.push(')');
        backtrack(result, new_current, open, closed + 1, max);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate_parenthesis_n1() {
        let mut result = Solution::generate_parenthesis(1);
        result.sort();
        let mut expected = vec!["()".to_string()];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_parenthesis_n2() {
        let mut result = Solution::generate_parenthesis(2);
        result.sort();
        let mut expected = vec!["(())".to_string(), "()()".to_string()];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_parenthesis_n3() {
        let mut result = Solution::generate_parenthesis(3);
        result.sort();
        let mut expected = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        expected.sort();
        assert_eq!(result, expected);
    }
}
