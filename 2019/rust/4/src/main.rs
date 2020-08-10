// Part 1: 25 minutes.
// Part 2: 35 minutes.

use common::read_line;

fn main() {
    let (lo, hi) = read_input();
    println!("{}", part1(lo, hi));
    println!("{}", part2(lo, hi));
}

fn part1(lo: u32, hi: u32) -> usize {
    (lo..hi).filter(|&p| is_valid(p)).count()
}

fn part2(lo: u32, hi: u32) -> usize {
    (lo..hi).filter(|&p| is_valid_2(p)).count()
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

/// Check the number, as a string of digits, is:
/// - non-decreasing (e.g. 12345), and
/// - contains a consecutive duplicate (e.g. 123345) which is
///     not a triplicate or higher (not 133345).
fn is_valid_2(password: u32) -> bool {
    let mut has_dup = false;

    let mut prev_digit = '0';
    let mut consecutive = 1;
    for c in password.to_string().chars() {
        if c < prev_digit {
            return false;
        }

        if c == prev_digit {
            consecutive += 1;
        } else {
            if consecutive == 2 {
                has_dup = true;
            }
            consecutive = 1;
        }

        prev_digit = c;
    }

    if consecutive == 2 {
        has_dup = true;
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
