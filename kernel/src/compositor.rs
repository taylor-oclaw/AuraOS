extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct Window {
    pub id: u32,
    pub title: String,
    pub rect: Rect,
    pub z_order: i32,
    pub visible: bool,
    pub buffer: Vec<u32>,
}

pub struct DamageRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub struct Compositor {
    pub windows: Vec<Window>,
    pub damage: Vec<DamageRect>,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Compositor {
    pub fn new(w: u32, h: u32) -> Self {
        Compositor {
            windows: Vec::new(),
            damage: Vec::new(),
            screen_width: w,
            screen_height: h,
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn remove_window(&mut self, id: u32) {
        self.windows.retain(|w| w.id != id);
    }

    pub fn raise_window(&mut self, id: u32) {
        if let Some(index) = self.windows.iter().position(|w| w.id == id) {
            let window = self.windows.remove(index);
            self.windows.push(window);
        }
    }

    pub fn composite(&mut self, output: &mut [u32]) {
        for window in self.windows.iter_mut() {
            if window.visible {
                let x1 = window.rect.x.max(0) as usize;
                let y1 = window.rect.y.max(0) as usize;
                let x2 = (window.rect.x + window.rect.width as i32).min(self.screen_width as i32) as usize;
                let y2 = (window.rect.y + window.rect.height as i32).min(self.screen_height as i32) as usize;

                for y in y1..y2 {
                    for x in x1..x2 {
                        let src_index = ((y - window.rect.y as usize) * window.rect.width as usize) + (x - window.rect.x as usize);
                        let dst_index = (y * self.screen_width as usize) + x;
                        output[dst_index] = window.buffer[src_index];
                    }
                }
            }
        }
    }

    pub fn mark_damage(&mut self, rect: DamageRect) {
        self.damage.push(rect);
    }

    pub fn window_at(&self, x: i32, y: i32) -> Option<u32> {
        for window in self.windows.iter().rev() {
            if window.visible && x >= window.rect.x && x < (window.rect.x + window.rect.width as i32) &&
               y >= window.rect.y && y < (window.rect.y + window.rect.height as i32) {
                return Some(window.id);
            }
        }
        None
    }
}
