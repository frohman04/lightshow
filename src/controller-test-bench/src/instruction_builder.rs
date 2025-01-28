use ls_controller_protocol::Instruction;

pub(crate) trait InstructionBuilder<T: Instruction> {
    /// Provide a brief description of the builder.
    fn help(&self) -> String;

    /// Build an instruction interactively using stdin.
    fn build_instruction(&self) -> Option<T>;
}

pub(crate) trait InstructionBuilderMeta {
    /// Get the display name for the instruction.
    fn display_name() -> String;
}
