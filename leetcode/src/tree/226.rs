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

  pub fn invert_tree_with_stack(
    root: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack = vec![root.clone()];

    while let Some(node) = stack.pop() {
      if let Some(node) = node {
        let node = Self::invert(Some(node.clone()));

        // ### Invert code
        // let node = node.as_ref().borrow();
        // let temp: Option<Rc<RefCell<TreeNode>>> = node.left.clone();
        // node.left = node.right.clone();
        // node.right = temp;

        if let Some(node) = node {
          let node = node.as_ref().borrow();
          stack.push(node.left.clone());
          stack.push(node.right.clone());
        }
      }
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

#[cfg(test)]
mod tests {
  use super::*;

  fn create_test_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    if let Some(root) = root.as_ref() {
      let mut node = root.borrow_mut();
      node.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
      node.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

      if let Some(left) = node.left.as_ref() {
        let mut left = left.borrow_mut();
        left.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        left.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
      }

      if let Some(right) = node.right.as_ref() {
        let mut right = right.borrow_mut();
        right.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
      }
    }
    root
  }

  fn are_trees_equal(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
      (None, None) => true,
      (Some(a), Some(b)) => {
        let a = a.as_ref().borrow();
        let b = b.as_ref().borrow();
        a.val == b.val
          && are_trees_equal(a.left.clone(), b.left.clone())
          && are_trees_equal(a.right.clone(), b.right.clone())
      }
      _ => false,
    }
  }

  #[test]
  fn test_empty_tree() {
    let empty: Option<Rc<RefCell<TreeNode>>> = None;
    assert!(are_trees_equal(
      Solution::invert_tree(empty.clone()),
      Solution::invert_tree_with_stack(empty)
    ));
  }

  #[test]
  fn test_single_node() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert!(are_trees_equal(
      Solution::invert_tree(root.clone()),
      Solution::invert_tree_with_stack(root.clone())
    ));
  }

  #[test]
  fn test_complete_binary_tree() {
    let tree = create_test_tree();
    let recursive_result = Solution::invert_tree(tree.clone());
    let iterative_result = Solution::invert_tree_with_stack(tree.clone());
    assert!(are_trees_equal(recursive_result, iterative_result));
  }
}
