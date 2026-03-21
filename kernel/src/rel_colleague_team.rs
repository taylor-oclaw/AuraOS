extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let team = RelColleagueTeam::new();
    team.add_colleague("Alice");
    team.add_colleague("Bob");
    team.add_colleague("Charlie");

    if team.is_member("Bob") {
    }

    let members = team.get_members();
    for member in members.iter() {
    }

    team.remove_colleague("Alice");
    let remaining_members = team.get_members();
    for member in remaining_members.iter() {
    }

    loop {}
}

pub struct RelColleagueTeam {
    members: Vec<String>,
}

impl RelColleagueTeam {
    pub fn new() -> Self {
        RelColleagueTeam {
            members: Vec::new(),
        }
    }

    pub fn add_colleague(&mut self, name: &str) {
        if !self.is_member(name) {
            self.members.push(String::from(name));
        }
    }

    pub fn remove_colleague(&mut self, name: &str) {
        self.members.retain(|member| member != name);
    }

    pub fn is_member(&self, name: &str) -> bool {
        self.members.iter().any(|member| member == name)
    }

    pub fn get_members(&self) -> Vec<String> {
        self.members.clone()
    }

    pub fn team_size(&self) -> usize {
        self.members.len()
    }
}
