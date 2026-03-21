extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct NotifyVideoWatchingDetect {
    video_list: Vec<String>,
    active_video: Option<String>,
    watch_time: u32,
    threshold: u32,
}

impl NotifyVideoWatchingDetect {
    pub fn new(threshold: u32) -> Self {
        NotifyVideoWatchingDetect {
            video_list: Vec::new(),
            active_video: None,
            watch_time: 0,
            threshold,
        }
    }

    pub fn add_video(&mut self, video_name: String) {
        if !self.video_list.contains(&video_name) {
            self.video_list.push(video_name);
        }
    }

    pub fn remove_video(&mut self, video_name: &str) {
        self.video_list.retain(|v| v != video_name);
        if let Some(active) = &self.active_video {
            if active == video_name {
                self.active_video = None;
                self.watch_time = 0;
            }
        }
    }

    pub fn start_watching(&mut self, video_name: String) {
        if self.video_list.contains(&video_name) {
            self.active_video = Some(video_name);
            self.watch_time = 0;
        }
    }

    pub fn stop_watching(&mut self) {
        self.active_video = None;
        self.watch_time = 0;
    }

    pub fn update_watch_time(&mut self, time: u32) -> bool {
        if let Some(_) = &self.active_video {
            self.watch_time += time;
            if self.watch_time >= self.threshold {
                return true;
            }
        }
        false
    }
}
