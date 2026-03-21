extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum CaptureRegion {
    FullScreen,
    Window(u64),
    Custom { x: u32, y: u32, w: u32, h: u32 },
}

pub struct Screenshot {
    pub id: u64,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,
    pub region: CaptureRegion,
}

pub struct Recording {
    pub id: u64,
    pub frames: Vec<Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub fps: u8,
    pub started_at: u64,
    pub active: bool,
}

pub struct ScreenCapture {
    pub screenshots: Vec<Screenshot>,
    pub active_recording: Option<Recording>,
    pub next_id: u64,
    pub save_path: String,
}

impl ScreenCapture {
    pub fn new() -> Self {
        Self {
            screenshots: Vec::new(),
            active_recording: None,
            next_id: 1,
            save_path: String::from("/screenshots"),
        }
    }

    pub fn take_screenshot(&mut self, region: CaptureRegion, w: u32, h: u32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.screenshots.push(Screenshot {
            id,
            data: Vec::new(),
            width: w,
            height: h,
            timestamp: 0,
            region,
        });
        id
    }

    pub fn start_recording(&mut self, w: u32, h: u32, fps: u8) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.active_recording = Some(Recording {
            id,
            frames: Vec::new(),
            width: w,
            height: h,
            fps,
            started_at: 0,
            active: true,
        });
        id
    }

    pub fn stop_recording(&mut self) -> Option<u64> {
        if let Some(rec) = &mut self.active_recording {
            rec.active = false;
            Some(rec.id)
        } else {
            None
        }
    }

    pub fn is_recording(&self) -> bool {
        self.active_recording.as_ref().map(|r| r.active).unwrap_or(false)
    }

    pub fn screenshot_count(&self) -> usize {
        self.screenshots.len()
    }
}
