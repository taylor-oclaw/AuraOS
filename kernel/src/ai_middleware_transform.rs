extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AIKernelModule {
    data: Vec<u8>,
    name: String,
    version: u32,
    active: bool,
}

impl AIKernelModule {
    pub fn new(name: &str, version: u32) -> Self {
        AIKernelModule {
            data: Vec::new(),
            name: String::from(name),
            version,
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}

pub extern "C" fn ai_middleware_transform_init() -> *const AIKernelModule {
    let module = AIKernelModule::new("AI Middleware Transform", 1);
    Box::leak(Box::new(module))
}

pub extern "C" fn ai_middleware_transform_activate(module: *mut AIKernelModule) {
    unsafe { (*module).activate() };
}

pub extern "C" fn ai_middleware_transform_deactivate(module: *mut AIKernelModule) {
    unsafe { (*module).deactivate() };
}

pub extern "C" fn ai_middleware_transform_is_active(module: *const AIKernelModule) -> bool {
    unsafe { (*module).is_active() }
}

pub extern "C" fn ai_middleware_transform_set_data(module: *mut AIKernelModule, data: Vec<u8>) {
    unsafe { (*module).set_data(data) };
}

pub extern "C" fn ai_middleware_transform_get_data(module: *const AIKernelModule) -> *const Vec<u8> {
    unsafe { (*module).get_data() }
}
