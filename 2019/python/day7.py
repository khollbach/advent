from typing import List, Iterable, Optional, Callable, Generator

import sys, doctest

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
        self._get_input_fn: Callable[[], int] = None
        self._send_output_fn: Callable[[int], None] = None

    def run(
        self,
        noun: Optional[int] = None,
        verb: Optional[int] = None,
        get_input_fn: Optional[Callable[[], int]] = None,
        send_output_fn: Optional[Callable[[int], None]] = None,
    ) -> int:
        """
        Initialize memory, and reset the program counter; then run the program
        to completion.

        This method optionally accepts a noun and a verb, which (if specified)
        are copied to memory addresses 1 and 2 respectively before running. The
        value returned is whatever is in memory address 0 upon program halt.

        Input and output are performed via the given functions (or stdin/stdout
        if unspecified).
        """
        self._mem = self._ORIGINAL_PROGRAM.copy()
        self._pc = 0

        if noun is not None:
            self._mem[1] = noun
        if verb is not None:
            self._mem[2] = verb

        def get_input() -> int:
            return int(input("> "))
        def send_output(val: int) -> None:
            print(val)
        self._get_input_fn = get_input_fn or get_input
        self._send_output_fn = send_output_fn or send_output

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
            self._mem[target_pos] = self._get_input_fn()
        elif opcode == self.OUTPUT:
            _, param = self._mem[self._pc:self._pc + 2]
            self._pc += 2
            val, = self._access_params(param_modes, param)
            self._send_output_fn(val)
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
            # Position mode (dereference the parameter)
            if param_modes % 10 == 0:
                val = self._mem[param]
            # Immediate mode
            else:
                assert param_modes % 10 == 1
                val = param
            vals.append(val)
            param_modes //= 10
        return vals

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
