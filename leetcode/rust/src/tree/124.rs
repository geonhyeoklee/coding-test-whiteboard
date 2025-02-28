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
  pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_sum = i32::MIN;
    Self::max_path_sum_recursive(root.as_ref(), &mut max_sum);
    max_sum
  }

  fn max_path_sum_recursive(node: Option<&Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
    if let Some(node) = node {
      let node = node.borrow();
      let left_sum = Self::max_path_sum_recursive(node.left.as_ref(), max_sum);
      let right_sum = Self::max_path_sum_recursive(node.right.as_ref(), max_sum);

      let sum = node.val + left_sum.max(0) + right_sum.max(0);
      *max_sum = (*max_sum).max(sum);

      node.val + left_sum.max(right_sum).max(0)
    } else {
      0
    }
  }

  pub fn max_path_sum_with_stack(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    // 루트가 없는 경우 최소값 반환
    if root.is_none() {
      return i32::MIN;
    }

    let mut max_sum = i32::MIN;
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut path_sums = HashMap::new();

    stack.push(Rc::clone(root.as_ref().unwrap()));

    while let Some(node) = stack.last().cloned() {
      let ptr = Rc::as_ptr(&node);

      if !visited.contains(&ptr) {
        visited.insert(ptr);
        let node = node.borrow();

        if let Some(right) = node.right.clone() {
          stack.push(right);
        }
        if let Some(left) = node.left.clone() {
          stack.push(left);
        }
      } else {
        stack.pop(); // 현재 노드를 스택에서 제거하여 사용
        let node = node.borrow();

        // 왼쪽, 오른쪽 자식 노드의 경로 합을 가져옴
        let left_sum = node
          .left
          .as_ref()
          .map_or(0, |n| *path_sums.get(&(Rc::as_ptr(n))).unwrap_or(&0));
        let right_sum = node
          .right
          .as_ref()
          .map_or(0, |n| *path_sums.get(&(Rc::as_ptr(n))).unwrap_or(&0));

        // 현재 노드의 경로 합을 저장
        let curr_val = node.val;
        let max_child = left_sum.max(right_sum).max(0);
        let path_sum = curr_val + max_child;
        path_sums.insert(ptr, path_sum);

        // 현재 노드를 포함한 경로의 합을 최대값과 비교
        max_sum = max_sum.max(curr_val + left_sum.max(0) + right_sum.max(0));
      }
    }

    max_sum
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
      return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while let Some(current) = queue.pop_front() {
      // 왼쪽 자식 처리
      if i < values.len() {
        if let Some(val) = values[i] {
          let left = Rc::new(RefCell::new(TreeNode::new(val)));
          current.borrow_mut().left = Some(Rc::clone(&left));
          queue.push_back(left);
        }
      }
      i += 1;

      // 오른쪽 자식 처리
      if i < values.len() {
        if let Some(val) = values[i] {
          let right = Rc::new(RefCell::new(TreeNode::new(val)));
          current.borrow_mut().right = Some(Rc::clone(&right));
          queue.push_back(right);
        }
      }
      i += 1;
    }

    Some(root)
  }

  #[test]
  fn test_max_path_sum_simple_tree() {
    // 테스트 케이스 1: 단순한 트리 구조 [1,2,3]
    let root = create_tree(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::max_path_sum(root.clone()), 6);
    assert_eq!(Solution::max_path_sum_with_stack(root), 6);
  }

  #[test]
  fn test_max_path_sum_complex_tree() {
    // 테스트 케이스 2: 복잡한 트리 구조 [-10,9,20,null,null,15,7]
    let root = create_tree(vec![
      Some(-10),
      Some(9),
      Some(20),
      None,
      None,
      Some(15),
      Some(7),
    ]);
    assert_eq!(Solution::max_path_sum(root.clone()), 42);
    assert_eq!(Solution::max_path_sum_with_stack(root), 42);
  }

  #[test]
  fn test_max_path_sum_single_node() {
    // 테스트 케이스 3: 단일 노드
    let root = create_tree(vec![Some(-3)]);
    assert_eq!(Solution::max_path_sum(root.clone()), -3);
    assert_eq!(Solution::max_path_sum_with_stack(root), -3);
  }

  #[test]
  fn test_max_path_sum_negative_values() {
    // 테스트 케이스 4: 음수값들로만 구성된 트리
    let root = create_tree(vec![Some(-1), Some(-2), Some(-3)]);
    assert_eq!(Solution::max_path_sum(root.clone()), -1);
    assert_eq!(Solution::max_path_sum_with_stack(root), -1);
  }

  #[test]
  fn test_max_path_sum_zigzag_path() {
    // 테스트 케이스: 지그재그 형태로 이어지는 경로를 가진 트리
    let root = create_tree(vec![
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      None,
      Some(5),
      Some(6),
    ]);
    assert_eq!(Solution::max_path_sum(root.clone()), 16); // 경로: 4 -> 2 -> 1 -> 3 -> 6
    assert_eq!(Solution::max_path_sum_with_stack(root), 16);
  }

  #[test]
  fn test_max_path_sum_alternating_signs() {
    // 테스트 케이스: 양수와 음수가 번갈아 나타나는 트리
    let root = create_tree(vec![
      Some(5),
      Some(-3),
      Some(8),
      Some(2),
      Some(-1),
      Some(4),
      Some(-5),
    ]);
    assert_eq!(Solution::max_path_sum(root.clone()), 17); // 경로: 2 -> -3 -> 5 -> 8 -> 4
    assert_eq!(Solution::max_path_sum_with_stack(root), 17);
  }

  #[test]
  fn test_max_path_sum_all_negative_except_one() {
    // 테스트 케이스: 하나의 양수를 제외하고 모두 음수인 트리
    let root = create_tree(vec![
      Some(-5),
      Some(-3),
      Some(10),
      Some(-4),
      Some(-2),
      Some(-1),
      Some(-6),
    ]);
    assert_eq!(Solution::max_path_sum(root.clone()), 10); // 10 노드만 선택
    assert_eq!(Solution::max_path_sum_with_stack(root), 10);
  }

  #[test]
  fn test_max_path_sum_with_stack_comparison() {
    // Test cases to compare recursive and stack-based implementations
    let test_cases = vec![
      vec![Some(1), Some(2), Some(3)],
      vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)],
      vec![Some(-3)],
      vec![Some(-1), Some(-2), Some(-3)],
      vec![Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6)],
    ];

    for values in test_cases {
      let root = create_tree(values.clone());
      let recursive_result = Solution::max_path_sum(root.clone());
      let stack_result = Solution::max_path_sum_with_stack(root);
      assert_eq!(
        recursive_result, stack_result,
        "Failed for tree: {:?}",
        values
      );
    }
  }
}

fn main() {}
