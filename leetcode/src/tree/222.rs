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
  pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
      None => 0,
      Some(root) => {
        let borrowed = root.borrow();
        1 + Self::count_nodes(borrowed.left.clone()) + Self::count_nodes(borrowed.right.clone())
      }
    }
  }

  pub fn count_nodes_with_stack(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut stack = vec![];
    let mut count = 0;

    if let Some(root) = root {
      stack.push(root);
    }

    while let Some(node) = stack.pop() {
      let node = node.borrow();

      count += 1;

      if let Some(left) = node.left.clone() {
        stack.push(left);
      }

      if let Some(right) = node.right.clone() {
        stack.push(right);
      }
    }

    count
  }
}

fn main() {
  let mut root = TreeNode::new(4);
  root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  root.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

  let result = Solution::count_nodes(Some(Rc::new(RefCell::new(root))));

  println!("{:?}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_test_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(1);
    let mut node2 = TreeNode::new(2);
    let mut node3 = TreeNode::new(3);
    let node4 = TreeNode::new(4);
    let node5 = TreeNode::new(5);
    let node6 = TreeNode::new(6);

    node2.left = Some(Rc::new(RefCell::new(node4)));
    node2.right = Some(Rc::new(RefCell::new(node5)));
    node3.left = Some(Rc::new(RefCell::new(node6)));

    root.left = Some(Rc::new(RefCell::new(node2)));
    root.right = Some(Rc::new(RefCell::new(node3)));

    Some(Rc::new(RefCell::new(root)))
  }

  #[test]
  fn test_empty_tree() {
    assert_eq!(Solution::count_nodes(None), 0);
    assert_eq!(Solution::count_nodes_with_stack(None), 0);
  }

  #[test]
  fn test_single_node() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(Solution::count_nodes(root.clone()), 1);
    assert_eq!(Solution::count_nodes_with_stack(root), 1);
  }

  #[test]
  fn test_complete_binary_tree() {
    let tree = create_test_tree(); // 6개의 노드를 가진 트리
    assert_eq!(Solution::count_nodes(tree.clone()), 6);
    assert_eq!(Solution::count_nodes_with_stack(tree), 6);
  }

  #[test]
  fn test_left_heavy_tree() {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    left.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left = Some(Rc::new(RefCell::new(left)));

    let tree = Some(Rc::new(RefCell::new(root)));
    assert_eq!(Solution::count_nodes(tree.clone()), 3);
    assert_eq!(Solution::count_nodes_with_stack(tree), 3);
  }

  #[test]
  fn test_right_heavy_tree() {
    let mut root = TreeNode::new(1);
    let mut right = TreeNode::new(2);
    right.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right = Some(Rc::new(RefCell::new(right)));

    let tree = Some(Rc::new(RefCell::new(root)));
    assert_eq!(Solution::count_nodes(tree.clone()), 3);
    assert_eq!(Solution::count_nodes_with_stack(tree), 3);
  }
}
