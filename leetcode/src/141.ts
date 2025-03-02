class ListNode {
    val: number;
    next: ListNode | null;
    constructor(val?: number, next?: ListNode | null) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
    }
}

function hasCycle(head: ListNode | null): boolean {
    let slow: ListNode | null = head;
    let fast: ListNode | null = head;

    while (fast && fast.next) {
        slow = slow!.next;
        fast = fast.next.next;
        if (slow === fast) {
            return true;
        }
    }

    return false;
}

function main() {
    const head = new ListNode(
        3,
    );
    const node1 = new ListNode(2);
    const node2 = new ListNode(2);
    const node3 = new ListNode(0);
    const node4 = new ListNode(-4);

    head.next = node1;
    node1.next = node2;
    node2.next = node3;
    node3.next = node4;
    node4.next = node2;

    console.log(hasCycle(head));
}

main();
