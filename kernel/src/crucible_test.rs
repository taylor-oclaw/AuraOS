extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn crucible_test_init() {
    // Initialization code for the module
}

pub extern "C" fn crucible_test_exit() {
    // Cleanup code for the module
}

pub struct CrucibleTest {
    data: Vec<u8>,
    name: String,
    counter: usize,
}

impl CrucibleTest {
    pub fn new(name: &str) -> Self {
        CrucibleTest {
            data: Vec::new(),
            name: String::from(name),
            counter: 0,
        }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

pub extern "C" fn crucible_test_create(name: *const u8, name_len: usize) -> *mut CrucibleTest {
    let name_slice = unsafe { core::slice::from_raw_parts(name, name_len) };
    let name_str = String::from_utf8_lossy(name_slice).into_owned();
    Box::leak(Box::new(CrucibleTest::new(&name_str))) as *mut _
}

pub extern "C" fn crucible_test_add_data(ct: *mut CrucibleTest, byte: u8) {
    unsafe { (*ct).add_data(byte); }
}

pub extern "C" fn crucible_test_get_counter(ct: *const CrucibleTest) -> usize {
    unsafe { (*ct).get_counter() }
}

pub extern "C" fn crucible_test_increment_counter(ct: *mut CrucibleTest) {
    unsafe { (*ct).increment_counter(); }
}

pub extern "C" fn crucible_test_clear_data(ct: *mut CrucibleTest) {
    unsafe { (*ct).clear_data(); }
}
