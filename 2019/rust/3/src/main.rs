// 20 minutes thinking of a solution that uses a representation of intervals and gets the space
// complexity down to O(N) where N is the sum of the wire-section lengths, instead of O(n) where n
// is the number of wire sections. The time complexity isn't any better, since we have to check all
// N spots in the second wire, even though we only store the corners of the first wire. So we use
// the first solution, which is much simpler.
//
// 1:00 hour writing and re-writing/re-factoring/re-organizing the read_wire logic.
//
// 1:10 (+10) wrote part 1 (simple set-intersection idea).
//
// 1:30 (+20) wrote part 2 (similar idea, but with maps instead of sets).

use std::collections::HashMap;
use std::io;

fn main() {
    let wire1 = read_wire();
    let wire2 = read_wire();
    println!("{}", part1(&wire1, &wire2));
    println!("{}", part2(&wire1, &wire2));
}

type Wire = HashMap<(i32, i32), u32>;

fn part1(wire1: &Wire, wire2: &Wire) -> i32 {
    let intersections = wire1.keys().filter(|p| wire2.contains_key(p));

    let dist_to_closest = intersections
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
        .expect("No intersections.");

    dist_to_closest
}

fn part2(wire1: &Wire, wire2: &Wire) -> u32 {
    let intersections = wire1.keys().filter(|p| wire2.contains_key(p));

    let min_total_dist = intersections
        .map(|p| wire1[p] + wire2[p])
        .min()
        .expect("No intersections.");

    min_total_dist
}

/// Return the set of points this wire covers, but not including the origin. Each point has the
/// corresponding distance travelled to reach it (for the first time).
fn read_wire() -> Wire {
    let mut map = HashMap::new();

    // Start at the origin.
    let mut x = 0;
    let mut y = 0;
    let mut total_dist = 0;

    let line = read_line().expect("Failed to read from stdin");
    for section in line.split(',') {
        let dir = section.chars().next().expect("Empty wire section");
        let dist: u32 = section[1..].parse().unwrap();

        // Put all points along the wire section in the map.
        match dir {
            'U' => {
                for _ in 0..dist {
                    y += 1;
                    total_dist += 1;
                    map.insert((x, y), total_dist);
                }
            }
            'D' => {
                for _ in 0..dist {
                    y -= 1;
                    total_dist += 1;
                    map.insert((x, y), total_dist);
                }
            }
            'L' => {
                for _ in 0..dist {
                    x -= 1;
                    total_dist += 1;
                    map.insert((x, y), total_dist);
                }
            }
            'R' => {
                for _ in 0..dist {
                    x += 1;
                    total_dist += 1;
                    map.insert((x, y), total_dist);
                }
            }
            c => panic!("Invalid direction: {}", c),
        }
    }

    map
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
