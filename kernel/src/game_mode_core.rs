extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModeCore {
    game_state: String,
    players: Vec<String>,
    score: u32,
    is_active: bool,
}

impl GameModeCore {
    pub fn new() -> Self {
        GameModeCore {
            game_state: String::from("initialized"),
            players: Vec::new(),
            score: 0,
            is_active: false,
        }
    }

    pub fn start_game(&mut self) {
        if !self.is_active {
            self.game_state = String::from("active");
            self.is_active = true;
        }
    }

    pub fn end_game(&mut self) {
        if self.is_active {
            self.game_state = String::from("ended");
            self.is_active = false;
        }
    }

    pub fn add_player(&mut self, player_name: &str) {
        if !self.players.contains(&String::from(player_name)) {
            self.players.push(String::from(player_name));
        }
    }

    pub fn remove_player(&mut self, player_name: &str) {
        self.players.retain(|player| player != player_name);
    }

    pub fn update_score(&mut self, points: u32) {
        if self.is_active {
            self.score += points;
        }
    }

    pub fn get_game_state(&self) -> &String {
        &self.game_state
    }

    pub fn get_players(&self) -> &Vec<String> {
        &self.players
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn is_game_active(&self) -> bool {
        self.is_active
    }
}
