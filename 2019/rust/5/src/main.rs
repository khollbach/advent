// 1:50 (110 mins) re-factoring CPU and implementing new features.

use cpu::misc::read_mem;
use cpu::CPU;

fn main() {
    let mem = read_mem().unwrap();

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
