use crate::exceptions::Exception;

pub const RAM_BASE: u64 = 0x8000_0000;

pub trait MemIntf {
	fn reset(&mut self);
	fn load(&self, addr: u64) -> Result<u32, Exception>;
	fn store(&mut self, addr: u64, val: u32) -> Result<(), Exception>;
}

pub struct Bus<'a> {
	pub ram: &'a mut dyn MemIntf,
	pub ram_size: u64,
}

impl<'a> Bus<'a> {
    pub fn new(ram: &'a mut dyn MemIntf, ram_size: u64) -> Self {
        Bus { ram, ram_size }
    }

    pub fn reset(&mut self) {
		self.ram.reset();
    }

    pub fn load8(&self, addr: u64) -> Result<u8, Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			Ok((self.ram.load(addr - RAM_BASE)? & 0xff) as u8)
		} else {
        	Err(Exception::Breakpoint(addr))
		}
    }

    pub fn load16(&self, addr: u64) -> Result<u16, Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			Ok((self.ram.load(addr - RAM_BASE)? & 0xffff) as u16)
		} else {
        	Err(Exception::Breakpoint(addr))
		}
    }

    pub fn load32(&self, addr: u64) -> Result<u32, Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			self.ram.load(addr - RAM_BASE)
		} else {
        	Err(Exception::Breakpoint(addr))
		}
    }

    pub fn load64(&self, addr: u64) -> Result<u64, Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			Ok(self.ram.load(addr - RAM_BASE)? as u64 
				| (self.ram.load(addr - RAM_BASE + 1)? as u64) << 32)
		} else {
        	Err(Exception::Breakpoint(addr))
		}
    }

    pub fn store8(&mut self, addr: u64, val: u8) -> Result<(), Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			let prev = self.ram.load(addr - RAM_BASE)?;

			self.ram.store(addr, (prev & 0xffffff00) | (val as u32))?;

			Ok(())
		} else {
        	Err(Exception::Breakpoint(addr))
		}
    }

    pub fn store16(&mut self, addr: u64, val: u16) -> Result<(), Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			let prev = self.ram.load(addr- RAM_BASE)?;

			self.ram.store(addr, (prev & 0xffff0000) | (val as u32))?;

			Ok(())
		} else {
        	Err(Exception::Breakpoint(addr))
		}
    }

    pub fn store32(&mut self, addr: u64, val: u32) -> Result<(), Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			self.ram.store(addr - RAM_BASE, val)?;

			Ok(())
		} else {
        	Err(Exception::Breakpoint(addr))
		}
    }

    pub fn store64(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
		if addr >= RAM_BASE && addr < (RAM_BASE + self.ram_size) {
			self.ram.store(addr - RAM_BASE, (val & 0xffffffff) as u32)?;
			self.ram.store(addr - RAM_BASE, ((val >> 32) & 0xffffffff) as u32)?;

			Ok(())
		} else {
        	Err(Exception::Breakpoint(addr))
		}
	}
}
