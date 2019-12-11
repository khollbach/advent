from typing import List, Iterable, Optional

import sys

class ProgramHalt(Exception):
    pass

class IntcodeComputer:
    """An interpreter for Intcode programs."""

    ADD = 1
    MUL = 2
    INPUT = 3
    OUTPUT = 4
    JUMP_IF_TRUE = 5
    JUMP_IF_FALSE = 6
    LT = 7
    EQ = 8
    HALT = 99

    def __init__(self, program: Iterable[int]):
        """Create a new IntcodeComputer for running the given program."""
        self._ORIGINAL_PROGRAM: List[int] = list(program)
        self._mem: List[int] = None
        self._pc: int = None
        self._inputs_iterator: Iterator[int] = None

    def run(
        self,
        noun: Optional[int] = None,
        verb: Optional[int] = None,
        inputs: Iterable[int] = [],
    ) -> int:
        """
        Initialize memory, and reset the program counter; then run the program
        to completion. Inputs are optional.
        """
        self._mem = self._ORIGINAL_PROGRAM.copy()
        self._pc = 0
        self._inputs_iterator = iter(inputs)

        if noun is not None:
            self._mem[1] = noun
        if verb is not None:
            self._mem[2] = verb

        # Run until halted.
        try:
            while True:
                self._step()
        except ProgramHalt:
            pass

        return self._mem[0]

    def _step(self) -> None:
        """
        Execute one instruction. Throw ProgramHalt if it was a halt
        instruction.
        """
        opcode = self._mem[self._pc] % 100  # Two right-most digits.
        param_modes = self._mem[self._pc] // 100  # Leading digits.

        if opcode == self.HALT:
            _, = self._mem[self._pc:self._pc + 1]
            self._pc += 1
            raise ProgramHalt
        elif opcode in (self.ADD, self.MUL):
            _, param1, param2, target_pos = self._mem[self._pc:self._pc + 4]
            self._pc += 4
            val1, val2 = self._access_params(param_modes, param1, param2)
            if opcode == self.ADD:
                self._mem[target_pos] = val1 + val2
            elif opcode == self.MUL:
                self._mem[target_pos] = val1 * val2
        elif opcode == self.INPUT:
            _, target_pos = self._mem[self._pc:self._pc + 2]
            self._pc += 2
            self._mem[target_pos] = next(self._inputs_iterator)
        elif opcode == self.OUTPUT:
            _, param = self._mem[self._pc:self._pc + 2]
            self._pc += 2
            val, = self._access_params(param_modes, param)
            print(val)
        elif opcode in (self.JUMP_IF_TRUE, self.JUMP_IF_FALSE):
            _, param1, param2 = self._mem[self._pc:self._pc + 3]
            self._pc += 3
            condition, target_instr = \
                self._access_params(param_modes, param1, param2)
            if opcode == self.JUMP_IF_TRUE and condition != 0 or \
                    opcode == self.JUMP_IF_FALSE and condition == 0:
                self._pc = target_instr
        elif opcode in (self.LT, self.EQ):
            _, param1, param2, target_pos = self._mem[self._pc:self._pc + 4]
            self._pc += 4
            val1, val2 = self._access_params(param_modes, param1, param2)
            if opcode == self.LT and val1 < val2 or \
                    opcode == self.EQ and val1 == val2:
                result = 1
            else:
                result = 0
            self._mem[target_pos] = result
        else:
            assert False  # Those are the only valid opcodes.

    def _access_params(self, param_modes: int, *params: int) -> List[int]:
        """
        Take the parameter mode flags (as described in the spec, in *REVERSE*
        order), and zero or more parameters. For each parameter, return either
        value itself or the value pointed to, depending on the flag (1 or 0
        respectively).
        """
        vals = []
        for param in params:
            # Position mode
            if param_modes % 10 == 0:
                val = self._mem[param]
            # Immediate mode
            else:
                assert param_modes % 10 == 1
                val = param
            vals.append(val)
            param_modes //= 10
        return vals

if __name__ == "__main__":
    program = list(map(int, sys.stdin.readline().split(",")))
    IntcodeComputer(program).run(inputs=[1])
    IntcodeComputer(program).run(inputs=[5])
