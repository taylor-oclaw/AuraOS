extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod namespace_isolate {
    use super::*;

    pub struct NamespaceIsolator {
        namespaces: Vec<String>,
    }

    impl NamespaceIsolator {
        pub fn new() -> Self {
            NamespaceIsolator {
                namespaces: Vec::new(),
            }
        }

        pub fn add_namespace(&mut self, name: &str) {
            if !self.namespaces.contains(&String::from(name)) {
                self.namespaces.push(String::from(name));
            }
        }

        pub fn remove_namespace(&mut self, name: &str) {
            self.namespaces.retain(|ns| ns != name);
        }

        pub fn list_namespaces(&self) -> Vec<String> {
            self.namespaces.clone()
        }

        pub fn namespace_exists(&self, name: &str) -> bool {
            self.namespaces.contains(&String::from(name))
        }

        pub fn clear_namespaces(&mut self) {
            self.namespaces.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_isolator() {
        let mut isolator = NamespaceIsolator::new();

        // Test adding namespaces
        isolator.add_namespace("user1");
        isolator.add_namespace("user2");
        assert_eq!(isolator.list_namespaces(), vec!["user1".to_string(), "user2".to_string()]);

        // Test removing a namespace
        isolator.remove_namespace("user1");
        assert_eq!(isolator.list_namespaces(), vec!["user2".to_string()]);

        // Test checking if a namespace exists
        assert!(isolator.namespace_exists("user2"));
        assert!(!isolator.namespace_exists("user3"));

        // Test clearing all namespaces
        isolator.clear_namespaces();
        assert_eq!(isolator.list_namespaces(), Vec::<String>::new());
    }
}
