use cpu::read_mem;
use robot::Robot;
use std::collections::{HashMap, HashSet, VecDeque};

mod robot;

fn main() {
    let mem = read_mem().unwrap();

    let grid = explore_grid(mem);
    let shortest = shortest_path_len(&grid).unwrap();

    println!("{}", shortest);
}

type Grid = HashMap<Point, Tile>;

/// (0, 0) is the robot's starting point; everything else is relative to that.
///
/// If the point is (x, y), then N/S is +y/-y and E/W is +x/-x.
type Point = (i32, i32);

const ORIGIN: Point = (0, 0);

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

const NESW: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

impl Dir {
    /// Negate this direction.
    fn opposite(self) -> Self {
        use Dir::*;
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }

    /// Take one step in this direction.
    fn step((x, y): Point, d: Dir) -> Point {
        use Dir::*;
        match d {
            N => (x, y + 1),
            E => (x + 1, y),
            S => (x, y - 1),
            W => (x - 1, y),
        }
    }
}

/// DFS to map the entire enclosure using the robot.
///
/// Could run infinitely if the enclosure is infinite. In that case,
/// we would want to implement an interative-deepening DFS instead.
fn explore_grid(mem: Vec<i64>) -> Grid {
    /// Visit the tile the robot is currently standing on, and recursively visit all neighbors.
    ///
    /// After visiting each neighbor, move the robot back to the current position.
    ///
    /// `grid` records the tiles have been visited.
    fn dfs(robot: &mut Robot, grid: &mut Grid) {
        // "Visit" the current tile.
        grid.insert(robot.pos, robot.tile);

        for &dir in &NESW {
            let new_pos = Dir::step(robot.pos, dir);
            if !grid.contains_key(&new_pos) {
                if robot.move_(dir) {
                    dfs(robot, grid);

                    // Move back after visiting that part of the map!
                    robot.move_(dir.opposite());
                } else {
                    // Can't move onto walls. Record this wall as explored, but don't recurse.
                    grid.insert(new_pos, Tile::Wall);
                }
            }
        }
    }

    let mut robot = Robot::new(mem);

    let mut grid = HashMap::new();
    dfs(&mut robot, &mut grid);
    grid
}

/// Use a BFS to find the shortest path from the origin to the target.
///
/// Requires that grid is enclosed in `Wall` tiles, otherwise this may
/// panic when accesses tiles that do not exist at the edges of the grid.
///
/// If no path exists after exhausting the search space, return None.
fn shortest_path_len(grid: &Grid) -> Option<u32> {
    let mut seen = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((ORIGIN, 0));

    while let Some((p, dist)) = queue.pop_front() {
        seen.insert(p);

        // Are we done?
        if grid[&p] == Tile::Target {
            return Some(dist);
        }

        // Enqueue p's neighbors (except walls).
        for &dir in &NESW {
            let other = Dir::step(p, dir);
            if grid[&other] != Tile::Wall && !seen.contains(&other) {
                queue.push_back((other, dist + 1));
            }
        }
    }

    None
}
