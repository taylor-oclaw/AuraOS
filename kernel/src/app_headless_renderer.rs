extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn app_headless_renderer_init() {
    // Initialization logic for the headless renderer module
}

pub extern "C" fn app_headless_renderer_exit() {
    // Cleanup logic for the headless renderer module
}

pub struct AppHeadlessRenderer {
    width: u32,
    height: u32,
    frame_buffer: Vec<u8>,
}

impl AppHeadlessRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer_size = (width * height * 4) as usize; // Assuming RGBA format
        AppHeadlessRenderer {
            width,
            height,
            frame_buffer: vec![0; buffer_size],
        }
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in self.frame_buffer.iter_mut() {
            *pixel = (color >> 8) as u8;
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            self.frame_buffer[index] = (color >> 16) as u8; // Red
            self.frame_buffer[index + 1] = (color >> 8) as u8;  // Green
            self.frame_buffer[index + 2] = color as u8;        // Blue
            self.frame_buffer[index + 3] = 0xFF;             // Alpha
        }
    }

    pub fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: u32) {
        let dx = if x2 > x1 { x2 - x1 } else { x1 - x2 };
        let dy = if y2 > y1 { y2 - y1 } else { y1 - y2 };

        let sx = if x2 > x1 { 1 } else { -1 };
        let sy = if y2 > y1 { 1 } else { -1 };

        let mut err = if dx > dy { dx / 2 } else { -(dy / 2) };

        let mut x = x1;
        let mut y = y1;

        loop {
            self.draw_pixel(x, y, color);
            if x == x2 && y == y2 {
                break;
            }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }
}
