extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModeAntiCheat {
    player_ids: Vec<u32>,
    banned_ips: Vec<String>,
    suspicious_activities: Vec<(u32, String)>, // (player_id, activity_description)
}

impl GameModeAntiCheat {
    pub fn new() -> Self {
        GameModeAntiCheat {
            player_ids: Vec::new(),
            banned_ips: Vec::new(),
            suspicious_activities: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player_id: u32) {
        if !self.player_ids.contains(&player_id) {
            self.player_ids.push(player_id);
        }
    }

    pub fn ban_ip(&mut self, ip_address: &str) {
        let ip = String::from(ip_address);
        if !self.banned_ips.contains(&ip) {
            self.banned_ips.push(ip);
        }
    }

    pub fn report_suspicious_activity(&mut self, player_id: u32, activity_description: &str) {
        if self.player_ids.contains(&player_id) {
            let description = String::from(activity_description);
            self.suspicious_activities.push((player_id, description));
        }
    }

    pub fn is_ip_banned(&self, ip_address: &str) -> bool {
        self.banned_ips.contains(&String::from(ip_address))
    }

    pub fn get_suspicious_activities(&self) -> Vec<(u32, String)> {
        self.suspicious_activities.clone()
    }
}
