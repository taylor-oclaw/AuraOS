extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    let mut context = ContextAloneDetect::new();
    context.initialize();
    loop {
        if context.is_alone() {
            context.handle_alone_state();
        } else {
            context.handle_non_alone_state();
        }
        context.update_status();
    }
}

pub struct ContextAloneDetect {
    status: String,
    history: Vec<String>,
    alone_threshold: usize,
}

impl ContextAloneDetect {
    pub fn new() -> Self {
        ContextAloneDetect {
            status: String::from("unknown"),
            history: Vec::new(),
            alone_threshold: 3,
        }
    }

    pub fn initialize(&mut self) {
        self.status = String::from("initialized");
        self.history.push(self.status.clone());
    }

    pub fn is_alone(&self) -> bool {
        self.history.len() >= self.alone_threshold
    }

    pub fn handle_alone_state(&mut self) {
        self.status = String::from("alone");
        self.history.push(self.status.clone());
    }

    pub fn handle_non_alone_state(&mut self) {
        self.status = String::from("non-alone");
        self.history.push(self.status.clone());
    }

    pub fn update_status(&mut self) {
        // Simulate status update
        if self.is_alone() {
            self.status = String::from("still alone");
        } else {
            self.status = String::from("still non-alone");
        }
        self.history.push(self.status.clone());
    }
}
