extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
    Custom(String),
}

pub struct PowerState {
    pub profile: PowerProfile,
    pub battery_pct: Option<u8>,
    pub charging: bool,
    pub screen_brightness: u8,
    pub cpu_governor: String,
    pub auto_sleep_mins: u16,
    pub auto_dim_secs: u16,
}

pub struct PowerManager {
    pub state: PowerState,
    pub profiles_available: Vec<String>,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            state: PowerState {
                profile: PowerProfile::Balanced,
                battery_pct: None,
                charging: false,
                screen_brightness: 80,
                cpu_governor: String::from("ondemand"),
                auto_sleep_mins: 30,
                auto_dim_secs: 120,
            },
            profiles_available: vec![
                String::from("Performance"),
                String::from("Balanced"),
                String::from("PowerSaver"),
            ],
        }
    }

    pub fn set_profile(&mut self, profile: PowerProfile) {
        match &profile {
            PowerProfile::Performance => {
                self.state.cpu_governor = String::from("performance");
                self.state.auto_sleep_mins = 0;
                self.state.screen_brightness = 100;
            }
            PowerProfile::Balanced => {
                self.state.cpu_governor = String::from("ondemand");
                self.state.auto_sleep_mins = 30;
                self.state.screen_brightness = 80;
            }
            PowerProfile::PowerSaver => {
                self.state.cpu_governor = String::from("powersave");
                self.state.auto_sleep_mins = 10;
                self.state.screen_brightness = 50;
            }
            PowerProfile::Custom(_) => {}
        }
        self.state.profile = profile;
    }

    pub fn set_brightness(&mut self, pct: u8) {
        self.state.screen_brightness = if pct > 100 { 100 } else { pct };
    }

    pub fn update_battery(&mut self, pct: u8, charging: bool) {
        self.state.battery_pct = Some(pct);
        self.state.charging = charging;
        if pct < 15 && !charging {
            self.set_profile(PowerProfile::PowerSaver);
        }
    }

    pub fn is_on_battery(&self) -> bool {
        self.state.battery_pct.is_some() && !self.state.charging
    }
}
