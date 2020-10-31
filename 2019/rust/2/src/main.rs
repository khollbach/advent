use cpu::{read_mem, CPUBuilder};

fn main() {
    let mem = read_mem().unwrap();

    println!("{}", part1(mem.clone()));

    let (noun, verb) = part2(mem);
    println!("{}", noun * 100 + verb);
}

fn part1(mem: Vec<i64>) -> i64 {
    CPUBuilder::new(mem).args(12, 2).default_io().run()
}

fn part2(mem: Vec<i64>) -> (i64, i64) {
    const TARGET: i64 = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let cpu = CPUBuilder::new(mem.clone()).args(noun, verb).default_io();
            if cpu.run() == TARGET {
                return (noun, verb);
            }
        }
    }

    panic!("No valid noun/verb pair found.");
}
