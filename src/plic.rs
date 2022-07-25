use crate::prelude::Exception;

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

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size != 32 {
            return Err(Exception::LoadAccessFault(addr));
        }

        match addr {
            PLIC_PENDING => Ok(self.pending),
        }
    }
}
