#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = vec![];
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let len = queue.len();
            let mut sum = 0;

            for _ in 0..len {
                let root = queue.pop_front().unwrap();
                match root {
                    Some(root) => {
                        let node = root.borrow();

                        sum += i64::from(node.val);

                        if node.left.is_some() {
                            queue.push_back(node.left.clone());
                        }

                        if node.right.is_some() {
                            queue.push_back(node.right.clone());
                        }
                    }
                    _ => unreachable!(),
                }
            }

            result.push(sum as f64 / len as f64);
        }

        result
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    if let Some(root) = root.clone() {
        let left = TreeNode::new(9);
        let mut right = TreeNode::new(20);
        right.left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

        root.borrow_mut().left = Some(Rc::new(RefCell::new(left)));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(right)));
    }

    let result = Solution::average_of_levels(root);
    println!("Result: {:?}", result);
}
