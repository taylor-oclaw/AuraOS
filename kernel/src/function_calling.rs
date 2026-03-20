extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    let mut module = FunctionCallingModule::new();
    module.initialize();
    loop {}
}

pub struct FunctionCallingModule {
    data: Vec<String>,
    counter: usize,
}

impl FunctionCallingModule {
    pub fn new() -> Self {
        FunctionCallingModule {
            data: Vec::new(),
            counter: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.data.push(String::from("Initialized"));
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }
}
