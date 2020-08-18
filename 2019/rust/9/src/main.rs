use cpu::misc::read_mem;
use cpu::CPU;

fn main() {
    let mem = read_mem().unwrap();

    day1(mem.clone());
    day2(mem);
}

fn day1(mem: Vec<i64>) {
    CPU::new(mem).input_iter(vec![1].into_iter()).run();
}

fn day2(mem: Vec<i64>) {
    CPU::new(mem).input_iter(vec![2].into_iter()).run();
}
