use crate::instruction::Instruction;

pub struct SetLeds<'a> {
    offset: u8,
    num_pixels: u8,
    pixel_colors: &'a [u8],
}

impl<'a> SetLeds<'a> {
    pub fn new(offset: u8, num_pixels: u8, pixel_colors: &'a [u8]) -> Self {
        Self {
            offset,
            num_pixels,
            pixel_colors,
        }
    }
}

impl Instruction for SetLeds<'_> {
    fn to_message(self) -> Vec<u8> {
        let mut mess: Vec<u8> = Vec::new();
        mess.push(1u8);
        mess.push(self.offset);
        mess.push(self.num_pixels);
        mess.extend_from_slice(self.pixel_colors);
        mess
    }
}
