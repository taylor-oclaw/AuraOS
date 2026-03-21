extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct MeshTeamPool {
    teams: Vec<String>,
}

impl MeshTeamPool {
    pub fn new() -> Self {
        MeshTeamPool {
            teams: Vec::new(),
        }
    }

    pub fn add_team(&mut self, team_name: &str) {
        if !self.teams.contains(&String::from(team_name)) {
            self.teams.push(String::from(team_name));
        }
    }

    pub fn remove_team(&mut self, team_name: &str) {
        self.teams.retain(|team| team != team_name);
    }

    pub fn get_teams(&self) -> Vec<String> {
        self.teams.clone()
    }

    pub fn has_team(&self, team_name: &str) -> bool {
        self.teams.contains(&String::from(team_name))
    }

    pub fn team_count(&self) -> usize {
        self.teams.len()
    }
}
