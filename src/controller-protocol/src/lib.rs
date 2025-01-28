#![forbid(unsafe_code)]

mod instruction;
mod set_leds;

use crate::instruction::Instruction;
pub use crate::set_leds::SetLeds;
use tracing::info;

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
