#![feature(or_patterns)]

use instruction::{Instruction, Operation, ParamMode, ParamType};
use memory::Memory;
use std::thread;
use std::thread::JoinHandle;

mod builder;
mod instruction;
mod memory;
mod misc;

pub use builder::CPUBuilder;
pub use misc::{parse_mem, read_mem};

/// A computer emulator that can run Intcode programs.
///
/// ```
/// use cpu::CPU;
///
/// assert_eq!(2, CPU::new(vec![1, 0, 0, 0, 99]).run());
/// ```
pub struct CPU<I, O> {
    /// The current state of memory.
    mem: Memory,

    /// The instruction pointer (aka "program counter"). An
    /// index into `self.mem`. Invariant: `0 <= pc < mem.len()`.
    pc: i64,

    /// The relative base, used as a memory address offset for operations with parameters in
    /// relative-mode. See Day 9 for more.
    relative_base: i64,

    /// I/O mechanism. I/O is described in Day 5.
    ///
    /// If an IO function returns None, we halt. This is the way to signal that we should halt from
    /// another thread.
    get_input: I,
    send_output: O,
}

impl CPU<fn() -> Option<i64>, fn(i64) -> Option<()>> {
    /// Create a new intcode computer with the given starting memory.
    ///
    /// For more fine-grained control, see CPUBuilder.
    pub fn new(initial_memory: Vec<i64>) -> Self {
        CPUBuilder::new(initial_memory).default_io()
    }
}

impl<I, O> CPU<I, O>
where
    I: 'static + Send + FnMut() -> Option<i64>,
    O: 'static + Send + FnMut(i64) -> Option<()>,
{
    /// Spawn a thread running this CPU. The thread's return value is the CPU's return value.
    ///
    /// Especially useful if combined with channels for IO. See e.g. Day 15.
    pub fn run_async(self) -> JoinHandle<i64> {
        thread::spawn(|| self.run())
    }
}

impl<I, O> CPU<I, O>
where
    I: FnMut() -> Option<i64>,
    O: FnMut(i64) -> Option<()>,
{
    /// Execute instructions until a Halt. Return the final value at memory address 0.
    ///
    /// Consumes the CPU.
    pub fn run(mut self) -> i64 {
        self.run_internal();
        self.mem.get(0)
    }

    /// Execute instructions until a Halt instruction, or until an IO function returns None.
    ///
    /// This helper function is handy for unit tests, since it doesn't consume the CPU. That way we
    /// can inspect memory afterwords.
    fn run_internal(&mut self) {
        while self.step() {}
    }

    /// Execute one instruction and update the program counter.
    ///
    /// Return false if execution should halt due to a Halt instruction, or due to an IO function
    /// returning None.
    fn step(&mut self) -> bool {
        use Operation::*;

        let instr = Instruction::new(self.mem.get(self.pc));
        let args = self.get_args(&instr);

        match instr.op {
            Add => {
                self.mem.set(args[2], args[0] + args[1]);
            }
            Multiply => {
                self.mem.set(args[2], args[0] * args[1]);
            }
            GetInput => match (self.get_input)() {
                Some(input) => {
                    self.mem.set(args[0], input);
                }
                None => return false,
            },
            SendOutput => match (self.send_output)(args[0]) {
                Some(()) => (),
                None => return false,
            },
            JumpIfTrue => {
                if args[0] != 0 {
                    self.pc = args[1];
                } else {
                    self.update_pc(&instr);
                }
            }
            JumpIfFalse => {
                if args[0] == 0 {
                    self.pc = args[1];
                } else {
                    self.update_pc(&instr);
                }
            }
            LessThan => {
                let b = if args[0] < args[1] { 1 } else { 0 };
                self.mem.set(args[2], b);
            }
            Equals => {
                let b = if args[0] == args[1] { 1 } else { 0 };
                self.mem.set(args[2], b);
            }
            RelativeBaseOffset => {
                self.relative_base += args[0];
            }
            Halt => {
                return false;
            }
        }

        if instr.op.auto_inc_pc() {
            self.update_pc(&instr);
        }

        true
    }

    /// Get arguments for instr. Arguments and "modes" are described in Day 5.
    /// Relative parameter mode was introduced in Day 9.
    fn get_args(&self, instr: &Instruction) -> Vec<i64> {
        instr
            .params
            .iter()
            .zip(1..)
            .map(|(&(typ, mode), offset)| {
                let mut param = self.mem.get(self.pc + offset);

                // Adjust for relative base.
                if mode == ParamMode::Relative {
                    param += self.relative_base;
                }

                self.deref_param(param, typ, mode)
            })
            .collect()
    }

    /// Helper for `get_args`. Optionally dereference `param` depending on the parameter-mode.
    ///
    /// Doesn't account for relative base, that happens elsewhere.
    fn deref_param(&self, param: i64, typ: ParamType, mode: ParamMode) -> i64 {
        use ParamMode::*;
        use ParamType::*;

        match (typ, mode) {
            (Write, Position | Relative) | (Read, Immediate) => {
                // Writes and immediate reads don't derefence;
                // they return the parameter value directly.
                param
            }
            (Read, Position | Relative) => self.mem.get(param),
            (Write, Immediate) => panic!("Write parameters cannot be in Immediate mode"),
        }
    }

    /// Increment program counter according to the number of arguments of instr.
    fn update_pc(&mut self, instr: &Instruction) {
        // 1 for the opcode, plus each of the args.
        let offset = 1 + instr.params.len() as i64;

        self.pc += offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mem {
        use super::*;

        /// Verify that the memory after running the program is as expected.
        /// `before` is the initial memory / program state.
        fn test_mem(before: Vec<i64>, after: Vec<i64>) {
            let mut cpu = CPU::new(before);
            cpu.run_internal();
            assert_eq!(after, cpu.mem._into_vec());
        }

        #[test]
        fn test1() {
            test_mem(vec![1, 0, 0, 3, 99], vec![1, 0, 0, 2, 99]);
        }

        #[test]
        fn test2() {
            test_mem(
                vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            );
        }

        #[test]
        fn test3() {
            test_mem(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        }

        #[test]
        fn test4() {
            test_mem(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
        }

        #[test]
        fn test5() {
            test_mem(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
        }

        #[test]
        fn test6() {
            test_mem(
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            );
        }

        /// This example uses parameter modes.
        #[test]
        fn test7() {
            test_mem(vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99]);
        }
    }

    mod io {
        use super::*;
        use std::cell::RefCell;
        use std::rc::Rc;

        /// Run the CPU and check the actual output against what's expected.
        fn test_io(mem: Vec<i64>, input: Vec<i64>, expected_output: Vec<i64>) {
            let actual_output = Rc::new(RefCell::new(vec![]));

            let mut cpu = CPUBuilder::new(mem)
                .input_iter(input.into_iter())
                .output_vec(Rc::clone(&actual_output))
                .finish();

            cpu.run_internal();

            assert_eq!(expected_output, *actual_output.borrow());
        }

        /// Test many io examples for the same CPU.
        fn test_many(mem: Vec<i64>, io_pairs: Vec<(Vec<i64>, Vec<i64>)>) {
            for (input, output) in io_pairs {
                test_io(mem.clone(), input, output);
            }
        }

        /// Produces same output as input (a.k.a. "echo").
        #[test]
        fn test1() {
            let mem = vec![3, 0, 4, 0, 99];
            let io_pairs = vec![
                (vec![1], vec![1]),
                (vec![1337], vec![1337]),
                (vec![0], vec![0]),
                (vec![-1], vec![-1]),
                (vec![-1001], vec![-1001]),
            ];

            test_many(mem, io_pairs);
        }

        /// Check if input equals 8 (output 1 or 0 accordingly).
        #[test]
        fn test2() {
            let mem = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
            let io_pairs = vec![
                (vec![8], vec![1]),
                (vec![7], vec![0]),
                (vec![0], vec![0]),
                (vec![-3], vec![0]),
                (vec![100], vec![0]),
            ];

            test_many(mem, io_pairs);
        }

        /// Same as test2.
        #[test]
        fn test3() {
            let mem = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
            let io_pairs = vec![
                (vec![8], vec![1]),
                (vec![7], vec![0]),
                (vec![0], vec![0]),
                (vec![-3], vec![0]),
                (vec![100], vec![0]),
            ];

            test_many(mem, io_pairs);
        }

        /// Check if input less than 8 (output 1 or 0 accordingly).
        #[test]
        fn test4() {
            let mem = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
            let io_pairs = vec![
                (vec![8], vec![0]),
                (vec![7], vec![1]),
                (vec![0], vec![1]),
                (vec![-3], vec![1]),
                (vec![100], vec![0]),
            ];

            test_many(mem, io_pairs);
        }

        /// Same as test4.
        #[test]
        fn test5() {
            let mem = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
            let io_pairs = vec![
                (vec![8], vec![0]),
                (vec![7], vec![1]),
                (vec![0], vec![1]),
                (vec![-3], vec![1]),
                (vec![100], vec![0]),
            ];

            test_many(mem, io_pairs);
        }

        /// Check if input is nonzero.
        #[test]
        fn test6() {
            let mem = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
            let io_pairs = vec![
                (vec![8], vec![1]),
                (vec![7], vec![1]),
                (vec![0], vec![0]),
                (vec![-3], vec![1]),
                (vec![100], vec![1]),
            ];

            test_many(mem, io_pairs);
        }

        /// Same as test6.
        #[test]
        fn test7() {
            let mem = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
            let io_pairs = vec![
                (vec![8], vec![1]),
                (vec![7], vec![1]),
                (vec![0], vec![0]),
                (vec![-3], vec![1]),
                (vec![100], vec![1]),
            ];

            test_many(mem, io_pairs);
        }

        /// Output 999/1000/1001 if input is lt/eq/gt the value 8.
        #[test]
        fn test8() {
            let mem = vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ];
            let io_pairs = vec![
                (vec![8], vec![1000]),
                (vec![7], vec![999]),
                (vec![0], vec![999]),
                (vec![-3], vec![999]),
                (vec![100], vec![1001]),
            ];

            test_many(mem, io_pairs);
        }

        /// Quine; takes no input and produces a copy of itself as output.
        #[test]
        fn test9() {
            let mem = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];

            test_io(mem.clone(), vec![], mem);
        }

        /// Outputs a 16-digit number.
        #[test]
        fn test10() {
            let mem = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

            test_io(mem, vec![], vec![1219070632396864]);
        }

        /// Outputs the large number in the middle.
        #[test]
        fn test11() {
            const TARGET: i64 = 1125899906842624;
            let mem = vec![104, TARGET, 99];

            test_io(mem, vec![], vec![TARGET]);
        }
    }
}
