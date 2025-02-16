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
  pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
      (None, None) => true,
      (Some(p), Some(q)) => {
        let p = p.borrow();
        let q = q.borrow();
        p.val == q.val
          && Self::is_same_tree(p.left.clone(), q.left.clone())
          && Self::is_same_tree(p.right.clone(), q.right.clone())
      }
      _ => false,
    }
  }

  pub fn is_same_tree_with_stack(
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
  ) -> bool {
    let mut stack = vec![(p, q)];

    while let Some((node1, node2)) = stack.pop() {
      match (node1, node2) {
        (None, None) => continue,
        (Some(n1), Some(n2)) => {
          let n1_b = n1.borrow();
          let n2_b = n2.borrow();
          if n1_b.val != n2_b.val {
            return false;
          }
          stack.push((n1_b.left.clone(), n2_b.left.clone()));
          stack.push((n1_b.right.clone(), n2_b.right.clone()));
        }
        _ => return false,
      }
    }
    true
  }
}

fn main() {
  let expected = true;
  let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  let result = Solution::is_same_tree(root.clone(), root);

  assert_eq!(expected, result);
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;
  use std::rc::Rc;

  #[test]
  fn test_same_tree_single_node() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    assert!(Solution::is_same_tree(tree1, tree2));
  }

  #[test]
  fn test_different_tree_single_node() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    assert!(!Solution::is_same_tree(tree1, tree2));
  }

  #[test]
  fn test_same_tree_complex() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
      val: 3,
      left: left.clone(),
      right: right.clone(),
    })));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
      val: 3,
      left,
      right,
    })));
    assert!(Solution::is_same_tree(tree1, tree2));
  }

  #[test]
  fn test_same_tree_with_stack() {
    // Single node tree
    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    assert!(Solution::is_same_tree_with_stack(tree1, tree2));

    // Different single node tree
    let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    assert!(!Solution::is_same_tree_with_stack(tree1, tree2));

    // Complex tree
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
      val: 3,
      left: left.clone(),
      right: right.clone(),
    })));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
      val: 3,
      left,
      right,
    })));
    assert!(Solution::is_same_tree_with_stack(tree1, tree2));
  }
}
