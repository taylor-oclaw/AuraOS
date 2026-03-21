extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct ContextRunningDetect {
    contexts: Vec<String>,
    current_context: Option<usize>,
}

impl ContextRunningDetect {
    pub fn new() -> Self {
        ContextRunningDetect {
            contexts: Vec::new(),
            current_context: None,
        }
    }

    pub fn add_context(&mut self, context_name: &str) {
        let name = String::from(context_name);
        self.contexts.push(name);
    }

    pub fn remove_context(&mut self, context_name: &str) -> bool {
        if let Some(index) = self.contexts.iter().position(|c| c == context_name) {
            self.contexts.remove(index);
            true
        } else {
            false
        }
    }

    pub fn switch_context(&mut self, context_name: &str) -> bool {
        if let Some(index) = self.contexts.iter().position(|c| c == context_name) {
            self.current_context = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_context(&self) -> Option<&String> {
        self.current_context.map(|index| &self.contexts[index])
    }

    pub fn list_all_contexts(&self) -> Vec<&String> {
        self.contexts.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_running_detect() {
        let mut detector = ContextRunningDetect::new();
        assert_eq!(detector.list_all_contexts(), vec![]);
        assert_eq!(detector.get_current_context(), None);

        detector.add_context("context1");
        detector.add_context("context2");
        assert_eq!(detector.list_all_contexts(), vec!["context1", "context2"]);
        assert_eq!(detector.get_current_context(), None);

        assert!(detector.switch_context("context1"));
        assert_eq!(detector.get_current_context(), Some(&String::from("context1")));

        assert!(detector.remove_context("context1"));
        assert_eq!(detector.list_all_contexts(), vec!["context2"]);
        assert_eq!(detector.get_current_context(), None);

        assert!(!detector.switch_context("context3"));
        assert_eq!(detector.get_current_context(), None);
    }
}
