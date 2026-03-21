extern crate alloc;
use alloc::string::String;

pub const TITLE_BAR_HEIGHT: u32 = 24;
pub const BORDER_WIDTH: u32 = 2;

pub struct WindowDecoration {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub focused: bool,
    pub title_color: u32,
    pub title_bg: u32,
    pub close_hover: bool,
}

impl WindowDecoration {
    pub fn new(title: &str, w: u32, h: u32) -> Self {
        Self {
            title: String::from(title),
            width: w,
            height: h,
            focused: true,
            title_color: 0xECEFF4,
            title_bg: 0x4C566A,
            close_hover: false,
        }
    }

    pub fn draw(&self, buf: &mut [u32], stride: usize, x: usize, y: usize) {
        let total_width = self.total_width() as usize;
        let total_height = self.total_height() as usize;

        // Draw title bar background
        for i in 0..total_width {
            for j in 0..TITLE_BAR_HEIGHT as usize {
                buf[(y + j) * stride + x + i] = self.title_bg;
            }
        }

        // Draw border around window
        for i in 0..total_width {
            buf[y * stride + x + i] = 0x2E3440; // Top border
            buf[(y + total_height - 1) * stride + x + i] = 0x2E3440; // Bottom border
        }
        for j in 0..total_height {
            buf[(y + j) * stride + x] = 0x2E3440; // Left border
            buf[(y + j) * stride + x + total_width - 1] = 0x2E3440; // Right border
        }

        // Draw close button as red square in top-right
        let (close_x, close_y, close_w, close_h) = self.close_button_rect(x, y);
        for i in 0..close_w {
            for j in 0..close_h {
                buf[(close_y + j) * stride + close_x + i] = 0xFF0000;
            }
        }
    }

    pub fn close_button_rect(&self, x: usize, y: usize) -> (usize, usize, usize, usize) {
        let button_size = 16;
        (
            x + self.width as usize - button_size - BORDER_WIDTH as usize,
            y + BORDER_WIDTH as usize,
            button_size,
            button_size,
        )
    }

    pub fn is_in_title_bar(&self, mx: usize, my: usize, wx: usize, wy: usize) -> bool {
        mx >= wx && mx < (wx + self.width as usize)
            && my >= wy && my < (wy + TITLE_BAR_HEIGHT as usize)
    }

    pub fn is_on_close(&self, mx: usize, my: usize, wx: usize, wy: usize) -> bool {
        let (close_x, close_y, close_w, close_h) = self.close_button_rect(wx, wy);
        mx >= close_x && mx < (close_x + close_w)
            && my >= close_y && my < (close_y + close_h)
    }

    pub fn total_height(&self) -> u32 {
        self.height + TITLE_BAR_HEIGHT + BORDER_WIDTH * 2
    }

    pub fn total_width(&self) -> u32 {
        self.width + BORDER_WIDTH * 2
    }
}
