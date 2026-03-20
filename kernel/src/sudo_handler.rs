extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SudoHandler {
    users: Vec<String>,
    commands: Vec<String>,
}

impl SudoHandler {
    pub fn new() -> Self {
        SudoHandler {
            users: Vec::new(),
            commands: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: String) {
        if !self.users.contains(&user) {
            self.users.push(user);
        }
    }

    pub fn remove_user(&mut self, user: &str) -> bool {
        let index = self.users.iter().position(|u| u == user);
        match index {
            Some(i) => {
                self.users.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn add_command(&mut self, command: String) {
        if !self.commands.contains(&command) {
            self.commands.push(command);
        }
    }

    pub fn remove_command(&mut self, command: &str) -> bool {
        let index = self.commands.iter().position(|c| c == command);
        match index {
            Some(i) => {
                self.commands.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn can_execute(&self, user: &str, command: &str) -> bool {
        self.users.contains(&user.to_string()) && self.commands.contains(&command.to_string())
    }
}
