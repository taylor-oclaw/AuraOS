extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraScreenRecordMgr {
    records: Vec<String>,
    max_records: usize,
}

impl AuraScreenRecordMgr {
    pub fn new(max_records: usize) -> Self {
        AuraScreenRecordMgr {
            records: Vec::new(),
            max_records,
        }
    }

    pub fn add_record(&mut self, record: String) {
        if self.records.len() >= self.max_records {
            self.records.remove(0);
        }
        self.records.push(record);
    }

    pub fn get_records(&self) -> &Vec<String> {
        &self.records
    }

    pub fn clear_records(&mut self) {
        self.records.clear();
    }

    pub fn has_record(&self, record: &str) -> bool {
        self.records.iter().any(|r| r == record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_record() {
        let mut mgr = AuraScreenRecordMgr::new(3);
        mgr.add_record(String::from("record1"));
        assert_eq!(mgr.get_records().len(), 1);
    }

    #[test]
    fn test_max_records() {
        let mut mgr = AuraScreenRecordMgr::new(2);
        mgr.add_record(String::from("record1"));
        mgr.add_record(String::from("record2"));
        mgr.add_record(String::from("record3"));
        assert_eq!(mgr.get_records().len(), 2);
    }

    #[test]
    fn test_clear_records() {
        let mut mgr = AuraScreenRecordMgr::new(3);
        mgr.add_record(String::from("record1"));
        mgr.clear_records();
        assert_eq!(mgr.get_records().len(), 0);
    }

    #[test]
    fn test_has_record() {
        let mut mgr = AuraScreenRecordMgr::new(3);
        mgr.add_record(String::from("record1"));
        assert!(mgr.has_record("record1"));
        assert!(!mgr.has_record("record2"));
    }
}
