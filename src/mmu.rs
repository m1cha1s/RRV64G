use alloc::boxed::Box;

use crate::XLEN;

pub trait Memory {
    fn get_size(&self) -> XLEN;

    fn get_byte(&self, addr: XLEN) -> Result<u8, ()>;
    fn get_half_word(&self, addr: XLEN) -> Result<u16, ()>;
    fn get_word(&self, addr: XLEN) -> Result<u32, ()>;
    fn get_double_word(&self, addr: XLEN) -> Result<u64, ()>;

    fn set_byte(&mut self, addr: XLEN, val: u8) -> Result<(), ()>;
    fn set_half_word(&mut self, addr: XLEN, val: u16) -> Result<(), ()>;
    fn set_word(&mut self, addr: XLEN, val: u32) -> Result<(), ()>;
    fn set_double_word(&mut self, addr: XLEN, val: u64) -> Result<(), ()>;
}

pub struct MMU {
    pub size: XLEN,
    pub mem: &'static mut dyn Memory,
}

impl MMU {
    pub fn new(mem: &'static mut dyn Memory) -> Self {
        MMU {
            size: mem.get_size(),
            mem,
        }
    }

    pub fn get_byte(&self, addr: XLEN) -> Result<u8, ()> {
        self.mem.get_byte(addr)
    }
    pub fn set_byte(&mut self, addr: XLEN, val: u8) -> Result<(), ()> {
        self.mem.set_byte(addr, val)
    }
}
