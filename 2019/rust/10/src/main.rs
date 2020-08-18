use num::integer;
use std::collections::HashSet;
use std::convert::TryInto;
use std::io::{self, prelude::*};
use std::iter;

fn main() {
    let grid = read_input();
    let asteroids = asteroids(&grid);

    // Day 1.
    println!("{}", max_visibility(&asteroids));
}

/// What is the visibility of the best monitoring station?
fn max_visibility(asteroids: &HashSet<Point>) -> usize {
    asteroids
        .iter()
        .map(|&a| num_visible(&asteroids, a))
        .max()
        .unwrap()
}

/// How many other asteroids are visible from this location?
fn num_visible(asteroids: &HashSet<Point>, base: Point) -> usize {
    asteroids
        .iter()
        .filter(|&&other| base != other && is_visible(asteroids, base, other))
        .count()
}

/// Can these two points see each other?
/// True if there are no asteroids on the line connecting them.
fn is_visible(asteroids: &HashSet<Point>, a: Point, b: Point) -> bool {
    assert!(a != b, "The points must be different");

    fn usizes_to_i32s(p: Point) -> (i32, i32) {
        let (x, y) = p;
        (x as i32, y as i32)
    }

    fn i32s_to_usizes(p: (i32, i32)) -> Point {
        let (x, y) = p;
        (x.try_into().unwrap(), y.try_into().unwrap())
    }

    let (x1, y1) = usizes_to_i32s(a);
    let (x2, y2) = usizes_to_i32s(b);
    let dx = x2 - x1;
    let dy = y2 - y1;

    let num_steps = integer::gcd(dx, dy);
    let x_step = dx / num_steps;
    let y_step = dy / num_steps;

    for i in 1..num_steps {
        let x = x1 + i * x_step;
        let y = y1 + i * y_step;

        if asteroids.contains(&i32s_to_usizes((x, y))) {
            return false;
        }
    }

    true
}

/// Indexed as `grid[y][x]`.
type Grid = Vec<Vec<Tile>>;

/// Stored as `(x, y)`.
type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Asteroid,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Asteroid,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

/// Read the input "treasure-map" into a grid.
fn read_input() -> Grid {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| Tile::new(c)).collect())
        .collect()
}

/// Get the coordinates of all asteroids in the grid.
fn asteroids(grid: &Grid) -> HashSet<Point> {
    let ys = 0..grid.len();
    ys.flat_map(|y| {
        let xs = 0..grid[y].len();
        xs.filter(move |&x| match grid[y][x] {
            Tile::Asteroid => true,
            Tile::Empty => false,
        })
        .zip(iter::repeat(y))
    })
    .collect()
}
