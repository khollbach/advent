from typing import Set, Dict, Optional, Tuple

import sys, re

class Node:
    """A node in a tree."""
    label: str
    parent: Optional["Node"]
    children: Set["Node"]
    def __init__(self, label: str) -> None:
        self.label = label
        self.parent = None
        self.children = set()

def read_tree_from_stdin() -> Tuple[Node, Dict[str, Node]]:
    nodes: Dict[str, Node] = {}
    pattern = re.compile(r"(\w+)\)(\w+)")
    for line in sys.stdin:
        match = re.match(pattern, line)
        assert match
        parent, child = match.groups()
        if parent not in nodes:
            nodes[parent] = Node(parent)
        if child not in nodes:
            nodes[child] = Node(child)
        nodes[child].parent = nodes[parent]
        nodes[parent].children.add(nodes[child])
    return nodes["COM"], nodes

def count_nodes(tree: Node) -> int:
    return 1 + sum(map(count_nodes, tree.children))

def depth_sum(tree: Node, depth: int = 0) -> int:
    total = depth
    for c in tree.children:
        total += depth_sum(c, depth + 1)
    return total

def depth(node: Optional[Node]) -> int:
    if not node:
        return -1
    return 1 + depth(node.parent)

def lowest_common_ancestor(
    node1: Optional[Node],
    node2: Optional[Node],
) -> Optional[Node]:
    d1 = depth(node1)
    d2 = depth(node2)

    # Swap so that d1 <= d2
    if d1 > d2:
        d1, d2 = d2, d1
        node1, node2 = node2, node1

    # Walk node2 up to the same level of depth as node1.
    while d2 != d1:
        assert node2
        node2 = node2.parent
        d2 -= 1

    # Walk both nodes up in sync until they meet!
    while node1 != node2:
        assert node1 and node2
        node1 = node1.parent
        node2 = node2.parent

    return node1

def distance(node1: Node, node2: Node) -> int:
    lca = lowest_common_ancestor(node1, node2)
    d = depth(lca)
    d1 = depth(node1)
    d2 = depth(node2)
    return (d1 - d) + (d2 - d)

if __name__ == "__main__":
    tree, nodes = read_tree_from_stdin()

    # Sanity checks
    assert tree.label == "COM"
    assert count_nodes(tree) == 1003

    # Part 1
    print(depth_sum(tree))

    # Part 2
    print(distance(nodes["YOU"], nodes["SAN"]) - 2)
