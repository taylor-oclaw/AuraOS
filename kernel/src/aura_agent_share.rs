extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

struct AuraAgentShare {
    data: Vec<u8>,
    name: String,
}

impl AuraAgentShare {
    pub fn new(name: &str) -> Self {
        AuraAgentShare {
            data: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_data(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_agent_share() {
        let mut agent = AuraAgentShare::new("TestAgent");
        assert_eq!(agent.get_name(), "TestAgent");

        agent.add_data(42);
        assert_eq!(agent.get_data(), &[42]);

        agent.clear_data();
        assert!(agent.get_data().is_empty());

        agent.set_name("NewName");
        assert_eq!(agent.get_name(), "NewName");
    }
}
