extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct SmartHomeScheduleManager {
    schedules: Vec<(String, String)>, // (device_id, schedule)
}

impl SmartHomeScheduleManager {
    pub fn new() -> Self {
        SmartHomeScheduleManager {
            schedules: Vec::new(),
        }
    }

    pub fn add_schedule(&mut self, device_id: &str, schedule: &str) {
        self.schedules.push((String::from(device_id), String::from(schedule)));
    }

    pub fn remove_schedule(&mut self, device_id: &str) -> bool {
        let pos = self.schedules.iter().position(|&(ref id, _)| id == device_id);
        if let Some(index) = pos {
            self.schedules.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_schedule(&self, device_id: &str) -> Option<&String> {
        self.schedules.iter().find_map(|&(ref id, ref schedule)| {
            if id == device_id {
                Some(schedule)
            } else {
                None
            }
        }
    }

    pub fn list_schedules(&self) -> Vec<(String, String)> {
        self.schedules.clone()
    }

    pub fn clear_schedules(&mut self) {
        self.schedules.clear();
    }
}
