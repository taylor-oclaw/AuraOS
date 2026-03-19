extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum OsEvent {
    KeyPress(u8),
    KeyRelease(u8),
    MouseMove(i32, i32),
    MouseClick(u8, i32, i32),
    Timer(u64),
    AgentMessage { from: u64, to: u64, payload: String },
    SurfaceCreate(u64),
    SurfaceFocus(u64),
    VoiceCommand(String),
    Shutdown,
    Resize(u32, u32)
}

pub struct EventHandler {
    pub name: String,
    pub priority: u8,
    pub handles: Vec<String>
}

pub struct EventLoop {
    pub queue: Vec<OsEvent>,
    pub handlers: Vec<EventHandler>,
    pub running: bool,
    pub tick: u64
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            handlers: Vec::new(),
            running: false,
            tick: 0
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn push_event(&mut self, event: OsEvent) {
        self.queue.push(event);
    }

    pub fn poll(&mut self) -> Option<OsEvent> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    pub fn process_all(&mut self) -> u64 {
        self.tick += 1;
        let count = self.queue.len() as u64;
        self.queue.clear();
        count
    }

    pub fn register_handler(&mut self, name: &str, priority: u8) {
        self.handlers.push(EventHandler {
            name: String::from(name),
            priority,
            handles: Vec::new()
        });
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}
