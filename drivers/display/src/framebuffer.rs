//! Linear framebuffer driver
//! Used during early boot and as fallback when GPU acceleration isn't available.

use super::PixelFormat;

/// A linear framebuffer mapped into memory
pub struct Framebuffer {
    /// Pointer to the framebuffer memory
    base: *mut u8,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Bytes per scanline (may include padding)
    pub stride: u32,
    /// Pixel format
    pub format: PixelFormat,
    /// Bytes per pixel
    pub bpp: u8,
}

impl Framebuffer {
    /// Fill the entire screen with a color
    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, r, g, b);
            }
        }
    }

    /// Set a single pixel
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = (y * self.stride + x * self.bpp as u32) as usize;
        unsafe {
            match self.format {
                PixelFormat::Bgr8 | PixelFormat::Bgra8 => {
                    *self.base.add(offset) = b;
                    *self.base.add(offset + 1) = g;
                    *self.base.add(offset + 2) = r;
                }
                PixelFormat::Rgb8 | PixelFormat::Rgba8 => {
                    *self.base.add(offset) = r;
                    *self.base.add(offset + 1) = g;
                    *self.base.add(offset + 2) = b;
                }
            }
        }
    }

    /// Draw a filled rectangle
    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_pixel(x + dx, y + dy, r, g, b);
            }
        }
    }

    /// Draw a horizontal gradient
    pub fn gradient_h(&mut self, x: u32, y: u32, w: u32, h: u32,
                       r1: u8, g1: u8, b1: u8,
                       r2: u8, g2: u8, b2: u8) {
        for dx in 0..w {
            let t = dx as f32 / w as f32;
            let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8;
            let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8;
            let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8;
            for dy in 0..h {
                self.set_pixel(x + dx, y + dy, r, g, b);
            }
        }
    }
}
