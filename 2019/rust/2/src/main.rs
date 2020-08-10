use cpu::CPU;
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
