type Mode = u64;
const USER: Mode = 0b00;
const SUPERVISOR: Mode = 0b00;
const MACHINE: Mode = 0b00;

use crate::{
    bus::Bus,
    csrs::*,
    exceptions::Exception,
    inst::{Inst, ENCODING_TABLE},
};

pub struct Cpu {
    pub x: [u64; 32],

    pub pc: u64,

    pub mode: Mode,

    pub csr: [u64; 4096],
}

impl Cpu {
    pub fn new() -> Self {
        let cpu = Cpu {
            x: [0; 32],
            pc: 0,
            csr: [0; 4096],
            mode: MACHINE,
        };

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

    pub fn handle_exception(&mut self, e: Exception) {
        let pc = self.pc;
        let mode = self.mode;
        let cause = e.code();

        // TODO: Stopped here in development
        let trap_in_s_mode =
            mode <= SUPERVISOR && (self.csr[MEDELEG].wrapping_shr(cause as u32) & 1) == 1;
        let (STATUS, TVEC, CAUSE, TVAL, EPC, MASK_PIE, pie_i, MASK_IE, ie_i, MASK_PP, pp_i) =
            if trap_in_s_mode {
                self.mode = SUPERVISOR;
                (
                    SSTATUS, STVEC, SCAUSE, STVAL, SEPC, MASK_SPIE, 5, MASK_SIE, 1, MASK_SPP, 8,
                )
            } else {
                self.mode = MACHINE;
                (
                    MSTATUS, MTVEC, MCAUSE, MTVAL, MEPC, MASK_MPIE, 7, MASK_MIE, 3, MASK_MPP, 11,
                )
            };

        self.pc = self.csr[TVEC] & !0b11;

        self.csr[EPC] = pc;

        self.csr[CAUSE] = cause;

        self.csr[TVAL] = e.value();

        let mut status = self.csr[STATUS];
        let ie = (status & MASK_IE) >> ie_i;

        status = (status & !MASK_PIE) | (ie << pie_i);
        status &= !MASK_IE;
        status = (status & !MASK_PP) | (mode << pp_i);

        self.csr[STATUS] = status;
    }

    fn fetch(&mut self, bus: &mut Bus) -> Result<u32, Exception> {
        let inst = bus.load32(self.pc)?;
        self.pc += 4;
        Ok(inst)
    }

    fn decode(&self, inst: u32) -> Result<Inst, Exception> {
        let opcode = inst & 0b1111111;

        if let Some(typ) = &ENCODING_TABLE[opcode as usize] {
            typ.decode(inst)
        } else {
            Err(Exception::IllegalInstruction(inst.into()))
        }
    }

    fn execute(&mut self, inst: u32, bus: &mut Bus) -> Result<Inst, Exception> {
        let inst = self.decode(inst)?;

        self.x[0] = 0;

        match inst {
            Inst::Addi { rd, rs1, imm } => {
                self.x[rd] = self.x[rs1].wrapping_add(imm as u64);
                Ok(inst)
            }
            Inst::Slti { rd, rs1, imm } => {
                if imm > self.x[rs1] as i64 {
                    self.x[rd] = 1;
                } else {
                    self.x[rd] = 0;
                }
                Ok(inst)
            }
            Inst::Sltiu { rd, rs1, imm } => {
                if imm as u64 > self.x[rs1] {
                    self.x[rd] = 1;
                } else {
                    self.x[rd] = 0;
                }
                Ok(inst)
            }
            Inst::Andi { rd, rs1, imm } => {
                self.x[rd] = self.x[rs1] & imm as u64;
                Ok(inst)
            }
            Inst::Ori { rd, rs1, imm } => {
                self.x[rd] = self.x[rs1] | imm as u64;
                Ok(inst)
            }
            Inst::Xori { rd, rs1, imm } => {
                self.x[rd] = self.x[rs1] ^ imm as u64;
                Ok(inst)
            }
            Inst::Slli { rd, rs1, shamt } => {
                self.x[rd] = self.x[rs1] << shamt;
                Ok(inst)
            }
            Inst::Srli { rd, rs1, shamt } => {
                self.x[rd] = self.x[rs1] >> shamt;
                Ok(inst)
            }
            Inst::Srai { rd, rs1, shamt } => {
                self.x[rd] = ((self.x[rs1] as i64) >> shamt) as u64;
                Ok(inst)
            }
            Inst::Addiw { rd, rs1, imm } => {
                self.x[rd] = (self.x[rs1] as i32).wrapping_add(imm as i32) as u64;
                Ok(inst)
            }
            Inst::Slliw { rd, rs1, shamt } => {
                self.x[rd] = ((self.x[rs1] as u32) << shamt) as i32 as u64;
                Ok(inst)
            }
            Inst::Srliw { rd, rs1, shamt } => {
                self.x[rd] = ((self.x[rs1] as u32) >> shamt) as i32 as u64;
                Ok(inst)
            }
            Inst::Sraiw { rd, rs1, shamt } => {
                self.x[rd] = ((self.x[rs1] as i32) >> shamt) as i32 as u64;
                Ok(inst)
            }
            Inst::Lui { rd, imm } => {
                self.x[rd] = imm as u64;
                Ok(inst)
            }
            Inst::Auipc { rd, imm } => {
                self.x[rd] = (self.pc).wrapping_add(imm as u64);
                Ok(inst)
            }
            Inst::Add { rd, rs1, rs2 } => {
                self.x[rd] = self.x[rs1].wrapping_add(self.x[rs2]);
                Ok(inst)
            }
            Inst::Sub { rd, rs1, rs2 } => {
                self.x[rd] = self.x[rs1].wrapping_sub(self.x[rs2]);
                Ok(inst)
            }
            Inst::Slt { rd, rs1, rs2 } => {
                if (self.x[rs1] as i64) < self.x[rs2] as i64 {
                    self.x[rd] = 1;
                } else {
                    self.x[rd] = 0;
                }
                Ok(inst)
            }
            Inst::Sltu { rd, rs1, rs2 } => {
                if self.x[rs1] < self.x[rs2] {
                    self.x[rd] = 1;
                } else {
                    self.x[rd] = 0;
                }
                Ok(inst)
            }
            Inst::Sll { rd, rs1, rs2 } => {
                self.x[rd] = self.x[rs1] << (self.x[rs2] & 0x0000001f);
                Ok(inst)
            }
            Inst::Srl { rd, rs1, rs2 } => {
                self.x[rd] = self.x[rs1] >> (self.x[rs2] & 0x0000001f);
                Ok(inst)
            }
            Inst::Sra { rd, rs1, rs2 } => {
                self.x[rd] = ((self.x[rs1] as i64) >> (self.x[rs2] & 0x0000001f)) as u64;
                Ok(inst)
            }
            Inst::Addw { rd, rs1, rs2 } => {
                let x = (self.x[rs1] & 0xffffffff) as i32;
                let y = (self.x[rs2] & 0xffffffff) as i32;
                self.x[rd] = x.wrapping_add(y) as i64 as u64;
                Ok(inst)
            }
            Inst::Subw { rd, rs1, rs2 } => {
                let x = (self.x[rs1] & 0xffffffff) as i32;
                let y = (self.x[rs2] & 0xffffffff) as i32;
                self.x[rd] = x.wrapping_sub(y) as i64 as u64;
                Ok(inst)
            }
            Inst::Sllw { rd, rs1, rs2 } => {
                let x = (self.x[rs1] & 0xffffffff) as u32;
                let y = (self.x[rs2] & 0xffffffff) as u32;
                self.x[rd] = (x << (y & 0x1f)) as u64;
                Ok(inst)
            }
            Inst::Srlw { rd, rs1, rs2 } => {
                let x = (self.x[rs1] & 0xffffffff) as u32;
                let y = (self.x[rs2] & 0xffffffff) as u32;
                self.x[rd] = (x >> (y & 0x1f)) as u64;
                Ok(inst)
            }
            Inst::Sraw { rd, rs1, rs2 } => {
                let x = (self.x[rs1] & 0xffffffff) as i32;
                let y = (self.x[rs2] & 0xffffffff) as i32;
                self.x[rd] = (x >> (y & 0x1f)) as u64;
                Ok(inst)
            }
            Inst::Jal { rd, imm } => {
                self.x[rd] = self.pc;
                self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
                Ok(inst)
            }
            Inst::Jalr { rd, rs1, imm } => {
                self.x[rd] = self.pc;
                self.pc = (self.x[rs1]).wrapping_add(imm as u64) & (!0b1);
                Ok(inst)
            }
            Inst::Beq { rs1, rs2, imm } => {
                if self.x[rs1] == self.x[rs2] {
                    self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
                }
                Ok(inst)
            }
            Inst::Bne { rs1, rs2, imm } => {
                if self.x[rs1] != self.x[rs2] {
                    self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
                }
                Ok(inst)
            }
            Inst::Blt { rs1, rs2, imm } => {
                if (self.x[rs1] as i64) < self.x[rs2] as i64 {
                    self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
                }
                Ok(inst)
            }
            Inst::Bltu { rs1, rs2, imm } => {
                if self.x[rs1] < self.x[rs2] {
                    self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
                }
                Ok(inst)
            }
            Inst::Bge { rs1, rs2, imm } => {
                if (self.x[rs1] as i64) >= (self.x[rs2] as i64) {
                    self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
                }
                Ok(inst)
            }
            Inst::Bgeu { rs1, rs2, imm } => {
                if self.x[rs1] >= self.x[rs2] {
                    self.pc = (self.pc).wrapping_add(imm as u64).wrapping_sub(4);
                }
                Ok(inst)
            }
            Inst::Ld { rd, rs1, imm } => {
                self.x[rd] = bus.load64((self.x[rs1]).wrapping_add(imm as u64))?;
                Ok(inst)
            }
            Inst::Lw { rd, rs1, imm } => {
                self.x[rd] =
                    bus.load32((self.x[rs1]).wrapping_add(imm as u64))? as i32 as i64 as u64;
                Ok(inst)
            }
            Inst::Lwu { rd, rs1, imm } => {
                self.x[rd] = bus.load32((self.x[rs1]).wrapping_add(imm as u64))? as u64;
                Ok(inst)
            }
            Inst::Lh { rd, rs1, imm } => {
                self.x[rd] =
                    bus.load16((self.x[rs1]).wrapping_add(imm as u64))? as i16 as i64 as u64;
                Ok(inst)
            }
            Inst::Lhu { rd, rs1, imm } => {
                self.x[rd] = bus.load16((self.x[rs1]).wrapping_add(imm as u64))? as u64;
                Ok(inst)
            }
            Inst::Lb { rd, rs1, imm } => {
                self.x[rd] = bus.load8((self.x[rs1]).wrapping_add(imm as u64))? as i8 as i64 as u64;
                Ok(inst)
            }
            Inst::Lbu { rd, rs1, imm } => {
                self.x[rd] = bus.load8((self.x[rs1]).wrapping_add(imm as u64))? as u64;
                Ok(inst)
            }
            Inst::Sd { rs1, rs2, imm } => {
                bus.store64((self.x[rs1]).wrapping_add(imm as u64), self.x[rs2])?;
                Ok(inst)
            }
            Inst::Sw { rs1, rs2, imm } => {
                bus.store32(
                    (self.x[rs1]).wrapping_add(imm as u64),
                    (self.x[rs2] & 0xffffffff) as u32,
                )?;
                Ok(inst)
            }
            Inst::Sh { rs1, rs2, imm } => {
                bus.store16(
                    (self.x[rs1]).wrapping_add(imm as u64),
                    (self.x[rs2] & 0xffff) as u16,
                )?;
                Ok(inst)
            }
            Inst::Sb { rs1, rs2, imm } => {
                bus.store8(
                    (self.x[rs1]).wrapping_add(imm as u64),
                    (self.x[rs2] & 0xff) as u8,
                )?;
                Ok(inst)
            }
            Inst::Fence {
                rd: _rd,
                rs1: _rs1,
                imm: _imm,
            } => Ok(inst),
            Inst::Sfencevma => Ok(inst),
            Inst::Ecall {} => Ok(inst),
            Inst::Ebreak {} => Ok(inst),

            // CSRs implementation
            Inst::Csrrw { rd, rs1, csr } => {
                if rd != 0 {
                    self.x[rd] = self.csr[csr];
                }

                self.csr[csr] = self.x[rs1];
                Ok(inst)
            }
            Inst::Csrrs { rd, rs1, csr } => {
                self.x[rd] = self.csr[csr];

                if rs1 != 0 {
                    self.csr[csr] |= self.x[rs1];
                }
                Ok(inst)
            }
            Inst::Csrrc { rd, rs1, csr } => {
                self.x[rd] = self.csr[csr];

                if rs1 != 0 {
                    self.csr[csr] &= !self.x[rs1];
                }
                Ok(inst)
            }
            Inst::Csrrwi { rd, uimm, csr } => {
                if rd != 0 {
                    self.x[rd] = self.csr[csr];
                }

                self.csr[csr] = uimm;
                Ok(inst)
            }
            Inst::Csrrsi { rd, uimm, csr } => {
                self.x[rd] = self.csr[csr];

                if uimm != 0 {
                    self.csr[csr] |= uimm;
                }
                Ok(inst)
            }
            Inst::Csrrci { rd, uimm, csr } => {
                self.x[rd] = self.csr[csr];

                if uimm != 0 {
                    self.csr[csr] &= !uimm;
                }
                Ok(inst)
            }
            Inst::Sret => {
                let mut sstatus = self.csr[SSTATUS];
                self.mode = (sstatus & MASK_SPP) >> 8;
                let spie = (sstatus & MASK_SPIE) >> 5;
                sstatus = (sstatus & !MASK_SIE) | (spie << 1);
                sstatus |= MASK_SPIE;
                sstatus &= !MASK_SPP;
                self.csr[SSTATUS] = sstatus;

                self.pc = self.csr[SEPC] & !0b11;

                Ok(inst)
            }
            Inst::Mret => {
                let mut mstatus = self.csr[MSTATUS];

                self.mode = (mstatus & MASK_MPP) >> 11;
                let mpie = (mstatus & MASK_MPIE) >> 7;
                mstatus = (mstatus & !MASK_MIE) | (mpie << 3);
                mstatus |= MASK_MPIE;
                mstatus &= !MASK_MPP;
                mstatus &= !MASK_MPRV;
                self.csr[MSTATUS] = mstatus;

                self.pc = self.csr[MEPC] & !0b11;

                Ok(inst)
            }
            Inst::Mul { rd, rs1, rs2 } => {
                self.x[rd] = self.x[rs1].wrapping_mul(self.x[rs2]);

                Ok(inst)
            }
            Inst::Div { rd, rs1, rs2 } => {
                self.x[rd] = self.x[rs1].wrapping_div(self.x[rs2]);

                Ok(inst)
            }
            Inst::Divu { rd, rs1, rs2 } => {
                self.x[rd] = match self.x[rs2] {
                    0 => 0xffff_ffff_ffff_ffff,
                    _ => {
                        let dividend = self.x[rs1];
                        let divisor = self.x[rs2];

                        dividend.wrapping_div(divisor)
                    }
                };

                Ok(inst)
            }
            Inst::Remuw { rd, rs1, rs2 } => {
                self.x[rd] = match self.x[rs2] {
                    0 => self.x[rs1],
                    _ => {
                        let dividend = self.x[rs1] as u32;
                        let divisor = self.x[rs2] as u32;
                        dividend.wrapping_rem(divisor) as i32 as u64
                    }
                };

                Ok(inst)
            }
            Inst::Amoaddw {
                rd,
                rs1,
                rs2,
                aq: _aq,
                rl: _rl,
            } => {
                let t = bus.load32(self.x[rs1])? as u64;
                bus.store32(self.x[rs1], t.wrapping_add(self.x[rs2]) as u32)?;
                self.x[rd] = t;

                Ok(inst)
            }
            Inst::Amoaddd {
                rd,
                rs1,
                rs2,
                aq: _aq,
                rl: _rl,
            } => {
                let t = bus.load64(self.x[rs1])?;
                bus.store64(self.x[rs1], t.wrapping_add(self.x[rs2]))?;
                self.x[rd] = t;

                Ok(inst)
            }
            Inst::Amoswapw {
                rd,
                rs1,
                rs2,
                aq: _aq,
                rl: _rl,
            } => {
                let t = bus.load32(self.x[rs1])?;
                bus.store32(self.x[rs1], self.x[rs2] as u32)?;
                self.x[rd] = t as u64;

                Ok(inst)
            }
            Inst::Amoswapd {
                rd,
                rs1,
                rs2,
                aq: _aq,
                rl: _rl,
            } => {
                let t = bus.load64(self.x[rs1])?;
                bus.store64(self.x[rs1], self.x[rs2])?;
                self.x[rd] = t;

                Ok(inst)
            }
            _ => Err(Exception::Breakpoint(self.pc)),
        }
    }
}
