/// A computer emulator that can run Intcode programs.
pub struct CPU {
    /// The current state of memory.
    mem: Vec<i32>,

    /// The instruction pointer (aka "program counter"). An
    /// index into `self.mem`. Invariant: `pc < mem.len()`.
    pc: usize,
}

impl CPU {
    /// Create a new intcode computer. `memory` must be non-empty.
    pub fn new(memory: Vec<i32>) -> Self {
        assert!(!memory.is_empty());
        Self { mem: memory, pc: 0 }
    }

    /// Execute instructions until a HALT. Consumes the CPU.
    /// Arguments and return values are described in Day 2.
    pub fn run(mut self, noun: i32, verb: i32) -> i32 {
        self.mem[1] = noun;
        self.mem[2] = verb;
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
                let x = self.mem[self.pc + 1] as usize;
                let y = self.mem[self.pc + 2] as usize;
                let z = self.mem[self.pc + 3] as usize;

                self.mem[z] = self.mem[x] + self.mem[y];
            }
            2 => {
                let x = self.mem[self.pc + 1] as usize;
                let y = self.mem[self.pc + 2] as usize;
                let z = self.mem[self.pc + 3] as usize;

                self.mem[z] = self.mem[x] * self.mem[y];
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
