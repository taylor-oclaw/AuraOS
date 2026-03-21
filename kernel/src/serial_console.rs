extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const COM1: u16 = 0x3F8;

fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") val);
    }
}

fn inb(port: u16) -> u8 {
    unsafe {
        let v: u8;
        core::arch::asm!("in al, dx", in("dx") port, out("al") v);
        v
    }
}

pub struct SerialConsole {
    pub port: u16,
}

impl SerialConsole {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn init(&self) {
        outb(self.port + 1, 0);
        outb(self.port + 3, 0x80);
        outb(self.port, 3);
        outb(self.port + 1, 0);
        outb(self.port + 3, 3);
        outb(self.port + 2, 0xC7);
        outb(self.port + 4, 0x0B);
    }

    pub fn write_byte(&self, b: u8) {
        while inb(self.port + 5) & 0x20 == 0 {}
        outb(self.port, b);
    }

    pub fn read_byte(&self) -> Option<u8> {
        if inb(self.port + 5) & 1 != 0 {
            Some(inb(self.port))
        } else {
            None
        }
    }

    pub fn write_str(&self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b);
        }
    }
}
