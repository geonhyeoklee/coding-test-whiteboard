use std::borrow::Borrow;
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = Self::invert(root);

        if let Some(root) = root.borrow() {
            let node = root.as_ref().borrow();
            Self::invert_tree(node.left.clone());
            Self::invert_tree(node.right.clone());
        }

        root
    }

    fn invert(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let mut node = root.as_ref().borrow_mut();
            let temp: Option<Rc<RefCell<TreeNode>>> = node.left.clone();
            node.left = node.right.clone();
            node.right = temp;
            Some(root.clone())
        } else {
            None
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    if let Some(root) = root.borrow() {
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    }

    // let expected = root.clone();
    let result = Solution::invert_tree(root.clone());
    println!("{:?}", result);
}
