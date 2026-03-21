extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AISecurityAgent {
    goals: Vec<String>,
    current_goal_index: usize,
}

impl AISecurityAgent {
    pub fn new(initial_goals: Vec<&str>) -> Self {
        let goals = initial_goals.into_iter().map(|g| g.to_string()).collect();
        AISecurityAgent {
            goals,
            current_goal_index: 0,
        }
    }

    pub fn add_goal(&mut self, goal: &str) {
        self.goals.push(goal.to_string());
    }

    pub fn remove_goal(&mut self, index: usize) -> Option<String> {
        if index < self.goals.len() {
            Some(self.goals.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_goal(&self) -> Option<&String> {
        self.goals.get(self.current_goal_index)
    }

    pub fn set_next_goal(&mut self) {
        if self.current_goal_index < self.goals.len() - 1 {
            self.current_goal_index += 1;
        }
    }

    pub fn reset_goals(&mut self) {
        self.current_goal_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_security_agent() {
        let mut agent = AISecurityAgent::new(vec!["Secure system", "Monitor network"]);
        assert_eq!(agent.get_current_goal(), Some(&String::from("Secure system")));

        agent.add_goal("Update software");
        assert_eq!(agent.goals.len(), 3);

        assert_eq!(agent.remove_goal(1), Some(String::from("Monitor network")));
        assert_eq!(agent.goals.len(), 2);

        agent.set_next_goal();
        assert_eq!(agent.get_current_goal(), Some(&String::from("Update software")));

        agent.reset_goals();
        assert_eq!(agent.get_current_goal(), Some(&String::from("Secure system")));
    }
}
