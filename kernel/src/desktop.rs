//! Desktop Mode — renders surfaces on the framebuffer
//! This is the GUI that replaces the text shell.

extern crate alloc;
use alloc::string::String;
use crate::surface::{SurfaceManager, SurfaceType, SurfaceContent, LayoutMode};
use crate::framebuffer;

static DESKTOP: spin::Lazy<spin::Mutex<Option<Desktop>>> = spin::Lazy::new(|| {
    spin::Mutex::new(None)
};

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
        
        // Render text content inside surfaces
        let sw = self.manager.screen_width as usize;
        let sh = self.manager.screen_height as usize;
        
        for surface in &self.manager.surfaces {
            if !surface.visible { continue; }
            let cx = surface.rect.x + 10; // Content area with padding
            let cy = surface.rect.y + 34; // Below title bar
            
            match surface.surface_type {
                crate::surface::SurfaceType::Terminal => {
                    // Live terminal content from input router
                    let lines = crate::input_router::display_lines();
                    let line_refs: alloc::vec::Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
                    crate::gui_text::draw_text_block(
                        fb, stride, bpp,
                        &line_refs,
                        cx, cy, 200, 220, 200, sw, sh,
                    );
                }
                crate::surface::SurfaceType::Companion => {
                    // Title
                    crate::gui_text::draw_text(
                        fb, stride, bpp,
                        "Aura", cx, cy,
                        0, 210, 255, sw, sh,
                    );
                    crate::gui_text::draw_text_block(
                        fb, stride, bpp,
                        &[
                            "",
                            "",
                            "Hi! I'm your AI companion.",
                            "I live inside this OS.",
                            "",
                            "Ask me anything, or just",
                            "talk naturally.",
                            "",
                            "Pattern match mode",
                            "(LLM loading soon...)",
                        ],
                        cx, cy, 180, 200, 180, sw, sh,
                    );
                }
                crate::surface::SurfaceType::Widget => {
                    // System info
                    let dt = crate::rtc::read_local_time();
                    let tz = crate::rtc::timezone_name();
                    
                    crate::gui_text::draw_text_block(
                        fb, stride, bpp,
                        &[
                            "System Info",
                            "",
                            "AuraOS v0.1.0",
                            "CPU: x86_64",
                            "RAM: 506 MB",
                        ],
                        cx, cy, 100, 200, 255, sw, sh,
                    );
                }
                _ => {}
            }
        }
        
        // Taskbar text
        crate::gui_text::draw_text(
            fb, stride, bpp,
            "Aura", 14, (h - self.taskbar_height + 8) as i32,
            255, 255, 255, sw, sh,
        );
        
        // Clock in taskbar
        let dt = crate::rtc::read_local_time();
        let (h12, ampm) = if dt.hour == 0 { (12, "AM") } 
            else if dt.hour < 12 { (dt.hour, "AM") }
            else if dt.hour == 12 { (12, "PM") }
            else { (dt.hour - 12, "PM") };
        
        // Format time string
        let time_str = {
            use core::fmt::Write;
            let mut buf = alloc::string::String::new();
            let _ = write!(buf, "{}:{:02} {}", h12, dt.minute, ampm);
            buf
        };
        crate::gui_text::draw_text(
            fb, stride, bpp,
            &time_str, (w - 80) as i32, (self.manager.screen_height - self.taskbar_height + 8) as i32,
            200, 200, 200, sw, sh,
        );
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
)}
