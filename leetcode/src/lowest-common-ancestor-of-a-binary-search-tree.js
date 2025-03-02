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
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function (root, p, q) {
  if (root.val < p.val && root.val < q.val) {
    return lowestCommonAncestor(root.right, p, q);
  }
  if (root.val > p.val && root.val > q.val) {
    return lowestCommonAncestor(root.left, p, q);
  }
  return root;
};

// p와 q가 모두 루트보다 큰 경우
// p와 q가 모두 루트보다 작은 경우

// p와 q 둘 중 하나가 루트인 경우
// 위에 두 케이스가 모두 아닌 경우에는 p 혹은 q가 lca이다.

// p와 q 둘 다 아닌 경우
// 부모 노드로부터 갈라진 케이스

const root = new TreeNode(
  6,
  new TreeNode(2, new TreeNode(0), new TreeNode(4)),
  new TreeNode(8, new TreeNode(7), new TreeNode(9))
);

lowestCommonAncestor(
  root,
  new TreeNode(2, new TreeNode(0), new TreeNode(4)),
  new TreeNode(8, new TreeNode(7), new TreeNode(9))
);
