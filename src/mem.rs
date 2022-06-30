use crate::exceptions::Exception;

pub trait Memory {
    fn get_size(&self) -> u64;
    fn reset(&mut self);

    fn load8(&self, addr: u64) -> Result<u8, Exception>;
    fn load16(&self, addr: u64) -> Result<u16, Exception>;
    fn load32(&self, addr: u64) -> Result<u32, Exception>;
    fn load64(&self, addr: u64) -> Result<u64, Exception>;

    fn store8(&mut self, addr: u64, val: u8) -> Result<(), Exception>;
    fn store16(&mut self, addr: u64, val: u16) -> Result<(), Exception>;
    fn store32(&mut self, addr: u64, val: u32) -> Result<(), Exception>;
    fn store64(&mut self, addr: u64, val: u64) -> Result<(), Exception>;
}
