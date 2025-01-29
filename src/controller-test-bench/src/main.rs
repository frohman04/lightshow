mod inst_builders;
mod instruction_builder;
mod util;

use crate::inst_builders::init::InitBuilder;
use crate::inst_builders::set_leds::SetLedsBuilder;
use crate::instruction_builder::{InstructionBuilder, InstructionBuilderMeta};
use crate::util::input;
use ls_controller_protocol::build_packet;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let ansi_enabled = fix_ansi_term();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
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

    let commands: HashMap<String, Box<dyn InstructionBuilder>> = {
        let mut map: HashMap<String, Box<dyn InstructionBuilder>> = HashMap::new();
        map.insert(InitBuilder::display_name(), Box::new(InitBuilder::new()));
        map.insert(
            SetLedsBuilder::display_name(),
            Box::new(SetLedsBuilder::new()),
        );
        map
    };

    while let Ok(cmd) = input("(exit, help)> ") {
        match cmd.as_str() {
            "exit" => break,
            "help" => print_help(&commands),
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
            _ => {
                let parts = cmd
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let cmd = parts[0].clone();
                let args = parts[1..].to_vec();

                match commands.get(&cmd) {
                    Some(cmd_processor) => match cmd_processor.build_instruction(args) {
                        Some(instruction) => {
                            let packet = build_packet(instruction);

                            info!("Sending packet: {:02x?}", packet);
                            arduino
                                .write_all(packet.as_slice())
                                .expect("Failed to send packet");
                        }
                        None => continue,
                    },
                    None => {
                        println!("Unknown command: {}", cmd);
                        print_help(&commands);
                    }
                }
            }
        }
    }
}

fn print_help(commands: &HashMap<String, Box<dyn InstructionBuilder>>) {
    commands
        .iter()
        .for_each(|(command, builder)| println!("{} - {}", command, builder.help()));
    println!("read - read all buffered output from serial device");
    println!("help - print this help message");
    println!("exit - exit the program");
}
