use cpu::read_mem;
use robot::Robot;
use std::collections::{HashMap, HashSet, VecDeque};
use types::{Dir, Grid, Point, Tile, NESW, ORIGIN};

mod robot;
mod types;

fn main() {
    let mem = read_mem().unwrap();

    let (grid, target) = explore_grid(mem);

    let shortest = shortest_path_len(&grid).unwrap();
    println!("{}", shortest);

    let farthest = greatest_distance(&grid, target);
    println!("{}", farthest);
}

/// DFS to map the entire enclosure using the robot. There must be exactly one reachable target.
/// Returns the grid and the target.
///
/// Could run infinitely if the enclosure is infinite. In that case, we would want to implement an
/// interative-deepening DFS instead.
fn explore_grid(mem: Vec<i64>) -> (Grid, Point) {
    // Explore the tile the robot is currently standing on, and recursively explore all neighbors.
    //
    // After visiting a neighbor, move the robot back to the current position.
    fn dfs(robot: &mut Robot, grid: &mut Grid, target: &mut Option<Point>) {
        grid.insert(robot.pos, robot.tile);

        if robot.tile == Tile::Target {
            assert!(target.is_none(), "Two targets; this can't be right.");
            *target = Some(robot.pos);
        }

        for &dir in &NESW {
            let new_pos = Dir::step(robot.pos, dir);
            if !grid.contains_key(&new_pos) {
                if robot.move_(dir) {
                    dfs(robot, grid, target);

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
    let mut target = None;

    dfs(&mut robot, &mut grid, &mut target);

    (grid, target.expect("No target; this can't be right."))
}

/// Use a BFS to find the shortest path from the origin to the target.
///
/// Requires that grid is enclosed in `Wall` tiles, otherwise this may panic when accessing tiles
/// that do not exist at the edges of the grid.
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

/// Use a BFS to find the longest distance from the source to any reachable tile.
///
/// The "distance" between two tiles is specifically the shortest path between them.
///
/// Requires that grid is enclosed in `Wall` tiles, otherwise this may panic when accessing tiles
/// that do not exist at the edges of the grid.
fn greatest_distance(grid: &Grid, source: Point) -> u32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((source, 0));

    let mut longest = 0;

    while let Some((p, dist)) = queue.pop_front() {
        seen.insert(p);

        longest = dist;

        // Enqueue p's neighbors (except walls).
        for &dir in &NESW {
            let other = Dir::step(p, dir);
            if grid[&other] != Tile::Wall && !seen.contains(&other) {
                queue.push_back((other, dist + 1));
            }
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::input_mem;

    #[test]
    fn part1() {
        let mem = input_mem!("../tests/input");
        let (grid, _) = explore_grid(mem);

        let shortest = shortest_path_len(&grid).unwrap();
        assert_eq!(234, shortest);
    }

    #[test]
    fn part2() {
        let mem = input_mem!("../tests/input");
        let (grid, target) = explore_grid(mem);

        let farthest = greatest_distance(&grid, target);
        assert_eq!(292, farthest);
    }
}
