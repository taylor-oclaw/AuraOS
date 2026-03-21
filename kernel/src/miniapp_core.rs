extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let mut app = MiniAppCore::new();
    app.initialize();
    loop {
        app.run();
    }
}

pub struct MiniAppCore {
    name: String,
    version: u32,
    features: Vec<String>,
    is_active: bool,
    log_buffer: Vec<String>,
}

impl MiniAppCore {
    pub fn new() -> Self {
        MiniAppCore {
            name: String::from("MiniAppCore"),
            version: 1,
            features: vec![String::from("Logging"), String::from("Initialization")],
            is_active: false,
            log_buffer: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.is_active = true;
        self.log("Application initialized");
    }

    pub fn run(&self) {
        if self.is_active {
            self.log("Application running");
        } else {
            self.log("Application is not active");
        }
    }

    pub fn log(&mut self, message: &str) {
        let log_entry = format!("{}: {}", self.name, message);
        self.log_buffer.push(log_entry);
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.log_buffer.clone()
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.log("Application deactivated");
    }
}
