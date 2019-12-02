from typing import List

import sys, unittest

class IntcodeComputer:
    ADD = 1
    MUL = 2
    HALT = 99

    def __init__(self, program: List[int]):
        self.code = program
        self.pc = 0  # Program counter

    def execute(self):
        """Run the program to completion."""
        while self.step():
            pass

    def step(self) -> bool:
        """Return True iff the program should continue."""
        if self.code[self.pc] == self.HALT:
            return False

        opcode, pos1, pos2, target_pos = self.code[self.pc:self.pc + 4]
        if opcode == self.ADD:
            self.code[target_pos] = self.code[pos1] + self.code[pos2]
        elif opcode == self.MUL:
            self.code[target_pos] = self.code[pos1] * self.code[pos2]
        else:
            assert False  # Those are the only valid opcodes.

        # Update the program counter.
        self.pc += 4
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
            IntcodeComputer(input_).execute()
            self.assertEqual(input_, expected)

if __name__ == "__main__":
    program = list(map(int, sys.stdin.readline().split(',')))

    program[1] = 12
    program[2] = 2

    cpu = IntcodeComputer(program)
    cpu.execute()

    print(program[0])

    unittest.main()
