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

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct BSTIterator {
  pub bst: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
  fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
    let mut bst = VecDeque::new();

    Self::preorder(root, &mut bst);

    BSTIterator { bst }
  }

  fn preorder(node: Option<Rc<RefCell<TreeNode>>>, bst: &mut VecDeque<i32>) {
    if let Some(node) = node {
      if node.borrow().left.is_some() {
        Self::preorder(node.borrow().left.clone(), bst)
      }

      let val = node.borrow().val;
      bst.push_back(val);

      if node.borrow().right.is_some() {
        Self::preorder(node.borrow().right.clone(), bst)
      }
    }
  }

  fn next(&mut self) -> i32 {
    let opt_val = self.bst.pop_front();
    match opt_val {
      Some(val) => val,
      None => -1,
    }
  }

  fn has_next(&self) -> bool {
    self.bst.len() > 0
  }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {
  let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  // if let Some(ref node) = root {
  //     node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
  //     node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
  // }

  // let left_root = root.as_ref().unwrap().borrow_mut().left.clone();
  // if let Some(ref node) = left_root {
  //     node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
  //     node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
  // }

  // let result = Solution::sum_numbers(root);
  // println!("{}", result);
}
