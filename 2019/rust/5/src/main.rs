use cpu::misc::read_mem;
use cpu::CPU;

fn main() {
    let mem = read_mem().unwrap();

    part1(mem.clone());
    part2(mem);
}

fn part1(mem: Vec<i32>) {
    let mut inputs = vec![1];

    CPU::new(mem)
        .input_fn(move || inputs.pop().unwrap())
        .output_fn(|x| {
            if x != 0 {
                println!("{}", x);
            };
        })
        .run();
}

fn part2(mem: Vec<i32>) {
    let mut inputs = vec![5];

    CPU::new(mem)
        .input_fn(move || inputs.pop().unwrap())
        .output_fn(|x| {
            println!("{}", x);
        })
        .run();
}
