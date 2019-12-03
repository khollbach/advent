from typing import List, Iterable, Optional

import sys, unittest

class IntcodeComputer:
    """An interpreter for Intcode programs."""

    ADD = 1
    MUL = 2
    HALT = 99

    def __init__(self, program: Iterable[int]):
        self._reinit(program)

    def _reinit(self, program: Iterable[int]):
        """Initialize the computer's memory and reset the program counter."""
        self._mem = list(program)  # Make a copy of the input.
        self._pc = 0  # Program counter

    def execute(
        self, noun: Optional[int] = None, verb: Optional[int] = None
    ) -> int:
        """Run the program to completion."""
        if noun is not None:
            self._mem[1] = noun
        if verb is not None:
            self._mem[2] = verb

        # Run until halted.
        while self._step():
            pass

        self._pc = None  # Prevent accidental re-use.
        return self._mem[0]

    def _step(self) -> bool:
        """Execute one instruction.
        Return True iff the program should continue."""
        if self._mem[self._pc] == self.HALT:
            return False

        opcode, pos1, pos2, target_pos = self._mem[self._pc:self._pc + 4]
        if opcode == self.ADD:
            self._mem[target_pos] = self._mem[pos1] + self._mem[pos2]
        elif opcode == self.MUL:
            self._mem[target_pos] = self._mem[pos1] * self._mem[pos2]
        else:
            assert False  # Those are the only valid opcodes.

        # Update the program counter.
        self._pc += 4
        return True

# todo: find a better way to organize test cases. They're probably meant to go
# in another file.
class TestIntcodeComputer(unittest.TestCase):
    EXAMPLES = [
        #(input, expected),
        ([1,0,0,3,99], [1,0,0,2,99]),
        ([1,9,10,3,2,3,11,0,99,30,40,50], [3500,9,10,70,2,3,11,0,99,30,40,50]),
        ([1,0,0,0,99], [2,0,0,0,99]),
        ([2,3,0,3,99], [2,3,0,6,99]),
        ([2,4,4,5,99,0], [2,4,4,5,99,9801]),
        ([1,1,1,4,99,5,6,0,99], [30,1,1,4,2,5,6,0,99]),
    ]

    def test_examples(self):
        for input_, expected in self.EXAMPLES:
            cpu = IntcodeComputer(input_)
            cpu.execute()
            self.assertEqual(expected, cpu._mem)

def find_correct_inputs(
    program: List[int], target_output: int
) -> (int, int):
    for noun in range(100):
        for verb in range(100):
            cpu = IntcodeComputer(program)
            if cpu.execute(noun, verb) == target_output:
                return noun, verb
    assert False

if __name__ == "__main__":
    program = list(map(int, sys.stdin.readline().split(",")))
    output = IntcodeComputer(program).execute(12, 2)
    print(f"output: {output}")

    target_output = 19690720
    noun, verb = find_correct_inputs(program, target_output)
    print(f"noun: {noun}, verb: {verb}")

    unittest.main()
