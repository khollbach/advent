// Part 1: 25 minutes.

use common::read_line;

fn main() {
    let (lo, hi) = read_input();

    let num_valid = (lo..hi).filter(|&p| is_valid(p)).count();
    println!("{}", num_valid);
}

/// Check the number, as a string of digits, is:
/// - non-decreasing (e.g. 12345), and
/// - contains a consecutive duplicate (e.g. 123345).
fn is_valid(password: u32) -> bool {
    let mut prev_digit = '0';
    let mut has_dup = false;
    for c in password.to_string().chars() {
        if c < prev_digit {
            return false;
        }

        if c == prev_digit {
            has_dup = true;
        }

        prev_digit = c;
    }

    has_dup
}

/// It won't matter, but assume the upper-bound is exclusive.
fn read_input() -> (u32, u32) {
    let line = read_line().unwrap();
    let mut words = line.split('-');

    let lo = words.next().unwrap().parse().unwrap();
    let hi = words.next().unwrap().parse().unwrap();
    (lo, hi)
}
