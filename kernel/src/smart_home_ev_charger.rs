extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn smart_home_ev_charger_init() {
    // Initialization logic for the EV charger module
}

#[no_mangle]
pub extern "C" fn smart_home_ev_charger_exit() {
    // Cleanup logic for the EV charger module
}

pub struct SmartHomeEvCharger {
    status: String,
    connected_cars: Vec<String>,
    max_power_output: u32,
    current_power_usage: u32,
    charging_sessions: usize,
}

impl SmartHomeEvCharger {
    pub fn new(max_power_output: u32) -> Self {
        SmartHomeEvCharger {
            status: String::from("Idle"),
            connected_cars: Vec::new(),
            max_power_output,
            current_power_usage: 0,
            charging_sessions: 0,
        }
    }

    pub fn connect_car(&mut self, car_id: &str) -> bool {
        if self.connected_cars.contains(&String::from(car_id)) {
            false
        } else {
            self.connected_cars.push(String::from(car_id));
            true
        }
    }

    pub fn disconnect_car(&mut self, car_id: &str) -> bool {
        if let Some(index) = self.connected_cars.iter().position(|x| x == car_id) {
            self.connected_cars.remove(index);
            true
        } else {
            false
        }
    }

    pub fn start_charging_session(&mut self, power_usage: u32) -> bool {
        if self.current_power_usage + power_usage <= self.max_power_output {
            self.current_power_usage += power_usage;
            self.charging_sessions += 1;
            self.status = String::from("Charging");
            true
        } else {
            false
        }
    }

    pub fn stop_charging_session(&mut self, power_usage: u32) -> bool {
        if self.current_power_usage >= power_usage {
            self.current_power_usage -= power_usage;
            if self.current_power_usage == 0 {
                self.status = String::from("Idle");
            }
            true
        } else {
            false
        }
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}
