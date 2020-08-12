use cpu::misc::read_mem;
use cpu::CPU;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mem = read_mem().unwrap();

    println!("{}", day1(&mem));
}

/// Compute the maximum thruster signal possible, over all permutations of phase settings.
fn day1(program: &[i32]) -> i32 {
    permutations(&[0, 1, 2, 3, 4])
        .into_iter()
        .map(|p| thruster_signal(&program, &p))
        .max()
        .unwrap()
}

/// Run the 5 amps in sequence and return the output amplitude for these
/// phase settings.
fn thruster_signal(program: &[i32], phase_settings: &[i32]) -> i32 {
    let mut signal = 0;

    for &phase in phase_settings {
        let output = Rc::new(RefCell::new(vec![]));

        CPU::new(Vec::from(program))
            .input_iter(vec![phase, signal].into_iter())
            .output_vec(&output)
            .run();

        assert!(output.borrow().len() == 1);
        signal = output.borrow_mut()[0];
    }

    signal
}

/// This would ideally be lazy (i.e. return an iterator instead of a vector).
fn permutations(values: &[i32]) -> Vec<Vec<i32>> {
    let n = values.len();
    if n == 0 {
        return vec![vec![]];
    }

    let mut perms = vec![];

    for p in permutations(&values[1..]) {
        assert!(p.len() == n - 1);

        for i in 0..=n - 1 {
            let mut perm = vec![];

            perm.extend(&p[..i]);
            perm.push(values[0]);
            perm.extend(&p[i..]);

            perms.push(perm);
        }
    }

    perms
}
