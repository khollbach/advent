use cpu::{read_mem, CPU};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

fn main() {
    let mem = read_mem().unwrap();

    let grid = map_grid(mem);

    let shortest = shortest_path(grid).unwrap();

    println!("{}", shortest);
}

type Grid = HashMap<Point, Tile>;

/// (0, 0) is the robot's starting point.
type Point = (i32, i32);

const ORIGIN: Point = (0, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
    /// Location of the oxygen system.
    Target,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
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
    fn step(p: Point, d: Dir) -> Point {
        use Dir::*;

        let (x, y) = p;
        match d {
            N => (x, y + 1),
            E => (x + 1, y),
            S => (x, y - 1),
            W => (x - 1, y),
        }
    }
}

/// The robot.
struct Robot {
    pos: Point,
    tile: Tile,

    /// The cpu runs in its own thread.
    /// Instructions and their sensor responses are handled via channels.
    _cpu: JoinHandle<()>,
    instructions: Sender<Dir>,
    sensor: Receiver<Tile>,
}

impl Robot {
    /// Spawns a new thread running the robot's cpu.
    fn new(mem: Vec<i64>) -> Self {
        let pos = ORIGIN;
        let tile = Tile::Floor;

        let (instructions, instr_rx) = mpsc::channel();
        let (sensor_sx, sensor) = mpsc::channel();

        // todo do I need to do anything with cpu?
        //    > do I need it at all??
        //
        // Ok, so the thread will currently panic, since there's
        // no way to stop the CPU running once it starts. Specifically,
        // the `input` closure will try to receive from a closed channel.
        //
        // We should add a way to stop the CPU from another thread
        // and use that to properly clean up, automatically upon `drop`.

        let _cpu = thread::spawn(move || {
            CPU::new(mem)
                .input(move || {
                    use Dir::*;
                    match instr_rx.recv().unwrap() {
                        // Note the unconventional encoding.
                        N => 1,
                        S => 2,
                        W => 3,
                        E => 4,
                    }
                })
                .output(move |x| {
                    use Tile::*;
                    let tile = match x {
                        0 => Wall,
                        1 => Floor,
                        2 => Target,
                        _ => panic!("Invalid sensor output: {}", x),
                    };
                    sensor_sx.send(tile).unwrap();
                })
                .run();
        });

        Self {
            pos,
            tile,
            instructions,
            sensor,
            _cpu,
        }
    }

    /// Try to move onto an adjacent tile. Return true if successful.
    ///
    /// Only fails if the destination tile is a wall, in which case the robot does not move.
    fn move_(&mut self, d: Dir) -> bool {
        self.instructions.send(d).unwrap();

        match self.sensor.recv().unwrap() {
            Tile::Wall => false,
            tile => {
                self.pos = Dir::step(self.pos, d);
                self.tile = tile;
                true
            }
        }
    }
}

/// DFS to map the entire enclosure using the robot.
///
/// Could run infinitely if the enclosure is infinite. In that case,
/// we would want to implement an interative-deepening DFS instead.
fn map_grid(mem: Vec<i64>) -> Grid {
    /// Visit the tile the robot is currently standing on, and then recursively visit all neighbors
    /// that aren't already visited. (If a neighbor is a wall tile, visit it but don't recurse,
    /// since we can't move onto it.)
    ///
    /// After recursively visiting a neighbor, we move the robot back to the current position.
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

                    // Move back after visiting that part of the map.
                    robot.move_(dir.opposite());
                } else {
                    // Can't move onto walls.
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
fn shortest_path(grid: Grid) -> Option<u32> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((ORIGIN, 0));
    while let Some((p, dist)) = queue.pop_front() {
        // Are we done?
        if grid[&p] == Tile::Target {
            return Some(dist);
        }

        // Visit p and enqueue its neighbors.
        visited.insert(p);
        for &dir in &NESW {
            let p2 = Dir::step(p, dir);
            if grid[&p2] != Tile::Wall && !visited.contains(&p2) {
                queue.push_back((p2, dist + 1));
            }
        }
    }

    None
}
