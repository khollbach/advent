use num::integer;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::io::prelude::*;

pub fn read_input<R: BufRead>(input: R) -> Vec<Moon> {
    const I: &str = r"(-?\d+)";
    let re = Regex::new(&format!("^<x={}, y={}, z={}>$", I, I, I)).unwrap();

    let moons: Vec<_> = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();
            let x: i64 = caps[1].parse().unwrap();
            let y: i64 = caps[2].parse().unwrap();
            let z: i64 = caps[3].parse().unwrap();

            Moon::new(Point { x, y, z })
        })
        .collect();

    assert_eq!(moons.len(), 4);
    moons
}

/// Simulate the system for `time_steps` rounds and the output the total energy at the end.
pub fn simulate(mut moons: Vec<Moon>, time_steps: usize) -> i64 {
    assert_eq!(moons.len(), 4);

    for _ in 0..time_steps {
        time_step(&mut moons);
    }

    moons.iter().map(|m| m.total_energy()).sum()
}

/// Simulate the system for 1 time step.
fn time_step(moons: &mut Vec<Moon>) {
    let n = moons.len();

    // Apply gravity to each pair of moons, update their velocities.
    for i in 0..n - 1 {
        for j in i + 1..n {
            let (fst, snd) = moons.split_at_mut(j);
            Moon::apply_gravity(&mut fst[i], &mut snd[0]);
        }
    }

    // Apply velocity to each moon, updating its position.
    for m in moons {
        m.apply_vel();
    }
}

/// How many steps does it take to return to a previous state? Note that the first repeated state
/// is always the initial state, since the state transition function is one-to-one. (You can check
/// this yourself; I won't prove it here.)
///
/// We take the following shortcut. Compute the time it takes for the (position, velocity) to
/// repeat in the x coordinate. Then do the same for the y coordinate, and the z coordinate. Find
/// the LCM of all these numbers, and you're done! It turns out the state transitions of each
/// coordinate are entirely independant of one another, so this suffices.
pub fn repeat_time(moons: Vec<Moon>) -> i64 {
    let x_time = repeat_time_coord(moons.clone(), |p| p.x);
    let y_time = repeat_time_coord(moons.clone(), |p| p.y);
    let z_time = repeat_time_coord(moons.clone(), |p| p.z);

    // lcm(a, b, c) == lcm(a, lcm(b, c))
    integer::lcm(x_time, integer::lcm(y_time, z_time))
}

fn repeat_time_coord<F>(mut moons: Vec<Moon>, coord: F) -> i64
where
    F: Fn(Point) -> i64,
{
    let coord_state = |moons: &[Moon]| -> (Vec<i64>, Vec<i64>) {
        let pos = moons.iter().map(|m| coord(m.pos)).collect();
        let vel = moons.iter().map(|m| coord(m.vel)).collect();
        (pos, vel)
    };

    let init = coord_state(&moons);
    for t in 0.. {
        if t != 0 && coord_state(&moons) == init {
            return t;
        }
        time_step(&mut moons);
    }

    unreachable!()
}

#[derive(Clone)]
pub struct Moon {
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
        Self::apply_gravity_coord(a, b, |p| &mut p.x);
        Self::apply_gravity_coord(a, b, |p| &mut p.y);
        Self::apply_gravity_coord(a, b, |p| &mut p.z);
    }

    fn apply_gravity_coord<F>(a: &mut Self, b: &mut Self, coord: F)
    where
        F: Fn(&mut Point) -> &mut i64,
    {
        let dir = match coord(&mut a.pos).cmp(&mut coord(&mut b.pos)) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        *coord(&mut a.vel) += dir;
        *coord(&mut b.vel) += -1 * dir;
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

    /// L1 norm.
    fn energy(self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}
