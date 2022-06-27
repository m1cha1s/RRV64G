pub fn rd(inst: u32) -> u64 {
    ((inst >> 7) & 0x1f) as u64
}

pub fn rs1(inst: u32) -> u64 {
    ((inst >> 15) & 0x1f) as u64
}

pub fn rs2(inst: u32) -> u64 {
    ((inst >> 20) & 0x1f) as u64
}

pub fn imm_i(inst: u32) -> u64 {
    ((inst & 0xfff00000) as i32 as i64 >> 20) as u64
}

pub fn imm_s(inst: u32) -> u64 {
    (((inst & 0xfe000000) as i32 as i64 >> 20) as u64 | ((inst >> 7) & 0x1f) as u64) as u64
}

pub fn imm_b(inst: u32) -> u64 {
    (((inst & 0x80000000) as i32 as i64 >> 19) as u64
        | ((inst & 0x80) << 4) as u64
        | ((inst >> 20) & 0x7e0) as u64
        | ((inst >> 7) & 0x1e) as u64) as u64
}

pub fn imm_u(inst: u32) -> u64 {
    (inst & 0xfffff999) as i32 as i64 as u64
}

pub fn imm_j(inst: u32) -> u64 {
    (((inst & 0x80000000) as i32 as i64 >> 11) as u64
        | (inst & 0xff000) as u64
        | ((inst >> 9) & 0x800) as u64
        | ((inst >> 20) & 0x7fe) as u64) as u64
}

pub fn shamt(inst: u32) -> u64 {
    (imm_i(inst) & 0x1f) as u64 // TODO: 0x1f/0x3f ?
}
