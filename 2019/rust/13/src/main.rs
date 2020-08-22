use cpu::{parse_mem, CPU};
use ncurses as nc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().skip(1).next().expect("No filename given.");
    let file = File::open(file_name)?;
    let mem = parse_mem(BufReader::new(file)).unwrap();

    println!("{}", num_blocks(mem.clone()));

    play_game(mem);

    Ok(())
}

/// Return the number of blocks that would be initially drawn to the screen.
fn num_blocks(mem: Vec<i64>) -> usize {
    let screen = Rc::new(RefCell::new(Screen::new()));
    let mut adapter = ScreenAdapter::new(Rc::clone(&screen));

    CPU::new(mem)
        .output_fn(move |x| {
            adapter.receive(x);
        })
        .run();

    let num = screen
        .borrow()
        .map
        .values()
        .filter(|&&t| t == Tile::Block)
        .count();
    num
}

/// Insert two quarters, and play the game.
fn play_game(mut mem: Vec<i64>) -> i64 {
    let screen = Rc::new(RefCell::new(Screen::new()));
    let mut adapter = ScreenAdapter::new(Rc::clone(&screen));

    // Insert 2 quarters.
    mem[0] = 2;

    nc::timeout(-1);
    nc::initscr();

    let screen2 = Rc::clone(&screen);
    CPU::new(mem)
        .output_fn(move |x| {
            adapter.receive(x);
        })
        .input_fn(move || {
            screen2.borrow().render();
            loop {
                match u8::try_from(nc::getch()).unwrap() as char {
                    'j' => break -1,
                    'k' => break 0,
                    'l' => break 1,
                    _ => continue,
                }
            }
        })
        .run();

    screen.borrow().render();

    nc::endwin();

    let score = screen.borrow().score.unwrap();
    score
}

#[derive(Debug)]
struct ScreenAdapter {
    screen: Rc<RefCell<Screen>>,
    msg_buf: Vec<i64>,
}

impl ScreenAdapter {
    const MSG_LEN: usize = 3;

    fn new(screen: Rc<RefCell<Screen>>) -> Self {
        Self {
            screen,
            msg_buf: vec![],
        }
    }

    fn receive(&mut self, partial_msg: i64) {
        assert!(self.msg_buf.len() < Self::MSG_LEN);

        self.msg_buf.push(partial_msg);
        if self.msg_buf.len() == Self::MSG_LEN {
            let x = self.msg_buf[0];
            let y = self.msg_buf[1];
            let tile_id = self.msg_buf[2];
            self.msg_buf.clear();

            if x == -1 && y == 0 {
                self.screen.borrow_mut().score = Some(tile_id);
            } else {
                self.screen
                    .borrow_mut()
                    .draw(Point { x, y }, Tile::new(tile_id));
            }
        }
    }
}

#[derive(Debug)]
struct Screen {
    map: HashMap<Point, Tile>,
    score: Option<i64>,
}

impl Screen {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            score: None,
        }
    }

    /// Update a cell with the contents. Doesn't render anything though, just records the update.
    fn draw(&mut self, coords: Point, tile: Tile) {
        assert!(coords.x >= 0 && coords.y >= 0);

        self.map.insert(coords, tile);
    }

    /// Careful with this! You must call ncurses::{initscr(), endwin()} before and after!
    fn render(&self) {
        if self.map.is_empty() {
            return;
        }

        // Clear the screen.
        nc::clear();

        if let Some(s) = self.score {
            nc::addstr(&format!("Score: {}\n", s));
        };

        let min_x = self.map.keys().map(|&p| p.x).min().unwrap();
        let min_y = self.map.keys().map(|&p| p.y).min().unwrap();
        let max_x = self.map.keys().map(|&p| p.x).max().unwrap();
        let max_y = self.map.keys().map(|&p| p.y).max().unwrap();

        for y in min_y..=max_y {
            let mut line: String = (min_x..=max_x)
                .map(|x| {
                    let tile = self.map.get(&Point { x, y }).unwrap_or(&Tile::Unknown);
                    tile.to_char()
                })
                .collect();
            line.push('\n');

            nc::addstr(&line);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
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

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
