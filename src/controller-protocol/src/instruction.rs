pub trait Instruction {
    fn to_message(self) -> Vec<u8>;
}
