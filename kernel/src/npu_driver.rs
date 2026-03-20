extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {}

struct NpuDriver {
    device_id: u32,
    status: String,
    tasks: Vec<String>,
}

impl NpuDriver {
    pub fn new(device_id: u32) -> Self {
        NpuDriver {
            device_id,
            status: String::from("Initialized"),
            tasks: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.status = String::from("Running");
    }

    pub fn stop(&mut self) {
        self.status = String::from("Stopped");
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(String::from(task));
    }

    pub fn remove_task(&mut self, task_index: usize) -> Option<String> {
        if task_index < self.tasks.len() {
            Some(self.tasks.remove(task_index))
        } else {
            None
        }
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[no_mangle]
pub extern "C" fn npu_driver_new(device_id: u32) -> *mut NpuDriver {
    Box::into_raw(Box::new(NpuDriver::new(device_id)))
}

#[no_mangle]
pub extern "C" fn npu_driver_start(driver: *mut NpuDriver) {
    unsafe { (*driver).start() };
}

#[no_mangle]
pub extern "C" fn npu_driver_stop(driver: *mut NpuDriver) {
    unsafe { (*driver).stop() };
}

#[no_mangle]
pub extern "C" fn npu_driver_add_task(driver: *mut NpuDriver, task: *const u8, task_len: usize) {
    let task_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(task, task_len)) };
    unsafe { (*driver).add_task(task_str) };
}

#[no_mangle]
pub extern "C" fn npu_driver_remove_task(driver: *mut NpuDriver, task_index: usize) -> *const u8 {
    match unsafe { (*driver).remove_task(task_index) } {
        Some(task) => task.as_ptr(),
        None => core::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn npu_driver_get_status(driver: *mut NpuDriver, status: *mut u8, max_len: usize) -> isize {
    let driver_status = unsafe { (*driver).get_status() };
    let len = core::cmp::min(max_len - 1, driver_status.len());
    unsafe {
        core::ptr::copy_nonoverlapping(driver_status.as_ptr(), status, len);
        *status.add(len) = 0; // Null-terminate the string
    }
    len as isize
}

#[no_mangle]
pub extern "C" fn npu_driver_free(driver: *mut NpuDriver) {
    unsafe { Box::from_raw(driver) };
}
