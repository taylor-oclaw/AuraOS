extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut game_mode = GameMode::new();
    game_mode.activate("Adventure");
    game_mode.set_difficulty(3);
    game_mode.add_player(String::from("Alice"));
    game_mode.add_player(String::from("Bob"));
    game_mode.start_game();
}

pub struct GameMode {
    mode: String,
    difficulty: u8,
    players: Vec<String>,
    is_active: bool,
}

impl GameMode {
    pub fn new() -> Self {
        GameMode {
            mode: String::new(),
            difficulty: 1,
            players: Vec::new(),
            is_active: false,
        }
    }

    pub fn activate(&mut self, mode: &str) {
        self.mode = String::from(mode);
        self.is_active = true;
    }

    pub fn set_difficulty(&mut self, level: u8) {
        if level > 0 && level <= 5 {
            self.difficulty = level;
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.push(name);
    }

    pub fn start_game(&self) {
        // Logic to start the game
        // For example, initialize game state, spawn players, etc.
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
