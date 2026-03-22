extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub struct BizCrmSalesforceSync {
    records: Vec<String>,
}

impl BizCrmSalesforceSync {
    pub fn new() -> Self {
        BizCrmSalesforceSync { records: Vec::new() }
    }

    pub fn add_record(&mut self, record: String) {
        self.records.push(record);
    }

    pub fn remove_record(&mut self, index: usize) -> Option<String> {
        if index < self.records.len() {
            Some(self.records.remove(index))
        } else {
            None
        }
    }

    pub fn get_records(&self) -> &Vec<String> {
        &self.records
    }

    pub fn find_record(&self, query: &str) -> Option<usize> {
        for (index, record) in self.records.iter().enumerate() {
            if record.contains(query) {
                return Some(index);
            }
        }
        None
    }

    pub fn update_record(&mut self, index: usize, new_record: String) -> bool {
        if index < self.records.len() {
            self.records[index] = new_record;
            true
        } else {
            false
        }
    }
}