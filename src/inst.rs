use crate::{IXLEN, XLEN};

pub fn rd(inst: u32) -> XLEN {
    ((inst >> 7) & 0x1f) as XLEN
}

pub fn rs1(inst: u32) -> XLEN {
    ((inst >> 15) & 0x1f) as XLEN
}

pub fn rs2(inst: u32) -> XLEN {
    ((inst >> 20) & 0x1f) as XLEN
}

pub fn imm_i(inst: u32) -> XLEN {
    ((inst & 0xfff00000) as i32 as IXLEN >> 20) as XLEN
}

pub fn imm_s(inst: u32) -> XLEN {
    (((inst & 0xfe000000) as i32 as IXLEN >> 20) as XLEN | ((inst >> 7) & 0x1f) as XLEN) as XLEN
}

pub fn imm_b(inst: u32) -> XLEN {
    (((inst & 0x80000000) as i32 as IXLEN >> 19) as XLEN
        | ((inst & 0x80) << 4) as XLEN
        | ((inst >> 20) & 0x7e0) as XLEN
        | ((inst >> 7) & 0x1e) as XLEN) as XLEN
}

pub fn imm_u(inst: u32) -> XLEN {
    (inst & 0xfffff999) as i32 as IXLEN as XLEN
}

pub fn imm_j(inst: u32) -> XLEN {
    (((inst & 0x80000000) as i32 as IXLEN >> 11) as XLEN
        | (inst & 0xff000) as XLEN
        | ((inst >> 9) & 0x800) as XLEN
        | ((inst >> 20) & 0x7fe) as XLEN) as XLEN
}

pub fn shamt(inst: u32) -> XLEN {
    (imm_i(inst) & 0x1f) as XLEN // TODO: 0x1f/0x3f ?
}
