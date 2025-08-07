use crate::utils::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        let mut carry = 0;

        let mut l1 = l1;
        let mut l2 = l2;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = match l1 {
                Some(ref node) => node.val,
                None => 0,
            } + match l2 {
                Some(ref node) => node.val,
                None => 0,
            } + carry;

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        // Test case: [2,4,3] + [5,6,4] = [7,0,8]
        // Represents: 342 + 465 = 807
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn test_single_digits() {
        // Test case: [0] + [0] = [0]
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0]);
    }

    #[test]
    fn test_multiple_carries() {
        // Test case: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
        // Represents: 9999999 + 9999 = 10009998
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn test_different_lengths() {
        // Test case: [1,8] + [0] = [1,8]
        // Represents: 81 + 0 = 81
        let l1 = ListNode::from_vec(vec![1, 8]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![1, 8]);
    }

    #[test]
    fn test_carry_at_end() {
        // Test case: [5] + [5] = [0,1]
        // Represents: 5 + 5 = 10
        let l1 = ListNode::from_vec(vec![5]);
        let l2 = ListNode::from_vec(vec![5]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0, 1]);
    }

    #[test]
    fn test_longer_first_list() {
        // Test case: [1,2,3,4] + [5,6] = [6,8,3,4]
        // Represents: 4321 + 65 = 4386
        let l1 = ListNode::from_vec(vec![1, 2, 3, 4]);
        let l2 = ListNode::from_vec(vec![5, 6]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![6, 8, 3, 4]);
    }

    #[test]
    fn test_longer_second_list() {
        // Test case: [1,2] + [3,4,5,6] = [4,6,5,6]
        // Represents: 21 + 6543 = 6564
        let l1 = ListNode::from_vec(vec![1, 2]);
        let l2 = ListNode::from_vec(vec![3, 4, 5, 6]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![4, 6, 5, 6]);
    }

    #[test]
    fn test_cascade_carries() {
        // Test case: [9,9] + [1] = [0,0,1]
        // Represents: 99 + 1 = 100
        let l1 = ListNode::from_vec(vec![9, 9]);
        let l2 = ListNode::from_vec(vec![1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![0, 0, 1]);
    }

    #[test]
    fn test_all_nines() {
        // Test case: [9,9,9] + [9,9,9] = [8,9,9,1]
        // Represents: 999 + 999 = 1998
        let l1 = ListNode::from_vec(vec![9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(result), vec![8, 9, 9, 1]);
    }

    #[test]
    fn test_large_numbers() {
        // Test case: [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1] + [5,6,4] = [6,6,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]
        let l1 = ListNode::from_vec(vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1,
        ]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(
            ListNode::to_vec(result),
            vec![
                6, 6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1
            ]
        );
    }
}
