/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */

function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val;
  this.left = left === undefined ? null : left;
  this.right = right === undefined ? null : right;
}

/**
 * Encodes a tree to a single string.
 *
 * @param {TreeNode} root
 * @return {string}
 */
const serialize = function (root) {
  let result = "";

  /**
   *
   * @param {TreeNode} node
   * @returns
   */
  const dfs = function (node) {
    if (!node) {
      result += `,null`;
      return;
    }

    const val = node.val;
    result += `,${val}`;

    dfs(node.left);
    dfs(node.right);
  };

  dfs(root);
  return result.slice(1);
};

/**
 * Decodes your encoded data to tree.
 *
 * @param {string} data
 * @return {TreeNode}
 */
const deserialize = function (data) {
  const values = data.split(",");

  const dfs = function () {
    if (values.length <= 0) {
      return;
    }
    const val = values.shift();

    if (val === "null") {
      return null;
    }

    const node = new TreeNode(parseInt(val, 10));
    node.left = dfs();
    node.right = dfs();
    return node;
  };

  return dfs();
};

/**
 * Your functions will be called as such:
 * deserialize(serialize(root));
 */

const root = new TreeNode(
  7,
  new TreeNode(3),
  new TreeNode(15, new TreeNode(9), new TreeNode(20))
);

const result = serialize(root);
const tree = deserialize(result);
console.log(tree);
