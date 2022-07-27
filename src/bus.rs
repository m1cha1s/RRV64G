use crate::{
    exceptions::Exception,
    prelude::{Clint, Plic, Uart},
};

pub const RAM_BASE: u64 = 0x8000_0000;

pub const PLIC_BASE: u64 = 0xc00_0000;
pub const PLIC_SIZE: u64 = 0x4000000;
pub const PLIC_END: u64 = PLIC_BASE + PLIC_SIZE - 1;

pub const CLINT_BASE: u64 = 0x200_0000;
pub const CLINT_SIZE: u64 = 0x10000;
pub const CLINT_END: u64 = CLINT_BASE + CLINT_SIZE - 1;

pub const UART_BASE: u64 = 0x1000_0000;
pub const UART_SIZE: u64 = 0x100;
pub const UART_END: u64 = UART_BASE + UART_SIZE - 1;

pub trait MemIntf {
    fn reset(&mut self);
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception>;
    fn store(&mut self, addr: u64, val: u64, size: u64) -> Result<(), Exception>;
}

pub struct Bus<'a> {
    pub ram: &'a mut dyn MemIntf,
    pub ram_size: u64,

    pub plic: Plic,
    pub clint: Clint,

    pub uart: Uart,
}

impl<'a> Bus<'a> {
    pub fn new(ram: &'a mut dyn MemIntf, ram_size: u64) -> Self {
        Bus {
            ram,
            ram_size,
            plic: Plic::new(),
            clint: Clint::new(),
            uart: Uart::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.plic.reset();
        self.clint.reset();
        self.uart.reset();
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match addr {
            RAM_BASE..=u64::MAX => self.ram.load(addr - RAM_BASE, size),
            PLIC_BASE..=PLIC_END => self.plic.load(addr - PLIC_BASE, size),
            CLINT_BASE..=CLINT_END => self.clint.load(addr - CLINT_BASE, size),
            UART_BASE..=UART_END => self.uart.load(addr - UART_BASE, size),
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }

    pub fn store(&mut self, addr: u64, val: u64, size: u64) -> Result<(), Exception> {
        match addr {
            RAM_BASE.. => self.ram.store(addr - RAM_BASE, val, size),
            PLIC_BASE..=PLIC_END => self.plic.store(addr - PLIC_BASE, val, size),
            CLINT_BASE..=CLINT_END => self.clint.store(addr - CLINT_BASE, val, size),
            UART_BASE..=UART_END => self.uart.store(addr - UART_BASE, val, size),
            _ => Err(Exception::StoreAMOAccessFault(addr)),
        }
    }
}
