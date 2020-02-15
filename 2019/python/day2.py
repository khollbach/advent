from cpu.intcode_computer import IntcodeComputer

from typing import List, Iterable, Optional, Tuple
import sys

def find_correct_inputs(
    program: List[int], target_output: int
) -> Tuple[int, int]:
    cpu = IntcodeComputer(program)
    for noun in range(100):
        for verb in range(100):
            if cpu.run(noun, verb) == target_output:
                return noun, verb
    assert False

if __name__ == "__main__":
    program = list(map(int, sys.stdin.readline().split(",")))
    output = IntcodeComputer(program).run(12, 2)
    print(f"output: {output}")

    target_output = 19690720
    noun, verb = find_correct_inputs(program, target_output)
    print(f"noun: {noun}, verb: {verb}")
