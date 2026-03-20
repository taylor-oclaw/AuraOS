extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentVectorIndex {
    data: Vec<String>,
}

impl AgentVectorIndex {
    pub fn new() -> Self {
        AgentVectorIndex { data: Vec::new() }
    }

    pub fn add(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn contains(&self, item: &str) -> bool {
        self.data.iter().any(|s| s == item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_vector_index() {
        let mut index = AgentVectorIndex::new();
        assert_eq!(index.len(), 0);

        index.add(String::from("agent1"));
        index.add(String::from("agent2"));
        assert_eq!(index.len(), 2);

        assert_eq!(index.get(0), Some(&String::from("agent1")));
        assert_eq!(index.get(1), Some(&String::from("agent2")));
        assert_eq!(index.get(2), None);

        assert_eq!(index.contains("agent1"), true);
        assert_eq!(index.contains("agent3"), false);

        let removed = index.remove(0);
        assert_eq!(removed, Some(String::from("agent1")));
        assert_eq!(index.len(), 1);

        assert_eq!(index.get(0), Some(&String::from("agent2")));
    }
}
