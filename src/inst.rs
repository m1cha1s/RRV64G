use crate::prelude::Exception;

#[derive(Debug)]
pub enum Inst {
	// RV64I instuctions
    Addi  { rd: usize, rs1: usize, imm: i64 },
    Slti  { rd: usize, rs1: usize, imm: i64 },
    Sltiu { rd: usize, rs1: usize, imm: i64 },
    Xori  { rd: usize, rs1: usize, imm: i64 },
    Ori   { rd: usize, rs1: usize, imm: i64 },
    Andi  { rd: usize, rs1: usize, imm: i64 },

    Addiw { rd: usize, rs1: usize, imm: i64 },

    Slli { rd: usize, rs1: usize, shamt: u32 },
    Srli { rd: usize, rs1: usize, shamt: u32 },
    Srai { rd: usize, rs1: usize, shamt: u32 },

    Slliw { rd: usize, rs1: usize, shamt: u32 },
    Srliw { rd: usize, rs1: usize, shamt: u32 },
    Sraiw { rd: usize, rs1: usize, shamt: u32 },

    Lb  { rd: usize, rs1: usize, imm: i64 },
    Lh  { rd: usize, rs1: usize, imm: i64 },
    Lw  { rd: usize, rs1: usize, imm: i64 },
    Lbu { rd: usize, rs1: usize, imm: i64 },
    Lhu { rd: usize, rs1: usize, imm: i64 },
    Lwu { rd: usize, rs1: usize, imm: i64 },
    Ld  { rd: usize, rs1: usize, imm: i64 },

    Fence { rd: usize, rs1: usize, imm: i64 },

    Jalr { rd: usize, rs1: usize, imm: i64 },

    Ebreak,
    Ecall,

    Lui   { rd: usize, imm: i64 },
    Auipc { rd: usize, imm: i64 },

    Sb { rs1: usize, rs2: usize, imm: i64 },
    Sh { rs1: usize, rs2: usize, imm: i64 },
    Sw { rs1: usize, rs2: usize, imm: i64 },
    Sd { rs1: usize, rs2: usize, imm: i64 },

    Add  { rd: usize, rs1: usize, rs2: usize },
    Sub  { rd: usize, rs1: usize, rs2: usize },
    Sll  { rd: usize, rs1: usize, rs2: usize },
    Slt  { rd: usize, rs1: usize, rs2: usize },
    Sltu { rd: usize, rs1: usize, rs2: usize },
    Xor  { rd: usize, rs1: usize, rs2: usize },
    Srl  { rd: usize, rs1: usize, rs2: usize },
    Sra  { rd: usize, rs1: usize, rs2: usize },
    Or   { rd: usize, rs1: usize, rs2: usize },
    And  { rd: usize, rs1: usize, rs2: usize },

    Addw { rd: usize, rs1: usize, rs2: usize },
    Subw { rd: usize, rs1: usize, rs2: usize },
    Sllw { rd: usize, rs1: usize, rs2: usize },
    Srlw { rd: usize, rs1: usize, rs2: usize },
    Sraw { rd: usize, rs1: usize, rs2: usize },

	Beq  { rs1: usize, rs2: usize, imm: i64 },
	Bne  { rs1: usize, rs2: usize, imm: i64 },
	Blt  { rs1: usize, rs2: usize, imm: i64 },
	Bge  { rs1: usize, rs2: usize, imm: i64 },
	Bltu { rs1: usize, rs2: usize, imm: i64 },
	Bgeu { rs1: usize, rs2: usize, imm: i64 },

	Jal { rd: usize, imm: i64 },

	// M extension
	Mul    { rd: usize, rs1: usize, rs2: usize },
	Mulh   { rd: usize, rs1: usize, rs2: usize },
	Mulw   { rd: usize, rs1: usize, rs2: usize },
	Mulhsu { rd: usize, rs1: usize, rs2: usize },
	Mulhu  { rd: usize, rs1: usize, rs2: usize },
	Div    { rd: usize, rs1: usize, rs2: usize },
	Divw   { rd: usize, rs1: usize, rs2: usize },
	Divu   { rd: usize, rs1: usize, rs2: usize },
	Divuw  { rd: usize, rs1: usize, rs2: usize },
	Rem    { rd: usize, rs1: usize, rs2: usize },
	Remw   { rd: usize, rs1: usize, rs2: usize },
	Remu   { rd: usize, rs1: usize, rs2: usize },
	Remuw  { rd: usize, rs1: usize, rs2: usize },

	// A extension
	Lrw      { rd: usize, rs1: usize, rl: bool, aq: bool },
	Lrd      { rd: usize, rs1: usize, rl: bool, aq: bool },
	Scw      { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Scd      { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoswapw { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoswapd { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoaddw  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoaddd  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoxorw  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoxord  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoandw  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoandd  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoorw   { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amoord   { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amominw  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amomind  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amomaxw  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amomaxd  { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amominuw { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amominud { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amomaxuw { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },
	Amomaxud { rd: usize, rs1: usize, rs2: usize, rl: bool, aq: bool },

	// TODO
	// F extension
	Flw { rd: usize, rs1: usize, imm: i64 }, 
	Fsw { rs1: usize, rs2: usize, imm: i64 },
	
	// TODO
	// D extension

	// Zicsr extension
	Csrrw  { rd: usize, rs1: usize, csr: usize },
	Csrrs  { rd: usize, rs1: usize, csr: usize },
	Csrrc  { rd: usize, rs1: usize, csr: usize },
	Csrrwi { rd: usize, uimm: u64, csr: usize },
	Csrrsi { rd: usize, uimm: u64, csr: usize },
	Csrrci { rd: usize, uimm: u64, csr: usize },

	// Zifencei extension
	Fencei { rd: usize, rs1: usize, imm: i64 },

	// Privilaged mode instuction
	Sret,
	Mret,

	Sfencevma,
}

pub enum ImmType {
    R,
    I,
    S,
    B,
    U,
    J,
}

impl ImmType {
    pub fn decode(&self, inst: u32) -> Result<Inst, Exception> {
        // Get opcode
        let opcode = inst & 0b1111111;

        match self {
            ImmType::I => {
                let imm = (inst >> 20) & 0b1111_1111_1111;
                let rd = ((inst >> 7) & 0b11111) as usize;
                let func3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;

                let shamt = imm & 0b111111;
                // Differentiate between SRLI and SRAI
                let shiftop = (imm * 0x400) == 0;

				// CSR selector
				let csr = imm as usize;

                // Sign extend the immediate
                let imm = ((imm as i32) << 20) >> 20;
				let imm = imm as i64;

                match opcode {
                    0b0000011 => match func3 {
                        0b000 => Ok(Inst::Lb { rd, rs1, imm }),
                        0b001 => Ok(Inst::Lh { rd, rs1, imm }),
                        0b010 => Ok(Inst::Lw { rd, rs1, imm }),
                        0b100 => Ok(Inst::Lbu { rd, rs1, imm }),
                        0b101 => Ok(Inst::Lhu { rd, rs1, imm }),
                        0b110 => Ok(Inst::Lwu { rd, rs1, imm }),
                        0b011 => Ok(Inst::Ld { rd, rs1, imm }),
                        _ => Err(Exception::IllegalInstruction(inst as u64)),
                    },
                    0b0001111 => match func3 {
                        0b000 => Ok(Inst::Fence { rd, rs1, imm }),
                        _ => Err(Exception::IllegalInstruction(inst as u64)),
                    },
                    0b0010011 => match func3 {
                        0b000 => Ok(Inst::Addi { rd, rs1, imm }),
                        0b010 => Ok(Inst::Slti { rd, rs1, imm }),
                        0b011 => Ok(Inst::Sltiu { rd, rs1, imm }),
                        0b100 => Ok(Inst::Xori { rd, rs1, imm }),
                        0b110 => Ok(Inst::Ori { rd, rs1, imm }),
                        0b111 => Ok(Inst::Andi { rd, rs1, imm }),
                        0b001 => Ok(Inst::Slli { rd, rs1, shamt }),
                        0b101 => {
                            if shiftop {
                                Ok(Inst::Srli { rd, rs1, shamt })
                            } else {
                                Ok(Inst::Srai { rd, rs1, shamt })
                            }
                        }
                        _ => Err(Exception::IllegalInstruction(inst as u64)),
                    },
                    0b1100111 => match func3 {
                        0b000 => Ok(Inst::Jalr { rd, rs1, imm: imm }),
                        _ => Err(Exception::IllegalInstruction(inst as u64)),
                    },
                    0b1110011 => {
						match func3 {
							0b000 => Ok(Inst::Sfencevma),
							0b001 => Ok(Inst::Csrrw     { rd, rs1, csr }),
							0b010 => Ok(Inst::Csrrs     { rd, rs1, csr }),
							0b011 => Ok(Inst::Csrrc     { rd, rs1, csr }),
							0b101 => Ok(Inst::Csrrwi    { rd, uimm: rs1 as u64, csr }),
							0b110 => Ok(Inst::Csrrsi    { rd, uimm: rs1 as u64, csr }),
							0b111 => Ok(Inst::Csrrci    { rd, uimm: rs1 as u64, csr }),
	                        _ => if func3 == 0 && rs1 == 0 && rd == 0 {
	                            match imm {
	                                0 => Ok(Inst::Ecall),
	                                1 => Ok(Inst::Ebreak),
									258 => Ok(Inst::Sret),
									770 => Ok(Inst::Mret),
	                                _ => Err(Exception::IllegalInstruction(inst as u64)),
	                            }
	                        } else {
	                            Err(Exception::IllegalInstruction(inst as u64))
                        	} 
						}
                    }
                    0b0011011 => match func3 {
                        0b000 => Ok(Inst::Addiw { rd, rs1, imm }),
                        0b001 => Ok(Inst::Slliw { rd, rs1, shamt }),
                        0b101 => if shiftop {
                            Ok(Inst::Srliw { rd, rs1, shamt })
                        } else {
                            Ok(Inst::Sraiw { rd, rs1, shamt })
                        },
                        _ => Err(Exception::IllegalInstruction(inst as u64)),
                    },
                    _ => Err(Exception::IllegalInstruction(inst as u64)),
                }
            }
            ImmType::U => {
                let imm = (inst & 0xfffff000) as i32;
				let imm = imm as i64;
                let rd = ((inst >> 7) & 0b11111) as usize;
                
                match opcode {
                    0b0010111 => Ok(Inst::Auipc { rd, imm }),
                    0b0110111 => Ok(Inst::Lui { rd, imm }),
                    _ => Err(Exception::IllegalInstruction(inst as u64)),
                }   
            },
            ImmType::S => {
                let imm115 = (inst >> 25) & 0b1111111;
                let imm40 = (inst >> 7) & 0b11111;

                let func3 = (inst >> 12) & 0b111;

                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;

                // Merge and sign extend the immediate 
                let imm = (imm115 << 5) | imm40;
                let imm = ((imm as i32) << 20) >> 20;
				let imm = imm as i64;

                match opcode {
                    0b0100011 => match func3 {
                        0b000 => Ok(Inst::Sb { rs1, rs2, imm }),
                        0b001 => Ok(Inst::Sh { rs1, rs2, imm }),
                        0b010 => Ok(Inst::Sw { rs1, rs2, imm }),
                        0b011 => Ok(Inst::Sd { rs1, rs2, imm }),
                        _ => Err(Exception::IllegalInstruction(inst as u64)),
                    },
                    _ => Err(Exception::IllegalInstruction(inst as u64)),
                }
            },
            ImmType::R => {
                let rd = ((inst >> 7) & 0b11111) as usize;
                let func3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;
                let func7 = (inst >> 25) & 0b1111111;

				// A extension
				let rl = (func7 & 0b1) != 0;
				let aq = ((func7 & 0b10) >> 1) != 0;

                match opcode {
                    0b0110011 => match (func3, func7) {
						(0b000, 0b0000000) => Ok(Inst::Add    { rd, rs1, rs2 }),
						(0b000, 0b0100000) => Ok(Inst::Sub    { rd, rs1, rs2 }),
						(0b001, 0b0000000) => Ok(Inst::Sll    { rd, rs1 ,rs2 }),
						(0b010, 0b0000000) => Ok(Inst::Slt    { rd, rs1 ,rs2 }),
						(0b011, 0b0000000) => Ok(Inst::Sltu   { rd, rs1 ,rs2 }),
						(0b100, 0b0000000) => Ok(Inst::Xor    { rd, rs1 ,rs2 }),
						(0b101, 0b0000000) => Ok(Inst::Srl    { rd, rs1, rs2 }),
						(0b101, 0b0100000) => Ok(Inst::Sra    { rd, rs1, rs2 }),
						(0b110, 0b0000000) => Ok(Inst::Or     { rd, rs1, rs2 }),
						(0b111, 0b0000000) => Ok(Inst::And    { rd, rs1, rs2 }),
						(0b000, 0b0000001) => Ok(Inst::Mul    { rd, rs1, rs2 }),
						(0b001, 0b0000001) => Ok(Inst::Mulh   { rd, rs1, rs2 }),
						(0b010, 0b0000001) => Ok(Inst::Mulhsu { rd, rs1, rs2 }),
						(0b011, 0b0000001) => Ok(Inst::Mulhu  { rd, rs1, rs2 }),
						(0b100, 0b0000001) => Ok(Inst::Div    { rd, rs1, rs2 }),
						(0b101, 0b0000001) => Ok(Inst::Divu   { rd, rs1, rs2 }),
						(0b110, 0b0000001) => Ok(Inst::Rem    { rd, rs1, rs2 }),
						(0b111, 0b0000001) => Ok(Inst::Remu   { rd, rs1, rs2 }),
                    	(_, _) => Err(Exception::IllegalInstruction(inst as u64)),
				    },
					0b0111011 => match (func3, func7) {
						(0b000, 0b0000000) => Ok(Inst::Addw { rd, rs1, rs2 }),
						(0b000, 0b0100000) => Ok(Inst::Subw { rd, rs1, rs2 }),
						(0b001, 0b0000000) => Ok(Inst::Sllw { rd, rs1, rs2 }),
						(0b101, 0b0000000) => Ok(Inst::Srlw { rd, rs1, rs2 }),
						(0b101, 0b0100000) => Ok(Inst::Sraw { rd, rs1, rs2 }),
						(0b000, 0b0000001) => Ok(Inst::Mulw { rd, rs1, rs2 }),
						(0b100, 0b0000001) => Ok(Inst::Divw { rd, rs1, rs2 }),
						(0b101, 0b0000001) => Ok(Inst::Divuw { rd, rs1, rs2 }),
						(0b110, 0b0000001) => Ok(Inst::Remw { rd, rs1, rs2 }),
						(0b111, 0b0000001) => Ok(Inst::Remuw { rd, rs1, rs2 }),
                    	(_, _) => Err(Exception::IllegalInstruction(inst as u64)),
					},
					0b0101111 => match (func3, func7>>2) {
						(0b010, 0b00010) => Ok(Inst::Lrw { rd, rs1, rl, aq }),
						(0b011, 0b00010) => Ok(Inst::Lrd { rd, rs1, rl, aq }),
						(0b010, 0b00011) => Ok(Inst::Scw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b00011) => Ok(Inst::Scd { rd, rs1, rs2, rl, aq }),
						(0b010, 0b00001) => Ok(Inst::Amoswapw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b00001) => Ok(Inst::Amoswapd { rd, rs1, rs2, rl, aq }),
						(0b010, 0b00000) => Ok(Inst::Amoaddw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b00000) => Ok(Inst::Amoaddd { rd, rs1, rs2, rl, aq }),
						(0b010, 0b00100) => Ok(Inst::Amoxorw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b00100) => Ok(Inst::Amoxord { rd, rs1, rs2, rl, aq }),
						(0b010, 0b01100) => Ok(Inst::Amoandw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b01100) => Ok(Inst::Amoandd { rd, rs1, rs2, rl, aq }),
						(0b010, 0b01000) => Ok(Inst::Amoorw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b01000) => Ok(Inst::Amoord { rd, rs1, rs2, rl, aq }),
						(0b010, 0b10000) => Ok(Inst::Amominw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b10000) => Ok(Inst::Amomind { rd, rs1, rs2, rl, aq }),
						(0b010, 0b10100) => Ok(Inst::Amomaxw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b10100) => Ok(Inst::Amomaxd { rd, rs1, rs2, rl, aq }),
						(0b010, 0b11000) => Ok(Inst::Amominuw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b11000) => Ok(Inst::Amominud { rd, rs1, rs2, rl, aq }),
						(0b010, 0b11100) => Ok(Inst::Amomaxuw { rd, rs1, rs2, rl, aq }),
						(0b011, 0b11100) => Ok(Inst::Amomaxud { rd, rs1, rs2, rl, aq }),
						(_, _) => Err(Exception::IllegalInstruction(inst as u64)),
					},
                    _ => Err(Exception::IllegalInstruction(inst as u64)),
                }
            },
			ImmType::B => {
				let imm12105 = (inst >> 25) & 0b1111111;
                let imm4111  = (inst >> 7) & 0b11111;

                let func3 = (inst >> 12) & 0b111;

                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;

				// Split the immediate 
				let imm12  = (imm12105 & 0b1000000) >> 6;
				let imm105 = imm12105 & 0b0111111;
				let imm41  = (imm4111 & 0b11110) >> 1;
				let imm11  = imm4111 & 0b00001;

				// Merge the immediate
				let imm = (imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);

				// Sign extend the immediate
				let imm = ((imm as i32) << 19) >> 19;
				let imm = imm as i64;

				match func3 {
					0b000 => Ok(Inst::Beq { rs1, rs2, imm: imm }),
					0b001 => Ok(Inst::Bne { rs1, rs2, imm: imm }),
					0b100 => Ok(Inst::Blt { rs1, rs2, imm: imm }),
					0b101 => Ok(Inst::Bge { rs1, rs2, imm: imm }),
					0b110 => Ok(Inst::Bltu { rs1, rs2, imm: imm }),
					0b111 => Ok(Inst::Bgeu { rs1, rs2, imm: imm }),
            		_ => Err(Exception::IllegalInstruction(inst as u64)),
				}
			},
			ImmType::J => {
				let rd = ((inst >> 7) & 0b11111) as usize;
				let imm20101111912 = (inst >> 12) & 0xfffff;

				// Split the immediate
				let imm20  = (imm20101111912 >> 19) & 0b1;
				let imm101 = (imm20101111912 >> 9) & 0b1111111111;
				let imm11  = (imm20101111912 >> 8) & 0b1;
				let imm1912 = imm20101111912 & 0b11111111;

				// Merge immediate
				let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) | (imm101 << 1);

				// Sign extend the immediate
				let imm = ((imm as i32) << 11) >> 11;
				let imm = imm as i64;

				match opcode {
					0b1101111 => Ok(Inst::Jal { rd, imm }),
            		_ => Err(Exception::IllegalInstruction(inst as u64)),
				}
			},
        }
    }
}

pub const ENCODING_TABLE: [Option<ImmType>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ Some(ImmType::I),
    /* 0b0000100 */ None,
    /* 0b0000101 */ None,
    /* 0b0000110 */ None,
    /* 0b0000111 */ None,
    /* 0b0001000 */ None,
    /* 0b0001001 */ None,
    /* 0b0001010 */ None,
    /* 0b0001011 */ None,
    /* 0b0001100 */ None,
    /* 0b0001101 */ None,
    /* 0b0001110 */ None,
    /* 0b0001111 */ Some(ImmType::I),
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(ImmType::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ Some(ImmType::U),
    /* 0b0011000 */ None,
    /* 0b0011001 */ None,
    /* 0b0011010 */ None,
    /* 0b0011011 */ Some(ImmType::I),
    /* 0b0011100 */ None,
    /* 0b0011101 */ None,
    /* 0b0011110 */ None,
    /* 0b0011111 */ None,
    /* 0b0100000 */ None,
    /* 0b0100001 */ None,
    /* 0b0100010 */ None,
    /* 0b0100011 */ Some(ImmType::S),
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ None,
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ Some(ImmType::R),
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ Some(ImmType::R),
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(ImmType::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ Some(ImmType::R),
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ None,
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ None,
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ None,
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ None,
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ None,
    /* 0b1010100 */ None,
    /* 0b1010101 */ None,
    /* 0b1010110 */ None,
    /* 0b1010111 */ None,
    /* 0b1011000 */ None,
    /* 0b1011001 */ None,
    /* 0b1011010 */ None,
    /* 0b1011011 */ None,
    /* 0b1011100 */ None,
    /* 0b1011101 */ None,
    /* 0b1011110 */ None,
    /* 0b1011111 */ None,
    /* 0b1100000 */ None,
    /* 0b1100001 */ None,
    /* 0b1100010 */ None,
    /* 0b1100011 */ Some(ImmType::B),
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ Some(ImmType::I),
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ Some(ImmType::J),
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ Some(ImmType::I),
    /* 0b1110100 */ None,
    /* 0b1110101 */ None,
    /* 0b1110110 */ None,
    /* 0b1110111 */ None,
    /* 0b1111000 */ None,
    /* 0b1111001 */ None,
    /* 0b1111010 */ None,
    /* 0b1111011 */ None,
    /* 0b1111100 */ None,
    /* 0b1111101 */ None,
    /* 0b1111110 */ None,
    /* 0b1111111 */ None,
];
