use alloc::boxed::Box;

use crate::XLEN;

pub trait Memory {
    fn get_size(&self) -> XLEN;
    fn get_word(&self, addr: XLEN) -> Result<XLEN, ()>;
    fn set_word(&mut self, addr: XLEN, val: XLEN) -> Result<(), ()>;
}

pub struct MMU {
    pub size: XLEN,
    pub mem: Box<dyn Memory>,
}

impl MMU {
    pub fn new(mem: Box<dyn Memory>) -> Self {
        MMU {
            size: mem.get_size(),
            mem,
        }
    }

    pub fn get_word(&self, addr: XLEN) -> Result<XLEN, ()> {
        self.mem.get_word(addr)
    }
    pub fn set_word(&mut self, addr: XLEN, val: XLEN) -> Result<(), ()> {
        self.mem.set_word(addr, val)
    }
}
