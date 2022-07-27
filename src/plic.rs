use crate::prelude::{Exception, MemIntf, PLIC_BASE};

pub const PLIC_PENDING: u64 = 0x1000;
pub const PLIC_SENABLE: u64 = 0x2000;
pub const PLIC_SPRIORITY: u64 = 0x201000;
pub const PLIC_SCLAIM: u64 = 0x201004;

pub struct Plic {
    pending: u64,
    senable: u64,
    spriority: u64,
    sclaim: u64,
}

impl Plic {
    pub fn new() -> Self {
        Self {
            pending: 0,
            senable: 0,
            spriority: 0,
            sclaim: 0,
        }
    }
}

impl MemIntf for Plic {
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size != 32 {
            return Err(Exception::LoadAccessFault(addr + PLIC_BASE));
        }

        match addr {
            PLIC_PENDING => Ok(self.pending),
            PLIC_SENABLE => Ok(self.senable),
            PLIC_SPRIORITY => Ok(self.spriority),
            PLIC_SCLAIM => Ok(self.sclaim),
            _ => Ok(0),
        }
    }

    fn store(&mut self, addr: u64, val: u64, size: u64) -> Result<(), Exception> {
        if size != 32 {
            return Err(Exception::StoreAMOAccessFault(addr + PLIC_BASE));
        }

        match addr {
            PLIC_PENDING => Ok(self.pending = val),
            PLIC_SENABLE => Ok(self.senable = val),
            PLIC_SPRIORITY => Ok(self.spriority = val),
            PLIC_SCLAIM => Ok(self.sclaim = val),
            _ => Ok(()),
        }
    }

    fn reset(&mut self) {
        self.pending = 0;
        self.senable = 0;
        self.spriority = 0;
        self.sclaim = 0;
    }
}
