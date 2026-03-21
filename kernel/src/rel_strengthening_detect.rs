extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut detector = RelStrengtheningDetect::new();
    detector.add_event("event1");
    detector.add_event("event2");
    println!("Total events: {}", detector.event_count());
    if detector.is_strongly_connected() {
        println!("The system is strongly connected.");
    } else {
        println!("The system is not strongly connected.");
    }
    let critical_events = detector.get_critical_events();
    for event in critical_events.iter() {
        println!("Critical Event: {}", event);
    }
}

pub struct RelStrengtheningDetect {
    events: Vec<String>,
    connections: usize,
}

impl RelStrengtheningDetect {
    pub fn new() -> Self {
        RelStrengtheningDetect {
            events: Vec::new(),
            connections: 0,
        }
    }

    pub fn add_event(&mut self, event: &str) {
        self.events.push(String::from(event));
    }

    pub fn remove_event(&mut self, event: &str) -> bool {
        if let Some(index) = self.events.iter().position(|e| e == event) {
            self.events.remove(index);
            true
        } else {
            false
        }
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn is_strongly_connected(&self) -> bool {
        // Simplified logic for demonstration purposes
        self.connections > 0 && self.event_count() > 1
    }

    pub fn get_critical_events(&self) -> Vec<String> {
        // Simplified logic for demonstration purposes
        self.events.iter().filter(|&e| e.starts_with("critical")).cloned().collect()
    }
}
