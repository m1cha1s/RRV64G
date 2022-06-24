use crate::{prelude::MMU, regs::Regs};

pub struct Cpu {
    mmu: MMU,
    regs: Regs,
}

impl Cpu {
    pub fn new(mmu: MMU, regs: Regs) -> Self {
        Cpu { mmu, regs }
    }

    pub fn tick(&mut self) {}
}
