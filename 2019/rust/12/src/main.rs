use day_12::{read_input, repeat_time, simulate};
use std::io;

fn main() {
    let moons = read_input(io::stdin().lock());
    println!("{}", simulate(moons.clone(), 1000));
    println!("{}", repeat_time(moons));
}
