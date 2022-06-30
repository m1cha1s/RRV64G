use crate::{
    bus::Bus,
    exceptions::Exception,
    inst::{Inst, ENCODING_TABLE},
    regs::Regs,
};

pub struct Cpu<'a> {
    pub bus: Bus<'a>,
    pub regs: Regs,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: Bus<'a>, regs: Regs) -> Self {
        Cpu { bus, regs }
    }

    pub fn tick(&mut self) -> Result<(), Exception> {
        let inst = self.fetch()?;

        self.execute(inst)?;

        Ok(())
    }

    pub fn reset(&mut self) -> &mut Self {
        self.bus.reset();
        self.regs.reset();
        self
    }

    fn fetch(&mut self) -> Result<u32, Exception> {
        let inst = self.bus.load32(self.regs.pc)?;
        self.regs.pc += 4;
        Ok(inst)
    }

    pub fn decode(inst: u32) -> Result<Inst, Exception> {
        let opcode = inst & 0b1111111;

        if let Some(typ) = &ENCODING_TABLE[opcode as usize] {
            typ.decode(inst)
        } else {
            Err(Exception::UnknownInstruction)
        }
    }

    fn execute(&mut self, inst: u32) -> Result<(), Exception> {
        let inst = Self::decode(inst);

        Ok(())
    }
}
