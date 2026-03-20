extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SerialPort {
    base_address: u16,
}

impl SerialPort {
    pub fn new(base_address: u16) -> Self {
        SerialPort { base_address }
    }

    pub fn initialize(&self) {
        unsafe {
            // Disable interrupts
            outb(self.base_address + 1, 0x00);
            // Enable DLAB (set baud rate divisor)
            outb(self.base_address + 3, 0x80);
            // Set divisor to 3 (lo byte) 19200 baud (hi byte is 0)
            outb(self.base_address, 0x03);
            outb(self.base_address + 1, 0x00);
            // Enable FIFO, clear them, with 14-byte threshold
            outb(self.base_address + 2, 0xC7);
            // IRQs enabled, RTS/DSR set
            outb(self.base_address + 4, 0x0B);
        }
    }

    pub fn write_byte(&self, byte: u8) {
        while self.is_transmit_fifo_full() {}
        unsafe { outb(self.base_address, byte); }
    }

    pub fn read_byte(&self) -> Option<u8> {
        if self.is_receive_fifo_empty() {
            None
        } else {
            Some(unsafe { inb(self.base_address) })
        }
    }

    pub fn write_string(&self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn read_line(&self) -> String {
        let mut line = String::new();
        loop {
            if let Some(byte) = self.read_byte() {
                if byte == b'\n' || byte == b'\r' {
                    break;
                }
                line.push(byte as char);
            }
        }
        line
    }

    fn is_transmit_fifo_full(&self) -> bool {
        unsafe { (inb(self.base_address + 5) & 0x20) != 0 }
    }

    fn is_receive_fifo_empty(&self) -> bool {
        unsafe { (inb(self.base_address + 5) & 0x1) == 0 }
    }
}

unsafe fn outb(port: u16, value: u8) {
    asm!("outb %al, %dx", in("al") value, in("dx") port);
}

unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("inb %dx, %al", out("al") result, in("dx") port);
    result
}
