extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut view = SmartHome3DView::new();
    view.add_room("Living Room");
    view.add_room("Kitchen");
    view.add_room("Bedroom");
    view.set_current_room("Living Room");
    view.describe_current_room();
    loop {}
}

pub struct SmartHome3DView {
    rooms: Vec<String>,
    current_room: Option<usize>,
}

impl SmartHome3DView {
    pub fn new() -> Self {
        SmartHome3DView {
            rooms: Vec::new(),
            current_room: None,
        }
    }

    pub fn add_room(&mut self, room_name: &str) {
        self.rooms.push(String::from(room_name));
    }

    pub fn set_current_room(&mut self, room_name: &str) {
        if let Some(index) = self.rooms.iter().position(|r| r == room_name) {
            self.current_room = Some(index);
        }
    }

    pub fn describe_current_room(&self) {
        if let Some(index) = self.current_room {
        } else {
        }
    }

    pub fn list_rooms(&self) {
        for (i, room) in self.rooms.iter().enumerate() {
            if let Some(index) = self.current_room {
                if i == index {
                } else {
                }
            } else {
            }
        }
    }

    pub fn remove_room(&mut self, room_name: &str) {
        if let Some(index) = self.rooms.iter().position(|r| r == room_name) {
            self.rooms.remove(index);
            if self.current_room == Some(index) {
                self.current_room = None;
            } else if index < self.current_room.unwrap_or(0) {
                self.current_room = Some(self.current_room.unwrap() - 1);
            }
        }
    }
}
