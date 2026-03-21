extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct GameMode {
    name: String,
    players: Vec<String>,
    max_players: usize,
    is_active: bool,
}

impl GameMode {
    pub fn new(name: &str, max_players: usize) -> Self {
        GameMode {
            name: String::from(name),
            players: Vec::new(),
            max_players,
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        if !self.is_active && self.players.len() > 0 {
            self.is_active = true;
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn add_player(&mut self, player_name: &str) -> bool {
        if self.players.len() < self.max_players {
            self.players.push(String::from(player_name));
            true
        } else {
            false
        }
    }

    pub fn remove_player(&mut self, player_name: &str) -> bool {
        let pos = self.players.iter().position(|x| x == player_name);
        if let Some(index) = pos {
            self.players.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_players(&self) -> Vec<String> {
        self.players.clone()
    }
}
