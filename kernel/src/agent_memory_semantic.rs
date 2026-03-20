extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentMemorySemantic {
    memory: Vec<String>,
}

impl AgentMemorySemantic {
    pub fn new() -> Self {
        AgentMemorySemantic {
            memory: Vec::new(),
        }
    }

    pub fn add_memory(&mut self, data: String) {
        self.memory.push(data);
    }

    pub fn get_memory(&self, index: usize) -> Option<&String> {
        self.memory.get(index)
    }

    pub fn remove_memory(&mut self, index: usize) -> Option<String> {
        if index < self.memory.len() {
            Some(self.memory.remove(index))
        } else {
            None
        }
    }

    pub fn clear_memory(&mut self) {
        self.memory.clear();
    }

    pub fn memory_size(&self) -> usize {
        self.memory.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_memory_semantic() {
        let mut agent = AgentMemorySemantic::new();
        assert_eq!(agent.memory_size(), 0);

        agent.add_memory(String::from("Hello"));
        agent.add_memory(String::from("World"));
        assert_eq!(agent.memory_size(), 2);

        assert_eq!(agent.get_memory(0), Some(&String::from("Hello")));
        assert_eq!(agent.get_memory(1), Some(&String::from("World")));

        let removed = agent.remove_memory(0);
        assert_eq!(removed, Some(String::from("Hello")));
        assert_eq!(agent.memory_size(), 1);

        agent.clear_memory();
        assert_eq!(agent.memory_size(), 0);
    }
}
