use ls_controller_protocol::Instruction;

pub(crate) trait InstructionBuilderMeta {
    /// Get the display name for the instruction.
    fn display_name() -> String;
}

pub(crate) trait InstructionBuilder {
    /// Provide a brief description of the builder.
    fn help(&self) -> String;

    /// Build an instruction interactively using stdin.
    fn build_instruction(&self, args: Vec<String>) -> Option<Box<dyn Instruction>>;
}
