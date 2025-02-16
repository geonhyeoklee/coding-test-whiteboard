use std::borrow::Borrow;

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
            (Some(l1), Some(l2)) => match l1.val <= l2.val {
                true => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
                false => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next),
                })),
            },
        }
    }
}
fn main() {
    let list1 = Some(Box::new(ListNode::new(1)));
    list1.clone().unwrap().next = Some(Box::new(ListNode::new(2)));
    list1.clone().unwrap().next.unwrap().next = Some(Box::new(ListNode::new(4)));

    let list2 = Some(Box::new(ListNode::new(1)));
    list2.clone().unwrap().next = Some(Box::new(ListNode::new(3)));
    list2.clone().unwrap().next.unwrap().next = Some(Box::new(ListNode::new(5)));

    let result = Solution::merge_two_lists(list1, list2);

    println!("{:?}", result);
}
