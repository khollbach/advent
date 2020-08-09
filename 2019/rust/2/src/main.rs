use std::error::Error;
use std::io;

fn main() {
    let mem = read_input().unwrap();

    println!("{}", part1(mem.clone()));

    let (noun, verb) = part2(mem);
    println!("{}", noun * 100 + verb);
}

fn part1(mem: Vec<i32>) -> i32 {
    CPU::new(mem).run(12, 2)
}

fn part2(mem: Vec<i32>) -> (i32, i32) {
    const TARGET: i32 = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            if CPU::new(mem.clone()).run(noun, verb) == TARGET {
                return (noun, verb);
            }
        }
    }

    panic!("No valid noun/verb pair found.");
}

/// Read the first line of stdin, and parse it as a comma-separated
/// list of integers: e.g. `1,2,-54,0`.
fn read_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let mut vec = vec![];
    for word in line.trim_end_matches('\n').split(',') {
        vec.push(word.parse()?);
    }
    Ok(vec)
}

/// A computer emulator that can run Intcode programs.
struct CPU {
    /// The current state of memory.
    mem: Vec<i32>,

    /// The instruction pointer (aka "program counter"). An
    /// index into `self.mem`. Invariant: `pc < mem.len()`.
    pc: usize,
}

impl CPU {
    /// Create a new intcode computer. `memory` must be non-empty.
    fn new(memory: Vec<i32>) -> Self {
        assert!(!memory.is_empty());
        Self { mem: memory, pc: 0 }
    }

    /// Execute instructions until a HALT. Consumes the CPU.
    /// Arguments and return values are described in Day 2.
    fn run(mut self, noun: i32, verb: i32) -> i32 {
        self.mem[1] = noun;
        self.mem[2] = verb;
        while self.step() {}
        self.mem[0]
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
