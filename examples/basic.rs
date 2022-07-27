use std::{
    fs::File,
    io::{self, Read},
};

use rrv64g::prelude::*;

struct Mem {
    pub mem: Vec<u8>,
}

impl MemIntf for Mem {
    fn reset(&mut self) {
        self.mem.clear();
    }

    fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        let addr = addr as usize;
        if addr + ((size as usize) / 8) - 1 >= self.mem.len() {
            return Err(Exception::LoadAccessFault(addr as u64));
        }

        match size {
            8 => Ok(self.mem[addr] as u64),
            16 => Ok((self.mem[addr] as u64) | (self.mem[addr + 1] as u64) << 8),
            32 => Ok((self.mem[addr] as u64)
                | (self.mem[addr + 1] as u64) << 8
                | (self.mem[addr + 2] as u64) << 16
                | (self.mem[addr + 3] as u64) << 24),
            64 => Ok((self.mem[addr] as u64)
                | (self.mem[addr + 1] as u64) << 8
                | (self.mem[addr + 2] as u64) << 16
                | (self.mem[addr + 3] as u64) << 24
                | (self.mem[addr + 4] as u64) << 32
                | (self.mem[addr + 5] as u64) << 40
                | (self.mem[addr + 6] as u64) << 48
                | (self.mem[addr + 7] as u64) << 56),
            _ => Err(Exception::LoadAccessFault(addr as u64)),
        }
    }

    fn store(&mut self, addr: u64, val: u64, size: u64) -> Result<(), Exception> {
        let addr = addr as usize;
        if addr + ((size as usize) / 8) - 1 >= self.mem.len() {
            return Err(Exception::StoreAMOAccessFault(addr as u64));
        }

        match size {
            8 => self.mem[addr] = (val & 0xff) as u8,
            16 => {
                self.mem[addr] = (val & 0xff) as u8;
                self.mem[addr + 1] = ((val >> 8) & 0xff) as u8;
            }
            32 => {
                self.mem[addr] = (val & 0xff) as u8;
                self.mem[addr + 1] = ((val >> 8) & 0xff) as u8;
                self.mem[addr + 2] = ((val >> 16) & 0xff) as u8;
                self.mem[addr + 3] = ((val >> 24) & 0xff) as u8;
            }
            64 => {
                self.mem[addr] = (val & 0xff) as u8;
                self.mem[addr + 1] = ((val >> 8) & 0xff) as u8;
                self.mem[addr + 2] = ((val >> 16) & 0xff) as u8;
                self.mem[addr + 3] = ((val >> 24) & 0xff) as u8;
                self.mem[addr + 4] = ((val >> 32) & 0xff) as u8;
                self.mem[addr + 5] = ((val >> 40) & 0xff) as u8;
                self.mem[addr + 6] = ((val >> 48) & 0xff) as u8;
                self.mem[addr + 7] = ((val >> 56) & 0xff) as u8;
            }
            _ => return Err(Exception::StoreAMOAccessFault(addr as u64)),
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Loading program from file
    //    let mut file = File::open("./examples/add-addi.bin")?;
    let mut file = File::open("./examples/fib.bin")?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;
    code.resize(1024 * 1024 * 128, 0);

    // Create a memory with our program
    let mut mem: Mem = Mem { mem: code };

    let mut vm = VM::new(&mut mem, 1024 * 1024 * 128);

    vm.cpu.pc = 0x8000_0000;
    println!("{}", vm.cpu.pc);

    println!("{:?}", vm.bus.load(0x8000_0000, 32));

    loop {
        let e = vm.tick();

        match e {
            Ok(()) => println!("Tick"),
            Err(err) => {
                println!("Err: {:?}, Cause: {}", err, vm.cpu.csr[MCAUSE]);
                break;
            }
        }
    }

    Ok(())
}
