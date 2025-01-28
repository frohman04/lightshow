use crate::instruction_builder::{InstructionBuilder, InstructionBuilderMeta};
use crate::util::{decode_hex, input, input_u8};
use ls_controller_protocol::SetLeds;
use tracing::{error, info};

pub struct SetLedsBuilder {}

impl SetLedsBuilder {
    pub fn new() -> SetLedsBuilder {
        SetLedsBuilder {}
    }
}

impl InstructionBuilderMeta for SetLedsBuilder {
    fn display_name() -> String {
        "set_leds".to_string()
    }
}

impl InstructionBuilder<SetLeds> for SetLedsBuilder {
    fn help(&self) -> String {
        "set the colors for a set of LEDs".to_string()
    }

    fn build_instruction(&self) -> Option<SetLeds> {
        let display_name = Self::display_name();

        let offset = input_u8(format!("{}: offset (exit)> ", display_name).as_str())?;

        let mut num_pixels: u8 = 0;
        let mut buffer: Vec<u8> = Vec::new();
        while let Ok(inp) = input(
            format!(
                "{}: led{} (done, exit, help)> ",
                display_name,
                offset + num_pixels
            )
            .as_str(),
        ) {
            match inp.as_str() {
                "exit" => return None,
                "help" => {
                    println!("rrggbb - provide the HTML hex colorcode for the next pixel to add to the buffer");
                    println!("peek - peek at the current buffer contents");
                    println!("done - send a message with the current buffer contents");
                    println!("help - show this help message");
                    println!("exit - abort the current builder and return to root shell");
                }
                "done" => break,
                "peek" => {
                    info!(
                        "(offset: {}, num_pixels: {}): {:02x?}",
                        offset, num_pixels, buffer
                    );
                }
                v => match decode_hex(v) {
                    Ok(rgb_hex) => {
                        if rgb_hex.len() == 3 {
                            num_pixels += 1;
                            buffer.extend_from_slice(&rgb_hex);
                        } else {
                            error!("Invalid hex color: {}", v);
                        }
                    }
                    Err(e) => error!("Invalid hex color: {}", e),
                },
            }
        }

        Some(SetLeds::new(offset, num_pixels, buffer))
    }
}
