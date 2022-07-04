use crate::{
    bus::{Bus, MemMapEntry, MemType},
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
        let mut cpu = Cpu { bus: Bus::new(mem_map), regs: Regs::new() };
		
		let (_, ram_loc, _) = cpu.bus.mem_map.iter().find(|(typ, _, _)| *typ == MemType::Ram).unwrap();
		cpu.regs.x[2] = ram_loc.start + ram_loc.len;

		cpu
    }

    pub fn tick(&mut self) -> Result<Inst, Exception> {
        let inst = self.fetch()?;

        self.execute(inst)
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

    fn execute(&mut self, inst: u32) -> Result<Inst, Exception> {
        let inst = Self::decode(inst)?;

		self.regs.x[0] = 0;

		match inst {
			Inst::Addi { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1].wrapping_add(imm as u64);
				Ok(inst)
			},
			Inst::Slti { rd, rs1, imm } => {
				if imm > self.regs.x[rs1] as i64 {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Sltiu { rd, rs1, imm } => {
				if imm as u64 > self.regs.x[rs1] {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Andi { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1] & imm as u64;
				Ok(inst)
			},
			Inst::Ori { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1] | imm as u64;
				Ok(inst)
			},
			Inst::Xori { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.x[rs1] ^ imm as u64;
				Ok(inst)
			},
			Inst::Slli { rd, rs1, shamt } => {
				self.regs.x[rd] = self.regs.x[rs1] << shamt;
				Ok(inst)
			},
			Inst::Srli { rd, rs1, shamt } => {
				self.regs.x[rd] = self.regs.x[rs1] >> shamt;
				Ok(inst)
			},
			Inst::Srai { rd, rs1, shamt } => {
				self.regs.x[rd] = ((self.regs.x[rs1] as i64) >> shamt) as u64;
				Ok(inst)
			},
			Inst::Addiw { rd, rs1, imm } => {
				self.regs.x[rd] = (self.regs.x[rs1] as i32).wrapping_add(imm as i32) as u64;
				Ok(inst)
			},
			Inst::Slliw { rd, rs1, shamt } => {
				self.regs.x[rd] = ((self.regs.x[rs1] as u32) << shamt) as i32 as u64;
				Ok(inst)
			},
			Inst::Srliw { rd, rs1, shamt } => {
				self.regs.x[rd] = ((self.regs.x[rs1] as u32) >> shamt) as i32 as u64;
				Ok(inst)
			},
			Inst::Sraiw { rd, rs1, shamt } => {
				self.regs.x[rd] = ((self.regs.x[rs1] as i32) >> shamt) as i32 as u64;
				Ok(inst)
			},
			Inst::Lui { rd, imm } => {
				self.regs.x[rd] = imm as u64;
				Ok(inst)
			},
			Inst::Auipc { rd, imm } => {
				self.regs.x[rd] = (self.regs.pc).wrapping_add(imm as u64); 
				Ok(inst)
			},
			Inst::Add { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1].wrapping_add(self.regs.x[rs2]);
				Ok(inst)
			},
			Inst::Sub { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1].wrapping_sub(self.regs.x[rs2]);
				Ok(inst)
			},
			Inst::Slt { rd, rs1, rs2 } => {
				if (self.regs.x[rs1] as i64) < self.regs.x[rs2] as i64 {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Sltu { rd, rs1, rs2 } => {
				if self.regs.x[rs1] < self.regs.x[rs2] {
					self.regs.x[rd] = 1;
				} else {
					self.regs.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Sll { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1] << (self.regs.x[rs2] & 0x0000001f);
				Ok(inst)
			},
			Inst::Srl { rd, rs1, rs2 } => {
				self.regs.x[rd] = self.regs.x[rs1] >> (self.regs.x[rs2] & 0x0000001f);
				Ok(inst)
			},
			Inst::Sra { rd, rs1, rs2 } => {
				self.regs.x[rd] = ((self.regs.x[rs1] as i64) >> (self.regs.x[rs2] & 0x0000001f)) as u64;
				Ok(inst)
			},
			Inst::Addw { rd, rs1, rs2} => {
				let x = (self.regs.x[rs1] & 0xffffffff) as i32;
				let y = (self.regs.x[rs2] & 0xffffffff) as i32;
				self.regs.x[rd] = x.wrapping_add(y) as i64 as u64;
				Ok(inst)
			},
			Inst::Subw { rd, rs1, rs2} => {
				let x = (self.regs.x[rs1] & 0xffffffff) as i32;
				let y = (self.regs.x[rs2] & 0xffffffff) as i32;
				self.regs.x[rd] = x.wrapping_sub(y) as i64 as u64;
				Ok(inst)
			},
			Inst::Sllw { rd, rs1, rs2 } => {
				let x = (self.regs.x[rs1] & 0xffffffff) as u32;
				let y = (self.regs.x[rs2] & 0xffffffff) as u32;
				self.regs.x[rd] = (x << (y & 0x1f)) as u64;
				Ok(inst)
			},
			Inst::Srlw { rd, rs1, rs2 } => {
				let x = (self.regs.x[rs1] & 0xffffffff) as u32;
				let y = (self.regs.x[rs2] & 0xffffffff) as u32;
				self.regs.x[rd] = (x >> (y & 0x1f)) as u64;
				Ok(inst)
			},
			Inst::Srlw { rd, rs1, rs2 } => {
				let x = (self.regs.x[rs1] & 0xffffffff) as i32;
				let y = (self.regs.x[rs2] & 0xffffffff) as i32;
				self.regs.x[rd] = (x >> (y & 0x1f)) as u64;
				Ok(inst)
			},
			Inst::Jal { rd, imm } => {
				self.regs.x[rd] = self.regs.pc;
				self.regs.pc = (self.regs.pc).wrapping_add(imm as u64).wrapping_sub(4);
				Ok(inst)
			},
			Inst::Jalr { rd, rs1, imm } => {
				self.regs.x[rd] = self.regs.pc;
				self.regs.pc = (self.regs.x[rs1]).wrapping_add(imm as u64) & (!0b1);
				Ok(inst)
			},
			Inst::Beq { rs1, rs2, imm } => {
				if self.regs.x[rs1] == self.regs.x[rs2] {
					self.regs.pc = (self.regs.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bne { rs1, rs2, imm } => {
				if self.regs.x[rs1] != self.regs.x[rs2] {
					self.regs.pc = (self.regs.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Blt { rs1, rs2, imm } => {
				if (self.regs.x[rs1] as i64) < self.regs.x[rs2] as i64 {
					self.regs.pc = (self.regs.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bltu { rs1, rs2, imm } => {
				if self.regs.x[rs1] < self.regs.x[rs2] {
					self.regs.pc = (self.regs.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bge { rs1, rs2, imm } => {
				if (self.regs.x[rs1] as i64) >= self.regs.x[rs2] as i64 {
					self.regs.pc = (self.regs.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bgeu { rs1, rs2, imm } => {
				if self.regs.x[rs1] >= self.regs.x[rs2] {
					self.regs.pc = (self.regs.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Ld { rd, rs1, imm } => {
				self.regs.x[rd] = self.bus.load64((self.regs.x[rs1]).wrapping_add(imm as u64))?;
				Ok(inst)
			},
			Inst::Lw { rd, rs1, imm } => {
				self.regs.x[rd] = self.bus.load32((self.regs.x[rs1]).wrapping_add(imm as u64))? as i32 as i64 as u64;
				Ok(inst)
			},
			Inst::Lwu { rd, rs1, imm } => {
				self.regs.x[rd] = self.bus.load32((self.regs.x[rs1]).wrapping_add(imm as u64))? as u64;
				Ok(inst)
			},
			Inst::Lh { rd, rs1, imm } => {
				self.regs.x[rd] = self.bus.load16((self.regs.x[rs1]).wrapping_add(imm as u64))? as i16 as i64 as u64;
				Ok(inst)
			},
			Inst::Lhu { rd, rs1, imm } => {
				self.regs.x[rd] = self.bus.load16((self.regs.x[rs1]).wrapping_add(imm as u64))? as u64;
				Ok(inst)
			},
			Inst::Lb { rd, rs1, imm } => {
				self.regs.x[rd] = self.bus.load8((self.regs.x[rs1]).wrapping_add(imm as u64))? as i8 as i64 as u64;
				Ok(inst)
			},
			Inst::Lbu { rd, rs1, imm } => {
				self.regs.x[rd] = self.bus.load8((self.regs.x[rs1]).wrapping_add(imm as u64))? as u64;
				Ok(inst)
			},
			Inst::Sd { rs1, rs2, imm } => {
				self.bus.store64((self.regs.x[rs1]).wrapping_add(imm as u64), self.regs.x[rs2])?;
				Ok(inst)
			},
			Inst::Sw { rs1, rs2, imm } => {
				self.bus.store32((self.regs.x[rs1]).wrapping_add(imm as u64), (self.regs.x[rs2] & 0xffffffff) as u32)?;
				Ok(inst)
			},
			Inst::Sh { rs1, rs2, imm } => {
				self.bus.store16((self.regs.x[rs1]).wrapping_add(imm as u64), (self.regs.x[rs2] & 0xffff) as u16)?;
				Ok(inst)
			},
			Inst::Sd { rs1, rs2, imm } => {
				self.bus.store8((self.regs.x[rs1]).wrapping_add(imm as u64), (self.regs.x[rs2] & 0xff) as u8)?;
				Ok(inst)
			},
			Inst::Fence { rd, rs1, imm } => Ok(inst),
			Inst::Ecall {  } => Ok(inst),
			Inst::Ebreak {  } => Ok(inst),
			_ => Err(Exception::InstructionNotImplemented(inst)),
		}
    }
}
