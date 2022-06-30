pub struct Regs {
    pub x: [u64; 32],
    pub pc: u64,
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
