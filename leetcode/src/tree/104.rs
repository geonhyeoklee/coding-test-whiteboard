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

  pub fn max_depth_with_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_depth(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      match node {
        Some(node) => {
          let node = node.borrow();
          let left_depth = max_depth(node.left.clone());
          let right_depth = max_depth(node.right.clone());
          left_depth.max(right_depth) + 1
        }
        None => 0,
      }
    }
    max_depth(root)
  }
}

fn main() {
  // Removed test logic from main.
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_max_depth() {
    let expected = 1;
    let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let result = Solution::max_depth(root);
    assert_eq!(expected, result);
  }

  #[test]
  fn test_max_depth_with_recursive() {
    // Create a tree with a root and two children for depth 2 testing.
    let root = Some(Rc::new(RefCell::new(TreeNode {
      val: 1,
      left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
      right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let expected = 2;
    let result = Solution::max_depth_with_recursive(root);
    assert_eq!(expected, result);
  }
}
