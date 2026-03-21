extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    let floorplan = SmartHomeFloorplan::new();
    floorplan.add_room("Living Room", 30);
    floorplan.add_room("Bedroom", 25);
    floorplan.add_room("Kitchen", 15);
    floorplan.add_room("Bathroom", 10);


    if let Some(area) = floorplan.room_area("Living Room") {
    } else {
    }

    floorplan.remove_room("Bathroom");


    loop {}
}

pub struct SmartHomeFloorplan {
    rooms: Vec<(String, u32)>,
}

impl SmartHomeFloorplan {
    pub fn new() -> Self {
        SmartHomeFloorplan { rooms: Vec::new() }
    }

    pub fn add_room(&mut self, name: &str, area: u32) {
        self.rooms.push((String::from(name), area));
    }

    pub fn remove_room(&mut self, name: &str) {
        self.rooms.retain(|(room_name, _)| room_name != name);
    }

    pub fn total_area(&self) -> u32 {
        self.rooms.iter().map(|(_, area)| area).sum()
    }

    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    pub fn room_area(&self, name: &str) -> Option<u32> {
        self.rooms.iter().find_map(|(room_name, area)| {
            if room_name == name {
                Some(*area)
            } else {
                None
            }
        }
    }
}
