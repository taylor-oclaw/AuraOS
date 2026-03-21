extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Monitor {
    pub id: u64,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub refresh_rate: u32,
    pub primary: bool,
    pub enabled: bool,
    pub scale: f32,
}

pub struct MultiMonitor {
    pub monitors: Vec<Monitor>,
    pub next_id: u64,
}

impl MultiMonitor {
    pub fn new() -> Self {
        let mut mm = Self {
            monitors: Vec::new(),
            next_id: 1,
        };
        mm.add_monitor("Primary", 1920, 1080, 0, 0, 60, true);
        mm
    }

    pub fn add_monitor(&mut self, name: &str, w: u32, h: u32, x: u32, y: u32, hz: u32, primary: bool) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.monitors.push(Monitor {
            id,
            name: String::from(name),
            width: w,
            height: h,
            x_offset: x,
            y_offset: y,
            refresh_rate: hz,
            primary,
            enabled: true,
            scale: 1.0,
        };
        id
    }

    pub fn set_primary(&mut self, id: u64) {
        for m in &mut self.monitors {
            m.primary = m.id == id;
        }
    }

    pub fn total_width(&self) -> u32 {
        self.monitors.iter().filter(|m| m.enabled).map(|m| m.x_offset + m.width).max().unwrap_or(0)
    }

    pub fn total_height(&self) -> u32 {
        self.monitors.iter().filter(|m| m.enabled).map(|m| m.y_offset + m.height).max().unwrap_or(0)
    }

    pub fn monitor_at(&self, x: u32, y: u32) -> Option<&Monitor> {
        self.monitors.iter().find(|m| m.enabled && x >= m.x_offset && x < m.x_offset + m.width && y >= m.y_offset && y < m.y_offset + m.height)
    }

    pub fn count(&self) -> usize {
        self.monitors.iter().filter(|m| m.enabled).count()
    }
)}
