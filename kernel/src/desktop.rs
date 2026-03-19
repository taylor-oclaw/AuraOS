//! Desktop Mode — renders surfaces on the framebuffer
//! This is the GUI that replaces the text shell.

extern crate alloc;
use alloc::string::String;
use crate::surface::{SurfaceManager, SurfaceType, SurfaceContent, LayoutMode};
use crate::framebuffer;

static DESKTOP: spin::Lazy<spin::Mutex<Option<Desktop>>> = spin::Lazy::new(|| {
    spin::Mutex::new(None)
});

pub struct Desktop {
    pub manager: SurfaceManager,
    pub active: bool,
    pub taskbar_height: u32,
    pub wallpaper_color: (u8, u8, u8),
}

impl Desktop {
    pub fn new(width: u32, height: u32) -> Self {
        Desktop {
            manager: SurfaceManager::new(width, height),
            active: false,
            taskbar_height: 32,
            wallpaper_color: (8, 8, 24), // Deep navy
        }
    }

    /// Create the default desktop layout
    pub fn setup_default(&mut self) {
        // Companion surface (right side)
        let companion_id = self.manager.create(
            "Aura Companion",
            SurfaceType::Companion,
            880, 40, 380, 640
        );

        // Terminal surface (left side)
        let terminal_id = self.manager.create(
            "Terminal",
            SurfaceType::Terminal,
            20, 40, 840, 400
        );

        // System info widget (bottom left)
        let widget_id = self.manager.create(
            "System",
            SurfaceType::Widget,
            20, 460, 400, 220
        );

        self.active = true;
    }

    /// Render the entire desktop to the framebuffer
    pub fn render(&self, fb: &mut [u8], stride: usize, bpp: usize) {
        let w = self.manager.screen_width;
        let h = self.manager.screen_height;

        // Wallpaper (gradient)
        for y in 0..h {
            let gradient = ((y as f32 / h as f32) * 20.0) as u8;
            for x in 0..w {
                let offset = (y as usize * stride + x as usize) * bpp;
                if offset + 2 < fb.len() {
                    fb[offset] = self.wallpaper_color.2 + gradient;     // B
                    fb[offset + 1] = self.wallpaper_color.1;             // G
                    fb[offset + 2] = self.wallpaper_color.0;             // R
                }
            }
        }

        // Taskbar at bottom
        let taskbar_y = h - self.taskbar_height;
        for y in taskbar_y..h {
            for x in 0..w {
                let offset = (y as usize * stride + x as usize) * bpp;
                if offset + 2 < fb.len() {
                    fb[offset] = 40;      // B
                    fb[offset + 1] = 30;  // G
                    fb[offset + 2] = 20;  // R
                }
            }
        }

        // Taskbar items — just colored blocks for now
        // "Aura" button (left)
        self.fill_rect(fb, stride, bpp, 4, taskbar_y + 4, 80, 24, 0, 120, 255);
        
        // Clock (right) — render time text later
        self.fill_rect(fb, stride, bpp, w - 120, taskbar_y + 4, 116, 24, 30, 30, 50);

        // Render all surfaces
        self.manager.render(fb, stride, bpp);
    }

    fn fill_rect(&self, fb: &mut [u8], stride: usize, bpp: usize,
                 x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8) {
        let sw = self.manager.screen_width;
        let sh = self.manager.screen_height;
        for dy in 0..h {
            let py = y + dy;
            if py >= sh { continue; }
            for dx in 0..w {
                let px = x + dx;
                if px >= sw { continue; }
                let offset = (py as usize * stride + px as usize) * bpp;
                if offset + 2 < fb.len() {
                    fb[offset] = b;
                    fb[offset + 1] = g;
                    fb[offset + 2] = r;
                }
            }
        }
    }
}

/// Initialize the desktop
pub fn init(width: u32, height: u32) {
    let mut desktop = Desktop::new(width, height);
    desktop.setup_default();
    *DESKTOP.lock() = Some(desktop);
}

/// Render desktop to framebuffer
pub fn render(fb: &mut [u8], stride: usize, bpp: usize) {
    if let Some(ref desktop) = *DESKTOP.lock() {
        desktop.render(fb, stride, bpp);
    }
}

/// Update mouse position
pub fn update_mouse(x: i32, y: i32) {
    if let Some(ref mut desktop) = *DESKTOP.lock() {
        desktop.manager.mouse_cursor = (x, y);
    }
}

/// Check if desktop mode is active
pub fn is_active() -> bool {
    if let Some(ref desktop) = *DESKTOP.lock() {
        desktop.active
    } else {
        false
    }
}
