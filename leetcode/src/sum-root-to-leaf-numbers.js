function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val;
  this.left = left === undefined ? null : left;
  this.right = right === undefined ? null : right;
}

/**
 * @param {TreeNode} root
 * @return {number}
 */
var sumNumbers = function (root) {
  let sum = 0;
  let temp = "";

  function dfs(node, temp) {
    if (!node) {
      return;
    }

    const val = node.val;
    temp += val;

    if (!node.left && !node.right) {
      sum += parseInt(temp, 10);
    }

    dfs(node.left, temp);
    dfs(node.right, temp);

    return val;
  }

  dfs(root, temp);

  return sum;
};

const root = new TreeNode(
  4,
  new TreeNode(9, new TreeNode(5), new TreeNode(1)),
  new TreeNode(0)
);

sumNumbers(root);
