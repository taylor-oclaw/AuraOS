extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct AuraPictureInPicture {
    title: String,
    width: u32,
    height: u32,
    position_x: i32,
    position_y: i32,
    visible: bool,
}

impl AuraPictureInPicture {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        AuraPictureInPicture {
            title: String::from(title),
            width,
            height,
            position_x: 0,
            position_y: 0,
            visible: false,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position_x = x;
        self.position_y = y;
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.position_x, self.position_y)
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
