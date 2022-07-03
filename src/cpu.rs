use crate::{
    bus::{Bus, MemMapEntry},
    exceptions::Exception,
    inst::{Inst, ENCODING_TABLE},
    regs::Regs,
};

pub struct Cpu<'a> {
    pub bus: Bus<'a>,
    pub regs: Regs,
}

impl<'a> Cpu<'a> {
    pub fn new(mem_map: &'a mut [MemMapEntry<'a>]) -> Self {
        Cpu { bus: Bus::new(mem_map), regs: Regs::new() }
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

    // TODO: Make it private
    pub fn decode(inst: u32) -> Result<Inst, Exception> {
        let opcode = inst & 0b1111111;

        if let Some(typ) = &ENCODING_TABLE[opcode as usize] {
            typ.decode(inst)
        } else {
            Err(Exception::UnknownInstruction)
        }
    }

    fn execute(&mut self, inst: u32) -> Result<(), Exception> {
        let inst = Self::decode(inst)?;

		match inst {
			Inst::Addi { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1].wrapping_add(imm as u64);
				Ok(())
			},
			Inst::Slti { rd, rs1, imm } => {
				if imm > self.regs.x[rs1] as i64 {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(())
			},
			Inst::Sltiu { rd, rs1, imm } => {
				if imm as u64 > self.regs.x[rs1] {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(())
			},
			Inst::Andi { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1] & imm as u64;
				Ok(())
			},
			Inst::Ori { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1] | imm as u64;
				Ok(())
			},
			Inst::Xori { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1] ^ imm as u64;
				Ok(())
			},
			Inst::Slli { rd, rs1, shamt } => {
				self.regs.x[rd] = self.regs.x[rs1] << shamt;
				Ok(())
			},
			Inst::Srli { rd, rs1, shamt } => {
				self.regs.x[rd] = self.regs.x[rs1] >> shamt;
				Ok(())
			},
			Inst::Srai { rd, rs1, shamt } => {
				self.regs.x[rd] = ((self.regs.x[rs1] as i64) >> shamt) as u64;
				Ok(())
			},
			Inst::Lui { rd, imm } => {
				self.regs.x[rd] = imm as u64;
				Ok(())
			},
			Inst::Auipc { rd, imm } => {
				self.regs.x[rd] = (self.regs.pc as i64).wrapping_add(imm) as u64; 
				Ok(())
			},
			Inst::Add { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1].wrapping_add(self.regs.x[rs2]);
				Ok(())
			},
			Inst::Sub { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1].wrapping_sub(self.regs.x[rs2]);
				Ok(())
			},
			Inst::Slt { rd, rs1, rs2 } => {
				if (self.regs.x[rs1] as i64) < self.regs.x[rs2] as i64 {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(())
			},
			Inst::Sltu { rd, rs1, rs2 } => {
				if self.regs.x[rs1] < self.regs.x[rs2] {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(())
			},
			Inst::Sll { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1] << (self.regs.x[rs2] & 0x0000001f);
				Ok(())
			},
			Inst::Srl { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1] >> (self.regs.x[rs2] & 0x0000001f);
				Ok(())
			},
			Inst::Sra { rd, rs1, rs2 } => {
				self.regs.x[rd] = ((self.regs.x[rs1] as i64) >> (self.regs.x[rs2] & 0x0000001f)) as u64;
				Ok(())
			},
			Inst::Jal { rd, imm } => {
				self.regs.x[rd] = self.regs.pc;
				self.regs.pc = (self.regs.pc as i64).wrapping_add(imm<<2) as u64;
				Ok(())
			},
			Inst::Jalr{ rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.pc;
				self.regs.pc = ((self.regs.x[rs1] as i64).wrapping_add(imm) & (!0b1)) as u64;
				Ok(())
			},
			_ => Err(Exception::InstructionNotImplemented(inst)),
		}
    }
}
