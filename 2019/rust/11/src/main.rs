use cpu::{read_mem, CPUBuilder};
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex};

fn main() {
    let mem = read_mem().unwrap();

    println!("{}", num_squares_painted(mem.clone()));

    let map = paint_hull(mem, 1);
    print_grid(&map);
}

fn num_squares_painted(mem: Vec<i64>) -> usize {
    let map = paint_hull(mem, 0);
    map.len()
}

fn paint_hull(mem: Vec<i64>, origin_color: i64) -> HashMap<Point, i64> {
    assert!(origin_color == 0 || origin_color == 1);

    // The painted grid the robot moves around on.
    // position -> color (0: black, 1: white)
    let map = Arc::new(Mutex::new(HashMap::new()));
    map.lock().unwrap().insert(Point::origin(), origin_color);

    let rob = Arc::new(Mutex::new(Robot::new()));

    let mut num_outputs = 0;

    CPUBuilder::new(mem)
        .input(|| {
            // Black by default.
            *map.lock()
                .unwrap()
                .get(&rob.lock().unwrap().pos)
                .unwrap_or(&0)
        })
        .output(|x| {
            assert!(x == 0 || x == 1, "Invalid robot output: {}", x);

            if num_outputs % 2 == 0 {
                // Paint a tile.
                map.lock().unwrap().insert(rob.lock().unwrap().pos, x);
            } else {
                match x {
                    0 => rob.lock().unwrap().turn_left(),
                    1 => rob.lock().unwrap().turn_right(),
                    _ => unreachable!(),
                };
                rob.lock().unwrap().step_forward();
            }
            num_outputs += 1;
        })
        .finish()
        .run();

    let ref_mut = &mut map.lock().unwrap();
    mem::replace(ref_mut, HashMap::new())
}

fn print_grid(map: &HashMap<Point, i64>) {
    let min_x = map.keys().map(|&p| p.x).min().unwrap();
    let max_x = map.keys().map(|&p| p.x).max().unwrap();
    let min_y = map.keys().map(|&p| p.y).min().unwrap();
    let max_y = map.keys().map(|&p| p.y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let col = map.get(&Point { x, y }).unwrap_or(&0);
            let ch = match col {
                0 => ' ',
                1 => '#',
                _ => unreachable!(),
            };
            print!("{}", ch);
        }
        println!();
    }
}

#[derive(Debug)]
struct Robot {
    pos: Point,
    dir: Dir,
}

impl Robot {
    /// Starts at (0, 0), facing north.
    fn new() -> Self {
        Self {
            pos: Point::origin(),
            dir: Dir::N,
        }
    }

    fn step_forward(&mut self) {
        use Dir::*;
        match self.dir {
            N => self.pos.y += 1,
            E => self.pos.x += 1,
            S => self.pos.y -= 1,
            W => self.pos.x -= 1,
        };
    }

    fn turn_left(&mut self) {
        self.dir = self.dir.rotate_counterclockwise();
    }

    fn turn_right(&mut self) {
        self.dir = self.dir.rotate_clockwise();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn rotate_clockwise(self) -> Self {
        use Dir::*;
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }

    fn rotate_counterclockwise(self) -> Self {
        use Dir::*;
        match self {
            N => W,
            E => N,
            S => E,
            W => S,
        }
    }
}
