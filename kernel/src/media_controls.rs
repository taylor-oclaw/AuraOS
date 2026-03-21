extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum PlayState {
    Stopped,
    Playing,
    Paused,
    Buffering,
}

pub struct NowPlaying {
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_ms: u64,
    pub position_ms: u64,
    pub state: PlayState,
}

pub struct MediaControls {
    pub now_playing: Option<NowPlaying>,
    pub volume: u8,
    pub muted: bool,
    pub repeat_mode: bool,
    pub shuffle: bool,
    pub queue: Vec<String>,
}

impl MediaControls {
    pub fn new() -> Self {
        Self {
            now_playing: None,
            volume: 75,
            muted: false,
            repeat_mode: false,
            shuffle: false,
            queue: Vec::new(),
        }
    }

    pub fn play(&mut self, title: &str, duration_ms: u64) {
        self.now_playing = Some(NowPlaying {
            title: String::from(title),
            artist: None,
            album: None,
            duration_ms,
            position_ms: 0,
            state: PlayState::Playing,
        });
    }

    pub fn pause(&mut self) {
        if let Some(np) = &mut self.now_playing {
            np.state = PlayState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let Some(np) = &mut self.now_playing {
            np.state = PlayState::Playing;
        }
    }

    pub fn stop(&mut self) {
        self.now_playing = None;
    }

    pub fn set_volume(&mut self, vol: u8) {
        self.volume = if vol > 100 { 100 } else { vol };
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn add_to_queue(&mut self, title: &str) {
        self.queue.push(String::from(title));
    }

    pub fn next_in_queue(&mut self) -> Option<String> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    pub fn is_playing(&self) -> bool {
        self.now_playing.as_ref().map(|np| matches!(np.state, PlayState::Playing)).unwrap_or(false)
    }
}
