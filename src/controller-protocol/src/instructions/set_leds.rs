use crate::instruction::Instruction;

pub struct SetLeds {
    offset: u8,
    num_pixels: u8,
    pixel_colors: Vec<u8>,
}

impl SetLeds {
    pub fn new(offset: u8, num_pixels: u8, pixel_colors: Vec<u8>) -> Self {
        Self {
            offset,
            num_pixels,
            pixel_colors,
        }
    }
}

impl Instruction for SetLeds {
    fn code() -> u8 {
        1u8
    }

    fn to_message(self) -> Vec<u8> {
        let mut mess: Vec<u8> = Vec::new();
        mess.push(SetLeds::code());
        mess.push(self.offset);
        mess.push(self.num_pixels);
        mess.extend_from_slice(&self.pixel_colors);
        mess
    }
}
