use std::error::Error;
use std::io;

// This may seem weird, but we need to re-export these since they're used in the `input_mem` macro.
pub use common::{get_line, input};

/// Read the first line of stdin, and parse it as a comma-separated list of integers: e.g.
/// `"1,2,-54,0"`. Must be non-empty.
pub fn read_mem() -> Result<Vec<i64>, Box<dyn Error>> {
    let line = get_line(&mut io::stdin().lock())?;
    parse_mem(&line)
}

/// Parse a string as a comma-separated list of integers. E.g., `"1,2,-54,0"`.
///
/// Must be non-empty, and no trailing newline.
pub fn parse_mem(line: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    if line.is_empty() {
        return Err("Empty intcode program.".into());
    }
    if line.chars().last().unwrap() == '\n' {
        return Err("Trailing newline in intcode program.".into());
    }

    let mut vec = vec![];
    for word in line.split(',') {
        vec.push(word.parse()?);
    }
    Ok(vec)
}

/// Convenience for writing unit tests. Reads the first line of a file and returns a Vec<i64>.
///
/// ```
/// use cpu::input_mem;
///
/// let mem = input_mem!("../tests/input");
/// assert_eq!(vec![1, 0, 0, 0, 99], mem);
/// ```
#[macro_export]
macro_rules! input_mem {
    ($file:expr) => {
        cpu::parse_mem(&cpu::get_line(&mut cpu::input!($file)).unwrap()).unwrap()
    };
}
