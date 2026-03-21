extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod miniapp_stack_manager {
    use super::*;

    pub struct MiniAppStackManager {
        stack: Vec<String>,
    }

    impl MiniAppStackManager {
        pub fn new() -> Self {
            MiniAppStackManager { stack: Vec::new() }
        }

        pub fn push(&mut self, app_name: String) {
            self.stack.push(app_name);
        }

        pub fn pop(&mut self) -> Option<String> {
            self.stack.pop()
        }

        pub fn peek(&self) -> Option<&String> {
            self.stack.last()
        }

        pub fn len(&self) -> usize {
            self.stack.len()
        }

        pub fn is_empty(&self) -> bool {
            self.stack.is_empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::miniapp_stack_manager::*;

    #[test]
    fn test_miniapp_stack_manager() {
        let mut manager = MiniAppStackManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);

        manager.push(String::from("App1"));
        manager.push(String::from("App2"));
        assert!(!manager.is_empty());
        assert_eq!(manager.len(), 2);
        assert_eq!(manager.peek(), Some(&String::from("App2")));

        let popped = manager.pop();
        assert_eq!(popped, Some(String::from("App2")));
        assert_eq!(manager.len(), 1);

        let peeked = manager.peek();
        assert_eq!(peeked, Some(&String::from("App1")));

        let popped = manager.pop();
        assert_eq!(popped, Some(String::from("App1")));
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }
}
