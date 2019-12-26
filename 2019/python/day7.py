from cpu.intcode_computer import IntcodeComputer

from typing import List, Iterable, Optional, Callable, Generator
import sys, doctest

def permutations(nums: List[int]) -> Generator[List[int], None, None]:
    """
    >>> list(permutations([]))
    [[]]
    >>> list(permutations([1]))
    [[1]]
    >>> set(map(tuple, permutations([1,2]))) == {(1,2), (2,1)}
    True
    >>> set(map(tuple, permutations([1,2,3]))) == {(1,2,3), (1,3,2), \
                                                   (2,1,3), (2,3,1), \
                                                   (3,1,2), (3,2,1)}
    True
    """
    if not nums:
        yield []
        return
    for p in permutations(nums[1:]):
        for i in range(len(p) + 1):
            yield p[:i] + [nums[0]] + p[i:]

def get_thruster_signal(program: List[int], settings: List[int]) -> int:
    prev_output = 0
    for amp_idx in range(len(settings)):
        inputs = [settings[amp_idx], prev_output]
        outputs = []
        IntcodeComputer(program).run(
            get_input_fn=(lambda: inputs.pop(0)),
            send_output_fn=(lambda val: outputs.append(val)))
        prev_output, = outputs
    return prev_output

def best_signal(program: List[int]) -> int:
    return max(
            get_thruster_signal(program, settings)
            for settings in permutations(list(range(5))))

if __name__ == "__main__":
    doctest.testmod()

    program = list(map(int, sys.stdin.readline().split(",")))

    print(best_signal(program))
