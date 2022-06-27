use crate::{exceptions::Exception, XLEN};

pub trait Memory {
    fn get_size(&self) -> XLEN;
    fn reset(&mut self);

    fn load8(&self, addr: XLEN) -> Result<u8, Exception>;
    fn load16(&self, addr: XLEN) -> Result<u16, Exception>;
    fn load32(&self, addr: XLEN) -> Result<u32, Exception>;
    fn load64(&self, addr: XLEN) -> Result<u64, Exception>;

    fn store8(&mut self, addr: XLEN, val: u8) -> Result<(), Exception>;
    fn store16(&mut self, addr: XLEN, val: u16) -> Result<(), Exception>;
    fn store32(&mut self, addr: XLEN, val: u32) -> Result<(), Exception>;
    fn store64(&mut self, addr: XLEN, val: u64) -> Result<(), Exception>;
}
