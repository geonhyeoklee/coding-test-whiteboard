use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut stack = vec![(root, 0)];
        while let Some((node, mut depth)) = stack.pop() {
            if let Some(node) = node {
                depth += 1;
                max_depth = max_depth.max(depth);

                stack.push((node.borrow().left.clone(), depth));
                stack.push((node.borrow().right.clone(), depth));
            }
        }

        max_depth
    }
}

fn main() {
    let expected = 1;
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let result = Solution::max_depth(root);

    assert_eq!(expected, result);
}
