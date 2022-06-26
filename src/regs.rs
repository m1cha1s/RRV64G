use crate::XLEN;

pub struct Regs {
    pub x: [XLEN; 32],
    pub pc: XLEN,
}

impl Regs {
    pub fn new() -> Self {
        Regs { x: [0; 32], pc: 0 }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.x = [0; 32];
        self.pc = 0;

        self
    }
}
