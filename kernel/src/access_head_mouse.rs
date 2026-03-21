extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut access_head_mouse = AccessHeadMouse::new();
    access_head_mouse.initialize();
    access_head_mouse.update_position(10, 20);
    access_head_mouse.click_left_button();
    access_head_mouse.scroll_wheel(5);
    access_head_mouse.move_cursor_relative(5, -5);
}

pub struct AccessHeadMouse {
    position: (i32, i32),
    button_state: u8,
    scroll_position: i32,
}

impl AccessHeadMouse {
    pub fn new() -> Self {
        AccessHeadMouse {
            position: (0, 0),
            button_state: 0,
            scroll_position: 0,
        }
    }

    pub fn initialize(&mut self) {
        // Initialize the mouse device
        println!("Mouse initialized at position {:?}", self.position);
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
        println!("Mouse moved to position {:?}", self.position);
    }

    pub fn click_left_button(&mut self) {
        self.button_state |= 1;
        println!("Left button clicked");
    }

    pub fn scroll_wheel(&mut self, delta: i32) {
        self.scroll_position += delta;
        println!("Scrolled by {} units", delta);
    }

    pub fn move_cursor_relative(&mut self, dx: i32, dy: i32) {
        self.position.0 += dx;
        self.position.1 += dy;
        println!("Cursor moved relatively to {:?}", (dx, dy));
    }
}
