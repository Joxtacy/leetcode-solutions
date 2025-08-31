pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let window_size = needle.len();
        if window_size > haystack.len() {
            return -1;
        }

        if window_size == 0 {
            return 0;
        }

        let needle = needle.as_bytes();
        let windows = haystack
            .as_bytes()
            .windows(window_size)
            .collect::<Vec<&[u8]>>();

        for i in 0..windows.len() {
            if windows[i] == needle {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(
            Solution::str_str("aaaaa".to_string(), "bba".to_string()),
            -1
        );
    }

    #[test]
    fn test_empty_needle() {
        assert_eq!(Solution::str_str("abc".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_empty_haystack() {
        assert_eq!(Solution::str_str("".to_string(), "a".to_string()), -1);
    }
}
