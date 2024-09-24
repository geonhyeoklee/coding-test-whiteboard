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

use std::rc::Rc;
use std::cell::RefCell;
use std::str::Split;

struct Codec {
	data: String,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {
          data: String::new(),
        }
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::serialize_dfs(self,root);

        self.data.to_string()
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
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let values = data.split(",");

        Self::deserialize_dfs(self, values)
    }

    fn deserialize_dfs(&self, mut values: Split<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        let val = values.next().unwrap();
        
        if val.eq("None") {
          None
        } else {
          let val = val.to_string().parse::<i32>().unwrap();
          let mut node = TreeNode::new(val);
          node.left = Self::deserialize_dfs(self, values.clone());
          node.right = Self::deserialize_dfs(self, values.clone());

          Some(Rc::new(RefCell::new(node)))
        }

    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
fn main() {

}