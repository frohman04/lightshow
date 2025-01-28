use ls_controller_protocol::Instruction;

pub(crate) trait InstructionBuilderMeta {
    /// Get the display name for the instruction.
    fn display_name() -> String;
}

pub(crate) trait InstructionBuilder<T: Instruction, A> {
    /// Provide a brief description of the builder.
    fn help(&self) -> String;

    /// Parse the arguments for an instruction builder from the original command invocation.
    fn parse_args(&self, args: Vec<String>) -> Option<A>;

    /// Build an instruction interactively using stdin.
    fn build_instruction(&self, args: A) -> Option<T>;
}
