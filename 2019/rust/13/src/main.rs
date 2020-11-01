use cpu::{read_mem, CPUBuilder};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

fn main() {
    let mem = read_mem().unwrap();

    println!("{}", num_blocks(mem.clone()));
    println!("{}", final_score(mem));
}

/// Return the number of blocks that would be initially drawn to the screen.
fn num_blocks(mem: Vec<i64>) -> usize {
    let mut screen = Screen::new();
    let mut adapter = ScreenAdapter::new(&mut screen);

    CPUBuilder::new(mem)
        .default_in()
        .output(|x| {
            adapter.receive(x);
        })
        .finish()
        .run();

    let num = screen.grid.values().filter(|&&t| t == Tile::Block).count();

    num
}

/// Insert two quarters, and play the game using a very simple AI.
///
/// Each turn, the AI moves the paddle in the direction of the ball.
fn final_score(mut mem: Vec<i64>) -> i64 {
    let mut screen = Screen::new();
    let adapter = Rc::new(RefCell::new(ScreenAdapter::new(&mut screen)));

    // Insert 2 quarters.
    mem[0] = 2;

    CPUBuilder::new(mem)
        .output(|x| {
            adapter.borrow_mut().receive(x);
        })
        .input(|| {
            let ball = adapter.borrow().screen.ball.unwrap();
            let paddle = adapter.borrow().screen.paddle.unwrap();

            match ball.x.cmp(&paddle.x) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        })
        .finish()
        .run();

    let score = screen.score.unwrap();
    score
}

/// Batch up partial messages in groups of 3, and decode them to draw to the screen.
#[derive(Debug)]
struct ScreenAdapter<'a> {
    screen: &'a mut Screen,
    msg_buf: Vec<i64>,
}

impl<'a> ScreenAdapter<'a> {
    fn new(screen: &'a mut Screen) -> Self {
        Self {
            screen,
            msg_buf: vec![],
        }
    }

    /// Receive a partial message. Then if the message buffer contains a full message, process the
    /// message and clear the buffer.
    fn receive(&mut self, partial_msg: i64) {
        const MSG_LEN: usize = 3;
        assert!(self.msg_buf.len() < MSG_LEN);

        self.msg_buf.push(partial_msg);

        if self.msg_buf.len() == MSG_LEN {
            self.process_msg(self.msg_buf[0], self.msg_buf[1], self.msg_buf[2]);
            self.msg_buf.clear();
        }
    }

    /// Process a message to draw to the screen (or update the score).
    fn process_msg(&mut self, x: i64, y: i64, tile_id: i64) {
        if x == -1 && y == 0 {
            self.screen.score = Some(tile_id);
        } else {
            assert!(x >= 0 && y >= 0);
            self.screen.draw(Point { x, y }, Tile::new(tile_id));
        }
    }
}

/// Record tiles drawn to the screen.
#[derive(Debug)]
struct Screen {
    grid: HashMap<Point, Tile>,
    score: Option<i64>,

    /// Stores the most-recently-drawn ball (if visible).
    ball: Option<Point>,

    /// Stores the most-recently-drawn paddle (if visible).
    paddle: Option<Point>,
}

impl Screen {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            score: None,
            ball: None,
            paddle: None,
        }
    }

    /// Update a cell with the contents. Doesn't render anything though, just records the update.
    fn draw(&mut self, coords: Point, tile: Tile) {
        assert!(coords.x >= 0 && coords.y >= 0);

        // Overwriting the ball or paddle.
        match self.grid.get(&coords) {
            Some(Tile::Ball) => self.ball = None,
            Some(Tile::Paddle) => self.paddle = None,
            _ => (),
        }

        self.grid.insert(coords, tile);

        // Writing a new ball or paddle.
        match tile {
            Tile::Ball => self.ball = Some(coords),
            Tile::Paddle => self.paddle = Some(coords),
            _ => (),
        }
    }

    /// Print the grid to stdout.
    fn _render(&self) {
        if let Some(s) = self.score {
            println!("Score: {}", s);
        };

        if self.grid.is_empty() {
            return;
        }

        let min_x = self.grid.keys().map(|&p| p.x).min().unwrap();
        let max_x = self.grid.keys().map(|&p| p.x).max().unwrap();
        let min_y = self.grid.keys().map(|&p| p.y).min().unwrap();
        let max_y = self.grid.keys().map(|&p| p.y).max().unwrap();

        for y in min_y..=max_y {
            let line: String = (min_x..=max_x)
                .map(|x| {
                    let tile = self.grid.get(&Point { x, y }).unwrap_or(&Tile::Unknown);
                    tile.to_char()
                })
                .collect();

            println!("{}", line);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,

    // The #[allow(dead_code)] shouldn't be needed, but there seems to be a bug in the compiler.
    // Since we only use Unknown as `&Unknown`, the compiler thinks we never use it. *shrug*
    #[allow(dead_code)]
    Unknown,
}

impl Tile {
    fn new(tile_id: i64) -> Self {
        use Tile::*;
        match tile_id {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => panic!("Invalid tile id: {}", tile_id),
        }
    }

    fn to_char(self) -> char {
        use Tile::*;
        match self {
            Empty => ' ',
            Wall => '#',
            Block => '+',
            Paddle => '=',
            Ball => '*',
            Unknown => '?',
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::input_mem;

    #[test]
    fn part1() {
        let mem = input_mem!("../tests/input");
        assert_eq!(296, num_blocks(mem));
    }

    #[test]
    fn part2() {
        let mem = input_mem!("../tests/input");
        assert_eq!(13824, final_score(mem));
    }
}
