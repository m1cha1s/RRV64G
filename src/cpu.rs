use crate::{
    bus::Bus,
    exceptions::Exception,
    inst::{imm_i, rd, rs1, rs2},
    regs::Regs,
    XLEN,
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

    fn execute(&mut self, inst: u32) -> Result<(), Exception> {
        let opcode = inst & 0x7f;

        // TODO: This is not efficient
        let rd = rd(inst) as usize;
        let rs1 = rs1(inst) as usize;
        let rs2 = rs2(inst) as usize;

        let func3 = (inst >> 12) & 0x7;
        let func7 = (inst >> 25) & 0x7f;

        self.regs.x[0] = 0;

        match opcode {
            0x13 => {
                // addi
                let imm = imm_i(inst);
                self.regs.x[rd] = self.regs.x[rs1].wrapping_add(imm as XLEN);

                Ok(())
            }
            0x33 => {
                self.regs.x[rd] = self.regs.x[rs1].wrapping_add(self.regs.x[rs2]);

                Ok(())
            }
            _ => Err(Exception::UnknownInstruction),
        }
    }
}
