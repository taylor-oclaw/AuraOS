extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

pub enum Event {
    KeyPress(char),
    MouseMove(i32, i32),
    MouseClick(i32, i32, u8),
    WindowFocus(u32),
    WindowClose(u32),
    Timer(u64),
    AppLaunch(String),
    AppQuit(String),
    Notification(String),
    SystemShutdown,
}

pub struct EventQueue {
    events: Vec<Event>,
    max_size: usize,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_size: 256,
        }
    }

    pub fn push(&mut self, event: Event) {
        if self.events.len() >= self.max_size {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&Event> {
        self.events.first()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear()
    }
}
