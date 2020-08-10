use cpu::misc::read_mem;
use cpu::CPU;

fn main() {
    let mem = read_mem().unwrap();

    println!("{}", part1(mem.clone()));

    let (noun, verb) = part2(mem);
    println!("{}", noun * 100 + verb);
}

fn part1(mem: Vec<i32>) -> i32 {
    CPU::new(mem).args(12, 2).run()
}

fn part2(mem: Vec<i32>) -> (i32, i32) {
    const TARGET: i32 = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            if CPU::new(mem.clone()).args(noun, verb).run() == TARGET {
                return (noun, verb);
            }
        }
    }

    panic!("No valid noun/verb pair found.");
}
