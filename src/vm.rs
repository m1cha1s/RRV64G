use crate::{
    bus::{Bus, MemIntf, RAM_BASE},
    cpu::Cpu,
    exceptions::Exception,
};

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

    pub fn tick(&mut self, char_in: Option<char>) -> Result<Option<char>, Exception> {
        match self.cpu.tick(&mut self.bus) {
            Ok(_inst) => {}
            Err(e) => {
                self.cpu.handle_exception(e);
                if e.is_fatal() {
                    return Err(e);
                }
            }
        }

        match self.cpu.check_pending_interrupt(&mut self.bus)? {
            Some(int) => self.cpu.handle_interrupt(int),
            None => (),
        }

        if self.cpu.pc == 0 {
            Err(Exception::Breakpoint(self.cpu.pc))
        } else {
            Ok(self.bus.uart.tick(char_in))
        }
    }
}
