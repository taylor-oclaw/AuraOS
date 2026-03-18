//! Framebuffer-based text rendering
//! Uses the bootloader-provided framebuffer instead of VGA text mode.
//! This works on ALL hardware (VGA text mode doesn't work on UEFI).

use bootloader_api::info::{FrameBuffer, PixelFormat};
use core::fmt;
use spin::Mutex;

/// Built-in 8x16 bitmap font (basic ASCII)
mod font {
    // Simple 8x16 font - each character is 16 bytes (one byte per row, 8 pixels wide)
    include!("font_data.rs");
}

pub static FB_WRITER: spin::Lazy<Mutex<Option<FbWriter>>> = spin::Lazy::new(|| Mutex::new(None));

pub struct FbWriter {
    fb: &'static mut [u8],
    width: usize,
    height: usize,
    stride: usize,
    bpp: usize,
    format: PixelFormat,
    col: usize,
    row: usize,
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
}

const CHAR_W: usize = 8;
const CHAR_H: usize = 16;

impl FbWriter {
    pub fn new(fb: &'static mut FrameBuffer) -> Self {
        let info = fb.info();
        let width = info.width;
        let height = info.height;
        let stride = info.stride;
        let bpp = info.bytes_per_pixel;
        let format = info.pixel_format;
        let buf = fb.buffer_mut();

        // Clear to dark background (fast — zero all bytes)
        let len = buf.len();
        let ptr = buf.as_mut_ptr();
        unsafe { core::ptr::write_bytes(ptr, 0, len); }

        Self {
            fb: buf,
            width, height, stride, bpp, format,
            col: 0, row: 0,
            fg: (0, 210, 255),    // AuraOS cyan
            bg: (8, 8, 24),       // Deep navy
        }
    }

    pub fn set_fg(&mut self, r: u8, g: u8, b: u8) { self.fg = (r, g, b); }
    pub fn set_bg(&mut self, r: u8, g: u8, b: u8) { self.bg = (r, g, b); }

    fn put_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height { return; }
        let offset = (y * self.stride + x) * self.bpp;
        if offset + 2 >= self.fb.len() { return; }
        match self.format {
            PixelFormat::Bgr => {
                self.fb[offset] = b;
                self.fb[offset + 1] = g;
                self.fb[offset + 2] = r;
            }
            PixelFormat::Rgb => {
                self.fb[offset] = r;
                self.fb[offset + 1] = g;
                self.fb[offset + 2] = b;
            }
            _ => {
                self.fb[offset] = r;
                self.fb[offset + 1] = g;
                self.fb[offset + 2] = b;
            }
        }
    }

    fn draw_char(&mut self, c: u8, col: usize, row: usize) {
        let x0 = col * CHAR_W;
        let y0 = row * CHAR_H;
        let glyph = font::get_glyph(c);

        for dy in 0..CHAR_H {
            let row_bits = glyph[dy];
            for dx in 0..CHAR_W {
                let pixel_set = (row_bits >> (7 - dx)) & 1 == 1;
                let (r, g, b) = if pixel_set { self.fg } else { self.bg };
                self.put_pixel(x0 + dx, y0 + dy, r, g, b);
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        let max_cols = self.width / CHAR_W;
        let max_rows = self.height / CHAR_H;

        match byte {
            b'\n' => {
                self.col = 0;
                self.row += 1;
                if self.row >= max_rows {
                    self.scroll();
                    self.row = max_rows - 1;
                }
            }
            byte => {
                if self.col >= max_cols {
                    self.col = 0;
                    self.row += 1;
                    if self.row >= max_rows {
                        self.scroll();
                        self.row = max_rows - 1;
                    }
                }
                self.draw_char(byte, self.col, self.row);
                self.col += 1;
            }
        }
    }

    fn scroll(&mut self) {
        // Move everything up by one character row
        let row_bytes = self.stride * self.bpp * CHAR_H;
        let total_bytes = self.stride * self.bpp * self.height;

        // Copy rows up
        for i in row_bytes..total_bytes {
            if i < self.fb.len() && i >= row_bytes {
                self.fb[i - row_bytes] = self.fb[i];
            }
        }

        // Clear bottom row
        let start = total_bytes.saturating_sub(row_bytes);
        for i in start..total_bytes.min(self.fb.len()) {
            self.fb[i] = self.bg.2; // Use bg blue channel as fill
        }
    }

    pub fn clear(&mut self) {
        // Fast clear using memset instead of pixel-by-pixel
        let len = self.fb.len();
        let ptr = self.fb.as_mut_ptr();
        unsafe { core::ptr::write_bytes(ptr, 0, len); }
        self.col = 0;
        self.row = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(b'?'),
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
            self.draw_char(b' ', self.col, self.row);
        }
    }
}

impl fmt::Write for FbWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn init(fb: &'static mut FrameBuffer) {
    let writer = FbWriter::new(fb);
    *FB_WRITER.lock() = Some(writer);
}

pub fn with_writer<F: FnOnce(&mut FbWriter)>(f: F) {
    if let Some(ref mut writer) = *FB_WRITER.lock() {
        f(writer);
    }
}

#[macro_export]
macro_rules! fb_print {
    ($($arg:tt)*) => {
        $crate::framebuffer::with_writer(|w| {
            use core::fmt::Write;
            w.write_fmt(format_args!($($arg)*)).unwrap();
        })
    };
}

#[macro_export]
macro_rules! fb_println {
    () => ($crate::fb_print!("\n"));
    ($($arg:tt)*) => ($crate::fb_print!("{}\n", format_args!($($arg)*)));
}
