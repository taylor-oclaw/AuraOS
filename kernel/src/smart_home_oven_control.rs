extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let oven = SmartHomeOvenControl::new();
    oven.set_temperature(200);
    oven.start_cooking();
    oven.add_recipe("Pizza");
    oven.check_status();
    oven.stop_cooking();
}

pub struct SmartHomeOvenControl {
    temperature: u32,
    is_cooking: bool,
    recipes: Vec<String>,
}

impl SmartHomeOvenControl {
    pub fn new() -> Self {
        SmartHomeOvenControl {
            temperature: 0,
            is_cooking: false,
            recipes: Vec::new(),
        }
    }

    pub fn set_temperature(&mut self, temp: u32) {
        if temp > 0 && temp < 500 {
            self.temperature = temp;
        } else {
            // Handle error or log
        }
    }

    pub fn start_cooking(&mut self) {
        if !self.is_cooking {
            self.is_cooking = true;
            // Simulate cooking start
        }
    }

    pub fn stop_cooking(&mut self) {
        if self.is_cooking {
            self.is_cooking = false;
            // Simulate cooking stop
        }
    }

    pub fn add_recipe(&mut self, recipe: &str) {
        self.recipes.push(String::from(recipe));
    }

    pub fn check_status(&self) -> String {
        let status = if self.is_cooking {
            format!("Cooking at {}°C with {} recipes.", self.temperature, self.recipes.len())
        } else {
            format!("Not cooking. Temperature: {}°C, Recipes: {}", self.temperature, self.recipes.len())
        };
        status
    }
}
