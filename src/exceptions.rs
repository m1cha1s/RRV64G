use crate::inst::Inst;

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisalignment,
    AddressOutOfBounds(u64),
    UnknownInstruction,
	InstructionNotImplemented(Inst),
	VMExit,
}
