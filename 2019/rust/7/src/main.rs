use cpu::misc::read_mem;
use cpu::CPU;
use factorial::Factorial;
use rayon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn main() {
    let mem = read_mem().unwrap();

    println!("{}", day1(&mem));
    println!("{}", day2(&mem));
}

/// Amplifiers connected in series.
fn day1(program: &[i32]) -> i32 {
    best_signal(program, &[0, 1, 2, 3, 4], thruster_signal_1)
}

/// Connected in a feedback loop.
fn day2(program: &[i32]) -> i32 {
    best_signal(program, &[5, 6, 7, 8, 9], thruster_signal_2)
}

/// Compute the maximum thruster signal possible, over all permutations of phase settings.
fn best_signal<F>(program: &[i32], phase_values: &[i32], thruster_signal: F) -> i32
where
    F: Fn(&[i32], &[i32]) -> i32 + Sync,
{
    permutations(phase_values)
        .par_iter()
        .map(|p| thruster_signal(&program, p))
        .max()
        .unwrap()
}

/// This would ideally be lazy (i.e. return an iterator instead of a vector). Not a big deal
/// though. If rust had a feature like python's generator syntax, this would be trivial...
fn permutations(values: &[i32]) -> Vec<Vec<i32>> {
    let n = values.len();
    if n == 0 {
        return vec![vec![]];
    }

    let mut perms = Vec::with_capacity(n.factorial());

    for p in permutations(&values[1..]) {
        assert!(p.len() == n - 1);

        for i in 0..=p.len() {
            let mut perm = Vec::with_capacity(n);

            perm.extend(&p[..i]);
            perm.push(values[0]);
            perm.extend(&p[i..]);

            perms.push(perm);
        }
    }

    perms
}

/// Run the 5 amps in sequence and return the output amplitude for these phase settings.
fn thruster_signal_1(program: &[i32], phase_settings: &[i32]) -> i32 {
    let mut signal = 0;

    for &phase in phase_settings {
        let output = Rc::new(RefCell::new(vec![]));

        CPU::new(Vec::from(program))
            .input_iter(vec![phase, signal].into_iter())
            .output_vec(&output)
            .run();

        assert!(output.borrow().len() == 1);
        signal = output.borrow()[0];
    }

    signal
}

/// Connect the 5 amps in a feedback loop and return the output amplitude for these phase settings.
fn thruster_signal_2(program: &[i32], phase_settings: &[i32]) -> i32 {
    let n = phase_settings.len();

    let chans: Vec<_> = (0..n)
        .map(|_| channel())
        .map(|(sx, rx)| (Arc::new(Mutex::new(sx)), Arc::new(Mutex::new(rx))))
        .collect();

    // Send initial input value of 0, to the first amp.
    let (sx, _) = &chans[0];
    sx.lock().unwrap().send(0).unwrap();

    let mut threads = Vec::with_capacity(n);
    for (i, &phase) in phase_settings.iter().enumerate() {
        // Receive from "my" channel, send to the "next guy's" channel.
        let (_, rx) = &chans[i];
        let (sx, _) = &chans[(i + 1) % n];

        threads.push(run_amp(Vec::from(program), phase, rx.clone(), sx.clone()));
    }

    for t in threads {
        t.join().unwrap();
    }

    // Get the final output value, from the last amp.
    let (_, rx) = &chans[0];
    #[allow(clippy::let_and_return)]
    let val = rx.lock().unwrap().recv().unwrap();
    val
}

fn run_amp(
    mem: Vec<i32>,
    phase: i32,
    input: Arc<Mutex<Receiver<i32>>>,
    output: Arc<Mutex<Sender<i32>>>,
) -> JoinHandle<()> {
    let mut first_input = true;

    thread::spawn(move || {
        CPU::new(mem)
            .input_fn(move || {
                if first_input {
                    // First input is the phase setting.
                    first_input = false;
                    phase
                } else {
                    input.lock().unwrap().recv().unwrap()
                }
            })
            .output_fn(move |x| {
                output.lock().unwrap().send(x).unwrap();
            })
            .run();
    })
}