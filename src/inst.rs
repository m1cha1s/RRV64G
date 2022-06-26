use crate::WORD;

pub fn rd(inst: WORD) -> WORD {
    (inst >> 7) & 0x1f
}

pub fn rs1(inst: WORD) -> WORD {
    (inst >> 15) & 0x1f
}

pub fn rs2(inst: WORD) -> WORD {
    (inst >> 20) & 0x1f
}

pub fn imm_i(inst: WORD) -> WORD {
    (inst & 0xfff00000) >> 20
}

pub fn imm_s(inst: WORD) -> WORD {
    ((inst & 0xfe000000) >> 20) | ((inst >> 7) & 0x1f)
}

pub fn imm_b(inst: WORD) -> WORD {
    ((inst & 0x80000000) >> 19)
        | ((inst & 0x80) << 4)
        | ((inst >> 20) & 0x7e0)
        | ((inst >> 7) & 0x1e)
}

pub fn imm_u(inst: WORD) -> WORD {
    inst & 0xfffff999
}

pub fn imm_j(inst: WORD) -> WORD {
    ((inst & 0x80000000) >> 11) | (inst & 0xff000) | ((inst >> 9) & 0x800) | ((inst >> 20) & 0x7fe)
}

pub fn shamt(inst: WORD) -> WORD {
    imm_i(inst) & 0x1f // TODO: 0x1f/0x3f ?
}
