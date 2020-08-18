//! Helper functions for solving parts 1 and 2.

use grid::{Grid, Tile};
use num::integer;
use std::collections::HashSet;
use std::iter;

pub mod grid;
pub mod part1;
pub mod part2;

/// Stored as `(x, y)`.
pub type Point = (i32, i32);

/// Get the coordinates of all asteroids in the grid.
pub fn asteroids(grid: &Grid) -> HashSet<Point> {
    let ys = 0..grid.len();
    ys.flat_map(|y| {
        let xs = 0..grid[y].len();
        xs.filter(move |&x| match grid[y][x] {
            Tile::Asteroid => true,
            Tile::Empty => false,
        })
        .map(|x| x as i32)
        .zip(iter::repeat(y as i32))
    })
    .collect()
}

/// Can these two points see each other?
/// True if there are no asteroids on the line connecting them.
fn is_visible(asteroids: &HashSet<Point>, a: Point, b: Point) -> bool {
    assert!(a != b, "The points must be different");
    interference(asteroids, a, b) == 0
}

/// How many asteroids are there on the line between these two points?
fn interference(asteroids: &HashSet<Point>, a: Point, b: Point) -> usize {
    assert!(a != b, "The points must be different");

    let (x1, y1) = a;
    let (x2, y2) = b;
    let dx = x2 - x1;
    let dy = y2 - y1;

    let num_steps = integer::gcd(dx, dy);
    let x_step = dx / num_steps;
    let y_step = dy / num_steps;

    (1..num_steps)
        .filter(|&i| {
            let x = x1 + i * x_step;
            let y = y1 + i * y_step;

            asteroids.contains(&(x, y))
        })
        .count()
}

/// What is the vector difference `a - b`?
/// Returned as the pair (dx, dy).
pub fn subtract_points(a: Point, b: Point) -> Point {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (x1 - x2, y1 - y2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtract() {
        let a = (5, 7);
        let b = (2, 3);

        assert_eq!((3, 4), subtract_points(a, b));
    }
}
