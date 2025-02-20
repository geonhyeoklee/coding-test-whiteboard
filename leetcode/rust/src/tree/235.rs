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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref root) = root {
      let root_val = root.borrow().val;

      let p_val = p.clone().unwrap().as_ref().borrow().val;
      let q_val = q.clone().unwrap().as_ref().borrow().val;

      // p와 q가 모두 루트보다 크면, LCA는 오른쪽 서브트리에 있습니다.
      if root_val < p_val && root_val < q_val {
        return Self::lowest_common_ancestor(root.borrow().right.clone(), p.clone(), q.clone());
      }

      // p와 q가 모두 루트보다 작으면, LCA는 왼쪽 서브트리에 있습니다.
      if root_val > p_val && root_val > q_val {
        return Self::lowest_common_ancestor(root.borrow().left.clone(), p.clone(), q.clone());
      }
    }

    root
  }

  pub fn lowest_common_ancestor_with_stack(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    let mut path_p = Vec::new();
    let mut path_q = Vec::new();

    let mut current = root.clone();
    while let Some(node) = current {
      let node_val = node.borrow().val;
      path_p.push(node.clone());

      if node_val == p_val {
        break;
      }

      current = if p_val < node_val {
        node.borrow().left.clone()
      } else {
        node.borrow().right.clone()
      };
    }

    current = root;
    while let Some(node) = current {
      let node_val = node.borrow().val;
      path_q.push(node.clone());

      if node_val == q_val {
        break;
      }
      current = if q_val < node_val {
        node.borrow().left.clone()
      } else {
        node.borrow().right.clone()
      };
    }

    let mut lca = None;
    let min_len = path_p.len().min(path_q.len());
    for i in 0..min_len {
      if path_p[i].borrow().val == path_q[i].borrow().val {
        lca = Some(path_p[i].clone());
      } else {
        break;
      }
    }

    lca
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_tree_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
  }

  fn create_test_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = create_tree_node(6);
    let node2 = create_tree_node(2);
    let node8 = create_tree_node(8);
    let node0 = create_tree_node(0);
    let node4 = create_tree_node(4);
    let node7 = create_tree_node(7);
    let node9 = create_tree_node(9);
    let node3 = create_tree_node(3);
    let node5 = create_tree_node(5);

    if let (
      Some(root),
      Some(node2),
      Some(node8),
      Some(node0),
      Some(node4),
      Some(node7),
      Some(node9),
      Some(node3),
      Some(node5),
    ) = (root, node2, node8, node0, node4, node7, node9, node3, node5)
    {
      root.borrow_mut().left = Some(node2.clone());
      root.borrow_mut().right = Some(node8.clone());
      node2.borrow_mut().left = Some(node0);
      node2.borrow_mut().right = Some(node4.clone());
      node8.borrow_mut().left = Some(node7);
      node8.borrow_mut().right = Some(node9);
      node4.borrow_mut().left = Some(node3);
      node4.borrow_mut().right = Some(node5);

      Some(root)
    } else {
      None
    }
  }

  #[test]
  fn test_common_ancestor_basic() {
    let root = create_test_tree();
    let p = create_tree_node(2);
    let q = create_tree_node(8);

    let result = Solution::lowest_common_ancestor(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 6);
  }

  #[test]
  fn test_common_ancestor_subtree() {
    let root = create_test_tree();
    let p = create_tree_node(2);
    let q = create_tree_node(4);

    let result = Solution::lowest_common_ancestor(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 2);
  }

  #[test]
  fn test_common_ancestor_deep() {
    let root = create_test_tree();
    let p = create_tree_node(3);
    let q = create_tree_node(5);

    let result = Solution::lowest_common_ancestor(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 4);
  }

  #[test]
  fn test_common_ancestor_with_stack_basic() {
    let root = create_test_tree();
    let p = create_tree_node(2);
    let q = create_tree_node(8);

    let result = Solution::lowest_common_ancestor_with_stack(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 6);
  }

  #[test]
  fn test_common_ancestor_with_stack_subtree() {
    let root = create_test_tree();
    let p = create_tree_node(2);
    let q = create_tree_node(4);

    let result = Solution::lowest_common_ancestor_with_stack(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 2);
  }

  #[test]
  fn test_common_ancestor_with_stack_deep() {
    let root = create_test_tree();
    let p = create_tree_node(3);
    let q = create_tree_node(5);

    let result = Solution::lowest_common_ancestor_with_stack(root.clone(), p, q);
    assert_eq!(result.unwrap().borrow().val, 4);
  }
}

fn main() {}
