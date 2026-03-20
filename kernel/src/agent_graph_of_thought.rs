extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentGraphOfThought {
    thoughts: Vec<String>,
}

impl AgentGraphOfThought {
    pub fn new() -> Self {
        AgentGraphOfThought {
            thoughts: Vec::new(),
        }
    }

    pub fn add_thought(&mut self, thought: String) {
        self.thoughts.push(thought);
    }

    pub fn get_thought(&self, index: usize) -> Option<&String> {
        self.thoughts.get(index)
    }

    pub fn remove_thought(&mut self, index: usize) -> Option<String> {
        if index < self.thoughts.len() {
            Some(self.thoughts.remove(index))
        } else {
            None
        }
    }

    pub fn get_all_thoughts(&self) -> &Vec<String> {
        &self.thoughts
    }

    pub fn clear_thoughts(&mut self) {
        self.thoughts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_graph_of_thought() {
        let mut agent = AgentGraphOfThought::new();

        assert_eq!(agent.get_all_thoughts().len(), 0);

        agent.add_thought(String::from("First thought"));
        agent.add_thought(String::from("Second thought"));

        assert_eq!(agent.get_all_thoughts().len(), 2);
        assert_eq!(agent.get_thought(0), Some(&String::from("First thought")));
        assert_eq!(agent.get_thought(1), Some(&String::from("Second thought")));

        let removed = agent.remove_thought(0);
        assert_eq!(removed, Some(String::from("First thought")));
        assert_eq!(agent.get_all_thoughts().len(), 1);

        agent.clear_thoughts();
        assert_eq!(agent.get_all_thoughts().len(), 0);
    }
}
