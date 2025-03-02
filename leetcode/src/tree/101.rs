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
  pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn compare(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
      match (l, r) {
        (None, None) => true,
        (None, Some(_n)) | (Some(_n), None) => false,
        (Some(l), Some(r)) => {
          if l.borrow().val != r.borrow().val {
            return false;
          }
          return compare(l.borrow().left.clone(), r.borrow().right.clone())
            && compare(l.borrow().right.clone(), r.borrow().left.clone());
        }
      }
    }
    match root {
      Some(r) => compare(r.borrow().left.clone(), r.borrow().right.clone()),
      None => true,
    }
  }

  pub fn is_symmetric_with_stack(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = vec![];
    if let Some(root) = root.as_ref() {
      stack.push((root.borrow().left.clone(), root.borrow().right.clone()));
    }

    while let Some((l, r)) = stack.pop() {
      match (l, r) {
        (None, None) => continue,
        (None, Some(_n)) | (Some(_n), None) => return false,
        (Some(l), Some(r)) => {
          if l.borrow().val != r.borrow().val {
            return false;
          }
          stack.push((l.borrow().left.clone(), r.borrow().right.clone()));
          stack.push((l.borrow().right.clone(), r.borrow().left.clone()));
        }
      }
    }

    true
  }
}

fn main() {
  let root = Some(Rc::new(RefCell::new(TreeNode::new(4))));
  if let Some(root) = root.as_ref() {
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
  }

  // let expected = root.clone();
  let result = Solution::is_symmetric(root.clone());
  println!("{:?}", result);
}

// New tests for is_symmetric_with_stack
#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;
  use std::rc::Rc;

  #[test]
  fn test_is_symmetric_with_stack_true() {
    // symmetric tree:     1
    //                   /     \
    //                  2       2
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    {
      let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
      let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
      root.as_ref().unwrap().borrow_mut().left = left;
      root.as_ref().unwrap().borrow_mut().right = right;
    }
    assert!(Solution::is_symmetric_with_stack(root));
  }

  #[test]
  fn test_is_symmetric_with_stack_false() {
    // asymmetric tree:    1
    //                   /     \
    //                  2       3
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    {
      let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
      let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
      root.as_ref().unwrap().borrow_mut().left = left;
      root.as_ref().unwrap().borrow_mut().right = right;
    }
    assert!(!Solution::is_symmetric_with_stack(root));
  }
}
