extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_qos_guarantees_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_qos_guarantees_exit() {
    // Cleanup logic for the module
}

pub struct MeshQoSGuarantees {
    guarantees: Vec<String>,
}

impl MeshQoSGuarantees {
    pub fn new() -> Self {
        MeshQoSGuarantees {
            guarantees: Vec::new(),
        }
    }

    pub fn add_guarantee(&mut self, guarantee: String) {
        self.guarantees.push(guarantee);
    }

    pub fn remove_guarantee(&mut self, index: usize) -> Option<String> {
        if index < self.guarantees.len() {
            Some(self.guarantees.remove(index))
        } else {
            None
        }
    }

    pub fn get_guarantee(&self, index: usize) -> Option<&String> {
        self.guarantees.get(index)
    }

    pub fn list_guarantees(&self) -> &[String] {
        &self.guarantees
    }

    pub fn clear_guarantees(&mut self) {
        self.guarantees.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_qos_guarantees() {
        let mut qos = MeshQoSGuarantees::new();

        assert_eq!(qos.list_guarantees().len(), 0);

        qos.add_guarantee(String::from("Low latency"));
        qos.add_guarantee(String::from("High bandwidth"));

        assert_eq!(qos.list_guarantees().len(), 2);
        assert_eq!(qos.get_guarantee(0), Some(&String::from("Low latency")));
        assert_eq!(qos.get_guarantee(1), Some(&String::from("High bandwidth")));

        let removed = qos.remove_guarantee(0);
        assert_eq!(removed, Some(String::from("Low latency")));
        assert_eq!(qos.list_guarantees().len(), 1);

        qos.clear_guarantees();
        assert_eq!(qos.list_guarantees().len(), 0);
    }
}
