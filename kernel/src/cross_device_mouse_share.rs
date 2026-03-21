extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CrossDeviceMouseShare {
    device_id: u32,
    position_x: i32,
    position_y: i32,
    buttons_state: u8,
    scroll_delta: i16,
}

impl CrossDeviceMouseShare {
    pub fn new(device_id: u32) -> Self {
        CrossDeviceMouseShare {
            device_id,
            position_x: 0,
            position_y: 0,
            buttons_state: 0,
            scroll_delta: 0,
        }
    }

    pub fn get_device_id(&self) -> u32 {
        self.device_id
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position_x = x;
        self.position_y = y;
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.position_x, self.position_y)
    }

    pub fn update_buttons_state(&mut self, button_mask: u8, is_pressed: bool) {
        if is_pressed {
            self.buttons_state |= button_mask;
        } else {
            self.buttons_state &= !button_mask;
        }
    }

    pub fn get_buttons_state(&self) -> u8 {
        self.buttons_state
    }

    pub fn update_scroll_delta(&mut self, delta: i16) {
        self.scroll_delta += delta;
    }

    pub fn get_scroll_delta(&self) -> i16 {
        self.scroll_delta
    }
}
