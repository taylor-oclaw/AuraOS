extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct NotifyCallActiveBlock {
    active_calls: Vec<String>,
}

impl NotifyCallActiveBlock {
    pub fn new() -> Self {
        NotifyCallActiveBlock {
            active_calls: Vec::new(),
        }
    }

    pub fn add_call(&mut self, call_id: &str) {
        if !self.active_calls.contains(&String::from(call_id)) {
            self.active_calls.push(String::from(call_id));
        }
    }

    pub fn remove_call(&mut self, call_id: &str) {
        if let Some(index) = self.active_calls.iter().position(|x| x == call_id) {
            self.active_calls.remove(index);
        }
    }

    pub fn is_call_active(&self, call_id: &str) -> bool {
        self.active_calls.contains(&String::from(call_id))
    }

    pub fn get_active_calls(&self) -> Vec<String> {
        self.active_calls.clone()
    }

    pub fn clear_all_calls(&mut self) {
        self.active_calls.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notify_call_active_block() {
        let mut block = NotifyCallActiveBlock::new();

        assert_eq!(block.get_active_calls().len(), 0);

        block.add_call("call1");
        assert_eq!(block.get_active_calls().len(), 1);
        assert!(block.is_call_active("call1"));

        block.add_call("call2");
        assert_eq!(block.get_active_calls().len(), 2);
        assert!(block.is_call_active("call2"));

        block.remove_call("call1");
        assert_eq!(block.get_active_calls().len(), 1);
        assert!(!block.is_call_active("call1"));
        assert!(block.is_call_active("call2"));

        block.clear_all_calls();
        assert_eq!(block.get_active_calls().len(), 0);
        assert!(!block.is_call_active("call2"));
    }
}
