extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct ProfileCoffeeShopVPN {
    username: String,
    password: String,
    connected: bool,
    location: String,
    coffee_preferences: Vec<String>,
}

impl ProfileCoffeeShopVPN {
    pub fn new(username: &str, password: &str) -> Self {
        ProfileCoffeeShopVPN {
            username: String::from(username),
            password: String::from(password),
            connected: false,
            location: String::new(),
            coffee_preferences: Vec::new(),
        }
    }

    pub fn connect(&mut self) {
        if !self.connected {
            // Simulate connection logic
            self.connected = true;
            println!("Connected to VPN as {}", self.username);
        } else {
            println!("Already connected");
        }
    }

    pub fn disconnect(&mut self) {
        if self.connected {
            // Simulate disconnection logic
            self.connected = false;
            println!("Disconnected from VPN");
        } else {
            println!("Not currently connected");
        }
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = String::from(location);
        println!("Location set to {}", self.location);
    }

    pub fn add_coffee_preference(&mut self, preference: &str) {
        self.coffee_preferences.push(String::from(preference));
        println!("Added coffee preference: {}", preference);
    }

    pub fn list_coffee_preferences(&self) -> Vec<String> {
        self.coffee_preferences.clone()
    }
}
