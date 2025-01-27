#![forbid(unsafe_code)]

use tracing::info;

pub trait Instruction {
    fn to_message(self) -> Vec<u8>;
}

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

/// Build a COBS-encoded packet for a chunk of data.
pub fn build_packet<T: Instruction>(instruction: T) -> Vec<u8> {
    info!("Building message for Arduino");

    let message = instruction.to_message();
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
