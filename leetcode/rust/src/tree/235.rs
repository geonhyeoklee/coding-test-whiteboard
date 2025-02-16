// Definition for a binary tree node.
pub struct Solution;

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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref root) = root {
            let root_val = root.borrow().val;

            let p_val = p.clone().unwrap().as_ref().borrow().val;
            let q_val = q.clone().unwrap().as_ref().borrow().val;

            if root_val < p_val && root_val < q_val {
                return Self::lowest_common_ancestor(root.borrow().right.clone(), p.clone(), q.clone());
            }

            if root_val > p_val && root_val > q_val {
                return Self::lowest_common_ancestor(root.borrow().left.clone(), p.clone(), q.clone());
            }
        }

        root
    }
}

fn main() {}
