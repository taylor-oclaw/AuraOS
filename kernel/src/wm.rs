extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct Window {
    pub id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    pub minimized: bool,
    pub focused: bool,
    pub z_order: u32,
}

pub struct WindowManager {
    pub windows: Vec<Window>,
    next_id: u32,
    focused_id: Option<u32>,
    next_z: u32,
}

impl WindowManager {
    pub fn new() -> Self {
        Self { windows: Vec::new(), next_id: 1, focused_id: None, next_z: 1 }
    }

    pub fn create_window(&mut self, title: &str, x: i32, y: i32, width: u32, height: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let z = self.next_z;
        self.next_z += 1;
        self.windows.push(Window {
            id, title: String::from(title), x, y, width, height,
            visible: true, minimized: false, focused: false, z_order: z,
        });
        self.focus_window(id);
        id
    }

    pub fn close_window(&mut self, id: u32) {
        self.windows.retain(|w| w.id != id);
        if self.focused_id == Some(id) {
            self.focused_id = self.windows.last().map(|w| w.id);
            if let Some(fid) = self.focused_id {
                if let Some(w) = self.windows.iter_mut().find(|w| w.id == fid) {
                    w.focused = true;
                }
            }
        }
    }

    pub fn focus_window(&mut self, id: u32) {
        for w in self.windows.iter_mut() {
            w.focused = w.id == id;
        }
        self.focused_id = Some(id);
        self.bring_to_front(id);
    }

    pub fn minimize(&mut self, id: u32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.minimized = true;
            w.focused = false;
        }
        if self.focused_id == Some(id) {
            self.focused_id = None;
        }
    }

    pub fn restore(&mut self, id: u32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.minimized = false;
        }
        self.focus_window(id);
    }

    pub fn move_window(&mut self, id: u32, x: i32, y: i32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.x = x;
            w.y = y;
        }
    }

    pub fn resize_window(&mut self, id: u32, width: u32, height: u32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.width = width;
            w.height = height;
        }
    }

    pub fn get_focused(&self) -> Option<&Window> {
        self.focused_id.and_then(|id| self.windows.iter().find(|w| w.id == id))
    }

    pub fn bring_to_front(&mut self, id: u32) {
        let z = self.next_z;
        self.next_z += 1;
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.z_order = z;
        }
    }

    pub fn windows_by_z_order(&self) -> Vec<&Window> {
        let mut sorted: Vec<&Window> = self.windows.iter().filter(|w| w.visible && !w.minimized).collect();
        sorted.sort_by_key(|w| w.z_order);
        sorted
    }
}
