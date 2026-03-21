extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut profile = ProfileWFHFocusBlockWork::new();
    profile.add_focus_area("Coding");
    profile.add_focus_area("Reading");
    profile.log_activity("Started coding session");
    profile.log_activity("Read a chapter of a book");
    profile.update_focus_time(30);
    profile.print_summary();

    loop {}
}

pub struct ProfileWFHFocusBlockWork {
    focus_areas: Vec<String>,
    activities_log: Vec<String>,
    total_focus_time: u32,
}

impl ProfileWFHFocusBlockWork {
    pub fn new() -> Self {
        ProfileWFHFocusBlockWork {
            focus_areas: Vec::new(),
            activities_log: Vec::new(),
            total_focus_time: 0,
        }
    }

    pub fn add_focus_area(&mut self, area: &str) {
        self.focus_areas.push(String::from(area));
    }

    pub fn log_activity(&mut self, activity: &str) {
        self.activities_log.push(String::from(activity));
    }

    pub fn update_focus_time(&mut self, minutes: u32) {
        self.total_focus_time += minutes;
    }

    pub fn get_focus_areas(&self) -> &[String] {
        &self.focus_areas
    }

    pub fn print_summary(&self) {
        for area in &self.focus_areas {
        }
        for activity in &self.activities_log {
        }
    }
}
