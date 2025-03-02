#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_helper(l1, l2, 0)
    }

    fn add_two_numbers_helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let l1 = l1.unwrap_or(Box::new(ListNode::new(0)));
        let l2 = l2.unwrap_or(Box::new(ListNode::new(0)));

        let sum = l1.val + l2.val + carry;
        let mut current_node = ListNode::new(sum % 10);
        current_node.next = Solution::add_two_numbers_helper(l1.next, l2.next, sum / 10);

        Some(Box::new(current_node))
    }
}

fn main() {
    let mut list1 = ListNode::new(1);
    list1.next = Some(Box::new(ListNode::new(2)));
    // list1.clone().unwrap().next.unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut list2 = ListNode::new(1);
    list2.next = Some(Box::new(ListNode::new(3)));
    // list2.clone().unwrap().next.unwrap().next = Some(Box::new(ListNode::new(5)));

    let result = Solution::add_two_numbers(Some(Box::new(list1)), Some(Box::new(list2)));

    println!("{:?}", result);
}
