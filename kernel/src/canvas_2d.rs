extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod canvas_2d {
    use super::*;

    pub struct Canvas2D {
        width: usize,
        height: usize,
        pixels: Vec<Vec<u32>>,
    }

    impl Canvas2D {
        pub fn new(width: usize, height: usize) -> Self {
            let mut pixels = Vec::with_capacity(height);
            for _ in 0..height {
                let row = vec![0u32; width];
                pixels.push(row);
            }
            Canvas2D { width, height, pixels }
        }

        pub fn get_width(&self) -> usize {
            self.width
        }

        pub fn get_height(&self) -> usize {
            self.height
        }

        pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
            if x < self.width && y < self.height {
                self.pixels[y][x] = color;
            }
        }

        pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
            if x < self.width && y < self.height {
                Some(self.pixels[y][x])
            } else {
                None
            }
        }

        pub fn clear(&mut self, color: u32) {
            for row in &mut self.pixels {
                for pixel in row.iter_mut() {
                    *pixel = color;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::canvas_2d::Canvas2D;

    #[test]
    fn test_new() {
        let canvas = Canvas2D::new(10, 10);
        assert_eq!(canvas.get_width(), 10);
        assert_eq!(canvas.get_height(), 10);
    }

    #[test]
    fn test_set_get_pixel() {
        let mut canvas = Canvas2D::new(5, 5);
        canvas.set_pixel(2, 3, 0xFF0000);
        assert_eq!(canvas.get_pixel(2, 3), Some(0xFF0000));
        assert_eq!(canvas.get_pixel(5, 5), None);
    }

    #[test]
    fn test_clear() {
        let mut canvas = Canvas2D::new(3, 3);
        canvas.set_pixel(1, 1, 0x00FF00);
        canvas.clear(0x0000FF);
        for row in &canvas.pixels {
            for &pixel in row {
                assert_eq!(pixel, 0x0000FF);
            }
        }
    }
}
