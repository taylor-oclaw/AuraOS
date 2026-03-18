use core::fmt;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0, Blue = 1, Green = 2, Cyan = 3,
    Red = 4, Magenta = 5, Brown = 6, LightGray = 7,
    DarkGray = 8, LightBlue = 9, LightGreen = 10, LightCyan = 11,
    LightRed = 12, Pink = 13, Yellow = 14, White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(fg: Color, bg: Color) -> Self {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color: ColorCode,
}

const BUF_H: usize = 25;
const BUF_W: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUF_W]; BUF_H],
}

pub struct Writer {
    col: usize,
    row: usize,
    color: ColorCode,
    buf: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.col >= BUF_W { self.newline(); }
                self.buf.chars[self.row][self.col] = ScreenChar { ascii: byte, color: self.color };
                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.buf.chars[self.row][self.col] = ScreenChar { ascii: b' ', color: self.color };
        }
    }

    fn newline(&mut self) {
        if self.row < BUF_H - 1 {
            self.row += 1;
        } else {
            for row in 1..BUF_H {
                for col in 0..BUF_W {
                    self.buf.chars[row - 1][col] = self.buf.chars[row][col];
                }
            }
            self.clear_row(BUF_H - 1);
        }
        self.col = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar { ascii: b' ', color: self.color };
        for col in 0..BUF_W { self.buf.chars[row][col] = blank; }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUF_H { self.clear_row(row); }
        self.row = 0;
        self.col = 0;
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = ColorCode::new(fg, bg);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub static WRITER: spin::Lazy<Mutex<Writer>> = spin::Lazy::new(|| {
    Mutex::new(Writer {
        col: 0, row: 0,
        color: ColorCode::new(Color::LightCyan, Color::Black),
        buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
    })
});

pub fn clear_screen() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| WRITER.lock().clear_screen());
}

pub fn set_color(fg: Color, bg: Color) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| WRITER.lock().set_color(fg, bg));
}

pub fn backspace() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| WRITER.lock().backspace());
}

pub fn print_banner() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut w = WRITER.lock();
        w.set_color(Color::LightCyan, Color::Black);
        w.write_string("\n");
        w.write_string("     _                        ___  ____  \n");
        w.write_string("    / \\  _   _ _ __ __ _     / _ \\/ ___| \n");
        w.write_string("   / _ \\| | | | '__/ _` |   | | | \\___ \\ \n");
        w.write_string("  / ___ \\ |_| | | | (_| |   | |_| |___) |\n");
        w.write_string(" /_/   \\_\\__,_|_|  \\__,_|    \\___/|____/ \n");
        w.write_string("\n");
        w.set_color(Color::DarkGray, Color::Black);
        w.write_string("  The Ambient Intelligence Operating System\n");
        w.set_color(Color::LightCyan, Color::Black);
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| WRITER.lock().write_fmt(args).unwrap());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
