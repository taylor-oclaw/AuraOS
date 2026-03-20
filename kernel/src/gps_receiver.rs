extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn gps_receiver_init() -> *const GPSReceiver {
    let receiver = Box::new(GPSReceiver::new());
    Box::leak(receiver) as *const _
}

pub extern "C" fn gps_receiver_free(ptr: *mut GPSReceiver) {
    unsafe { drop(Box::from_raw(ptr)) }
}

pub struct GPSReceiver {
    satellites: Vec<String>,
    location: String,
    speed: u32,
    altitude: f32,
    time: String,
}

impl GPSReceiver {
    pub fn new() -> Self {
        GPSReceiver {
            satellites: Vec::new(),
            location: String::from("Unknown"),
            speed: 0,
            altitude: 0.0,
            time: String::from("00:00:00"),
        }
    }

    pub fn add_satellite(&mut self, satellite_name: &str) {
        self.satellites.push(String::from(satellite_name));
    }

    pub fn get_satellites(&self) -> Vec<String> {
        self.satellites.clone()
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = String::from(location);
    }

    pub fn get_location(&self) -> String {
        self.location.clone()
    }

    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn set_altitude(&mut self, altitude: f32) {
        self.altitude = altitude;
    }

    pub fn get_altitude(&self) -> f32 {
        self.altitude
    }

    pub fn set_time(&mut self, time: &str) {
        self.time = String::from(time);
    }

    pub fn get_time(&self) -> String {
        self.time.clone()
    }
}
