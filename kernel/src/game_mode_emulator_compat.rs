extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut emulator = GameModeEmulatorCompat::new();
    emulator.initialize_game_mode("Adventure");
    emulator.load_level(1);
    emulator.start_timer(60);
    emulator.update_score(100);
    emulator.end_game();
}

pub struct GameModeEmulatorCompat {
    game_mode: String,
    current_level: u32,
    timer: u32,
    score: u32,
    is_active: bool,
}

impl GameModeEmulatorCompat {
    pub fn new() -> Self {
        GameModeEmulatorCompat {
            game_mode: String::from("Default"),
            current_level: 0,
            timer: 0,
            score: 0,
            is_active: false,
        }
    }

    pub fn initialize_game_mode(&mut self, mode: &str) {
        self.game_mode = String::from(mode);
        self.is_active = true;
    }

    pub fn load_level(&mut self, level: u32) {
        if self.is_active {
            self.current_level = level;
        }
    }

    pub fn start_timer(&mut self, duration: u32) {
        if self.is_active {
            self.timer = duration;
        }
    }

    pub fn update_score(&mut self, points: u32) {
        if self.is_active {
            self.score += points;
        }
    }

    pub fn end_game(&mut self) {
        if self.is_active {
            self.is_active = false;
        }
    }
}
