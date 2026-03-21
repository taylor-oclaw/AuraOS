extern crate alloc;
use alloc::string::String;
use core::fmt::Write;

struct SerialWriter;

impl SerialWriter {
    fn write_byte(&self, byte: u8) {
        unsafe {
            core::arch::asm!("out dx, al", in("dx") 0x3F8u16, in("al") byte);
        }
    }
}

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }
}

pub struct PanicRecord {
    pub file: Option<String>,
    pub line: u32,
    pub message: Option<String>
}

pub fn log_panic(info: &core::panic::PanicInfo) -> PanicRecord {
    let mut w = SerialWriter;
    let _ = write!(w, "KERNEL PANIC: {}", info);
    let file = info.location().map(|l| String::from(l.file()));
    let line = info.location().map(|l| l.line()).unwrap_or(0);
    PanicRecord {
        file,
        line,
        message: None
    }
}

pub fn halt_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}