use crate::{exceptions::Exception, prelude::MMU, regs::Regs};

enum OpCode {}

pub struct Cpu {
    mmu: MMU,
    regs: Regs,
}

impl Cpu {
    pub fn new(mmu: MMU, regs: Regs) -> Self {
        Cpu { mmu, regs }
    }

    pub fn clock(&mut self) -> Result<(), Exception> {
        Ok(())
    }

    pub fn reset(&mut self) -> &mut Self {
        self
    }
}
