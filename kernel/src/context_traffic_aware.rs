extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut context = ContextTrafficAware::new();
    context.add_vehicle("Car1");
    context.add_vehicle("Truck2");
    context.update_traffic_light_status(true);
    context.log_current_state();
    loop {}
}

pub struct ContextTrafficAware {
    vehicles: Vec<String>,
    traffic_light_green: bool,
}

impl ContextTrafficAware {
    pub fn new() -> Self {
        ContextTrafficAware {
            vehicles: Vec::new(),
            traffic_light_green: false,
        }
    }

    pub fn add_vehicle(&mut self, vehicle_name: &str) {
        self.vehicles.push(String::from(vehicle_name));
    }

    pub fn remove_vehicle(&mut self, vehicle_name: &str) -> bool {
        if let Some(index) = self.vehicles.iter().position(|v| v == vehicle_name) {
            self.vehicles.remove(index);
            true
        } else {
            false
        }
    }

    pub fn update_traffic_light_status(&mut self, status: bool) {
        self.traffic_light_green = status;
    }

    pub fn get_vehicle_count(&self) -> usize {
        self.vehicles.len()
    }

    pub fn log_current_state(&self) {
        // Simulate logging the current state
        for vehicle in &self.vehicles {
            // No actual logging, just a placeholder
        }
        // Log traffic light status
        // No actual logging, just a placeholder
    }
}
