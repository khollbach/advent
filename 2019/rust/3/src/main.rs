// 20 minutes thinking of a solution that uses a representation of intervals and gets the space
// complexity down to O(N) where N is the sum of the wire-section lengths, instead of O(n) where n
// is the number of wire sections. The time complexity isn't any better, since we have to check all
// N spots in the second wire, even though we only store the corners of the first wire. So we use
// the first solution, which is much simpler.
//
// 1 hour writing and re-writing/re-factoring/re-organizing the read_wire logic.
//
// 10 minutes writing part 1 (simple set-intersection idea).

use std::collections::HashSet;
use std::io;

fn main() {
    let wire1 = read_wire();
    let wire2 = read_wire();

    let intersections = wire1.into_iter().filter(|p| wire2.contains(&p));

    let dist_to_closest = intersections
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .expect("No intersections.");

    println!("{}", dist_to_closest);
}

/// Return the set of points this wire covers, but not including the origin.
fn read_wire() -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();

    // Start at the origin.
    let mut x = 0;
    let mut y = 0;

    let line = read_line().expect("Failed to read from stdin");
    for section in line.split(',') {
        let dir = section.chars().next().expect("Empty wire section");
        let dist: u32 = section[1..].parse().unwrap();

        // Put all points along the wire section in the set.
        match dir {
            'U' => {
                for _ in 0..dist {
                    y += 1;
                    set.insert((x, y));
                }
            }
            'D' => {
                for _ in 0..dist {
                    y -= 1;
                    set.insert((x, y));
                }
            }
            'L' => {
                for _ in 0..dist {
                    x -= 1;
                    set.insert((x, y));
                }
            }
            'R' => {
                for _ in 0..dist {
                    x += 1;
                    set.insert((x, y));
                }
            }
            c => panic!("Invalid direction: {}", c),
        }
    }

    set
}

/// Read a line from stdin. Strip any trailing newline.
fn read_line() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    // Strip newline.
    if buf.chars().last() == Some('\n') {
        buf.pop();
    }

    Ok(buf)
}
