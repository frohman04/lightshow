use std::fmt::Display;
use std::io;
use std::io::{BufRead, Write};
use std::num::ParseIntError;
use tracing::error;

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

/// Prompt a user for input and return the text that they typed, parsed into a different format.
/// If exit is typed, return None.
pub fn input_parse<T, P, E>(prompt: &str, parser: P) -> Option<T>
where
    P: FnOnce(String) -> Result<T, E>,
    P: Copy,
    E: Display,
{
    match input(prompt) {
        Ok(inp) => {
            if inp.trim() == "exit" {
                None
            } else {
                match parser(inp) {
                    Ok(res) => Some(res),
                    Err(e) => {
                        error!("Unable to parse input: {}", e);
                        input_parse(prompt, parser)
                    }
                }
            }
        }
        Err(e) => {
            error!("Unable to read input: {}", e);
            input_parse(prompt, parser)
        }
    }
}

/// Prompt a user for input and return the text that they typed, parsed into a u8.
/// If exit is typed, return None.
pub fn input_u8(prompt: &str) -> Option<u8> {
    input_parse(prompt, |input| input.trim().parse::<u8>())
}

/// Take a textual representation of hexadecimal values and convert it to a series of bytes.
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
