/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var maxPathSum = function (root) {
  let max = -Infinity;

  const stack = [];
  stack.push(root);

  while (stack.length > 0) {
    const node = stack.pop();
    const leftSum = node.val + node?.left?.val || 0;
    const rightSum = node.val + node?.right?.val || 0;
    const allSum = leftSum + rightSum - node.val;

    max = Math.max(node.val, leftSum, rightSum, allSum, max);

    node?.left && stack.push(node.left);
    node?.right && stack.push(node.right);
  }

  return max;
};
