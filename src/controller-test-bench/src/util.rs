use std::io;
use std::io::{BufRead, Write};
use std::num::ParseIntError;

/// Prompt a user for input and return the text that they typed.
pub fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|line| line.trim().to_owned())
}

/// Take a textual representation of hexadecimal values and convert it to a series of bytes.
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
