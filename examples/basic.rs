use std::{
    fs::File,
    io::{self, Read},
};

use rrv64g::prelude::*;

struct Mem {
    pub mem: Vec<u8>,
}

impl Memory for Mem {
    fn get_size(&self) -> u64 {
        self.mem.len() as u64
    }

    fn reset(&mut self) {
        self.mem.clear();
    }

    fn load8(&self, addr: u64) -> Result<u8, Exception> {
        Ok(self.mem[addr as usize])
    }

    fn load16(&self, addr: u64) -> Result<u16, Exception> {
        let addr = addr as usize;
        Ok(self.mem[addr] as u16 | (self.mem[addr + 1] as u16) << 8)
    }

    fn load32(&self, addr: u64) -> Result<u32, Exception> {
        let addr = addr as usize;
        Ok(self.mem[addr] as u32
            | (self.mem[addr + 1] as u32) << 8
            | (self.mem[addr + 2] as u32) << 16
            | (self.mem[addr + 3] as u32) << 24)
    }

    fn load64(&self, addr: u64) -> Result<u64, Exception> {
        let addr = addr as usize;
        Ok(self.mem[addr] as u64
            | (self.mem[addr + 1] as u64) << 8
            | (self.mem[addr + 2] as u64) << 16
            | (self.mem[addr + 3] as u64) << 24
            | (self.mem[addr + 4] as u64) << 32
            | (self.mem[addr + 5] as u64) << 40
            | (self.mem[addr + 6] as u64) << 48
            | (self.mem[addr + 7] as u64) << 56)
    }

    fn store8(&mut self, addr: u64, val: u8) -> Result<(), Exception> {
        let addr = addr as usize;

        self.mem[addr] = val;

        Ok(())
    }

    fn store16(&mut self, addr: u64, val: u16) -> Result<(), Exception> {
        let addr = addr as usize;

        self.mem[addr] = (val & 0xff) as u8;
        self.mem[addr + 1] = ((val >> 8) & 0xff) as u8;

        Ok(())
    }

    fn store32(&mut self, addr: u64, val: u32) -> Result<(), Exception> {
        let addr = addr as usize;

        self.mem[addr] = (val & 0xff) as u8;
        self.mem[addr + 1] = ((val >> 8) & 0xff) as u8;
        self.mem[addr + 2] = ((val >> 16) & 0xff) as u8;
        self.mem[addr + 3] = ((val >> 24) & 0xff) as u8;

        Ok(())
    }

    fn store64(&mut self, addr: u64, val: u64) -> Result<(), Exception> {
        let addr = addr as usize;

        self.mem[addr] = (val & 0xff) as u8;
        self.mem[addr + 1] = ((val >> 8) & 0xff) as u8;
        self.mem[addr + 2] = ((val >> 16) & 0xff) as u8;
        self.mem[addr + 3] = ((val >> 24) & 0xff) as u8;
        self.mem[addr + 4] = ((val >> 32) & 0xff) as u8;
        self.mem[addr + 5] = ((val >> 40) & 0xff) as u8;
        self.mem[addr + 6] = ((val >> 48) & 0xff) as u8;
        self.mem[addr + 7] = ((val >> 56) & 0xff) as u8;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("./examples/add-addi.bin")?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let regs = Regs::new();
    let mut mem: Mem = Mem { mem: code };
    let bus = Bus::new(&mut mem);
    let mut cpu = Cpu::new(bus, regs);

    loop {
        let e = cpu.tick();

        println!("Regs: {:?}, PC: {}", cpu.regs.x, cpu.regs.pc);

        match e {
            Ok(_) => {}
            Err(err) => {
				println!("{:?}", err);
                break;
            }
        }
    }

    Ok(())
}
