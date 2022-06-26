use crate::{exceptions::Exception, prelude::MMU, regs::Regs, WORD};

pub struct Cpu {
    mmu: MMU,
    regs: Regs,
}

impl Cpu {
    pub fn new(mmu: MMU, regs: Regs) -> Self {
        Cpu { mmu, regs }
    }

    pub fn tick(&mut self) -> Result<(), Exception> {
        let inst = self.fetch()?;

        self.regs.x[0] = 0;
        Ok(())
    }

    pub fn reset(&mut self) -> &mut Self {
        self.mmu.reset();
        self.regs.reset();
        self
    }

    fn fetch(&mut self) -> Result<WORD, Exception> {
        let inst = self.mmu.get_word(self.regs.pc)?;
        self.regs.pc += 4;
        Ok(inst)
    }

    fn execute(&mut self, inst: u32) {}
}
