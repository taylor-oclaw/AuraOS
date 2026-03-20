extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn agent_self_critique_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn agent_self_critique_exit() {
    // Cleanup logic for the module
}

pub struct AgentSelfCritique {
    critiques: Vec<String>,
}

impl AgentSelfCritique {
    pub fn new() -> Self {
        AgentSelfCritique {
            critiques: Vec::new(),
        }
    }

    pub fn add_critique(&mut self, critique: String) {
        self.critiques.push(critique);
    }

    pub fn get_all_critiques(&self) -> &Vec<String> {
        &self.critiques
    }

    pub fn remove_critique(&mut self, index: usize) -> Option<String> {
        if index < self.critiques.len() {
            Some(self.critiques.remove(index))
        } else {
            None
        }
    }

    pub fn clear_all_critiques(&mut self) {
        self.critiques.clear();
    }

    pub fn count_critiques(&self) -> usize {
        self.critiques.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_self_critique() {
        let mut critique = AgentSelfCritique::new();
        assert_eq!(critique.count_critiques(), 0);

        critique.add_critique(String::from("Good performance"));
        critique.add_critique(String::from("Needs improvement in security"));

        assert_eq!(critique.count_critiques(), 2);
        assert_eq!(critique.get_all_critiques()[0], "Good performance");
        assert_eq!(critique.get_all_critiques()[1], "Needs improvement in security");

        let removed = critique.remove_critique(0);
        assert_eq!(removed, Some(String::from("Good performance")));
        assert_eq!(critique.count_critiques(), 1);

        critique.clear_all_critiques();
        assert_eq!(critique.count_critiques(), 0);
    }
}
