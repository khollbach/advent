use builder::CPUBuilder;
use instruction::{Instruction, Operation, ParamMode, ParamType};
use std::convert::TryFrom;

mod builder;
mod instruction;
pub mod misc;

/// A computer emulator that can run Intcode programs.
pub struct CPU {
    /// The current state of memory.
    mem: Vec<i32>,

    /// The instruction pointer (aka "program counter"). An
    /// index into `self.mem`. Invariant: `pc < mem.len()`.
    pc: usize,

    /// I/O mechanism.
    get_input: Box<dyn FnMut() -> i32>,
    send_output: Box<dyn FnMut(i32)>,
}

impl CPU {
    /// Create a new intcode computer. `memory` must be non-empty.
    /// This follows the builder pattern; see CPUBuilder for more.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(memory: Vec<i32>) -> CPUBuilder {
        CPUBuilder::new(memory)
    }

    /// Execute instructions until a Halt. Return the final value at memory address 0.
    /// Consumes the CPU.
    pub fn run(mut self) -> i32 {
        self.run_internal();

        self.mem[0]
    }

    /// Execute instructions until a Halt.
    fn run_internal(&mut self) {
        while self.step() {}
    }

    /// Execute one instruction and update the program counter.
    /// Return false only if execution should halt due to a Halt instruction.
    fn step(&mut self) -> bool {
        use Operation::*;

        let instr = Instruction::new(self.mem[self.pc]);
        let args = self.get_args(&instr);

        match instr.op {
            Add => {
                self.mem[addr(args[2])] = args[0] + args[1];
            }
            Multiply => {
                self.mem[addr(args[2])] = args[0] * args[1];
            }
            GetInput => {
                self.mem[addr(args[0])] = (self.get_input)();
            }
            SendOutput => {
                (self.send_output)(args[0]);
            }
            JumpIfTrue => {
                if args[0] != 0 {
                    self.pc = addr(args[1]);
                } else {
                    self.update_pc(&instr);
                }
            }
            JumpIfFalse => {
                if args[0] == 0 {
                    self.pc = addr(args[1]);
                } else {
                    self.update_pc(&instr);
                }
            }
            LessThan => {
                let b = if args[0] < args[1] { 1 } else { 0 };
                self.mem[addr(args[2])] = b;
            }
            Equals => {
                let b = if args[0] == args[1] { 1 } else { 0 };
                self.mem[addr(args[2])] = b;
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
    /// Returns a memory address for Write parameters; otherwise a value.
    fn get_args(&mut self, instr: &Instruction) -> Vec<i32> {
        use ParamMode::*;
        use ParamType::*;

        (1..)
            .zip(instr.params.iter())
            .map(|(offset, &(typ, mode))| match (typ, mode) {
                (Read, Position) => {
                    let addr = addr(self.mem[self.pc + offset]);
                    self.mem[addr]
                }
                (Read, Immediate) | (Write, Position) => self.mem[self.pc + offset],
                (Write, Immediate) => unreachable!(),
            })
            .collect()
    }

    /// Increment program counter according to the number of arguments of instr.
    fn update_pc(&mut self, instr: &Instruction) {
        // 1 for the opcode, plus each of the args.
        let offset = 1 + instr.params.len();

        let new_pc = self.pc + offset;
        assert!(
            new_pc < self.mem.len(),
            "Program counter would be out of range"
        );

        self.pc = new_pc;
    }
}

/// Perform checked conversion from i32 to usize. Panics if negative or overflow.
fn addr(mem_addr: i32) -> usize {
    usize::try_from(mem_addr).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mem {
        use super::*;

        /// Verify that the memory after running the program is as expected.
        /// `before` is the initial memory / program state.
        fn test_mem(before: Vec<i32>, after: Vec<i32>) {
            let mut cpu = CPU::new(before).finish();
            cpu.run_internal();

            let actual = cpu.mem;
            assert_eq!(after, actual);
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

    #[cfg(test)]
    mod io {
        use std::cell::RefCell;
        use std::rc::Rc;

        use super::*;

        fn test_io(mem: Vec<i32>, input: Vec<i32>, expected_output: Vec<i32>) {
            let actual_output = Rc::new(RefCell::new(vec![]));

            let mut cpu = CPU::new(mem)
                .input_iter(input.into_iter())
                .output_vec(&actual_output)
                .finish();
            cpu.run_internal();

            assert_eq!(expected_output, *actual_output.borrow());
        }

        fn test_many(mem: Vec<i32>, io_pairs: Vec<(Vec<i32>, Vec<i32>)>) {
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
    }
}
