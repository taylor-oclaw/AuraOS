extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct WindowInfo {
    pub id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z_order: i32,
    pub focused: bool,
    pub visible: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub buffer: Vec<u32>,
    pub needs_redraw: bool,
}

pub struct WindowManager {
    pub windows: Vec<WindowInfo>,
    next_id: u32,
    pub focused_id: Option<u32>,
    pub screen_width: u32,
    pub screen_height: u32,
    pub dragging: Option<(u32, i32, i32)>,
    pub resizing: Option<(u32, i32, i32)>,
}

impl WindowManager {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
            focused_id: None,
            screen_width: w,
            screen_height: h,
            dragging: None,
            resizing: None,
        }
    }

    pub fn create_window(&mut self, title: &str, x: i32, y: i32, w: u32, h: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let buf_size = (w * h) as usize;
        let mut buf = Vec::with_capacity(buf_size);
        for _ in 0..buf_size { buf.push(0x3B4252); }
        let z = self.windows.len() as i32;
        self.windows.push(WindowInfo {
            id, title: String::from(title), x, y, width: w, height: h,
            z_order: z, focused: true, visible: true, minimized: false,
            maximized: false, buffer: buf, needs_redraw: true,
        });
        self.focus_window(id);
        id
    }

    pub fn close_window(&mut self, id: u32) {
        self.windows.retain(|w| w.id != id);
        if self.focused_id == Some(id) {
            self.focused_id = self.windows.last().map(|w| w.id);
        }
    }

    pub fn focus_window(&mut self, id: u32) {
        let max_z = self.windows.iter().map(|w| w.z_order).max().unwrap_or(0);
        for w in &mut self.windows {
            w.focused = w.id == id;
            if w.id == id { w.z_order = max_z + 1; }
        }
        self.focused_id = Some(id);
    }

    pub fn move_window(&mut self, id: u32, new_x: i32, new_y: i32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.x = new_x;
            w.y = new_y;
            w.needs_redraw = true;
        }
    }

    pub fn resize_window(&mut self, id: u32, new_w: u32, new_h: u32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.width = new_w;
            w.height = new_h;
            let size = (new_w * new_h) as usize;
            w.buffer.resize(size, 0x3B4252);
            w.needs_redraw = true;
        }
    }

    pub fn minimize_window(&mut self, id: u32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.minimized = !w.minimized;
        }
    }

    pub fn maximize_window(&mut self, id: u32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            if w.maximized {
                w.maximized = false;
            } else {
                w.x = 0;
                w.y = 0;
                w.width = self.screen_width;
                w.height = self.screen_height - 32; // leave taskbar
                w.maximized = true;
            }
            w.needs_redraw = true;
        }
    }

    pub fn window_at(&self, x: i32, y: i32) -> Option<u32> {
        let mut visible: Vec<&WindowInfo> = self.windows.iter()
            .filter(|w| w.visible && !w.minimized)
            .collect();
        visible.sort_by(|a, b| b.z_order.cmp(&a.z_order));
        for w in visible {
            if x >= w.x && x < w.x + w.width as i32 && y >= w.y && y < w.y + w.height as i32 {
                return Some(w.id);
            }
        }
        None
    }

    pub fn start_drag(&mut self, id: u32, mouse_x: i32, mouse_y: i32) {
        if let Some(w) = self.windows.iter().find(|w| w.id == id) {
            self.dragging = Some((id, mouse_x - w.x, mouse_y - w.y));
        }
    }

    pub fn update_drag(&mut self, mouse_x: i32, mouse_y: i32) {
        if let Some((id, off_x, off_y)) = self.dragging {
            self.move_window(id, mouse_x - off_x, mouse_y - off_y);
        }
    }

    pub fn end_drag(&mut self) {
        self.dragging = None;
    }

    pub fn composite(&self, output: &mut [u32]) {
        // Clear to desktop background
        for p in output.iter_mut() { *p = 0x2E3440; }

        // Sort by z-order (lowest first = drawn first)
        let mut sorted: Vec<&WindowInfo> = self.windows.iter()
            .filter(|w| w.visible && !w.minimized)
            .collect();
        sorted.sort_by(|a, b| a.z_order.cmp(&b.z_order));

        for win in sorted {
            // Draw title bar
            let title_h = 24i32;
            for ty in 0..title_h {
                for tx in 0..win.width as i32 {
                    let sx = win.x + tx;
                    let sy = win.y - title_h + ty;
                    if sx >= 0 && sx < self.screen_width as i32 && sy >= 0 && sy < self.screen_height as i32 {
                        let idx = (sy as u32 * self.screen_width + sx as u32) as usize;
                        if idx < output.len() {
                            output[idx] = if win.focused { 0x5E81AC } else { 0x4C566A };
                        }
                    }
                }
            }

            // Draw window content
            for row in 0..win.height {
                for col in 0..win.width {
                    let sx = win.x + col as i32;
                    let sy = win.y + row as i32;
                    if sx >= 0 && sx < self.screen_width as i32 && sy >= 0 && sy < self.screen_height as i32 {
                        let src = (row * win.width + col) as usize;
                        let dst = (sy as u32 * self.screen_width + sx as u32) as usize;
                        if src < win.buffer.len() && dst < output.len() {
                            output[dst] = win.buffer[src];
                        }
                    }
                }
            }
        }
    }

    pub fn window_count(&self) -> usize { self.windows.len() }
    pub fn visible_count(&self) -> usize { self.windows.iter().filter(|w| w.visible && !w.minimized).count() }
}
