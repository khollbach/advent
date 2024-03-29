use common::input;
use day_12::{read_input, repeat_time, simulate};

#[test]
fn example1() {
    let moons = read_input(input!("examples/1"));
    assert_eq!(179, simulate(moons.clone(), 10));
    assert_eq!(2772, repeat_time(moons));
}

#[test]
fn example2() {
    let moons = read_input(input!("examples/2"));
    assert_eq!(1940, simulate(moons.clone(), 100));
    assert_eq!(4686774924, repeat_time(moons));
}

#[test]
fn real() {
    let moons = read_input(input!("input"));
    assert_eq!(8625, simulate(moons.clone(), 1000));
    assert_eq!(332477126821644, repeat_time(moons));
}
