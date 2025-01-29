use crate::instruction_builder::{InstructionBuilder, InstructionBuilderMeta};
use ls_controller_protocol::{Init, Instruction};
use tracing::error;

pub struct InitBuilder {}

impl InitBuilder {
    pub fn new() -> InitBuilder {
        InitBuilder {}
    }
}

impl InstructionBuilderMeta for InitBuilder {
    fn display_name() -> String {
        "init".to_string()
    }
}

impl InstructionBuilder for InitBuilder {
    fn help(&self) -> String {
        "[num_pixels] [pin] - initialize the LED strip".to_string()
    }

    fn build_instruction(&self, args: Vec<String>) -> Option<Box<dyn Instruction>> {
        if args.is_empty() {
            error!(
                "Must specify number of pixels as argument to {}",
                Self::display_name()
            );
            return None;
        }
        if args.len() == 1 {
            error!(
                "Must specify data pin as argument to {}",
                Self::display_name()
            );
            return None;
        }
        let num_pixels = match args[0].clone().parse::<u8>() {
            Ok(num_pixels) => num_pixels,
            Err(e) => {
                error!("Unable to parse num_pixels: {}", e);
                return None;
            }
        };
        let pin = match args[1].clone().parse::<u8>() {
            Ok(pin) => pin,
            Err(e) => {
                error!("Unable to parse pin: {}", e);
                return None;
            }
        };

        Some(Box::new(Init::new(num_pixels, pin)))
    }
}
