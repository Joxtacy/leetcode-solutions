pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut ptr_a: usize = 0;
        let mut ptr_b: usize = 0;
        let mut res: Vec<i32> = vec![];

        while res.len() < nums1.len() + nums2.len() {
            if ptr_a >= nums1.len() {
                res.push(nums2[ptr_b]);
                ptr_b += 1;
            } else if ptr_b >= nums2.len() {
                res.push(nums1[ptr_a]);
                ptr_a += 1;
            } else {
                let a = nums1[ptr_a];
                let b = nums2[ptr_b];
                if a <= b {
                    res.push(a);
                    ptr_a += 1;
                } else {
                    res.push(b);
                    ptr_b += 1;
                }
            }
        }

        if res.is_empty() {
            return 0.0;
        }
        if res.len() % 2 == 1 {
            res[res.len() / 2] as f64
        } else {
            ({ res[res.len() / 2] + res[res.len() / 2 - 1] }) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_even_total_length() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2, 4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test_odd_total_length() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test_one_empty() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn test_both_empty() {
        let nums1: Vec<i32> = vec![];
        let nums2: Vec<i32> = vec![];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 0.0);
    }
}
