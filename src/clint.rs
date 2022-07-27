use crate::prelude::{Exception, MemIntf, CLINT_BASE};

pub const CLINT_MTIMECMP: u64 = 0x4000;
pub const CLINT_MTIME: u64 = 0xbff8;

pub struct Clint {
    mtime: u64,
    mtimecmp: u64,
}

impl Clint {
    pub fn new() -> Self {
        Self {
            mtime: 0,
            mtimecmp: 0,
        }
    }
}

impl MemIntf for Clint {
    fn reset(&mut self) {
        self.mtime = 0;
        self.mtimecmp = 0;
    }

    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size != 64 {
            return Err(Exception::LoadAccessFault(addr + CLINT_BASE));
        }

        match addr {
            CLINT_MTIME => Ok(self.mtime),
            CLINT_MTIMECMP => Ok(self.mtimecmp),
            _ => Err(Exception::LoadAccessFault(addr + CLINT_BASE)),
        }
    }

    fn store(&mut self, addr: u64, val: u64, size: u64) -> Result<(), Exception> {
        if size != 64 {
            return Err(Exception::LoadAccessFault(addr + CLINT_BASE));
        }

        match addr {
            CLINT_MTIME => Ok(self.mtime = val),
            CLINT_MTIMECMP => Ok(self.mtimecmp = val),
            _ => Err(Exception::StoreAMOAccessFault(addr + CLINT_BASE)),
        }
    }
}
