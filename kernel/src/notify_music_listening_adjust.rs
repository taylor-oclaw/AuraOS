extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyMusicListeningAdjust {
    user_id: u32,
    music_list: Vec<String>,
    current_track_index: usize,
    volume_level: u8,
    is_shuffle_enabled: bool,
}

impl NotifyMusicListeningAdjust {
    pub fn new(user_id: u32) -> Self {
        NotifyMusicListeningAdjust {
            user_id,
            music_list: Vec::new(),
            current_track_index: 0,
            volume_level: 50, // Default volume level
            is_shuffle_enabled: false,
        }
    }

    pub fn add_track(&mut self, track_name: String) {
        self.music_list.push(track_name);
    }

    pub fn remove_track(&mut self, track_index: usize) -> Option<String> {
        if track_index < self.music_list.len() {
            Some(self.music_list.remove(track_index))
        } else {
            None
        }
    }

    pub fn play_next(&mut self) {
        if !self.is_shuffle_enabled {
            self.current_track_index = (self.current_track_index + 1) % self.music_list.len();
        } else {
            // Simple shuffle logic: pick a random index different from the current one
            let mut new_index = self.current_track_index;
            while new_index == self.current_track_index {
                new_index = rand::random::<usize>() % self.music_list.len();
            }
            self.current_track_index = new_index;
        }
    }

    pub fn play_previous(&mut self) {
        if !self.is_shuffle_enabled {
            self.current_track_index = if self.current_track_index == 0 {
                self.music_list.len() - 1
            } else {
                self.current_track_index - 1
            };
        } else {
            // In shuffle mode, previous doesn't make sense, so we just play the current track again
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        if volume <= 100 {
            self.volume_level = volume;
        }
    }

    pub fn toggle_shuffle(&mut self) {
        self.is_shuffle_enabled = !self.is_shuffle_enabled;
    }

    pub fn get_current_track(&self) -> Option<&String> {
        self.music_list.get(self.current_track_index)
    }

    pub fn get_volume_level(&self) -> u8 {
        self.volume_level
    }

    pub fn is_shuffle_on(&self) -> bool {
        self.is_shuffle_enabled
    }
}
