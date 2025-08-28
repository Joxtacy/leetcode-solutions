pub struct Solution;

enum Trampoline<T> {
    Return(T),
    Call(Box<dyn FnOnce() -> Trampoline<T>>),
}

fn run_trampoline<T>(mut tramp: Trampoline<T>) -> T {
    loop {
        match tramp {
            Trampoline::Return(result) => return result,
            Trampoline::Call(f) => tramp = f(),
        }
    }
}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let current = String::new();
        run_trampoline(generate_parenthesis_trampoline(current, 0, 0, n))
    }
}

fn generate_parenthesis_trampoline(
    current: String,
    open: i32,
    close: i32,
    max: i32,
) -> Trampoline<Vec<String>> {
    if current.len() as i32 == max * 2 {
        return Trampoline::Return(vec![current]);
    }

    let results = Vec::new();

    if open < max {
        let next_current = current.clone() + "(";
        return Trampoline::Call(Box::new(move || {
            let mut left_results = run_trampoline(generate_parenthesis_trampoline(
                next_current,
                open + 1,
                close,
                max,
            ));

            if close < open {
                let next_current = current + ")";
                let right_results = run_trampoline(generate_parenthesis_trampoline(
                    next_current,
                    open,
                    close + 1,
                    max,
                ));
                left_results.extend(right_results);
            }

            Trampoline::Return(left_results)
        }));
    }

    if close < open {
        let next_current = current + ")";
        return Trampoline::Call(Box::new(move || {
            generate_parenthesis_trampoline(next_current, open, close + 1, max)
        }));
    }

    Trampoline::Return(results)
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
