use std::io::{self, prelude::*};

/// Read a line from stdin. Does not include a trailing newline.
pub fn read_line() -> io::Result<String> {
    io::stdin().lock().lines().next().unwrap_or_else(|| {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "End of standard input.",
        ))
    })
}

/// For using files as test inputs.
///
/// Returns a BufReader.
#[macro_export]
macro_rules! input {
    ($file:expr) => {
        BufReader::new(include_str!($file).as_bytes())
    };
}
