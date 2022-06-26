use crate::{exceptions::Exception, BYTE, DOUBLEWORD, HALFWORD, WORD, XLEN};

pub trait Memory {
    fn get_size(&self) -> XLEN;
    fn reset(&mut self);

    fn get_byte(&self, addr: XLEN) -> Result<BYTE, Exception>;
    fn get_half_word(&self, addr: XLEN) -> Result<HALFWORD, Exception>;
    fn get_word(&self, addr: XLEN) -> Result<WORD, Exception>;
    fn get_double_word(&self, addr: XLEN) -> Result<DOUBLEWORD, Exception>;

    fn set_byte(&mut self, addr: XLEN, val: BYTE) -> Result<(), Exception>;
    fn set_half_word(&mut self, addr: XLEN, val: HALFWORD) -> Result<(), Exception>;
    fn set_word(&mut self, addr: XLEN, val: WORD) -> Result<(), Exception>;
    fn set_double_word(&mut self, addr: XLEN, val: DOUBLEWORD) -> Result<(), Exception>;
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

    pub fn reset(&mut self) -> &mut Self {
        self.mem.reset();
        self
    }

    pub fn get_byte(&self, addr: XLEN) -> Result<BYTE, Exception> {
        self.mem.get_byte(addr)
    }
    pub fn get_half_word(&self, addr: XLEN) -> Result<HALFWORD, Exception> {
        self.mem.get_half_word(addr)
    }
    pub fn get_word(&self, addr: XLEN) -> Result<WORD, Exception> {
        self.mem.get_word(addr)
    }
    pub fn get_double_word(&self, addr: XLEN) -> Result<DOUBLEWORD, Exception> {
        self.mem.get_double_word(addr)
    }

    pub fn set_byte(&mut self, addr: XLEN, val: BYTE) -> Result<(), Exception> {
        self.mem.set_byte(addr, val)
    }
}
