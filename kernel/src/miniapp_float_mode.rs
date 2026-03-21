extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut app = MiniAppFloatMode::new();
    app.initialize();
    app.process_data(vec![1.0, 2.0, 3.0, 4.0]);
    let result = app.calculate_average();
    app.display_result(result);
}

pub struct MiniAppFloatMode {
    data: Vec<f64>,
    total_sum: f64,
    count: usize,
}

impl MiniAppFloatMode {
    pub fn new() -> Self {
        MiniAppFloatMode {
            data: Vec::new(),
            total_sum: 0.0,
            count: 0,
        }
    }

    pub fn initialize(&mut self) {
        // Reset the internal state
        self.data.clear();
        self.total_sum = 0.0;
        self.count = 0;
    }

    pub fn process_data(&mut self, data: Vec<f64>) {
        for value in data {
            self.data.push(value);
            self.total_sum += value;
            self.count += 1;
        }
    }

    pub fn calculate_average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_sum / self.count as f64
        }
    }

    pub fn display_result(&self, result: f64) {
        // Simulate displaying the result (no actual output in kernel mode)
        // In a real scenario, this might involve interacting with hardware or other kernel components
    }

    pub fn get_data_count(&self) -> usize {
        self.count
    }
}
