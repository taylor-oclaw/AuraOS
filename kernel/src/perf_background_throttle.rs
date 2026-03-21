extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct PerfBackgroundThrottle {
    tasks: Vec<String>,
    max_load: u8,
    current_load: u8,
}

impl PerfBackgroundThrottle {
    pub fn new(max_load: u8) -> Self {
        PerfBackgroundThrottle {
            tasks: Vec::new(),
            max_load,
            current_load: 0,
        }
    }

    pub fn add_task(&mut self, task_name: &str) {
        if self.current_load < self.max_load {
            self.tasks.push(String::from(task_name));
            self.current_load += 1;
        }
    }

    pub fn remove_task(&mut self, task_name: &str) {
        if let Some(index) = self.tasks.iter().position(|t| t == task_name) {
            self.tasks.remove(index);
            self.current_load -= 1;
        }
    }

    pub fn get_tasks(&self) -> Vec<String> {
        self.tasks.clone()
    }

    pub fn is_throttled(&self) -> bool {
        self.current_load >= self.max_load
    }

    pub fn current_load(&self) -> u8 {
        self.current_load
    }
}

#[no_mangle]
pub extern "C" fn perf_background_throttle_init(max_load: u8) -> *mut PerfBackgroundThrottle {
    Box::into_raw(Box::new(PerfBackgroundThrottle::new(max_load)))
}

#[no_mangle]
pub extern "C" fn perf_background_throttle_add_task(throttle_ptr: *mut PerfBackgroundThrottle, task_name: &str) {
    unsafe {
        (*throttle_ptr).add_task(task_name);
    }
}

#[no_mangle]
pub extern "C" fn perf_background_throttle_remove_task(throttle_ptr: *mut PerfBackgroundThrottle, task_name: &str) {
    unsafe {
        (*throttle_ptr).remove_task(task_name);
    }
}

#[no_mangle]
pub extern "C" fn perf_background_throttle_get_tasks(throttle_ptr: *const PerfBackgroundThrottle) -> Vec<String> {
    unsafe {
        (*throttle_ptr).get_tasks()
    }
}

#[no_mangle]
pub extern "C" fn perf_background_throttle_is_throttled(throttle_ptr: *const PerfBackgroundThrottle) -> bool {
    unsafe {
        (*throttle_ptr).is_throttled()
    }
}

#[no_mangle]
pub extern "C" fn perf_background_throttle_current_load(throttle_ptr: *const PerfBackgroundThrottle) -> u8 {
    unsafe {
        (*throttle_ptr).current_load()
    }
}
