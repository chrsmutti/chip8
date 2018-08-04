#[derive(Debug, Fail)]
pub(crate) enum ProcessorError {
    #[fail(display = "uninplemented opcode: {}", opcode)]
    UnimplementedOpcode { opcode: u16 },
}
