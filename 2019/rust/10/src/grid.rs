//! Handles reading the input grid.

use std::io::{self, prelude::*};

/// Indexed as `grid[y][x]`.
pub type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Asteroid,
}

impl Tile {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Asteroid,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

/// Read the input "treasure-map" into a grid.
pub fn read_input() -> Grid {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| Tile::new(c)).collect())
        .collect()
}
