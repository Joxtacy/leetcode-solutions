pub struct Solution;

impl Solution {
    /// Returns the longest common prefix string amongst an array of strings.
    ///
    /// # Arguments
    ///
    /// * `strs` - A vector of strings to find the common prefix from.
    ///
    /// # Returns
    ///
    /// A `String` representing the longest common prefix. If there is no common prefix, returns an empty string.
    ///
    /// # Examples
    ///
    /// ```
    /// let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    /// assert_eq!(Solution::longest_common_prefix(strs), "fl");
    /// ```
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let first = strs
            .first()
            .expect("Should have at least one entry since it's not empty")
            .as_bytes();
        let mut prefix_len = first.len();

        for s in &strs[1..] {
            let bytes = s.as_bytes();
            let min_len = prefix_len.min(bytes.len());
            let mut i = 0;
            while i < min_len && first[i] == bytes[i] {
                i += 1;
            }
            prefix_len = i;
            if prefix_len == 0 {
                return String::new();
            }
        }
        String::from_utf8_lossy(&first[..prefix_len]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let strs = vec!["flower", "flow", "flight"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "fl");
    }

    #[test]
    fn test_example_2() {
        let strs = vec!["dog", "racecar", "car"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "");
    }

    #[test]
    fn test_empty() {
        let strs: Vec<String> = vec![];
        assert_eq!(Solution::longest_common_prefix(strs), "");
    }

    #[test]
    fn test_single() {
        let strs = vec!["alone"].into_iter().map(String::from).collect();
        assert_eq!(Solution::longest_common_prefix(strs), "alone");
    }
}
