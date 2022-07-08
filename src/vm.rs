use crate::{ bus::{ MemIntf, Bus, RAM_BASE }, cpu::Cpu, exceptions::Exception };

pub struct VM<'a> {
	pub cpu: Cpu,
	pub bus: Bus<'a>,
}

impl<'a> VM<'a> {
	pub fn new(ram_intf: &'a mut dyn MemIntf, ram_len: u64) -> Self {
		let bus = Bus::new(ram_intf, ram_len);
		let mut cpu = Cpu::new();

		cpu.x[2] = RAM_BASE + ram_len;
		
		VM { bus, cpu }
	}

	pub fn tick(&mut self) -> Result<(), Exception>{ 
		self.cpu.tick(&mut self.bus)?;

		if self.cpu.pc == 0 {
			Err(Exception::VMExit)
		} else {
			Ok(())
		}
	}
}
