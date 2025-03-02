use std::collections::HashMap;

pub struct Solution;

#[derive(Debug)]
struct TrieNode {
  children: HashMap<char, TrieNode>,
  is_end: bool,
}

impl TrieNode {
  fn new() -> Self {
    TrieNode {
      children: HashMap::new(),
      is_end: false,
    }
  }
}

#[derive(Debug)]
struct Trie {
  root: TrieNode,
}

impl Trie {
  fn new() -> Self {
    Trie {
      root: TrieNode::new(),
    }
  }

  fn insert(&mut self, word: &str) -> () {
    let mut curr = &mut self.root;

    for c in word.chars() {
      curr = curr.children.entry(c).or_insert(TrieNode::new());
    }

    curr.is_end = true;
  }

  fn longest_common_prefix(&mut self) -> String {
    let mut curr = &mut self.root;
    let mut prefix: String = String::new();

    while !curr.children.is_empty() {
      if curr.children.len() > 1 || curr.is_end {
        return prefix;
      }

      let key = curr.children.keys().next().unwrap().clone();
      curr = curr.children.get_mut(&key).unwrap();
      prefix.push(key);
    }

    prefix
  }

  fn longest_common_prefix_with_stack(&mut self) -> String {
    let curr = &mut self.root;
    let mut prefix: String = String::new();

    let mut stack = vec![curr];

    while let Some(curr) = stack.pop() {
      if curr.children.len() > 1 || curr.is_end {
        return prefix;
      }

      let key = curr.children.keys().next().unwrap().clone();
      let curr = curr.children.get_mut(&key).unwrap();
      stack.push(curr);
      prefix.push(key);
    }

    prefix
  }
}

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut trie = Trie::new();

    for s in strs.iter() {
      trie.insert(s);
    }

    trie.longest_common_prefix()
  }
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_longest_common_prefix() {
    let test = ["flower", "flow", "flight"].map(|s| s.to_string()).to_vec();
    let expected = "fl";
    let result = Solution::longest_common_prefix(test);
    assert_eq!(expected, result);
  }
}
