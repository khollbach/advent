use common::read_line;
use std::error::Error;

/// Read the first line of stdin, and parse it as a comma-separated
/// list of integers: e.g. `1,2,-54,0`.
pub fn read_mem() -> Result<Vec<i32>, Box<dyn Error>> {
    let line = read_line()?;

    let mut vec = vec![];
    for word in line.split(',') {
        vec.push(word.parse()?);
    }
    Ok(vec)
}
