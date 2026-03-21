extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct NotifyGamingActiveBlock {
    active: bool,
    game_list: Vec<String>,
    user_id: u32,
    session_start_time: u64,
    last_activity_time: u64,
}

impl NotifyGamingActiveBlock {
    pub fn new(user_id: u32) -> Self {
        NotifyGamingActiveBlock {
            active: false,
            game_list: Vec::new(),
            user_id,
            session_start_time: 0,
            last_activity_time: 0,
        }
    }

    pub fn activate(&mut self, game_name: &str) {
        if !self.active {
            self.active = true;
            self.session_start_time = current_time();
        }
        self.last_activity_time = current_time();
        self.game_list.push(String::from(game_name));
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.last_activity_time = current_time();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_game_list(&self) -> &Vec<String> {
        &self.game_list
    }

    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    pub fn get_session_duration(&self) -> u64 {
        if self.active {
            current_time() - self.session_start_time
        } else {
            self.last_activity_time - self.session_start_time
        }
    }
}

fn current_time() -> u64 {
    // Placeholder for actual time retrieval logic
    1234567890
}
