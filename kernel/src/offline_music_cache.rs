extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineMusicCache {
    cache: Vec<(String, String)>, // (song_name, file_path)
}

impl OfflineMusicCache {
    pub fn new() -> Self {
        OfflineMusicCache {
            cache: Vec::new(),
        }
    }

    pub fn add_song(&mut self, song_name: &str, file_path: &str) {
        let entry = (String::from(song_name), String::from(file_path));
        self.cache.push(entry);
    }

    pub fn remove_song(&mut self, song_name: &str) -> bool {
        let pos = self.cache.iter().position(|(name, _)| name == song_name);
        if let Some(index) = pos {
            self.cache.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_song_path(&self, song_name: &str) -> Option<&String> {
        self.cache.iter().find_map(|(name, path)| {
            if name == song_name {
                Some(path)
            } else {
                None
            }
        })
    }

    pub fn list_all_songs(&self) -> Vec<&String> {
        self.cache.iter().map(|(name, _)| name).collect()
    }

    pub fn is_song_cached(&self, song_name: &str) -> bool {
        self.cache.iter().any(|(name, _)| name == song_name)
    }
}
