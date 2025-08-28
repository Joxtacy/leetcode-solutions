pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => queue.push(c),
                ')' => {
                    if let Some(popped) = queue.pop() {
                        if popped != '(' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if let Some(popped) = queue.pop() {
                        if popped != '[' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if let Some(popped) = queue.pop() {
                        if popped != '{' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_valid_parentheses_simple() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test_valid_parentheses_mixed() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_valid_parentheses_nested() {
        assert!(Solution::is_valid("{[]}".to_string()));
    }

    #[test]
    fn test_valid_parentheses_empty() {
        assert!(Solution::is_valid("".to_string()));
    }

    #[test]
    fn test_invalid_parentheses_wrong_order() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn test_invalid_parentheses_crossed() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }

    #[test]
    fn test_invalid_parentheses_unclosed() {
        assert!(!Solution::is_valid("(".to_string()));
    }

    #[test]
    fn test_invalid_parentheses_unopened() {
        assert!(!Solution::is_valid("]".to_string()));
    }
}
