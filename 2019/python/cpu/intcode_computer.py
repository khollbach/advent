from typing import List, Iterable, Optional, Callable, Generator, Union
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
    ADJUST_RB = 9
    HALT = 99

    def __init__(self, program: Iterable[int]):
        """Create a new IntcodeComputer for running the given program."""
        self._ORIGINAL_PROGRAM: List[int] = list(program)
        self._mem: Memory = None
        self._pc: int = None
        self._relative_base: int = None
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
        self._mem = Memory(self._ORIGINAL_PROGRAM)
        self._pc = 0
        self._relative_base = 0

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
                self._mem[target_pos] = val1 + val2  # TODO: relative mode!
            elif opcode == self.MUL:
                self._mem[target_pos] = val1 * val2  # TODO: relative mode!
        elif opcode == self.INPUT:
            _, target_pos = self._mem[self._pc:self._pc + 2]
            self._pc += 2

            # TODO: implement this for all instrs that write to memory.
            #   You might want a separate class for instruction, representing
            #   the types of parameters the instruction has.
            # Weird edge case: writing to an address using relative mode is
            # allowed.
            #assert param_modes in (0, 2)
            #if param_modes == 2:
            #    target_pos += self._relative_base

            self._mem[target_pos] = self._get_input_fn()  # TODO: relative mode!
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
            self._mem[target_pos] = result  # TODO: relative mode!
        elif opcode == self.ADJUST_RB:
            _, param = self._mem[self._pc:self._pc + 2]
            self._pc += 2
            val, = self._access_params(param_modes, param)
            self._relative_base += val
        else:
            print("Unexpected opcode:", opcode)
            assert False

    def _access_params(self, param_modes: int, *params: int) -> List[int]:
        """
        Take the parameter mode flags (as described in the spec, in *REVERSE*
        order), and zero or more parameters. For each parameter, return either
        value itself or the value pointed to, or the pointed value relative to
        the `relative_base`, depending on the flag (1 or 0 or 2, respectively).
        """
        vals = []
        for param in params:
            mode = param_modes % 10

            # Position mode / relative mode (dereference the parameter)
            if mode in (0, 2):
                # In relative mode, the offset is relative to the r.b.
                base = self._relative_base if mode == 2 else 0
                val = self._mem[base + param]
            # Immediate mode
            else:
                assert mode == 1
                val = param

            vals.append(val)
            param_modes //= 10
        return vals

class Memory(list):
    """List[int] wrapper that extends and fills with zeros as neccessary."""
    def __init__(self, program: Iterable[int]):
        super().__init__(program)

    def _extend(self, newlen: int) -> None:
        """Entend memory to be at least this long by padding with zeros."""
        diff = newlen - len(self)
        if diff > 0:
            self.extend([0] * diff)

    def __getitem__(self, address: Union[int, slice]) -> int:
        if isinstance(address, int):
            assert address >= 0
            if address >= len(self):
                self._extend(address + 1)
        elif isinstance(address, slice):
            # (I'll implement the general case if/when the time comes...)
            assert address.step is None or address.step > 0

            if address.stop - 1 >= len(self):
                self._extend(address.stop)
        return super().__getitem__(address)

    def __setitem__(self, address: int, value: int) -> None:
        assert address >= 0
        if address >= len(self):
            self._extend(address + 1)
        super().__setitem__(address, value)

if __name__ == "__main__":
    doctest.testmod()
