// Definition for a binary tree node.
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

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let sum = 0;
        return Self::dfs(root, target_sum, sum);
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, mut sum: i32) -> bool {
        match root {
            Some(root) => {
                let borrowed = root.as_ref().borrow();
                sum = sum + borrowed.val;

                match (borrowed.left.is_some(), borrowed.right.is_some()) {
                    (true, false) => return Self::dfs(borrowed.left.clone(), target_sum, sum),
                    (false, true) => return Self::dfs(borrowed.right.clone(), target_sum, sum),
                    _ => {
                        return Self::dfs(borrowed.left.clone(), target_sum, sum)
                            || Self::dfs(borrowed.right.clone(), target_sum, sum)
                    }
                }
            }
            None => target_sum == sum,
        }
    }
}

fn main() {
    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let result = Solution::has_path_sum(Some(Rc::new(RefCell::new(root))), 11);

    println!("{:?}", result);
}
