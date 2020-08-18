use cpu::misc::read_mem;
use cpu::CPU;

fn main() {
    let mem = read_mem().unwrap();

    part1(mem.clone());
    part2(mem);
}

fn part1(mem: Vec<i64>) {
    CPU::new(mem).input_iter(vec![1].into_iter()).run();
}

fn part2(mem: Vec<i64>) {
    CPU::new(mem).input_iter(vec![2].into_iter()).run();
}
