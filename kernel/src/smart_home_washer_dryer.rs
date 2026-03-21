extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let washer_dryer = SmartHomeWasherDryer::new();
    washer_dryer.load_program("wash_and_dry");
    washer_dryer.start_cycle();
    washer_dryer.check_status();
    washer_dryer.stop_cycle();
    washer_dryer.unload_program();

    loop {}
}

pub struct SmartHomeWasherDryer {
    program: Option<String>,
    cycle_active: bool,
    status: String,
}

impl SmartHomeWasherDryer {
    pub fn new() -> Self {
        SmartHomeWasherDryer {
            program: None,
            cycle_active: false,
            status: String::from("Idle"),
        }
    }

    pub fn load_program(&mut self, program_name: &str) {
        if !self.cycle_active {
            self.program = Some(program_name.to_string());
            self.status = String::from("info");
        } else {
            self.status = String::from("Cycle is active. Cannot load new program.");
        }
    }

    pub fn start_cycle(&mut self) {
        if let Some(ref program) = self.program {
            self.cycle_active = true;
            self.status = String::from("info");
        } else {
            self.status = String::from("No program loaded. Cannot start cycle.");
        }
    }

    pub fn check_status(&self) -> &str {
        &self.status
    }

    pub fn stop_cycle(&mut self) {
        if self.cycle_active {
            self.cycle_active = false;
            self.status = String::from("Cycle stopped");
        } else {
            self.status = String::from("No active cycle to stop.");
        }
    }

    pub fn unload_program(&mut self) {
        if !self.cycle_active {
            self.program = None;
            self.status = String::from("Program unloaded");
        } else {
            self.status = String::from("Cycle is active. Cannot unload program.");
        }
    }
}
