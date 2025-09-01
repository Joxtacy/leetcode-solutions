pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        bin_search(0, nums.len() - 1, nums, target)
    }
}

fn bin_search(start: usize, end: usize, nums: Vec<i32>, target: i32) -> i32 {
    let mid = { start + end } / 2;

    if start == end {
        let x = nums[mid];
        if target < x {
            return 0.max(mid as i32 - 1);
        } else if target > x {
            return { nums.len() as i32 }.min(mid as i32 + 1);
        }
        return start as i32;
    }

    let x = nums[mid];
    if target < x {
        bin_search(start, mid - 1, nums, target)
    } else if target > x {
        bin_search(mid + 1, end, nums, target)
    } else {
        mid as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_insert_found() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test_search_insert_insert_middle() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_search_insert_insert_end() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_search_insert_insert_start() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
