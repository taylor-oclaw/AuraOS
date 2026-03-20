extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug)]
pub struct AgentAccessControl {
    agents: Vec<String>,
    permissions: Vec<Vec<bool>>,
}

impl AgentAccessControl {
    pub fn new() -> Self {
        AgentAccessControl {
            agents: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent_name: &str) -> Result<(), String> {
        if self.agents.contains(&agent_name.to_string()) {
            Err(String::from("Agent already exists"))
        } else {
            self.agents.push(agent_name.to_string());
            self.permissions.push(vec![false; self.agents.len()]);
            Ok(())
        }
    }

    pub fn remove_agent(&mut self, agent_name: &str) -> Result<(), String> {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            self.agents.remove(index);
            for perm in self.permissions.iter_mut() {
                perm.remove(index);
            }
            Ok(())
        } else {
            Err(String::from("Agent not found"))
        }
    }

    pub fn grant_permission(&mut self, agent_name: &str, resource_index: usize) -> Result<(), String> {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            if resource_index < self.permissions.len() {
                self.permissions[index][resource_index] = true;
                Ok(())
            } else {
                Err(String::from("Resource index out of bounds"))
            }
        } else {
            Err(String::from("Agent not found"))
        }
    }

    pub fn revoke_permission(&mut self, agent_name: &str, resource_index: usize) -> Result<(), String> {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            if resource_index < self.permissions.len() {
                self.permissions[index][resource_index] = false;
                Ok(())
            } else {
                Err(String::from("Resource index out of bounds"))
            }
        } else {
            Err(String::from("Agent not found"))
        }
    }

    pub fn check_permission(&self, agent_name: &str, resource_index: usize) -> Result<bool, String> {
        if let Some(index) = self.agents.iter().position(|a| a == agent_name) {
            if resource_index < self.permissions.len() {
                Ok(self.permissions[index][resource_index])
            } else {
                Err(String::from("Resource index out of bounds"))
            }
        } else {
            Err(String::from("Agent not found"))
        }
    }
}
