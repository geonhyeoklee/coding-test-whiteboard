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
    if root.is_none() || root.eq(&p) || root.eq(&q) {
      return root;
    }

    let left = Self::lowest_common_ancestor(
      root.clone().unwrap().borrow().left.clone(),
      p.clone(),
      q.clone(),
    );

    let right = Self::lowest_common_ancestor(
      root.clone().unwrap().borrow().right.clone(),
      p.clone(),
      q.clone(),
    );

    if left.is_none() {
      return right;
    }

    if right.is_none() {
      return left;
    }

    root
  }

  pub fn lowest_common_ancestor_with_stack(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
      return None;
    }

    let mut stack = vec![(root.clone(), false)];
    let mut parent_map = std::collections::HashMap::new();
    let mut p_found = false;
    let mut q_found = false;

    // DFS로 노드 탐색하며 parent map 구성
    while let Some((node, visited)) = stack.pop() {
      if let Some(node) = node {
        if !visited {
          stack.push((Some(node.clone()), true));

          // right child push
          let right = node.borrow().right.clone();
          if right.is_some() {
            stack.push((right.clone(), false));
            parent_map.insert(right.unwrap().borrow().val, node.clone());
          }

          // left child push
          let left = node.borrow().left.clone();
          if left.is_some() {
            stack.push((left.clone(), false));
            parent_map.insert(left.unwrap().borrow().val, node.clone());
          }
        }

        if let Some(p) = p.clone() {
          if p.borrow().val == node.borrow().val {
            p_found = true;
          }
        }

        if let Some(q) = q.clone() {
          if q.borrow().val == node.borrow().val {
            q_found = true;
          }
        }
      }
    }

    // p와 q 둘 다 찾지 못했다면 None 반환
    if !p_found || !q_found {
      return None;
    }

    // [Important]
    // p부터 시작해서 루트까지 가는 경로를 저장
    let mut p_path = std::collections::HashSet::new();
    let mut current = p;
    while let Some(node) = current {
      p_path.insert(node.borrow().val);
      current = parent_map.get(&node.borrow().val).cloned();
    }

    // [Important]
    // q부터 시작해서 올라가면서 p의 경로와 처음 만나는 지점이 LCA
    let mut current = q;
    while let Some(node) = current {
      if p_path.contains(&node.borrow().val) {
        return Some(node);
      }
      current = parent_map.get(&node.borrow().val).cloned();
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_tree(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
  }

  #[test]
  fn test_lowest_common_ancestor() {
    // Test case 1: Tree structure:
    //      3
    //     / \
    //    5   1
    //   / \
    //  6   2
    let node6 = create_tree(6, None, None);
    let node2 = create_tree(2, None, None);
    let node5 = create_tree(5, node6.clone(), node2);
    let node1 = create_tree(1, None, None);
    let root = create_tree(3, node5.clone(), node1.clone());

    // Test LCA of nodes 5 and 1
    let result = Solution::lowest_common_ancestor(root.clone(), node5.clone(), node1);
    assert_eq!(result.unwrap().borrow().val, 3);

    // Test LCA of nodes 5 and 6
    let result = Solution::lowest_common_ancestor(root.clone(), node5.clone(), node6);
    assert_eq!(result.unwrap().borrow().val, 5);

    // Test case 2: Single node tree
    let single_node = create_tree(1, None, None);
    let result =
      Solution::lowest_common_ancestor(single_node.clone(), single_node.clone(), single_node);
    assert_eq!(result.unwrap().borrow().val, 1);
  }

  #[test]
  fn test_lowest_common_ancestor_with_stack() {
    // Test case 1: Tree structure:
    //      3
    //     / \
    //    5   1
    //   / \
    //  6   2
    let node6 = create_tree(6, None, None);
    let node2 = create_tree(2, None, None);
    let node5 = create_tree(5, node6.clone(), node2);
    let node1 = create_tree(1, None, None);
    let root = create_tree(3, node5.clone(), node1.clone());

    // Test LCA of nodes 5 and 1
    let result = Solution::lowest_common_ancestor_with_stack(root.clone(), node5.clone(), node1);
    assert_eq!(result.unwrap().borrow().val, 3);

    // Test LCA of nodes 5 and 6
    let result = Solution::lowest_common_ancestor_with_stack(root.clone(), node5.clone(), node6);
    assert_eq!(result.unwrap().borrow().val, 5);

    // Test case 2: Single node tree
    let single_node = create_tree(1, None, None);
    let result = Solution::lowest_common_ancestor_with_stack(
      single_node.clone(),
      single_node.clone(),
      single_node,
    );
    assert_eq!(result.unwrap().borrow().val, 1);
  }
}

fn main() {}
