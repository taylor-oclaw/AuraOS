extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct CameraInfo {
    pub id: u64,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fps: u8,
    pub facing: CameraFacing
}

pub enum CameraFacing {
    Front,
    Back,
    External
}

pub enum CameraState {
    Off,
    Previewing,
    Recording,
    Error(String)
}

pub struct CameraDriver {
    pub cameras: Vec<CameraInfo>,
    pub active_camera: Option<u64>,
    pub state: CameraState,
    pub frames_captured: u64,
    pub privacy_indicator: bool
}

impl CameraDriver {
    pub fn new() -> Self {
        Self {
            cameras: Vec::new(),
            active_camera: None,
            state: CameraState::Off,
            frames_captured: 0,
            privacy_indicator: false
        }
    }

    pub fn detect_cameras(&mut self) {
        self.cameras.push(CameraInfo {
            id: 1,
            name: String::from("Built-in Camera"),
            width: 1920,
            height: 1080,
            fps: 30,
            facing: CameraFacing::Front
        });
    }

    pub fn start_preview(&mut self, camera_id: u64) -> bool {
        if self.cameras.iter().any(|c| c.id == camera_id) {
            self.active_camera = Some(camera_id);
            self.state = CameraState::Previewing;
            self.privacy_indicator = true;
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self) {
        self.state = CameraState::Off;
        self.active_camera = None;
        self.privacy_indicator = false;
    }

    pub fn is_recording(&self) -> bool {
        matches!(self.state, CameraState::Recording)
    }

    pub fn privacy_light_on(&self) -> bool {
        self.privacy_indicator
    }
}
