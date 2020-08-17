/// Information about the cpu instruction for some opcode.
#[derive(Debug)]
pub struct Instruction {
    pub op: Operation,

    /// How many arguments does this instruction accept, and what are their modes.
    /// If a param has type Write, it will never be in Immediate mode.
    pub params: Vec<(ParamType, ParamMode)>,
}

impl Instruction {
    /// Parse an opcode into its operation and parameter mode flags.
    pub fn new(opcode: i32) -> Self {
        let op = Operation::new(opcode % 100);

        let mut params = vec![];

        let mut mode_bits = opcode / 100;
        for typ in op.param_types() {
            let mode = ParamMode::new(mode_bits % 10);
            if typ == ParamType::Write && mode == ParamMode::Immediate {
                panic!("Invalid mode: Write param in Immediate mode.");
            }
            params.push((typ, mode));

            mode_bits /= 10;
        }

        Self { op, params }
    }
}

/// Supported cpu operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
    GetInput,
    SendOutput,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffset,
    Halt,
}

impl Operation {
    /// Parse an operation code into its operation. Must not include mode flags.
    pub fn new(code: i32) -> Self {
        use Operation::*;

        match code {
            1 => Add,
            2 => Multiply,
            3 => GetInput,
            4 => SendOutput,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => RelativeBaseOffset,
            99 => Halt,
            _ => panic!("Not an operation: {}", code),
        }
    }

    /// What is the operation code for this op?
    pub fn _code(&self) -> i32 {
        use Operation::*;

        match self {
            Add => 1,
            Multiply => 2,
            GetInput => 3,
            SendOutput => 4,
            JumpIfTrue => 5,
            JumpIfFalse => 6,
            LessThan => 7,
            Equals => 8,
            RelativeBaseOffset => 9,
            Halt => 99,
        }
    }

    /// How many parameters (and of what type) does this op expect?
    pub fn param_types(&self) -> Vec<ParamType> {
        use Operation::*;
        use ParamType::*;

        match self {
            Add => vec![Read, Read, Write],
            Multiply => vec![Read, Read, Write],
            GetInput => vec![Write],
            SendOutput => vec![Read],
            JumpIfTrue => vec![Read, Read],
            JumpIfFalse => vec![Read, Read],
            LessThan => vec![Read, Read, Write],
            Equals => vec![Read, Read, Write],
            RelativeBaseOffset => vec![Read],
            Halt => vec![],
        }
    }

    /// Should the program counter be automatically incremented after executing this operation?
    /// Returns true or false accordingly, but does no mutation.
    #[must_use]
    pub fn auto_inc_pc(&self) -> bool {
        use Operation::*;

        match self {
            JumpIfTrue => false,
            JumpIfFalse => false,
            Halt => false,
            _ => true,
        }
    }
}

/// Type of parameter: either read or write. This indicates which modes are valid when using this
/// parameter.
///
/// Params that are `read` can be in any mode. Params that are `write` *cannot* be in immediate
/// mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    Read,
    Write,
}

/// Parameter modes are described in Day 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    /// Construct a new ParamMode from a single-bit flag: either 0 or 1.
    pub fn new(mode_flag: i32) -> Self {
        use ParamMode::*;

        match mode_flag {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Invalid mode flag: {}", mode_flag),
        }
    }
}
