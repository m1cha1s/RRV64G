#![no_std]

extern crate alloc;

#[cfg(feature = "rv32i")]
type XLEN = u32;

#[cfg(feature = "rv64i")]
type XLEN = u64;

type BYTE = u8;
type HALFWORD = u16;
type WORD = u32;
type DOUBLEWORD = u64;

pub mod cpu;
pub mod exceptions;
pub mod inst;
pub mod mmu;
pub mod regs;

pub mod prelude {
    pub use super::cpu::*;
    pub use super::exceptions::*;
    pub use super::mmu::*;
}

#[cfg(test)]
mod tests;
