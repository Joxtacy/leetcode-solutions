// Definition for singly-linked list (matching LeetCode's format)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    // Helper methods for testing
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }
}

#[cfg(test)]
mod list_node_tests {
    use super::*;

    #[test]
    fn test_from_vec_empty() {
        let result = ListNode::from_vec(vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_from_vec_single() {
        let result = ListNode::from_vec(vec![42]);
        assert_eq!(ListNode::to_vec(result), vec![42]);
    }

    #[test]
    fn test_to_vec_empty() {
        let result = ListNode::to_vec(None);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_round_trip() {
        let original = vec![1, 2, 3, 4, 5];
        let list = ListNode::from_vec(original.clone());
        let back_to_vec = ListNode::to_vec(list);
        assert_eq!(original, back_to_vec);
    }
}
