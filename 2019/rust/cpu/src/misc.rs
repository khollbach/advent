use common::get_line;
use std::error::Error;
use std::io;
use std::io::prelude::*;

/// Read the first line of stdin, and parse it as a comma-separated list of integers: e.g.
/// `1,2,-54,0`.
pub fn read_mem() -> Result<Vec<i64>, Box<dyn Error>> {
    parse_mem(io::stdin().lock())
}

// todo: why doesn't this just take &str ????

/// Read one line from a BufRead, and parse it as a comma-separated list of integers: e.g.
/// `1,2,-54,0`.
pub fn parse_mem(input: impl BufRead) -> Result<Vec<i64>, Box<dyn Error>> {
    let line = get_line(input)?;

    let mut vec = vec![];
    for word in line.split(',') {
        vec.push(word.parse()?);
    }
    Ok(vec)
}
