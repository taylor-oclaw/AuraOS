extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let controller = GameModeControllerXbox::new();
    controller.initialize();
    controller.set_game_mode("Survival");
    controller.enable_cheats(true);
    controller.load_map("DesertIsland");
    controller.start_game();
}

pub struct GameModeControllerXbox {
    game_mode: String,
    cheats_enabled: bool,
    current_map: String,
    players: Vec<String>,
}

impl GameModeControllerXbox {
    pub fn new() -> Self {
        GameModeControllerXbox {
            game_mode: String::from("Creative"),
            cheats_enabled: false,
            current_map: String::from("DefaultMap"),
            players: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        // Initialize the controller with default settings
    }

    pub fn set_game_mode(&mut self, mode: &str) {
        self.game_mode = String::from(mode);
    }

    pub fn enable_cheats(&mut self, enabled: bool) {
        self.cheats_enabled = enabled;
        if self.cheats_enabled {
        } else {
        }
    }

    pub fn load_map(&mut self, map_name: &str) {
        self.current_map = String::from(map_name);
    }

    pub fn start_game(&self) {
            "Starting game with mode {} on map {}.",
            self.game_mode, self.current_map
        ;
    }
}
