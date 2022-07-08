use crate::{
    bus::Bus,
    exceptions::Exception,
    inst::{Inst, ENCODING_TABLE},
};

pub struct Cpu {
    pub x: [u64; 32],

	pub pc: u64,
}

impl Cpu {
    pub fn new() -> Self {
        let cpu = Cpu { x: [0; 32], pc: 0 };
		
		cpu
    }

    pub fn tick(&mut self, bus: &mut Bus) -> Result<Inst, Exception> {
        let inst = self.fetch(bus)?;

        self.execute(inst, bus)
    }

    pub fn reset(&mut self) -> &mut Self {
		self.pc = 0;
		self.x = [0; 32];

        self
    }

    fn fetch(&mut self, bus: &mut Bus) -> Result<u32, Exception> {
        let inst = bus.load32(self.pc)?;
		self.pc += 4;
        Ok(inst)
    }

    fn decode(inst: u32) -> Result<Inst, Exception> {
        let opcode = inst & 0b1111111;

        if let Some(typ) = &ENCODING_TABLE[opcode as usize] {
            typ.decode(inst)
        } else {
            Err(Exception::UnknownInstruction)
        }
    }

    fn execute(&mut self, inst: u32, bus: &mut Bus) -> Result<Inst, Exception> {
        let inst = Self::decode(inst)?;

		self.x[0] = 0;

		match inst {
			Inst::Addi { rd, rs1, imm } => {
				self.x[rd] = self.x[rs1].wrapping_add(imm as u64);
				Ok(inst)
			},
			Inst::Slti { rd, rs1, imm } => {
				if imm > self.x[rs1] as i64 {
					self.x[rd] = 1;
				} else {
					self.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Sltiu { rd, rs1, imm } => {
				if imm as u64 > self.x[rs1] {
					self.x[rd] = 1;
				} else {
					self.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Andi { rd, rs1, imm } => {
				self.x[rd] = self.x[rs1] & imm as u64;
				Ok(inst)
			},
			Inst::Ori { rd, rs1, imm } => {
				self.x[rd] = self.x[rs1] | imm as u64;
				Ok(inst)
			},
			Inst::Xori { rd, rs1, imm } => {
				self.x[rd] = self.x[rs1] ^ imm as u64;
				Ok(inst)
			},
			Inst::Slli { rd, rs1, shamt } => {
				self.x[rd] = self.x[rs1] << shamt;
				Ok(inst)
			},
			Inst::Srli { rd, rs1, shamt } => {
				self.x[rd] = self.x[rs1] >> shamt;
				Ok(inst)
			},
			Inst::Srai { rd, rs1, shamt } => {
				self.x[rd] = ((self.x[rs1] as i64) >> shamt) as u64;
				Ok(inst)
			},
			Inst::Addiw { rd, rs1, imm } => {
				self.x[rd] = (self.x[rs1] as i32).wrapping_add(imm as i32) as u64;
				Ok(inst)
			},
			Inst::Slliw { rd, rs1, shamt } => {
				self.x[rd] = ((self.x[rs1] as u32) << shamt) as i32 as u64;
				Ok(inst)
			},
			Inst::Srliw { rd, rs1, shamt } => {
				self.x[rd] = ((self.x[rs1] as u32) >> shamt) as i32 as u64;
				Ok(inst)
			},
			Inst::Sraiw { rd, rs1, shamt } => {
				self.x[rd] = ((self.x[rs1] as i32) >> shamt) as i32 as u64;
				Ok(inst)
			},
			Inst::Lui { rd, imm } => {
				self.x[rd] = imm as u64;
				Ok(inst)
			},
			Inst::Auipc { rd, imm } => {
				self.x[rd] = (self.pc).wrapping_add(imm as u64); 
				Ok(inst)
			},
			Inst::Add { rd, rs1, rs2 } => {
				self.x[rd] = self.x[rs1].wrapping_add(self.x[rs2]);
				Ok(inst)
			},
			Inst::Sub { rd, rs1, rs2 } => {
				self.x[rd] = self.x[rs1].wrapping_sub(self.x[rs2]);
				Ok(inst)
			},
			Inst::Slt { rd, rs1, rs2 } => {
				if (self.x[rs1] as i64) < self.x[rs2] as i64 {
					self.x[rd] = 1;
				} else {
					self.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Sltu { rd, rs1, rs2 } => {
				if self.x[rs1] < self.x[rs2] {
					self.x[rd] = 1;
				} else {
					self.x[rd] = 0;
				}
				Ok(inst)
			},
			Inst::Sll { rd, rs1, rs2 } => {
				self.x[rd] = self.x[rs1] << (self.x[rs2] & 0x0000001f);
				Ok(inst)
			},
			Inst::Srl { rd, rs1, rs2 } => {
				self.x[rd] = self.x[rs1] >> (self.x[rs2] & 0x0000001f);
				Ok(inst)
			},
			Inst::Sra { rd, rs1, rs2 } => {
				self.x[rd] = ((self.x[rs1] as i64) >> (self.x[rs2] & 0x0000001f)) as u64;
				Ok(inst)
			},
			Inst::Addw { rd, rs1, rs2} => {
				let x = (self.x[rs1] & 0xffffffff) as i32;
				let y = (self.x[rs2] & 0xffffffff) as i32;
				self.x[rd] = x.wrapping_add(y) as i64 as u64;
				Ok(inst)
			},
			Inst::Subw { rd, rs1, rs2} => {
				let x = (self.x[rs1] & 0xffffffff) as i32;
				let y = (self.x[rs2] & 0xffffffff) as i32;
				self.x[rd] = x.wrapping_sub(y) as i64 as u64;
				Ok(inst)
			},
			Inst::Sllw { rd, rs1, rs2 } => {
				let x = (self.x[rs1] & 0xffffffff) as u32;
				let y = (self.x[rs2] & 0xffffffff) as u32;
				self.x[rd] = (x << (y & 0x1f)) as u64;
				Ok(inst)
			},
			Inst::Srlw { rd, rs1, rs2 } => {
				let x = (self.x[rs1] & 0xffffffff) as u32;
				let y = (self.x[rs2] & 0xffffffff) as u32;
				self.x[rd] = (x >> (y & 0x1f)) as u64;
				Ok(inst)
			},
			Inst::Sraw { rd, rs1, rs2 } => {
				let x = (self.x[rs1] & 0xffffffff) as i32;
				let y = (self.x[rs2] & 0xffffffff) as i32;
				self.x[rd] = (x >> (y & 0x1f)) as u64;
				Ok(inst)
			},
			Inst::Jal { rd, imm } => {
				self.x[rd] = self.pc;
				self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
				Ok(inst)
			},
			Inst::Jalr { rd, rs1, imm } => {
				self.x[rd] = self.pc;
				self.pc = (self.x[rs1]).wrapping_add(imm as u64) & (!0b1);
				Ok(inst)
			},
			Inst::Beq { rs1, rs2, imm } => {
				if self.x[rs1] == self.x[rs2] {
					self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bne { rs1, rs2, imm } => {
				if self.x[rs1] != self.x[rs2] {
					self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Blt { rs1, rs2, imm } => {
				if (self.x[rs1] as i64) < self.x[rs2] as i64 {
					self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bltu { rs1, rs2, imm } => {
				if self.x[rs1] < self.x[rs2] {
					self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bge { rs1, rs2, imm } => {
				if (self.x[rs1] as i64) >= (self.x[rs2] as i64) {
					self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Bgeu { rs1, rs2, imm } => {
				if self.x[rs1] >= self.x[rs2] {
					self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
				}
				Ok(inst)
			},
			Inst::Ld { rd, rs1, imm } => {
				self.x[rd] = bus.load64((self.x[rs1]).wrapping_add(imm as u64))?;
				Ok(inst)
			},
			Inst::Lw { rd, rs1, imm } => {
				self.x[rd] = bus.load32((self.x[rs1]).wrapping_add(imm as u64))? as i32 as i64 as u64;
				Ok(inst)
			},
			Inst::Lwu { rd, rs1, imm } => {
				self.x[rd] = bus.load32((self.x[rs1]).wrapping_add(imm as u64))? as u64;
				Ok(inst)
			},
			Inst::Lh { rd, rs1, imm } => {
				self.x[rd] = bus.load16((self.x[rs1]).wrapping_add(imm as u64))? as i16 as i64 as u64;
				Ok(inst)
			},
			Inst::Lhu { rd, rs1, imm } => {
				self.x[rd] = bus.load16((self.x[rs1]).wrapping_add(imm as u64))? as u64;
				Ok(inst)
			},
			Inst::Lb { rd, rs1, imm } => {
				self.x[rd] = bus.load8((self.x[rs1]).wrapping_add(imm as u64))? as i8 as i64 as u64;
				Ok(inst)
			},
			Inst::Lbu { rd, rs1, imm } => {
				self.x[rd] = bus.load8((self.x[rs1]).wrapping_add(imm as u64))? as u64;
				Ok(inst)
			},
			Inst::Sd { rs1, rs2, imm } => {
				bus.store64((self.x[rs1]).wrapping_add(imm as u64), self.x[rs2])?;
				Ok(inst)
			},
			Inst::Sw { rs1, rs2, imm } => {
				bus.store32((self.x[rs1]).wrapping_add(imm as u64), (self.x[rs2] & 0xffffffff) as u32)?;
				Ok(inst)
			},
			Inst::Sh { rs1, rs2, imm } => {
				bus.store16((self.x[rs1]).wrapping_add(imm as u64), (self.x[rs2] & 0xffff) as u16)?;
				Ok(inst)
			},
			Inst::Sb { rs1, rs2, imm } => {
				bus.store8((self.x[rs1]).wrapping_add(imm as u64), (self.x[rs2] & 0xff) as u8)?;
				Ok(inst)
			},
			Inst::Fence { rd: _rd, rs1: _rs1, imm: _imm } => Ok(inst),
			Inst::Ecall {  } => Ok(inst),
			Inst::Ebreak {  } => Ok(inst),
			_ => Err(Exception::InstructionNotImplemented(inst)),
		}
    }
}
