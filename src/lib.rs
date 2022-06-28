#![no_std]

extern crate alloc;

#[cfg(feature = "rv32i")]
mod conf {
    pub type XLEN = u32;
    pub type IXLEN = i32;
}

#[cfg(feature = "rv64i")]
mod conf {
    pub type XLEN = u64;
    pub type IXLEN = i64;
}

pub use conf::*;

pub mod bus;
pub mod cpu;
pub mod exceptions;
pub mod inst;
pub mod mem;
pub mod opcodes;
pub mod regs;

pub mod prelude {
    pub use super::bus::*;
    pub use super::cpu::*;
    pub use super::exceptions::*;
    pub use super::mem::*;
    pub use super::regs::*;
    pub use super::XLEN;
}

#[cfg(test)]
mod tests;
