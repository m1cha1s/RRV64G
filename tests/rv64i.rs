extern crate std;

use rrv64g::prelude::*;

pub struct Mem {
    pub mem: Vec<u8>,
}

impl MemIntf for Mem {
    fn reset(&mut self) {
        self.mem.clear();
    }

    fn load(&self, addr: u64) -> Result<u32, Exception> {
        let addr = addr as usize;
        if addr + 3 < self.mem.len() {
            Ok((self.mem[addr] as u32)
                | (self.mem[addr+1] as u32) << 8
                | (self.mem[addr+2] as u32) << 16
                | (self.mem[addr+3] as u32) << 24)
        } else {
            Err(Exception::AddressOutOfBounds(addr as u64))
        }
    }

    fn store(&mut self, addr: u64, val: u32) -> Result<(), Exception> {
        let addr = addr as usize;
        if addr < self.mem.len() {
            self.mem[addr]     = (val & 0xff) as u8;
            self.mem[addr + 1] = ((val >> 8) & 0xff) as u8;
            self.mem[addr + 2] = ((val >> 16) & 0xff) as u8;
            self.mem[addr + 3] = ((val >> 24) & 0xff) as u8;
            Ok(())
        } else {
            Err(Exception::AddressOutOfBounds(addr as u64))
        }
    }
}

#[test]
fn decoding() {
	println!("{:?}", Cpu::decode(0xfae7d4e3));

	assert_eq!(0, 1);
}

#[test]
fn integer_arithmetic() {
	let code = vec![
		0x13, 0x08, 0x50, 0x00, // addi x16, x0, 5 => mv 5 to x16
	];

	// Create a memory with our program
	let mut mem: Mem = Mem { mem: code };

	// Create a memory map with our memory
	let mem_map: &mut [MemMapEntry] = &mut [
		(
			MemType::Ram,
			MemLoc { start: 0x00000000, len: mem.mem.len() as u64 },
			&mut mem,
		),
	];

	// Create the rest of the emulator
	let mut cpu = Cpu::new(mem_map);

	let e = cpu.tick();

	println!("{:?}", e);

	assert!(cpu.regs.x[16] == 5, "Addi fail");
}
