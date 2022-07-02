use crate::{exceptions::Exception, mem::Memory};

pub const RAM_START: u64 = 0x00000000;

pub struct MemMapEntry {
    pub start: u64,
    pub end: u64,
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

    pub fn load8(&self, addr: u64) -> Result<u8, Exception> {
        if addr >= RAM_START {
            return self.mem.load8(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn load16(&self, addr: u64) -> Result<u16, Exception> {
        if addr >= RAM_START {
            return self.mem.load16(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn load32(&self, addr: u64) -> Result<u32, Exception> {
        if addr >= RAM_START {
            return self.mem.load32(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn load64(&self, addr: u64) -> Result<u64, Exception> {
        if addr >= RAM_START {
            return self.mem.load64(addr - RAM_START);
        }

        Err(Exception::AddressOutOfBounds)
    }

    pub fn store8(&mut self, addr: u64, val: u8) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store8(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }

    pub fn store16(&mut self, addr: u64, val: u16) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store16(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }

    pub fn store32(&mut self, addr: u64, val: u32) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store32(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }

    pub fn store64(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
        if addr >= RAM_START {
            return self.mem.store64(addr, val);
        }
        Err(Exception::AddressOutOfBounds)
    }
}
