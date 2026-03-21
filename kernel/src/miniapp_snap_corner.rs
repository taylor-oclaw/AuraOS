extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("miniapp_snap_corner module loaded!");
    0
}

pub struct SnapCornerApp {
    corners: Vec<(i32, i32)>,
    active_corner: Option<usize>,
}

impl SnapCornerApp {
    pub fn new() -> Self {
        SnapCornerApp {
            corners: Vec::new(),
            active_corner: None,
        }
    }

    pub fn add_corner(&mut self, x: i32, y: i32) {
        self.corners.push((x, y));
    }

    pub fn remove_corner(&mut self, index: usize) -> bool {
        if index < self.corners.len() {
            self.corners.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_corners(&self) -> &Vec<(i32, i32)> {
        &self.corners
    }

    pub fn set_active_corner(&mut self, index: usize) -> bool {
        if index < self.corners.len() {
            self.active_corner = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_active_corner(&self) -> Option<(i32, i32)> {
        self.active_corner.map(|index| self.corners[index])
    }
}
