extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModePartySystem {
    players: Vec<String>,
    max_players: usize,
}

impl GameModePartySystem {
    pub fn new(max_players: usize) -> Self {
        GameModePartySystem {
            players: Vec::new(),
            max_players,
        }
    }

    pub fn add_player(&mut self, player_name: String) -> Result<(), &'static str> {
        if self.players.len() >= self.max_players {
            Err("Maximum number of players reached")
        } else if self.players.contains(&player_name) {
            Err("Player already in the party")
        } else {
            self.players.push(player_name);
            Ok(())
        }
    }

    pub fn remove_player(&mut self, player_name: &str) -> Result<(), &'static str> {
        if let Some(index) = self.players.iter().position(|p| p == player_name) {
            self.players.remove(index);
            Ok(())
        } else {
            Err("Player not found in the party")
        }
    }

    pub fn list_players(&self) -> Vec<String> {
        self.players.clone()
    }

    pub fn is_full(&self) -> bool {
        self.players.len() >= self.max_players
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}
