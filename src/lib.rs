#![no_std]

extern crate alloc;

pub mod bus;
pub mod cpu;
pub mod exceptions;
pub mod inst;
pub mod mem;
pub mod regs;

pub mod prelude {
    pub use super::bus::*;
    pub use super::cpu::*;
    pub use super::exceptions::*;
    pub use super::mem::*;
    pub use super::regs::*;
}

#[cfg(test)]
mod tests;
