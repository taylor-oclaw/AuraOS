extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;

pub struct DisplayBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub dirty: bool,
}

impl DisplayBuffer {
    pub fn new(w: u32, h: u32) -> Self {
        let size = (w * h * 4) as usize;
        Self { width: w, height: h, pixels: vec![0u8; size], dirty: true }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        if x < self.width && y < self.height {
            let idx = ((y * self.width + x) * 4) as usize;
            if idx + 3 < self.pixels.len() {
                self.pixels[idx] = b;
                self.pixels[idx + 1] = g;
                self.pixels[idx + 2] = r;
                self.pixels[idx + 3] = 255;
                self.dirty = true;
            }
        }
    }

    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_pixel(x + dx, y + dy, r, g, b);
            }
        }
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        self.fill_rect(0, 0, self.width, self.height, r, g, b);
    }

    pub fn draw_string(&mut self, mut x: u32, y: u32, text: &str, r: u8, g: u8, b: u8) {
        for byte in text.bytes() {
            // Simple 8x8 block font - each char is a filled rectangle
            if byte != b' ' {
                self.fill_rect(x, y, 6, 8, r, g, b);
            }
            x += 8;
        }
    }
}

pub struct DisplayServer {
    pub framebuffer: DisplayBuffer,
    pub cursor_x: u32,
    pub cursor_y: u32,
    pub cursor_visible: bool,
    pub initialized: bool,
}

impl DisplayServer {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            framebuffer: DisplayBuffer::new(w, h),
            cursor_x: 0, cursor_y: 0,
            cursor_visible: true, initialized: false,
        }
    }

    pub fn boot_screen(&mut self) {
        self.framebuffer.clear(10, 10, 30); // Dark blue background
        self.framebuffer.draw_string(10, 10, "AuraOS v0.1", 0, 255, 128);
        self.framebuffer.draw_string(10, 30, "Data-Enriched File System: DEFS", 200, 200, 200);
        self.framebuffer.draw_string(10, 50, "Agent-Native Operating System", 200, 200, 200);
        self.framebuffer.draw_string(10, 80, "Booting...", 255, 200, 0);
        self.initialized = true;
    }

    pub fn show_boot_progress(&mut self, step: &str, pct: u8) {
        let y = 100 + (pct as u32 / 12) * 20;
        self.framebuffer.draw_string(10, y, step, 180, 180, 180);
        // Progress bar
        let bar_w = (pct as u32 * 4);
        self.framebuffer.fill_rect(10, y + 12, bar_w, 4, 0, 200, 100);
    }

    pub fn move_cursor(&mut self, x: u32, y: u32) {
        self.cursor_x = x;
        self.cursor_y = y;
    }
}
