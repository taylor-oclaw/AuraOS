extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PageFaultHandler {
    fault_count: usize,
    fault_log: Vec<String>,
}

impl PageFaultHandler {
    pub fn new() -> Self {
        PageFaultHandler {
            fault_count: 0,
            fault_log: Vec::new(),
        }
    }

    pub fn handle_page_fault(&mut self, address: usize) {
        self.fault_count += 1;
        let log_entry = format!("Page fault at address: {:#x}", address);
        self.fault_log.push(log_entry);
    }

    pub fn get_fault_count(&self) -> usize {
        self.fault_count
    }

    pub fn get_fault_log(&self) -> &Vec<String> {
        &self.fault_log
    }

    pub fn clear_fault_log(&mut self) {
        self.fault_log.clear();
    }

    pub fn analyze_faults(&self) -> String {
        if self.fault_count == 0 {
            return String::from("No page faults recorded.");
        }
        let mut analysis = format!("Total page faults: {}\n", self.fault_count);
        for (index, log_entry) in self.fault_log.iter().enumerate() {
            analysis.push_str(&format!("Fault {}: {}\n", index + 1, log_entry));
        }
        analysis
    }
}
