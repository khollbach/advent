use std::convert::TryFrom;

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
    pub fn input_fn(self, get_input: Box<dyn FnMut() -> i32>) -> Self {
        assert!(self.get_input.is_none());
        Self {
            get_input: Some(get_input),
            ..self
        }
    }

    /// Set output function. (Optional.) I/O is described in Day 5.
    pub fn output_fn(self, send_output: Box<dyn FnMut(i32)>) -> Self {
        assert!(self.send_output.is_none());
        Self {
            send_output: Some(send_output),
            ..self
        }
    }

    /// Execute instructions until a HALT. Consumes the CPU.
    pub fn run(mut self) -> i32 {
        if let Some((noun, verb)) = self.args {
            self.mem[1] = noun;
            self.mem[2] = verb;
        }
        self.run_internal();
        self.mem[0]
    }

    /// Execute instructions until a HALT.
    fn run_internal(&mut self) {
        while self.step() {}
    }

    /// Execute one instruction and update the program counter.
    /// Return false only if execution should halt due to a HALT instruction.
    fn step(&mut self) -> bool {
        match self.mem[self.pc] {
            1 => {
                let x = usize::try_from(self.mem[self.pc + 1]).unwrap();
                let y = usize::try_from(self.mem[self.pc + 2]).unwrap();
                let z = usize::try_from(self.mem[self.pc + 3]).unwrap();

                self.mem[z] = self.mem[x] + self.mem[y];
            }
            2 => {
                let x = usize::try_from(self.mem[self.pc + 1]).unwrap();
                let y = usize::try_from(self.mem[self.pc + 2]).unwrap();
                let z = usize::try_from(self.mem[self.pc + 3]).unwrap();

                self.mem[z] = self.mem[x] * self.mem[y];
            }
            3 => {
                let x = usize::try_from(self.mem[self.pc + 1]).unwrap();

                let get_input = self.get_input.as_mut().unwrap();
                self.mem[x] = get_input();
            }
            4 => {
                let x = usize::try_from(self.mem[self.pc + 1]).unwrap();

                let send_output = self.send_output.as_mut().unwrap();
                send_output(self.mem[x]);
            }
            99 => {
                return false;
            }
            other => panic!("Not an opcode: {}", other),
        }

        // Update program counter.
        if self.pc + 4 >= self.mem.len() {
            panic!(
                "Program counter would be out of range: {} vs {}",
                self.pc + 4,
                self.mem.len()
            );
        }
        self.pc += 4;

        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_memory_examples() {
        // Each example is a (before, after) pair.
        let mem_examples = vec![
            (vec![1, 0, 0, 3, 99], vec![1, 0, 0, 2, 99]),
            (
                vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ),
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ];

        for (mem, expected) in mem_examples {
            let mut cpu = CPU::new(mem);
            cpu.run_internal();
            let actual = cpu.mem;
            assert_eq!(expected, actual);
        }
    }
}
