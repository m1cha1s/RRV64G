use crate::exceptions::Exception;

pub trait MemIntf {
	fn reset(&mut self);
	fn load(&self, addr: u64) -> Result<u32, Exception>;
	fn store(&mut self, addr: u64, val: u32) -> Result<(), Exception>;
}

pub struct MemLoc {
    pub start: u64,
    pub len: u64,
}

#[derive(PartialEq)]
pub enum MemType {
	Ram,
}

pub type MemMapEntry<'a> = (MemType, MemLoc, &'a mut dyn MemIntf);

pub struct Bus<'a> {
	pub mem_map: &'a mut [MemMapEntry<'a>],
}

impl<'a> Bus<'a> {
    pub fn new(mem_map: &'a mut [MemMapEntry<'a>]) -> Self {
        Bus { mem_map }
    }

    pub fn reset(&mut self) {
		for (_, _, intf) in self.mem_map.iter_mut() {
			intf.reset();
		}
    }

    pub fn load8(&self, addr: u64) -> Result<u8, Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter()
			.find(|(_, entry, _)| addr >= entry.start && addr < entry.len + entry.start ) {
			Ok((intf.load(addr)? & 0xff) as u8)
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }

    pub fn load16(&self, addr: u64) -> Result<u16, Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter()
			.find(|(_, entry, _)| addr >= entry.start && addr < entry.len + entry.start ) {
			Ok((intf.load(addr)? & 0xffff) as u16)
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }

    pub fn load32(&self, addr: u64) -> Result<u32, Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter()
			.find(|(_, entry, _)| addr >= entry.start && addr < entry.len + entry.start ) {
			intf.load(addr - entry.start)
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }

    pub fn load64(&self, addr: u64) -> Result<u64, Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter()
			.find(|(_, entry, _)| addr >= entry.start && addr + 1 < entry.len + entry.start ) {
			Ok(intf.load(addr - entry.start)? as u64 
				| (intf.load(addr - entry.start + 1)? as u64) << 32)
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }

    pub fn store8(&mut self, addr: u64, val: u8) -> Result<(), Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter_mut()
			.find(|(_, entry, _)| addr >= entry.start && addr < entry.len + entry.start ) {
			let prev = intf.load(addr)?;

			intf.store(addr, (prev & 0xffffff00) | (val as u32))?;

			Ok(())
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }

    pub fn store16(&mut self, addr: u64, val: u16) -> Result<(), Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter_mut()
			.find(|(_, entry, _)| addr >= entry.start && addr < entry.len + entry.start ) {
			let prev = intf.load(addr)?;

			intf.store(addr, (prev & 0xffff0000) | (val as u32))?;

			Ok(())
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }

    pub fn store32(&mut self, addr: u64, val: u32) -> Result<(), Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter_mut()
			.find(|(_, entry, _)| addr >= entry.start && addr < entry.len + entry.start ) {
			intf.store(addr, val)?;

			Ok(())
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }

    pub fn store64(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
		if let Some((_, entry, intf)) = self.mem_map.iter_mut()
			.find(|(_, entry, _)| addr >= entry.start && addr < entry.len + entry.start ) {

			intf.store(addr, (val & 0xffffffff) as u32)?;
			intf.store(addr, ((val >> 32) & 0xffffffff) as u32)?;

			Ok(())
		} else {
        	Err(Exception::AddressOutOfBounds(addr))
		}
    }
}
