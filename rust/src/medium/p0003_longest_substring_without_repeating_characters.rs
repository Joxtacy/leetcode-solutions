pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;

        let chars = s.chars().collect::<Vec<char>>();

        s.chars().enumerate().for_each(|(i, c)| {
            let mut res = vec![];
            res.push(c);

            for c in chars.iter().skip(i + 1) {
                if !res.contains(c) {
                    res.push(*c);
                } else {
                    break;
                }
            }
            let this_max = res.len();
            if this_max > max {
                max = this_max;
            }
        });
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        let s = String::from("abcabcbb");

        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn bbb() {
        let s = String::from("bbbb");

        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn wke() {
        let s = String::from("pwwkew");

        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
