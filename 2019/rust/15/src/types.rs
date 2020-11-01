use std::collections::HashMap;

/// A sparse grid of floor and wall tiles.
///
/// Ideally the floor tiles should be enclosed in wall tiles.
pub type Grid = HashMap<Point, Tile>;

/// (0, 0) is the robot's starting point; everything else is relative to that.
///
/// If the point is (x, y), then N/S is +y/-y and E/W is +x/-x.
pub type Point = (i32, i32);

pub const ORIGIN: Point = (0, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,

    /// Location of the oxygen system.
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

pub const NESW: [Dir; 4] = {
    use Dir::*;
    [N, E, S, W]
};

impl Dir {
    /// Negate this direction.
    pub fn opposite(self) -> Self {
        use Dir::*;
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }

    /// Take one step in this direction.
    pub fn step((x, y): Point, d: Dir) -> Point {
        use Dir::*;
        match d {
            N => (x, y + 1),
            E => (x + 1, y),
            S => (x, y - 1),
            W => (x - 1, y),
        }
    }
}
