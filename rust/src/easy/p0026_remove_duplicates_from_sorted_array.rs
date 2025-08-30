pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut curr_num = i32::MIN;
        let mut index: usize = 0;

        for i in 0..nums.len() {
            let v = nums[i];
            if v > curr_num {
                curr_num = v;
                nums[index] = v;
                index += 1;
            }
        }
        index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_empty() {
        let mut nums = vec![];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 0);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_remove_duplicates_single() {
        let mut nums = vec![1];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(nums[..k as usize], [1]);
    }

    #[test]
    fn test_remove_duplicates_multiple() {
        let mut nums = vec![1, 1, 2, 2, 3];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 3);
        assert_eq!(nums[..k as usize], [1, 2, 3]);
    }

    #[test]
    fn test_remove_duplicates_no_duplicates() {
        let mut nums = vec![1, 2, 3, 4];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 4);
        assert_eq!(nums[..k as usize], [1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_duplicates_all_duplicates() {
        let mut nums = vec![2, 2, 2, 2];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(nums[..k as usize], [2]);
    }
}
