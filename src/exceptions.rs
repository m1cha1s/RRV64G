#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisalignment,
    AddressOutOfBounds,
    UnknownInstruction,
	InstructionNotImplemented
}
