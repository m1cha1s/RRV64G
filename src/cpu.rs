use crate::{
    bus::Bus,
    exceptions::Exception,
    inst::{imm_i, imm_s, rd, rs1, rs2},
    regs::Regs,
    IXLEN, XLEN,
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
                // Add immidiet instruction.
                let imm = imm_i(inst);
                self.regs.x[rd] = self.regs.x[rs1].wrapping_add(imm as XLEN);

                Ok(())
            }
            0x33 => {
                // Add instruction.
                self.regs.x[rd] = self.regs.x[rs1].wrapping_add(self.regs.x[rs2]);

                Ok(())
            }
            0x03 => {
                // Load instructions.
                let imm = imm_i(inst);
                let addr = self.regs.x[rs1].wrapping_add(imm);

                match func3 {
                    0x0 => {
                        // lb
                        let val = self.bus.load8(addr)?;
                        self.regs.x[rd] = val as i8 as IXLEN as XLEN;
                    }
                    0x1 => {
                        // lh
                        let val = self.bus.load16(addr)?;
                        self.regs.x[rd] = val as i16 as IXLEN as XLEN;
                    }
                    0x2 => {
                        // lw
                        let val = self.bus.load32(addr)?;
                        self.regs.x[rd] = val as i32 as IXLEN as XLEN;
                    }
                    #[cfg(feature = "rv64i")]
                    0x3 => {
                        // ld
                        let val = self.bus.load64(addr)?;
                        self.regs.x[rd] = val;
                    }
                    0x4 => {
                        // lbu
                        let val = self.bus.load8(addr)?;
                        self.regs.x[rd] = val as XLEN;
                    }
                    0x5 => {
                        // lhu
                        let val = self.bus.load16(addr)?;
                        self.regs.x[rd] = val as XLEN;
                    }
                    #[cfg(feature = "rv64i")]
                    0x6 => {
                        // lwu
                        let val = self.bus.load32(addr)?;
                        self.regs.x[rd] = val as XLEN;
                    }
                    _ => {}
                }
                Ok(())
            }
            0x23 => {
                // Store instructions.

                let imm = imm_s(inst);
                let addr = self.regs.x[rs1].wrapping_add(imm);

                match func3 {
                    0x0 => self.bus.store8(addr, self.regs.x[rs2] as u8)?, // sb
                    0x1 => self.bus.store16(addr, self.regs.x[rs2] as u16)?, // sh
                    0x2 => self.bus.store32(addr, self.regs.x[rs2] as u32)?, // sw
                    #[cfg(feature = "rv64i")]
                    0x3 => self.bus.store64(addr, self.regs.x[rs2] as u64)?, // sd
                    _ => {}
                }

                Ok(())
            }
            _ => Err(Exception::UnknownInstruction),
        }
    }
}
