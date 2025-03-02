function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val;
  this.left = left === undefined ? null : left;
  this.right = right === undefined ? null : right;
}

/**
 * @param {TreeNode} root
 * @param {number} k
 * @return {number}
 */
var kthSmallest = function (root, k) {
  const result = [];

  const dfs = (node) => {
    if (!node) {
      return;
    }

    result.push(node.val);

    dfs(node.left);
    dfs(node.right);
  };

  dfs(root);

  result.sort((a, b) => a - b);
  return result[k - 1];
};

const root = new TreeNode(
  1,
  new TreeNode(2, undefined, new TreeNode(5)),
  new TreeNode(3, undefined, new TreeNode(4))
);

console.log(kthSmallest(root, 2));
