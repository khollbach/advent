use std::io;
use std::io::prelude::*;

/// Read a line from stdin. Does not include a trailing newline.
pub fn read_line() -> io::Result<String> {
    get_line(io::stdin().lock())
}

/// Read one line from the input source. No trailing newline.
pub fn get_line<R: BufRead>(input: R) -> io::Result<String> {
    input.lines().next().unwrap_or_else(|| {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "End of input.",
        ))
    })
}

/// For using files as test inputs.
///
/// Returns a BufReader.
#[macro_export]
macro_rules! input {
    ($file:expr) => {
        std::io::BufReader::new(include_str!($file).as_bytes())
    };
}
