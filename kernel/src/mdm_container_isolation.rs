extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn mdm_container_isolation_init() {
    // Initialization logic for the module
}

pub extern "C" fn mdm_container_isolation_exit() {
    // Cleanup logic for the module
}

pub struct ContainerIsolation {
    containers: Vec<String>,
}

impl ContainerIsolation {
    pub fn new() -> Self {
        ContainerIsolation {
            containers: Vec::new(),
        }
    }

    pub fn add_container(&mut self, name: &str) {
        let container_name = String::from(name);
        if !self.containers.contains(&container_name) {
            self.containers.push(container_name);
        }
    }

    pub fn remove_container(&mut self, name: &str) -> bool {
        let container_name = String::from(name);
        match self.containers.iter().position(|x| *x == container_name) {
            Some(index) => {
                self.containers.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn list_containers(&self) -> Vec<String> {
        self.containers.clone()
    }

    pub fn is_container_present(&self, name: &str) -> bool {
        let container_name = String::from(name);
        self.containers.contains(&container_name)
    }

    pub fn clear_containers(&mut self) {
        self.containers.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_isolation() {
        let mut isolation = ContainerIsolation::new();

        assert_eq!(isolation.list_containers(), Vec::<String>::new());

        isolation.add_container("container1");
        assert!(isolation.is_container_present("container1"));
        assert_eq!(isolation.list_containers(), vec![String::from("container1")]);

        isolation.add_container("container2");
        assert!(isolation.is_container_present("container2"));
        assert_eq!(
            isolation.list_containers(),
            vec![String::from("container1"), String::from("container2")]
        ;

        assert!(isolation.remove_container("container1"));
        assert!(!isolation.is_container_present("container1"));
        assert_eq!(isolation.list_containers(), vec![String::from("container2")]);

        isolation.clear_containers();
        assert_eq!(isolation.list_containers(), Vec::<String>::new());
    }
)}
