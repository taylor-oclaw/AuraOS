extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let robot = SmartHomeVacuumRobot::new(String::from("RoboClean"), 100, true);
    robot.clean_room();
    robot.charge_battery();
    robot.report_status();
    robot.update_firmware(String::from("v2.0"));
    robot.set_cleaning_mode(true);
}

pub struct SmartHomeVacuumRobot {
    name: String,
    battery_level: u8,
    is_charging: bool,
    cleaning_mode: bool,
    firmware_version: String,
}

impl SmartHomeVacuumRobot {
    pub fn new(name: String, battery_level: u8, is_charging: bool) -> Self {
        SmartHomeVacuumRobot {
            name,
            battery_level,
            is_charging,
            cleaning_mode: false,
            firmware_version: String::from("v1.0"),
        }
    }

    pub fn clean_room(&mut self) {
        if self.battery_level > 20 && !self.is_charging {
            println!("{} is cleaning the room.", self.name);
            self.battery_level -= 20;
        } else {
            println!("{} needs charging or battery level too low to clean.", self.name);
        }
    }

    pub fn charge_battery(&mut self) {
        if !self.is_charging {
            self.is_charging = true;
            println!("{} is now charging.", self.name);
            while self.battery_level < 100 {
                self.battery_level += 5;
            }
            self.is_charging = false;
            println!("{} battery fully charged.", self.name);
        } else {
            println!("{} is already charging.", self.name);
        }
    }

    pub fn report_status(&self) {
        println!(
            "{} - Battery: {}%, Charging: {}, Cleaning Mode: {}, Firmware: {}",
            self.name,
            self.battery_level,
            self.is_charging,
            self.cleaning_mode,
            self.firmware_version
        );
    }

    pub fn update_firmware(&mut self, new_version: String) {
        println!("{} is updating firmware to version {}", self.name, new_version);
        self.firmware_version = new_version;
    }

    pub fn set_cleaning_mode(&mut self, mode: bool) {
        if mode != self.cleaning_mode {
            self.cleaning_mode = mode;
            if mode {
                println!("{} cleaning mode enabled.", self.name);
            } else {
                println!("{} cleaning mode disabled.", self.name);
            }
        } else {
            if mode {
                println!("{} cleaning mode is already enabled.", self.name);
            } else {
                println!("{} cleaning mode is already disabled.", self.name);
            }
        }
    }
}
