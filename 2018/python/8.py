from typing import Iterable, Iterator

import sys

class Tree:
    def __init__(self) -> None:
        self.children = []
        self.metadata = []

    @classmethod
    def build_tree(cls, nums: Iterable[int]) -> "Tree":
        return cls._build_tree_helper(iter(nums))

    @classmethod
    def _build_tree_helper(
            cls, nums: Iterator[int], is_toplevel: bool = True) -> "Tree":
        """
        `nums` is an *ITERATOR* not an iterable. It is partially or entirely
        consumed. Consume the first node and all of its children.
        """
        num_children = next(nums)
        num_metadata = next(nums)

        root = Tree()

        for _ in range(num_children):
            root.children.append(
                    cls._build_tree_helper(nums, is_toplevel = False))

        for _ in range(num_metadata):
            root.metadata.append(next(nums))

        if is_toplevel:
            try:
                next(nums)  # This should throw.
                assert False
            except StopIteration:
                pass

        return root

    def metadata_sum(self) -> int:
        return sum(self.metadata) + \
                sum(child.metadata_sum() for child in self.children)

if __name__ == "__main__":
    nums = list(map(int, sys.stdin.readline().split(" ")))
    tree = Tree.build_tree(nums)
    print(tree.metadata_sum())
