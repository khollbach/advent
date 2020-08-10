use std::io;

/// Read a line from stdin. Strip any trailing newline.
pub fn read_line() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    // Strip newline.
    if buf.chars().last() == Some('\n') {
        buf.pop();
    }

    Ok(buf)
}
