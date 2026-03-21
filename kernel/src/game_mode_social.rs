extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModeSocial {
    players: Vec<String>,
    active_player: Option<usize>,
}

impl GameModeSocial {
    pub fn new() -> Self {
        GameModeSocial {
            players: Vec::new(),
            active_player: None,
        }
    }

    pub fn add_player(&mut self, name: &str) {
        let player_name = String::from(name);
        self.players.push(player_name);
        if self.active_player.is_none() {
            self.active_player = Some(0);
        }
    }

    pub fn remove_player(&mut self, index: usize) -> Option<String> {
        if index < self.players.len() {
            let removed_player = self.players.remove(index);
            if self.active_player == Some(index) {
                self.active_player = None;
            } else if let Some(active_index) = self.active_player {
                if active_index > index {
                    self.active_player = Some(active_index - 1);
                }
            }
            Some(removed_player)
        } else {
            None
        }
    }

    pub fn get_active_player(&self) -> Option<&String> {
        self.active_player.map(|index| &self.players[index])
    }

    pub fn set_active_player(&mut self, index: usize) -> bool {
        if index < self.players.len() {
            self.active_player = Some(index);
            true
        } else {
            false
        }
    }

    pub fn list_players(&self) -> Vec<&String> {
        self.players.iter().collect()
    }
}
