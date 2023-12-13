class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


head = Node(1)
indices = {1: head}
cur = head

for val in [6, 7, 2, 4, 8, 3, 5, 9]:
    node = Node(val)
    cur.next = node
    indices[val] = node
    cur = node

for val in range(10, 1000001):
    node = Node(val)
    cur.next = node
    indices[val] = node
    cur = node
cur.next = head

val = 1

for j in range(0, 10000000):
    if j % 100000 == 0:
        one = indices[1]
        print(one.next.val)
        print(one.next.next.val)
    index_node = indices[val]
    removed_nodes = []
    for _ in range(0, 3):
        removed_nodes.append(index_node.next)
        index_node.next = index_node.next.next
    target_val = val - 1
    while True:
        if target_val == 0:
            target_val = 1000000
        found = False
        for node in removed_nodes:
            if node.val == target_val:
                target_val -= 1
                found = True
                break
        if not found:
            break
    target_node = indices[target_val]
    stored = target_node.next
    for removed in removed_nodes:
        target_node.next = removed
        target_node = removed
    target_node.next = stored
    val = index_node.next.val

one = indices[1]
print(one.next.val)
print(one.next.next.val)
print(one.next.val * one.next.next.val)
