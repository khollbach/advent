from cpu.intcode_computer import IntcodeComputer

from typing import List, Iterable, Optional
import sys

if __name__ == "__main__":
    program = list(map(int, sys.stdin.readline().split(",")))
    inputs = iter([1])
    IntcodeComputer(program).run(get_input_fn=lambda: next(inputs))
    inputs = iter([5])
    IntcodeComputer(program).run(get_input_fn=lambda: next(inputs))
