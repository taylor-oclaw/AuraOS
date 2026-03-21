//! Surface System — AuraOS's UI primitive
//!
//! Surfaces replace "windows" in traditional OSes.
//! A Surface is a rectangular area that can contain:
//! - A web view (like a browser tab)
//! - A native app
//! - A terminal
//! - AI-generated content
//! - A widget (clock, weather, calendar)
//!
//! Surfaces are managed by the AI — they appear, disappear,
//! resize, and rearrange based on what you're doing.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// A Surface — the fundamental UI element
#[derive(Clone)]
pub struct Surface {
    pub id: u64,
    pub title: String,
    pub surface_type: SurfaceType,
    pub rect: Rect,
    pub visible: bool,
    pub focused: bool,
    pub z_order: i32,
    pub content: SurfaceContent,
    pub decoration: Decoration,
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SurfaceType {
    /// Full application
    App,
    /// Terminal/shell
    Terminal,
    /// Web content
    Web,
    /// AI companion chat
    Companion,
    /// Widget (small, always visible)
    Widget,
    /// System overlay (notifications, etc)
    Overlay,
    /// Fullscreen
    Fullscreen,
}

#[derive(Clone)]
pub enum SurfaceContent {
    /// Raw pixel buffer (RGBA)
    Pixels { width: u32, height: u32, data: Vec<u8> },
    /// Text content (rendered by compositor)
    Text { lines: Vec<String>, scroll_offset: u32 },
    /// Placeholder (surface exists but no content yet)
    Empty,
}

#[derive(Clone, Copy)]
pub struct Decoration {
    pub title_bar: bool,
    pub title_bar_height: u32,
    pub border: bool,
    pub border_width: u32,
    pub rounded_corners: bool,
    pub shadow: bool,
    pub title_bar_color: (u8, u8, u8),
    pub border_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
}

impl Decoration {
    pub fn default_window() -> Self {
        Decoration {
            title_bar: true,
            title_bar_height: 28,
            border: true,
            border_width: 1,
            rounded_corners: true,
            shadow: true,
            title_bar_color: (30, 30, 50),
            border_color: (60, 60, 80),
            background_color: (15, 15, 25),
        }
    }

    pub fn borderless() -> Self {
        Decoration {
            title_bar: false, title_bar_height: 0,
            border: false, border_width: 0,
            rounded_corners: false, shadow: false,
            title_bar_color: (0, 0, 0),
            border_color: (0, 0, 0),
            background_color: (15, 15, 25),
        }
    }

    pub fn widget() -> Self {
        Decoration {
            title_bar: false, title_bar_height: 0,
            border: true, border_width: 1,
            rounded_corners: true, shadow: true,
            title_bar_color: (0, 0, 0),
            border_color: (40, 40, 60),
            background_color: (20, 20, 35),
        }
    }
}

/// Layout modes — how surfaces are arranged
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    /// One surface fills the screen
    Focus,
    /// Two surfaces side by side
    Split,
    /// Grid of surfaces
    Grid,
    /// Stacked (swipe between)
    Stack,
    /// Free-form (traditional window management)
    Free,
}

/// The Surface Manager
pub struct SurfaceManager {
    pub surfaces: Vec<Surface>,
    pub layout: LayoutMode,
    pub screen_width: u32,
    pub screen_height: u32,
    pub next_id: u64,
    pub focused_id: Option<u64>,
    pub mouse_cursor: (i32, i32),
    pub cursor_visible: bool,
}

impl SurfaceManager {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        SurfaceManager {
            surfaces: Vec::new(),
            layout: LayoutMode::Free,
            screen_width,
            screen_height,
            next_id: 1,
            focused_id: None,
            mouse_cursor: (screen_width as i32 / 2, screen_height as i32 / 2),
            cursor_visible: true,
        }
    }

    /// Create a new surface
    pub fn create(&mut self, title: &str, stype: SurfaceType, x: i32, y: i32, w: u32, h: u32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let decoration = match stype {
            SurfaceType::App | SurfaceType::Terminal => Decoration::default_window(),
            SurfaceType::Widget => Decoration::widget(),
            SurfaceType::Fullscreen | SurfaceType::Overlay => Decoration::borderless(),
            _ => Decoration::default_window(),
        };

        self.surfaces.push(Surface {
            id,
            title: String::from(title),
            surface_type: stype,
            rect: Rect { x, y, width: w, height: h },
            visible: true,
            focused: false,
            z_order: self.surfaces.len() as i32,
            content: SurfaceContent::Empty,
            decoration,
        };

        self.focus(id);
        id
    }

    /// Focus a surface (bring to front)
    pub fn focus(&mut self, id: u64) {
        let max_z = self.surfaces.len() as i32;
        for s in &mut self.surfaces {
            s.focused = s.id == id;
            if s.id == id {
                s.z_order = max_z;
            }
        }
        self.focused_id = Some(id);
    }

    /// Close a surface
    pub fn close(&mut self, id: u64) {
        self.surfaces.retain(|s| s.id != id);
        if self.focused_id == Some(id) {
            self.focused_id = self.surfaces.last().map(|s| s.id);
        }
    }

    /// Move a surface
    pub fn move_surface(&mut self, id: u64, x: i32, y: i32) {
        if let Some(s) = self.surfaces.iter_mut().find(|s| s.id == id) {
            s.rect.x = x;
            s.rect.y = y;
        }
    }

    /// Resize a surface
    pub fn resize(&mut self, id: u64, w: u32, h: u32) {
        if let Some(s) = self.surfaces.iter_mut().find(|s| s.id == id) {
            s.rect.width = w;
            s.rect.height = h;
        }
    }

    /// Apply layout
    pub fn apply_layout(&mut self) {
        let visible: Vec<u64> = self.surfaces.iter()
            .filter(|s| s.visible)
            .map(|s| s.id)
            .collect();

        match self.layout {
            LayoutMode::Focus => {
                if let Some(&id) = visible.last() {
                    for s in &mut self.surfaces {
                        if s.id == id {
                            s.rect = Rect { x: 0, y: 0, width: self.screen_width, height: self.screen_height };
                        }
                    }
                }
            }
            LayoutMode::Split => {
                let half_w = self.screen_width / 2;
                for (i, &id) in visible.iter().rev().take(2).enumerate() {
                    if let Some(s) = self.surfaces.iter_mut().find(|s| s.id == id) {
                        s.rect = Rect {
                            x: if i == 0 { 0 } else { half_w as i32 },
                            y: 0,
                            width: half_w,
                            height: self.screen_height,
                        };
                    }
                }
            }
            LayoutMode::Grid => {
                let count = visible.len();
                let cols = libm::ceilf(crate::math::sqrtf(count as f32)) as u32;
                let rows = (count as u32 + cols - 1) / cols;
                let cell_w = self.screen_width / cols;
                let cell_h = self.screen_height / rows;

                for (i, &id) in visible.iter().enumerate() {
                    let col = (i as u32) % cols;
                    let row = (i as u32) / cols;
                    if let Some(s) = self.surfaces.iter_mut().find(|s| s.id == id) {
                        s.rect = Rect {
                            x: (col * cell_w) as i32,
                            y: (row * cell_h) as i32,
                            width: cell_w,
                            height: cell_h,
                        };
                    }
                }
            }
            _ => {} // Free and Stack keep current positions
        }
    }

    /// Render all surfaces to the framebuffer
    pub fn render(&self, fb: &mut [u8], stride: usize, bpp: usize) {
        // Sort by z_order
        let mut sorted: Vec<&Surface> = self.surfaces.iter()
            .filter(|s| s.visible)
            .collect();
        sorted.sort_by_key(|s| s.z_order);

        for surface in &sorted {
            self.render_surface(surface, fb, stride, bpp);
        }

        // Draw mouse cursor
        if self.cursor_visible {
            self.draw_cursor(fb, stride, bpp);
        }
    }

    fn render_surface(&self, surface: &Surface, fb: &mut [u8], stride: usize, bpp: usize) {
        let r = &surface.rect;
        let d = &surface.decoration;

        // Background
        self.fill_rect(fb, stride, bpp,
            r.x, r.y, r.width, r.height,
            d.background_color.0, d.background_color.1, d.background_color.2);

        // Title bar
        if d.title_bar {
            let title_color = if surface.focused {
                (0, 150, 255) // Bright blue when focused
            } else {
                d.title_bar_color
            };
            self.fill_rect(fb, stride, bpp,
                r.x, r.y, r.width, d.title_bar_height,
                title_color.0, title_color.1, title_color.2);

            // Title text (simplified — just draw colored block for now)
            // TODO: render actual text using font
        }

        // Border
        if d.border {
            let bw = d.border_width;
            let bc = if surface.focused {
                (0, 180, 255)
            } else {
                d.border_color
            };
            // Top
            self.fill_rect(fb, stride, bpp, r.x, r.y, r.width, bw, bc.0, bc.1, bc.2);
            // Bottom
            self.fill_rect(fb, stride, bpp, r.x, r.y + r.height as i32 - bw as i32, r.width, bw, bc.0, bc.1, bc.2);
            // Left
            self.fill_rect(fb, stride, bpp, r.x, r.y, bw, r.height, bc.0, bc.1, bc.2);
            // Right
            self.fill_rect(fb, stride, bpp, r.x + r.width as i32 - bw as i32, r.y, bw, r.height, bc.0, bc.1, bc.2);
        }
    }

    fn fill_rect(&self, fb: &mut [u8], stride: usize, bpp: usize,
                 x: i32, y: i32, w: u32, h: u32, r: u8, g: u8, b: u8) {
        for dy in 0..h as i32 {
            let py = y + dy;
            if py < 0 || py >= self.screen_height as i32 { continue; }
            for dx in 0..w as i32 {
                let px = x + dx;
                if px < 0 || px >= self.screen_width as i32 { continue; }
                let offset = (py as usize * stride + px as usize) * bpp;
                if offset + 2 < fb.len() {
                    fb[offset] = b;     // BGR format
                    fb[offset + 1] = g;
                    fb[offset + 2] = r;
                }
            }
        }
    }

    fn draw_cursor(&self, fb: &mut [u8], stride: usize, bpp: usize) {
        let (cx, cy) = self.mouse_cursor;
        // Simple arrow cursor (8x12)
        let cursor = [
            0b10000000u8,
            0b11000000,
            0b11100000,
            0b11110000,
            0b11111000,
            0b11111100,
            0b11111110,
            0b11111111,
            0b11111000,
            0b11011000,
            0b10001100,
            0b00001100,
        ];
        for (dy, row) in cursor.iter().enumerate() {
            for dx in 0..8 {
                if (row >> (7 - dx)) & 1 == 1 {
                    let px = cx + dx;
                    let py = cy + dy as i32;
                    if px >= 0 && px < self.screen_width as i32 &&
                       py >= 0 && py < self.screen_height as i32 {
                        let offset = (py as usize * stride + px as usize) * bpp;
                        if offset + 2 < fb.len() {
                            fb[offset] = 255;
                            fb[offset + 1] = 255;
                            fb[offset + 2] = 255;
                        }
                    }
                }
            }
        }
    }

    pub fn surface_count(&self) -> usize {
        self.surfaces.len()
    }
)}
