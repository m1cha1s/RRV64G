#![no_std]

extern crate alloc;

#[cfg(feature = "rv32i")]
type XLEN = u32;

#[cfg(feature = "rv64i")]
type XLEN = u64;

pub mod cpu;
pub mod mmu;
pub mod regs;

pub mod prelude {
    pub use super::mmu::*;
}

#[cfg(test)]
mod tests;
