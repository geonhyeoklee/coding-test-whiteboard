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

  pub fn has_path_sum_with_stack(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
      return false;
    }

    let sum = 0;
    let mut stack = vec![(root, sum)];

    while let Some((node, sum)) = stack.pop() {
      let node = node.unwrap();
      let borrowed = node.borrow();
      let sum = sum + borrowed.val;

      if borrowed.left.is_none() && borrowed.right.is_none() {
        if sum == target_sum {
          return true;
        }
      }

      if borrowed.left.is_some() {
        stack.push((borrowed.left.clone(), sum));
      }

      if borrowed.right.is_some() {
        stack.push((borrowed.right.clone(), sum));
      }
    }

    false
  }
}

fn main() {
  let mut root = TreeNode::new(4);
  root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
  root.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

  let result = Solution::has_path_sum(Some(Rc::new(RefCell::new(root))), 11);

  println!("{:?}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_tree() {
    assert_eq!(Solution::has_path_sum(None, 0), false);
    assert_eq!(Solution::has_path_sum_with_stack(None, 0), false);
  }

  #[test]
  fn test_single_node() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(Solution::has_path_sum(root.clone(), 1), true);
    assert_eq!(Solution::has_path_sum_with_stack(root, 1), true);
  }

  #[test]
  fn test_negative_values() {
    let mut root = TreeNode::new(5);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let root = Some(Rc::new(RefCell::new(root)));

    assert_eq!(Solution::has_path_sum(root.clone(), 2), true); // 5 -> -3
    assert_eq!(Solution::has_path_sum_with_stack(root, 2), true);
  }

  #[test]
  fn test_complex_tree() {
    let mut root = TreeNode::new(5);
    let mut left = TreeNode::new(4);
    let mut right = TreeNode::new(8);

    left.left = Some(Rc::new(RefCell::new(TreeNode::new(11))));
    right.left = Some(Rc::new(RefCell::new(TreeNode::new(13))));
    right.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    let root = Some(Rc::new(RefCell::new(root)));

    assert_eq!(Solution::has_path_sum(root.clone(), 26), true); // 5 -> 8 -> 13
    assert_eq!(Solution::has_path_sum_with_stack(root.clone(), 26), true);
    assert_eq!(Solution::has_path_sum(root.clone(), 100), false); // No such path
    assert_eq!(Solution::has_path_sum_with_stack(root, 100), false);
  }

  #[test]
  fn test_has_path_sum() {
    // Build the binary tree: 4, 2, 7
    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let root = Some(Rc::new(RefCell::new(root)));

    // 4 -> 7 gives a sum of 11.
    assert_eq!(Solution::has_path_sum(root, 11), true);
  }

  #[test]
  fn test_has_path_sum_with_stack() {
    // Build the binary tree: 4, 2, 7
    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let root = Some(Rc::new(RefCell::new(root)));

    // 4 -> 7 gives a sum of 11.
    assert_eq!(Solution::has_path_sum_with_stack(root, 11), true);
  }
}
