use cpu::{read_mem, CPU};

fn main() {
    let mem = read_mem().unwrap();

    part1(mem.clone());
    part2(mem);
}

fn part1(mem: Vec<i64>) {
    let mut inputs = vec![1];

    CPU::new(mem)
        .input(move || inputs.pop().unwrap())
        .output(|x| {
            if x != 0 {
                println!("{}", x);
            };
        })
        .run();
}

fn part2(mem: Vec<i64>) {
    let mut inputs = vec![5];

    CPU::new(mem)
        .input(move || inputs.pop().unwrap())
        .output(|x| {
            println!("{}", x);
        })
        .run();
}
