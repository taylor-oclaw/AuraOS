extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AuraAgentInstaller {
    installed_agents: Vec<String>,
}

impl AuraAgentInstaller {
    pub fn new() -> Self {
        AuraAgentInstaller {
            installed_agents: Vec::new(),
        }
    }

    pub fn install_agent(&mut self, agent_name: &str) -> bool {
        if !self.is_agent_installed(agent_name) {
            self.installed_agents.push(String::from(agent_name));
            true
        } else {
            false
        }
    }

    pub fn uninstall_agent(&mut self, agent_name: &str) -> bool {
        let pos = self.installed_agents.iter().position(|x| x == agent_name);
        if let Some(index) = pos {
            self.installed_agents.remove(index);
            true
        } else {
            false
        }
    }

    pub fn is_agent_installed(&self, agent_name: &str) -> bool {
        self.installed_agents.contains(&String::from(agent_name))
    }

    pub fn list_installed_agents(&self) -> Vec<String> {
        self.installed_agents.clone()
    }

    pub fn count_installed_agents(&self) -> usize {
        self.installed_agents.len()
    }
}
