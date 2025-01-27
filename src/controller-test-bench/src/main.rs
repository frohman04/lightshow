use std::io;
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::time::Duration;
use tracing::{error, info, Level};
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

    let mut num_pixels = 0;
    let mut buffer: Vec<u8> = Vec::new();
    while let Ok(pixel) = input("Enter RGB for pixes as HTML hex ('send', 'read', 'exit'): ") {
        match pixel.as_str() {
            "send" => {
                let packet = build_packet(num_pixels, &buffer);
                num_pixels = 0;
                buffer.clear();

                info!("Sending packet: {:02x?}", packet);
                arduino
                    .write_all(packet.as_slice())
                    .expect("Failed to send packet");
            }
            "read" => {
                let mut reader = BufReader::new(&mut arduino);
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(_) => info!("recv> {}", line),
                    Err(e) => error!("Error while reading data: {}", e),
                };
            }
            "exit" => {
                break;
            }
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

/// Build a COBS-encoded packet for a chunk of data.
fn build_packet(num_pixels: u8, buffer: &[u8]) -> Vec<u8> {
    info!("Building message for Arduino");

    let message = {
        let mut mess: Vec<u8> = Vec::new();
        mess.push(num_pixels);
        mess.extend_from_slice(buffer);
        mess
    };
    info!("Constructed message: {:02x?}", message);

    let checksum = {
        let calc = crc::Crc::<u16>::new(&crc::CRC_16_ARC);
        let mut digest = calc.digest();
        digest.update(message.as_slice());
        digest.finalize()
    };
    info!("Computed CRC16:      {} / {:x}", checksum, checksum);

    let packet = {
        let mut p: Vec<u8> = Vec::new();
        p.extend_from_slice(message.as_slice());
        p.extend_from_slice(&checksum.to_be_bytes());
        p
    };
    info!("Constructed packet:  {:02x?}", packet);

    let encoded_packet = {
        let mut p = cobs2::cobs::encode_vector(packet.as_slice()).unwrap();
        p.push(0);
        p
    };
    info!("Encoded packet:      {:02x?}", encoded_packet);

    encoded_packet
}
