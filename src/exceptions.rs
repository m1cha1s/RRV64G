use crate::inst::Inst;

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisalignment,
    AddressOutOfBounds,
    UnknownInstruction,
	InstructionNotImplemented(Inst),
}
