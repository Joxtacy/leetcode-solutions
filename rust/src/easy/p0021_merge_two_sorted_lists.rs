use crate::utils::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let (val1, val2) = (list1.as_ref().unwrap().val, list2.as_ref().unwrap().val);
            if val1 < val2 {
                let next = list1.as_mut().unwrap().next.take();
                tail.next = list1;
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                tail.next = list2;
                list2 = next;
            }
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists_typical() {
        let l1 = ListNode::from_vec(vec![1, 2, 4]);
        let l2 = ListNode::from_vec(vec![1, 3, 4]);
        let merged = Solution::merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_two_lists_both_empty() {
        let l1 = ListNode::from_vec(vec![]);
        let l2 = ListNode::from_vec(vec![]);
        let merged = Solution::merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(merged), vec![]);
    }

    #[test]
    fn test_merge_two_lists_one_empty() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![]);
        let merged = Solution::merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(merged), vec![0]);
    }

    #[test]
    fn test_merge_two_lists_other_empty() {
        let l1 = ListNode::from_vec(vec![]);
        let l2 = ListNode::from_vec(vec![0]);
        let merged = Solution::merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(merged), vec![0]);
    }
}
