// Definition for singly-linked list.
class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

// 1 -> 2 -> 3 -> 4 -> 5
// 1 -> 4 -> 3 -> 2 -> 5
function reverseBetween(head: ListNode | null, left: number, right: number): ListNode | null {
    const dummy = new ListNode(0);
    dummy.next = head;

    let prev = dummy;
    let current = head!;

    let i = 1;

    while (i < left) {
        prev = current;
        current = current.next!;
        i++;
    }

    let prevLeftNode = prev; // 1
    let leftNode = current; // 2
    let next: ListNode | null = null;

    while (i <= right) {
        next = current.next!;
        console.log(next.val);
        current.next = prev;
        prev = current;
        current = next!;
        i++;
    }

    prevLeftNode.next = prev;
    leftNode.next = current; // 5

    return dummy.next;
};

function main() {
    // Create test ListNodes
    let node5 = new ListNode(5);
    let node4 = new ListNode(4, node5);
    let node3 = new ListNode(3, node4);
    let node2 = new ListNode(2, node3);
    let node1 = new ListNode(1, node2);

    // Call reverseBetween function
    let result = reverseBetween(node1, 2, 4);

    // Print the result
    let current = result;
    while (current !== null) {
        // console.log(current.val);
        current = current.next;
    }
}

main();

