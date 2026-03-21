extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut music_control = MiniAppMusicControl::new();
    music_control.play_song("Song1");
    music_control.pause_song();
    music_control.next_song();
    music_control.previous_song();
    music_control.set_volume(50);
    loop {}
}

pub struct MiniAppMusicControl {
    current_song: Option<String>,
    playlist: Vec<String>,
    volume: u8,
    is_playing: bool,
}

impl MiniAppMusicControl {
    pub fn new() -> Self {
        MiniAppMusicControl {
            current_song: None,
            playlist: vec![
                String::from("Song1"),
                String::from("Song2"),
                String::from("Song3"),
            ],
            volume: 50,
            is_playing: false,
        }
    }

    pub fn play_song(&mut self, song_name: &str) {
        if let Some(index) = self.playlist.iter().position(|s| s == song_name) {
            self.current_song = Some(self.playlist[index].clone());
            self.is_playing = true;
            // Simulate playing the song
            println!("Playing {}", song_name);
        } else {
            println!("Song not found in playlist");
        }
    }

    pub fn pause_song(&mut self) {
        if self.is_playing {
            self.is_playing = false;
            println!("Paused current song");
        } else {
            println!("No song is playing to pause");
        }
    }

    pub fn next_song(&mut self) {
        if let Some(current_index) = self.current_song.as_ref().and_then(|song| {
            self.playlist.iter().position(|s| s == song)
        }) {
            let next_index = (current_index + 1) % self.playlist.len();
            self.current_song = Some(self.playlist[next_index].clone());
            if self.is_playing {
                println!("Playing next song: {}", self.current_song.as_ref().unwrap());
            } else {
                println!("Next song set to: {}", self.current_song.as_ref().unwrap());
            }
        } else {
            println!("No current song");
        }
    }

    pub fn previous_song(&mut self) {
        if let Some(current_index) = self.current_song.as_ref().and_then(|song| {
            self.playlist.iter().position(|s| s == song)
        }) {
            let prev_index = if current_index == 0 {
                self.playlist.len() - 1
            } else {
                current_index - 1
            };
            self.current_song = Some(self.playlist[prev_index].clone());
            if self.is_playing {
                println!("Playing previous song: {}", self.current_song.as_ref().unwrap());
            } else {
                println!("Previous song set to: {}", self.current_song.as_ref().unwrap());
            }
        } else {
            println!("No current song");
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        if volume <= 100 {
            self.volume = volume;
            println!("Volume set to {}", volume);
        } else {
            println!("Invalid volume level. Please use a value between 0 and 100.");
        }
    }
}
