extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct CallLogEntry {
    caller_id: String,
    timestamp: u64, // Unix timestamp in seconds
    duration: u32,  // Duration of the call in seconds
    notes: String,
}

impl CallLogEntry {
    pub fn new(caller_id: &str, timestamp: u64, duration: u32, notes: &str) -> Self {
        CallLogEntry {
            caller_id: String::from(caller_id),
            timestamp,
            duration,
            notes: String::from(notes),
        }
    }

    pub fn get_caller_id(&self) -> &str {
        &self.caller_id
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    pub fn get_notes(&self) -> &str {
        &self.notes
    }
}

struct CallLog {
    entries: Vec<CallLogEntry>,
}

impl CallLog {
    pub fn new() -> Self {
        CallLog {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: CallLogEntry) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<CallLogEntry> {
        &self.entries
    }

    pub fn find_by_caller_id(&self, caller_id: &str) -> Vec<&CallLogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.get_caller_id() == caller_id)
            .collect()
    }

    pub fn total_duration(&self) -> u32 {
        self.entries.iter().map(|entry| entry.get_duration()).sum()
    }
}
