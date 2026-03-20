extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    println!("Agent Memory Episodic Module Loaded");
    0
}

pub struct Episode {
    id: u32,
    events: Vec<String>,
}

impl Episode {
    pub fn new(id: u32) -> Self {
        Episode {
            id,
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> &Vec<String> {
        &self.events
    }

    pub fn remove_event(&mut self, index: usize) -> Option<String> {
        if index < self.events.len() {
            Some(self.events.remove(index))
        } else {
            None
        }
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

pub struct AgentMemoryEpisodic {
    episodes: Vec<Episode>,
}

impl AgentMemoryEpisodic {
    pub fn new() -> Self {
        AgentMemoryEpisodic {
            episodes: Vec::new(),
        }
    }

    pub fn add_episode(&mut self, episode: Episode) {
        self.episodes.push(episode);
    }

    pub fn get_episodes(&self) -> &Vec<Episode> {
        &self.episodes
    }

    pub fn remove_episode(&mut self, index: usize) -> Option<Episode> {
        if index < self.episodes.len() {
            Some(self.episodes.remove(index))
        } else {
            None
        }
    }

    pub fn clear_episodes(&mut self) {
        self.episodes.clear();
    }

    pub fn find_episode_by_id(&self, id: u32) -> Option<&Episode> {
        self.episodes.iter().find(|episode| episode.id == id)
    }
}
