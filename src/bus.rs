use crate::{exceptions::Exception, mem::Memory, XLEN};

pub const RAM_START: XLEN = 0x80000000;

pub struct MemMapEntry {
    pub start: XLEN,
    pub end: XLEN,
}

pub struct Bus<'a> {
    pub mem: &'a mut dyn Memory,
}

impl<'a> Bus<'a> {
    pub fn new(mem: &'a mut dyn Memory) -> Self {
        Bus { mem }
    }

    pub fn reset(&mut self) {
        self.mem.reset();
    }

    pub fn load8(&self, addr: XLEN) -> Result<u8, Exception> {
        if addr >= RAM_START {
            return self.mem.load8(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn load16(&self, addr: XLEN) -> Result<u16, Exception> {
        if addr >= RAM_START {
            return self.mem.load16(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn load32(&self, addr: XLEN) -> Result<u32, Exception> {
        if addr >= RAM_START {
            return self.mem.load32(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn load64(&self, addr: XLEN) -> Result<u64, Exception> {
        if addr >= RAM_START {
            return self.mem.load64(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn store8(&mut self, addr: XLEN, val: u8) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store8(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }

    pub fn store16(&mut self, addr: XLEN, val: u16) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store16(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }

    pub fn store32(&mut self, addr: XLEN, val: u32) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store32(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }

    pub fn store64(&mut self, addr: XLEN, val: u64) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store64(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }
}
