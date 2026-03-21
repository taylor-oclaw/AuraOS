extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let mut profile = ProfileCoffeeShopDetect::new();
    profile.add_drink("Espresso");
    profile.add_drink("Latte");
    profile.add_drink("Americano");
    profile.add_drink("Cappuccino");
    profile.add_drink("Mocha");

    profile.set_location("Central Park");
    profile.set_rating(4.5);


    for drink in profile.get_drinks() {
    }
}

pub struct ProfileCoffeeShopDetect {
    drinks: Vec<String>,
    location: String,
    rating: f32,
}

impl ProfileCoffeeShopDetect {
    pub fn new() -> Self {
        ProfileCoffeeShopDetect {
            drinks: Vec::new(),
            location: String::from("Unknown"),
            rating: 0.0,
        }
    }

    pub fn add_drink(&mut self, drink: &str) {
        self.drinks.push(String::from(drink));
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = String::from(location);
    }

    pub fn set_rating(&mut self, rating: f32) {
        if rating >= 0.0 && rating <= 5.0 {
            self.rating = rating;
        }
    }

    pub fn get_drinks(&self) -> &Vec<String> {
        &self.drinks
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn get_rating(&self) -> f32 {
        self.rating
    }
}
