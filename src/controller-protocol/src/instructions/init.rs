use crate::instruction::Instruction;

pub struct Init {
    num_pixels: u8,
    pin: u8,
}

impl Init {
    pub fn new(num_pixels: u8, pin: u8) -> Self {
        Self { num_pixels, pin }
    }
}

impl Instruction for Init {
    fn code(&self) -> u8 {
        0u8
    }

    fn to_message(&self) -> Vec<u8> {
        vec![self.code(), self.num_pixels, self.pin]
    }
}
