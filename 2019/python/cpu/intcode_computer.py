from typing import List, Iterable, Optional, Callable, Generator, Union
from enum import Enum
from collections import namedtuple

Instruction = namedtuple("Instruction", "opcode, param_types")

class Instr:
    ADD = Instruction(1, "rrw")
    MUL = Instruction(2, "rrw")
    INPUT = Instruction(3, "w")
    OUTPUT = Instruction(4, "r")
    JUMP_IF_TRUE = Instruction(5, "rr")
    JUMP_IF_FALSE = Instruction(6, "rr")
    LT = Instruction(7, "rrw")
    EQ = Instruction(8, "rrw")
    ADJUST_RB = Instruction(9, "r")
    HALT = Instruction(99, "")

class _ProgramHalt(Exception):
    """Raised when a halt instruction executes."""
    pass


class IntcodeComputer:
    """An interpreter for Intcode programs."""

    def __init__(self, program: Iterable[int]):
        """Create a new IntcodeComputer for running the given program."""
        self._ORIGINAL_PROGRAM: List[int] = list(program)
        self._mem: _Memory = None
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
        self._mem = _Memory(self._ORIGINAL_PROGRAM)
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
        except _ProgramHalt:
            pass

        return self._mem[0]

    def _step(self) -> None:
        """
        Execute one instruction and update the program counter.
        Throw _ProgramHalt if it was a halt instruction.
        """
        opcode = self._mem[self._pc] % 100  # Two right-most digits.
        param_modes = self._mem[self._pc] // 100  # Leading digits.

        if opcode == Instr.HALT.opcode:
            () = self._consume_args(Instr.HALT, param_modes)
            raise _ProgramHalt
        elif opcode == Instr.ADD.opcode:
            val1, val2, target_pos = self._consume_args(Instr.ADD, param_modes)
            self._mem[target_pos] = val1 + val2
        elif opcode == Instr.MUL.opcode:
            val1, val2, target_pos = self._consume_args(Instr.MUL, param_modes)
            self._mem[target_pos] = val1 * val2
        elif opcode == Instr.INPUT.opcode:
            target_pos, = self._consume_args(Instr.INPUT, param_modes)
            self._mem[target_pos] = self._get_input_fn()
        elif opcode == Instr.OUTPUT.opcode:
            val, = self._consume_args(Instr.OUTPUT, param_modes)
            self._send_output_fn(val)
        elif opcode == Instr.JUMP_IF_TRUE.opcode:
            condition, target_instr = self._consume_args(Instr.JUMP_IF_TRUE, param_modes)
            if condition != 0:
                self._pc = target_instr
        elif opcode == Instr.JUMP_IF_FALSE.opcode:
            condition, target_instr = self._consume_args(Instr.JUMP_IF_FALSE, param_modes)
            if condition == 0:
                self._pc = target_instr
        elif opcode == Instr.LT.opcode:
            val1, val2, target_pos = self._consume_args(Instr.LT, param_modes)
            self._mem[target_pos] = int(val1 < val2)
        elif opcode == Instr.EQ.opcode:
            val1, val2, target_pos = self._consume_args(Instr.EQ, param_modes)
            self._mem[target_pos] = int(val1 == val2)
        elif opcode == Instr.ADJUST_RB.opcode:
            val, = self._consume_args(Instr.ADJUST_RB, param_modes)
            self._relative_base += val
        else:
            print("Unexpected opcode:", opcode)
            assert False

    def _consume_args(self, instr: Instruction, param_modes: int) -> List[int]:
        """
        Consume the instruction's parameters; deferencing as necessary
        according to the parameter mode flags (as described in the spec, in
        *REVERSE* order).

        Update the program counter accordingly, to point to the beginning of
        the next instruction.
        """
        # Consume params; some may be dereferenced before returning, depending
        # on param_modes. (Update the program counter!)
        num_params = len(instr.param_types)
        params = self._mem[self._pc + 1 : self._pc + 1 + num_params]
        self._pc += 1 + num_params

        # Handle param_modes.
        vals = []
        for param_type, param in zip(instr.param_types, params):
            mode = param_modes % 10

            # Immediate mode
            if mode == 1:
                # Write-params are never in immediate mode.
                assert param_type == "r"
                val = param
            # Position mode / relative mode
            # Read-params are dereferenced; write params are not.
            else:
                assert mode in (0, 2)
                # In relative mode, the address is relative to the r.b.
                base = self._relative_base if mode == 2 else 0
                if param_type == "r":
                    val = self._mem[base + param]
                else:
                    assert param_type == "w"
                    val = base + param

            vals.append(val)
            param_modes //= 10
        assert param_modes == 0
        return vals

class _Memory(list):
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
