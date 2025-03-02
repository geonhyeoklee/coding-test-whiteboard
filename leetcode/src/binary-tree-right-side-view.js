function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val;
  this.left = left === undefined ? null : left;
  this.right = right === undefined ? null : right;
}

/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var rightSideView = function (root) {
  const result = [];

  const dfs = (node, height) => {
    if (!node) {
      return;
    }

    const val = node.val;
    result[height] = val;

    dfs(node.left, height + 1);
    dfs(node.right, height + 1);
  };

  dfs(root, 0);

  return result;
};

const root = new TreeNode(
  1,
  new TreeNode(2, undefined, new TreeNode(5)),
  new TreeNode(3, undefined, new TreeNode(4))
);

rightSideView(root);
