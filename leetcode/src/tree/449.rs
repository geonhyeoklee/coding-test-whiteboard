// Definition for a binary search tree node.
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

  fn print_tree(root: Option<Rc<RefCell<TreeNode>>>, prefix: String, is_left: bool) {
    if let Some(node) = root {
      let node = node.borrow();
      println!(
        "{}{}{}",
        prefix,
        if is_left { "├── " } else { "└── " },
        node.val
      );

      let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });

      if node.left.is_some() || node.right.is_some() {
        Self::print_tree(node.left.clone(), new_prefix.clone(), true);
        Self::print_tree(node.right.clone(), new_prefix, false);
      }
    }
  }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
  fn new() -> Self {
    Codec {}
  }

  fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut serialized = String::new();

    while let Some(current) = queue.pop_front() {
      if let Some(node) = current {
        let node = node.borrow();
        serialized.push_str(&node.val.to_string());
        serialized.push(',');

        queue.push_back(node.left.clone());
        queue.push_back(node.right.clone());
      } else {
        serialized.push_str("None,");
      }
    }

    serialized.pop();
    serialized
  }

  fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() {
      return None;
    }

    let values: Vec<&str> = data.split(',').collect();
    if values[0] == "None" {
      return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].parse().unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;

    while let Some(current) = queue.pop_front() {
      if values[i] != "None" {
        let left = Rc::new(RefCell::new(TreeNode::new(values[i].parse().unwrap())));
        current.borrow_mut().left = Some(Rc::clone(&left));
        queue.push_back(left);
      }
      i += 1;

      if values[i] != "None" {
        let right = Rc::new(RefCell::new(TreeNode::new(values[i].parse().unwrap())));
        current.borrow_mut().right = Some(Rc::clone(&right));
        queue.push_back(right);
      }
      i += 1;
    }

    Some(root)
  }
}

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

  fn create_test_tree() -> Option<Rc<RefCell<TreeNode>>> {
    // Create a test tree:
    //      1
    //     / \
    //    2   3
    //       / \
    //      4   5
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));

    root.borrow_mut().left = Some(node2);
    root.borrow_mut().right = Some(node3.clone());
    node3.borrow_mut().left = Some(node4);
    node3.borrow_mut().right = Some(node5);

    Some(root)
  }

  #[test]
  fn test_empty_tree() {
    let codec = Codec::new();
    let empty: Option<Rc<RefCell<TreeNode>>> = None;
    let serialized = codec.serialize(empty);
    assert!(check_same_tree(codec.deserialize(serialized), None));
  }

  #[test]
  fn test_single_node() {
    let codec = Codec::new();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let serialized = codec.serialize(root.clone());
    assert!(check_same_tree(codec.deserialize(serialized), root));
  }

  #[test]
  fn test_complete_tree() {
    let codec = Codec::new();
    let root = create_test_tree();
    let serialized = codec.serialize(root.clone());
    let deserialized = codec.deserialize(serialized.clone());

    TreeNode::print_tree(root.clone(), String::new(), false);
    println!("serialized: {}", serialized);
    TreeNode::print_tree(deserialized.clone(), String::new(), false);

    assert!(check_same_tree(deserialized, root));
  }
}

fn main() {} // empty main function
