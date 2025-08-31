pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index: usize = 0;

        for i in 0..nums.len() {
            let v = nums[i];
            if v != val {
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
    fn test_remove_element_basic() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(k, 2);
        let result: Vec<i32> = nums[..k as usize].to_vec();
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn test_remove_element_none() {
        let mut nums = vec![1, 2, 3, 4];
        let val = 5;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(k, 4);
        let result: Vec<i32> = nums[..k as usize].to_vec();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_element_all() {
        let mut nums = vec![1, 1, 1, 1];
        let val = 1;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(k, 0);
    }
}
