use ls_controller_protocol::{build_packet, SetLeds};
use std::io;
use std::io::{BufRead, Read, Write};
use std::num::ParseIntError;
use std::time::Duration;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let ansi_enabled = fix_ansi_term();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_ansi(ansi_enabled)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    cmd_loop("COM3", 115_200)
}

#[cfg(target_os = "windows")]
fn fix_ansi_term() -> bool {
    nu_ansi_term::enable_ansi_support().is_ok_and(|()| true)
}

#[cfg(not(target_os = "windows"))]
fn fix_ansi_term() -> bool {
    true
}

/// Handle user interaction for the test bench.
fn cmd_loop(port: &str, baud_rate: u32) {
    let mut arduino = serialport::new(port, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|_| panic!("Failed to open serial port {}", port));

    let mut offset: u8 = 0;
    let mut num_pixels: u8 = 0;
    let mut buffer: Vec<u8> = Vec::new();
    while let Ok(pixel) =
        input("Enter RGB for pixes as HTML hex ('offset [n]', 'send', 'read', 'exit'): ")
    {
        match pixel.as_str() {
            "send" => {
                let packet = build_packet(SetLeds::new(offset, num_pixels, &buffer));
                num_pixels = 0;
                buffer.clear();

                info!("Sending packet: {:02x?}", packet);
                arduino
                    .write_all(packet.as_slice())
                    .expect("Failed to send packet");
            }
            "read" => match arduino.bytes_to_read() {
                Ok(bytes_available) if bytes_available > 0 => {
                    let mut input_buffer = vec![0u8; bytes_available as usize];
                    match arduino.read_exact(&mut input_buffer) {
                        Ok(_) => {
                            input_buffer
                                .split(|char| *char == b'\n')
                                .filter(|line| !line.is_empty())
                                .for_each(|line| info!("recv> {}", String::from_utf8_lossy(line)));
                        }
                        Err(e) => error!("Error while reading data: {}", e),
                    }
                }
                Ok(_) => warn!("recv empty"),
                Err(e) => error!("Error while reading data: {}", e),
            },
            "exit" => {
                break;
            }
            x if x.starts_with("offset") => match x.replace("offset", "").trim().parse::<u8>() {
                Ok(of) => offset = of,
                Err(e) => error!("Unable to parse offset: {}", e),
            },
            x if x.len() != 6 => {
                error!("Must enter value six characters long (got {})", x.len());
            }
            x => {
                num_pixels += 1;
                buffer.extend_from_slice(decode_hex(x).unwrap().as_slice());
                info!("Curr ({}): {:02x?}", num_pixels, buffer);
            }
        }
    }
}

/// Prompt a user for input and return the text that they typed.
fn input(prompt: &str) -> io::Result<String> {
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
fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
