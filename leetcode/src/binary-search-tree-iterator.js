function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val;
  this.left = left === undefined ? null : left;
  this.right = right === undefined ? null : right;
}

/**
 * @param {TreeNode} root
 */
var BSTIterator = function (root) {
  this.bst = [];

  const bst_search = (node) => {
    if (!node) {
      return;
    }

    bst_search(node.left);
    this.bst?.push(node.val);
    bst_search(node.right);
  };

  bst_search(root);
};

/**
 * @return {number}
 */
BSTIterator.prototype.next = function () {
  return this.bst.shift();
};

/**
 * @return {boolean}
 */
BSTIterator.prototype.hasNext = function () {
  return this.bst.length > 0;
};

/**
 * Your BSTIterator object will be instantiated and called as such:
 * var obj = new BSTIterator(root)
 * var param_1 = obj.next()
 * var param_2 = obj.hasNext()
 */

const root = new TreeNode(
  7,
  new TreeNode(3),
  new TreeNode(15, new TreeNode(9), new TreeNode(20))
);

const iterator = new BSTIterator(root);
console.log(iterator.next());
console.log(iterator.next());
console.log(iterator.hasNext());
console.log(iterator.next());
console.log(iterator.hasNext());
console.log(iterator.next());
console.log(iterator.hasNext());
console.log(iterator.next());
console.log(iterator.hasNext());
