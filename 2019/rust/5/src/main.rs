use cpu::{read_mem, CPUBuilder};

fn main() {
    let mem = read_mem().unwrap();

    part1(mem.clone());
    part2(mem);
}

fn part1(mem: Vec<i64>) {
    CPUBuilder::new(mem)
        .input_iter(vec![1].into_iter())
        .output(|x| {
            if x != 0 {
                println!("{}", x);
            };
        })
        .finish()
        .run();
}

fn part2(mem: Vec<i64>) {
    CPUBuilder::new(mem)
        .input_iter(vec![5].into_iter())
        .default_out()
        .finish()
        .run();
}
