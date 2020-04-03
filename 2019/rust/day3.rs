// todo: Come back to this after reading more of the Rust book:
//          - iterators
//          - closures
//          - etc.

use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    //let visited: HashSet<Point> = (p in wire1).collect();
    //(p in wire2).filter(visited.contains).min_by_key(Point::l1_norm).unwrap()
}

struct Point(i32, i32);

impl Point {
    fn origin() -> Self {
        Point(0, 0)
    }

    fn unit(direction: Direction) -> Self {
        match direction {
            Direction::L => Point(-1, 0),
            Direction::R => Point(1, 0),
            Direction::U => Point(0, 1),
            Direction::D => Point(0, -1),
        }
    }

    fn add(&mut self, other: Self) {
        // todo: is there cute syntax sugar for this?
        //          - pattern matching / destructuring etc?
        //          - What would happen if I assigned self?
        self.0 += other.0;
        self.1 += other.1;
    }

    fn l1_norm(&self) -> i32 {
        self.l1_distance(&Self::origin())
    }

    fn l1_distance(&self, other: &Self) -> i32 {
        let &Point(x1, y1) = self;
        let &Point(x2, y2) = other;
        (x1 - x2).abs() + (y1 - y2).abs()
    }
}

enum Direction {
    L,
    R,
    U,
    D,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            "U" => Ok(Self::U),
            "D" => Ok(Self::D),
            _ => Err(()),
        }
    }
}

struct WireMove {
    direction: Direction,
    amount: u32,
}

impl WireMove {
    fn new(s: &str) -> Self {
        WireMove {
            direction: s[..1].parse().unwrap(),
            amount: s[1..].parse().unwrap(),
        }
    }
}

// todo: can I make this return an iterable instead of a set?
fn wire_points(wire: String) -> HashSet<Point> {
    let mut pos = Point::origin();
    wire.split(',')
        .map(WireMove::new)
        // todo: do I need to use the move keyword here?
        .map(|WireMove { direction, amount }| {
            for _ in 0..amount {
                pos.add(Point::unit(direction));
                // todo: is there a way to now "yield" pos?
            }
        });
}
