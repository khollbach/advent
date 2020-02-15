from typing import List, Iterable, Optional, Callable, Generator, Union
from collections import namedtuple
from typing import overload

Instruction = namedtuple("Instruction", "opcode, param_types")

class _Instr:
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

    def __init__(self, program: Iterable[int]) -> None:
        """
        Create a new IntcodeComputer for running the given program.

        The program can be run multiple times using the same IntcodeComputer.
        """
        self._original_program: List[int] = list(program)
        self._mem: _Memory
        self._pc: int
        self._relative_base: int

    def run(
        self,
        noun: Optional[int] = None,
        verb: Optional[int] = None,
        get_input_fn: Optional[Callable[[], int]] = None,
        send_output_fn: Optional[Callable[[int], None]] = None,
    ) -> int:
        """
        Initialize memory, and reset the program counter and the relative base;
        then run the program to completion.

        This method optionally accepts a noun and a verb, which (if specified)
        are copied to memory addresses 1 and 2 respectively before running. The
        value returned is whatever is in memory address 0 upon program halt.

        Input and output are performed via the given functions (or stdin/stdout
        if unspecified).
        """
        self._mem = _Memory(self._original_program)
        self._pc = 0
        self._relative_base = 0

        if noun is not None:
            self._mem[1] = noun
        if verb is not None:
            self._mem[2] = verb

        # Default to stdio
        def get_input() -> int:
            return int(input("> "))
        def send_output(val: int) -> None:
            print(val)
        get_input_fn = get_input_fn or get_input
        send_output_fn = send_output_fn or send_output

        # Run until halted.
        try:
            while True:
                self._step(get_input_fn, send_output_fn)
        except _ProgramHalt:
            pass

        return self._mem[0]

    def _step(
        self,
        get_input_fn: Callable[[], int],
        send_output_fn: Callable[[int], None],
    ) -> None:
        """
        Execute one instruction and update the program counter.
        Throw _ProgramHalt if it was a halt instruction.
        """
        opcode = self._mem[self._pc] % 100  # Two right-most digits.
        param_modes = self._mem[self._pc] // 100  # Leading digits.

        if opcode == _Instr.HALT.opcode:
            # mypy won't let me do this ...
            () = self._consume_args(_Instr.HALT, param_modes)  # type: ignore
            raise _ProgramHalt
        elif opcode == _Instr.ADD.opcode:
            val1, val2, target_pos = self._consume_args(_Instr.ADD, param_modes)
            self._mem[target_pos] = val1 + val2
        elif opcode == _Instr.MUL.opcode:
            val1, val2, target_pos = self._consume_args(_Instr.MUL, param_modes)
            self._mem[target_pos] = val1 * val2
        elif opcode == _Instr.INPUT.opcode:
            target_pos, = self._consume_args(_Instr.INPUT, param_modes)
            self._mem[target_pos] = get_input_fn()
        elif opcode == _Instr.OUTPUT.opcode:
            val, = self._consume_args(_Instr.OUTPUT, param_modes)
            send_output_fn(val)
        elif opcode == _Instr.JUMP_IF_TRUE.opcode:
            condition, target_instr = self._consume_args(_Instr.JUMP_IF_TRUE, param_modes)
            if condition != 0:
                self._pc = target_instr
        elif opcode == _Instr.JUMP_IF_FALSE.opcode:
            condition, target_instr = self._consume_args(_Instr.JUMP_IF_FALSE, param_modes)
            if condition == 0:
                self._pc = target_instr
        elif opcode == _Instr.LT.opcode:
            val1, val2, target_pos = self._consume_args(_Instr.LT, param_modes)
            self._mem[target_pos] = int(val1 < val2)
        elif opcode == _Instr.EQ.opcode:
            val1, val2, target_pos = self._consume_args(_Instr.EQ, param_modes)
            self._mem[target_pos] = int(val1 == val2)
        elif opcode == _Instr.ADJUST_RB.opcode:
            val, = self._consume_args(_Instr.ADJUST_RB, param_modes)
            self._relative_base += val
        else:
            print("Unexpected opcode:", opcode)
            assert False

    def _consume_args(
        self,
        instruction: Instruction,
        param_mode_flags: int,
    ) -> List[int]:
        """
        Consume the instruction's parameters; deferencing as necessary
        according to the parameter mode flags (as described in the spec, in
        *REVERSE* order).

        Update the program counter accordingly, to point to the beginning of
        the next instruction.
        """
        # Consume params; some may be dereferenced before returning, depending
        # on param_mode_flags. (Update the program counter!)
        num_params = len(instruction.param_types)
        params = self._mem[self._pc + 1 : self._pc + 1 + num_params]
        self._pc += 1 + num_params

        vals = []

        # Handle param_mode_flags.
        for param_type, param in zip(instruction.param_types, params):
            mode = param_mode_flags % 10
            param_mode_flags //= 10

            # Immediate mode
            if mode == 1:
                assert param_type != "w"
                val = param

            # Position mode / relative mode
            else:
                assert mode in (0, 2)

                # In relative mode, the address is relative to the r.b.
                base = self._relative_base if mode == 2 else 0

                # Read-params are dereferenced; write params are not.
                if param_type == "r":
                    val = self._mem[base + param]
                else:
                    assert param_type == "w"
                    val = base + param

            vals.append(val)

        assert param_mode_flags == 0
        return vals

class _Memory(list):
    """
    List[int] wrapper that extends and fills with zeros as neccessary.

    This simulates an "infinite" list, all initialized to zero. I.e.,
    infinite memory.
    """

    def __init__(self, program: Iterable[int]) -> None:
        super().__init__(program)

    def _extend(self, newlen: int) -> None:
        """Entend memory to be at least this long by padding with zeros."""
        diff = newlen - len(self)
        if diff > 0:
            self.extend([0] * diff)

    @overload
    def __getitem__(self, address: int) -> int: ...
    @overload
    def __getitem__(self, address: slice) -> List[int]: ...
    def __getitem__(self, address):
        if isinstance(address, int):
            assert address >= 0
            self._extend(address + 1)
            return super().__getitem__(address)
        else:
            # I'll implement the general case if/when the time comes...
            assert address.step is None or address.step > 0
            self._extend(address.stop)
            return super().__getitem__(address)

    @overload
    def __setitem__(self, i: int, o: int) -> None: ...
    @overload
    def __setitem__(self, s: slice, o: Iterable[int]) -> None: ...
    def __setitem__(self, address, value) -> None:
        assert isinstance(address, int)
        assert address >= 0
        self._extend(address + 1)
        super().__setitem__(address, value)
