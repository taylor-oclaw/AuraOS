extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut tracker = IntroductionTracker::new();
    tracker.add_introduction("Alice");
    tracker.add_introduction("Bob");
    tracker.add_introduction("Charlie");

    if tracker.has_introduced("Alice") {
        println!("Alice has introduced herself.");
    }

    let introductions = tracker.get_all_introductions();
    for intro in introductions.iter() {
        println!("Introduced: {}", intro);
    }

    loop {}
}

pub struct IntroductionTracker {
    introductions: Vec<String>,
}

impl IntroductionTracker {
    pub fn new() -> Self {
        IntroductionTracker {
            introductions: Vec::new(),
        }
    }

    pub fn add_introduction(&mut self, name: &str) {
        if !self.has_introduced(name) {
            self.introductions.push(String::from(name));
        }
    }

    pub fn has_introduced(&self, name: &str) -> bool {
        self.introductions.contains(&String::from(name))
    }

    pub fn get_all_introductions(&self) -> Vec<String> {
        self.introductions.clone()
    }

    pub fn remove_introduction(&mut self, name: &str) {
        if let Some(index) = self.introductions.iter().position(|n| n == name) {
            self.introductions.remove(index);
        }
    }

    pub fn count_introductions(&self) -> usize {
        self.introductions.len()
    }
}
