use regex::Regex;
use std::fmt;
use std::io::{self, prelude::*};

fn main() {
    let points = read_input();
    let moons: Vec<_> = points.into_iter().map(|p| Moon::new(p)).collect();

    println!("{}", simulate(moons, 1000));
}

fn simulate(mut moons: Vec<Moon>, time_steps: usize) -> i64 {
    let n = moons.len();
    assert_eq!(n, 4);

    for _ in 0..time_steps {
        // Apply gravity to each pair of moons, update their velocities.
        for i in 0..n - 1 {
            for j in i + 1..n {
                let (fst, snd) = moons.split_at_mut(j);
                Moon::apply_gravity(&mut fst[i], &mut snd[0]);
            }
        }

        // Apply velocity to each moon, updating its position.
        for m in &mut moons {
            m.apply_vel();
        }
    }

    moons.iter().map(|m| m.total_energy()).sum()
}

struct Moon {
    pos: Point,
    vel: Point,
}

impl Moon {
    fn new(initial_pos: Point) -> Self {
        Self {
            pos: initial_pos,
            vel: Point::origin(),
        }
    }

    fn apply_gravity(a: &mut Self, b: &mut Self) {
        if a.pos.x < b.pos.x {
            a.vel.x += 1;
            b.vel.x -= 1;
        } else if a.pos.x > b.pos.x {
            a.vel.x -= 1;
            b.vel.x += 1;
        }

        if a.pos.y < b.pos.y {
            a.vel.y += 1;
            b.vel.y -= 1;
        } else if a.pos.y > b.pos.y {
            a.vel.y -= 1;
            b.vel.y += 1;
        }

        if a.pos.z < b.pos.z {
            a.vel.z += 1;
            b.vel.z -= 1;
        } else if a.pos.z > b.pos.z {
            a.vel.z -= 1;
            b.vel.z += 1;
        }
    }

    fn apply_vel(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn total_energy(&self) -> i64 {
        let potential = self.pos.energy();
        let kinetic = self.vel.energy();
        potential * kinetic
    }
}

impl fmt::Debug for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={:?}, vel={:?}", self.pos, self.vel)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    /// L-1 norm.
    fn energy(self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}

fn read_input() -> Vec<Point> {
    const I: &str = r"(-?\d+)";
    let re = Regex::new(&format!("^<x={}, y={}, z={}>$", I, I, I)).unwrap();

    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();
            let x: i64 = caps[1].parse().unwrap();
            let y: i64 = caps[2].parse().unwrap();
            let z: i64 = caps[3].parse().unwrap();

            Point { x, y, z }
        })
        .collect()
}
