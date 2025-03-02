class _Node {
    val: number;
    next: _Node | null;
    random: _Node | null;

    constructor(val?: number, next?: _Node, random?: _Node) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
        this.random = random === undefined ? null : random;
    }
}

function copyRandomList(head: _Node | null): _Node | null {
    const cache = new Map<_Node, _Node>();
    return deepCopy(head, cache);
}

function deepCopy(
    head: _Node | null,
    cache: Map<_Node, _Node>,
): _Node | null {
    if (!head) {
        return null;
    }

    if (cache.has(head)) {
        return cache.get(head)!;
    }

    const newHead = new _Node(head.val);
    cache.set(head, newHead);
    newHead.next = deepCopy(head.next, cache);
    newHead.random = deepCopy(head.random, cache);
    return newHead;
}

function test() {
    const head = new _Node(1);
    const node = new _Node(2);
    head.next = node;
    head.random = node;

    copyRandomList(head);
}

test();
