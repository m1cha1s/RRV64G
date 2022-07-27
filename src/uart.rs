use crate::prelude::{Exception, MemIntf, UART_BASE, UART_SIZE};

// uart interrupt request
pub const UART_IRQ: u64 = 10;
// Receive holding register (for input bytes).
pub const UART_RHR: u64 = 0;
// Transmit holding register (for output bytes).
pub const UART_THR: u64 = 0;
// Line control register.
pub const UART_LCR: u64 = 3;
// Line status register.
// LSR BIT 0:
//     0 = no data in receive holding register or FIFO.
//     1 = data has been receive and saved in the receive holding register or FIFO.
// LSR BIT 5:
//     0 = transmit holding register is full. 16550 will not accept any data for transmission.
//     1 = transmitter hold register (or FIFO) is empty. CPU can load the next character.
pub const UART_LSR: u64 = 5;
// The receiver (RX) bit MASK.
pub const MASK_UART_LSR_RX: u8 = 1;
// The transmitter (TX) bit MASK.
pub const MASK_UART_LSR_TX: u8 = 1 << 5;

pub struct Uart {
    uart: [u8; UART_SIZE as usize],
    new_tx: bool,
    interrupt: bool,
}

impl Uart {
    pub fn new() -> Self {
        let mut uart = [0; UART_SIZE as usize];
        uart[UART_LSR as usize] |= MASK_UART_LSR_TX;

        let new_tx = false;
        let interrupt = false;

        Self {
            uart,
            new_tx,
            interrupt,
        }
    }

    pub fn tick(&mut self, rx: Option<char>) -> Option<char> {
        if self.uart[UART_LSR as usize] & MASK_UART_LSR_RX == 0 {
            if let Some(rx_char) = rx {
                self.uart[UART_RHR as usize] = rx_char as u8;

                self.interrupt = true;

                self.uart[UART_LSR as usize] |= MASK_UART_LSR_RX;
            }
        }

        if self.new_tx {
            self.new_tx = false;
            return Some(self.uart[UART_THR as usize] as char);
        }

        None
    }
}

impl MemIntf for Uart {
    fn reset(&mut self) {
        self.uart.fill(0);
        self.new_tx = false;
        self.interrupt = false;

        self.uart[UART_LSR as usize] |= MASK_UART_LSR_TX;
    }

    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size != 8 {
            return Err(Exception::LoadAccessFault(addr + UART_BASE));
        }

        match addr {
            UART_RHR => {
                self.uart[UART_RHR as usize] &= !MASK_UART_LSR_RX;
                Ok(self.uart[UART_RHR as usize] as u64)
            }
            _ => Ok(self.uart[addr as usize] as u64),
        }
    }

    fn store(&mut self, addr: u64, val: u64, size: u64) -> Result<(), Exception> {
        if size != 8 {
            return Err(Exception::StoreAMOAccessFault(addr + UART_BASE));
        }

        match addr {
            UART_THR => {
                self.new_tx = true;

                self.uart[UART_THR as usize] = val as u8;

                Ok(())
            }
            _ => {
                self.uart[addr as usize] = val as u8;

                Ok(())
            }
        }
    }
}
