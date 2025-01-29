pub trait Instruction {
    /// Get the instruction code used to represent this instruction.
    fn code(&self) -> u8;

    /// Convert this instruction into the binary data to be transmitted.  CRC and COBS encoding will
    /// be applied to this data.
    fn to_message(&self) -> Vec<u8>;
}
