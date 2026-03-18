use spin::Mutex;
use x86_64::instructions::port::Port;

pub struct SerialPort {
    data: Port<u8>,
    int_enable: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_status: Port<u8>,
}

impl SerialPort {
    pub const unsafe fn new(base: u16) -> Self {
        SerialPort {
            data: Port::new(base),
            int_enable: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_status: Port::new(base + 5),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.int_enable.write(0x00);
            self.line_ctrl.write(0x80);
            self.data.write(0x03);
            self.int_enable.write(0x00);
            self.line_ctrl.write(0x03);
            self.fifo_ctrl.write(0xC7);
            self.modem_ctrl.write(0x0B);
            self.int_enable.write(0x01);
        }
    }

    fn line_status(&mut self) -> u8 {
        unsafe { self.line_status.read() }
    }

    pub fn send(&mut self, byte: u8) {
        while self.line_status() & 0x20 == 0 {}
        unsafe { self.data.write(byte) }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.send(byte);
        }
    }
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

pub fn init() {
    SERIAL1.lock().init();
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_serial_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _serial_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).unwrap();
}
