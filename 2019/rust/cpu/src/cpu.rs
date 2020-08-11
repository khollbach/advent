use instruction::{Instruction, Operation, ParamMode, ParamType};
use std::convert::TryFrom;

mod instruction;

/// A computer emulator that can run Intcode programs.
#[derive(Default)]
pub struct CPU {
    /// The current state of memory.
    mem: Vec<i32>,

    /// The instruction pointer (aka "program counter"). An
    /// index into `self.mem`. Invariant: `pc < mem.len()`.
    pc: usize,

    /// Program arguments.
    args: Option<(i32, i32)>,

    /// I/O mechanism.
    get_input: Option<Box<dyn FnMut() -> i32>>,
    send_output: Option<Box<dyn FnMut(i32)>>,
}

impl CPU {
    /// Create a new intcode computer. `memory` must be non-empty. This follows the builder
    /// pattern; you can supply optional parameters via their corresponding methods.
    pub fn new(memory: Vec<i32>) -> Self {
        assert!(!memory.is_empty());
        Self {
            mem: memory,
            pc: 0,
            ..Default::default()
        }
    }

    /// Set the arguments to execute with. (Optional!)
    /// Arguments and return values are described in Day 2.
    pub fn args(self, noun: i32, verb: i32) -> Self {
        assert!(self.args.is_none());
        Self {
            args: Some((noun, verb)),
            ..self
        }
    }

    /// Set input function. (Optional.) I/O is described in Day 5.
    pub fn input_fn<F: 'static>(self, get_input: F) -> Self
    where
        F: FnMut() -> i32,
    {
        assert!(self.get_input.is_none());
        Self {
            get_input: Some(Box::new(get_input)),
            ..self
        }
    }

    /// Set output function. (Optional.) I/O is described in Day 5.
    pub fn output_fn<F: 'static>(self, send_output: F) -> Self
    where
        F: FnMut(i32),
    {
        assert!(self.send_output.is_none());
        Self {
            send_output: Some(Box::new(send_output)),
            ..self
        }
    }

    /// Execute instructions until a Halt. Consumes the CPU.
    pub fn run(mut self) -> i32 {
        if let Some((noun, verb)) = self.args {
            self.mem[1] = noun;
            self.mem[2] = verb;
        }
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
                self.mem[args[2] as usize] = args[0] + args[1];
            }
            Multiply => {
                self.mem[args[2] as usize] = args[0] * args[1];
            }
            GetInput => {
                let get_input = self.get_input.as_mut().unwrap();
                self.mem[args[0] as usize] = get_input();
            }
            SendOutput => {
                let send_output = self.send_output.as_mut().unwrap();
                send_output(args[0]);
            }
            Halt => {
                return false;
            }
        }

        self.update_pc(instr);

        return true;
    }

    /// Get arguments for instr. Arguments and "modes" are described in Day 5.
    /// Returns a memory address for Write parameters; otherwise a value.
    /// Panics if any address is negative.
    fn get_args(&mut self, instr: &Instruction) -> Vec<i32> {
        use ParamMode::*;
        use ParamType::*;

        (1..)
            .zip(instr.params.iter())
            .map(|(offset, &(typ, mode))| match (typ, mode) {
                (Read, Position) => {
                    let addr = self.mem[self.pc + offset];
                    let addr = usize::try_from(addr).unwrap();
                    self.mem[addr]
                }
                (Read, Immediate) | (Write, Position) => {
                    let val = self.mem[self.pc + offset];
                    if typ == Write {
                        let _addr = usize::try_from(val).unwrap();
                    }
                    val
                }
                (Write, Immediate) => unreachable!(),
            })
            .collect()
    }

    /// Update program counter according to the number of arguments of instr.
    fn update_pc(&mut self, instr: Instruction) {
        if instr.op == Operation::Halt {
            return;
        }

        // 1 for the opcode, plus the args.
        let offset = 1 + instr.params.len();

        let new_pc = self.pc + offset;
        if new_pc >= self.mem.len() {
            panic!(
                "Program counter would be out of range: {} vs {}",
                new_pc,
                self.mem.len()
            );
        }

        self.pc = new_pc;
    }
}

#[cfg(test)]
mod mem_tests {
    use super::*;

    /// Verify that the memory after running the program is as expected.
    /// `before` is the initial memory / program state.
    fn test_mem(before: Vec<i32>, after: Vec<i32>) {
        let mut cpu = CPU::new(before);
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
mod io_tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    fn test_io(mem: Vec<i32>, input: Vec<i32>, expected_output: Vec<i32>) {
        let mut input = input.into_iter();

        let actual_output = Rc::new(RefCell::new(vec![]));
        let output_clone = Rc::clone(&actual_output);

        let mut cpu = CPU::new(mem)
            .input_fn(move || input.next().unwrap())
            .output_fn(move |x| output_clone.borrow_mut().push(x));
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
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
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
