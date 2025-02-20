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

struct Codec {
  data: String,
}

impl Codec {
  fn new() -> Self {
    Codec {
      data: String::new(),
    }
  }

  fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
    self.data.clear();

    Self::serialize_dfs(self, root);

    if self.data.is_empty() {
      return String::new();
    }
    // 맨 앞의 쉼표 제거
    self.data[1..].to_string()
  }

  fn serialize_dfs(&mut self, node: Option<Rc<RefCell<TreeNode>>>) {
    if node.is_none() {
      self.data.push_str(",None");
      return;
    }

    if let Some(node) = node {
      let val = node.borrow().val;
      self.data.push_str(",");
      self.data.push_str(&val.to_string());

      Self::serialize_dfs(self, node.borrow().left.clone());
      Self::serialize_dfs(self, node.borrow().right.clone());
    }
  }

  fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() {
      return None;
    }

    let values: Vec<&str> = data.split(",").collect();

    if values.len() <= 1 {
      return None;
    }

    let (node, _) = Self::deserialize_dfs(&values, 0);
    node
  }

  fn deserialize_dfs(values: &Vec<&str>, index: usize) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
    if index >= values.len() {
      return (None, index);
    }

    let val = values[index];
    let next_index = index + 1;

    if val.eq("None") {
      (None, next_index)
    } else {
      let val = val.parse::<i32>().unwrap();
      let mut node = TreeNode::new(val);

      let (left, left_index) = Self::deserialize_dfs(values, next_index);
      let (right, right_index) = Self::deserialize_dfs(values, left_index);

      node.left = left;
      node.right = right;

      (Some(Rc::new(RefCell::new(node))), right_index)
    }
  }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  fn check_same_tree(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
      (None, None) => true,
      (Some(a), Some(b)) => {
        let a = a.borrow();
        let b = b.borrow();
        a.val == b.val
          && check_same_tree(a.left.clone(), b.left.clone())
          && check_same_tree(a.right.clone(), b.right.clone())
      }
      _ => false,
    }
  }

  #[test]
  fn test_serialize_deserialize() {
    let mut codec = Codec::new();

    // Test case 1: Simple tree
    let mut root = TreeNode::new(1);
    let left = TreeNode::new(2);
    let right = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    let tree = Some(Rc::new(RefCell::new(root)));
    let serialized: String = codec.serialize(tree.clone());
    let deserialized = codec.deserialize(serialized.clone());

    assert!(check_same_tree(tree, deserialized));

    // Test case 2: Empty tree
    let empty_tree: Option<Rc<RefCell<TreeNode>>> = None;
    let serialized = codec.serialize(empty_tree.clone());
    let deserialized = codec.deserialize(serialized);

    assert!(check_same_tree(empty_tree, deserialized));

    // Test case 3: Complex tree
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let right = TreeNode::new(3);
    let left_left = TreeNode::new(4);

    left.left = Some(Rc::new(RefCell::new(left_left)));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    let tree = Some(Rc::new(RefCell::new(root)));
    let serialized = codec.serialize(tree.clone());
    let deserialized = codec.deserialize(serialized);

    assert!(check_same_tree(tree, deserialized));
  }
}
