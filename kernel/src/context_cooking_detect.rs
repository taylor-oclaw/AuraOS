extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct ContextCookingDetect {
    detected: bool,
    temperature: u32,
    ingredients: Vec<String>,
    cooking_time: u32,
    recipe_name: String,
}

impl ContextCookingDetect {
    pub fn new() -> Self {
        ContextCookingDetect {
            detected: false,
            temperature: 0,
            ingredients: Vec::new(),
            cooking_time: 0,
            recipe_name: String::from("Unknown"),
        }
    }

    pub fn set_detected(&mut self, detected: bool) {
        self.detected = detected;
    }

    pub fn is_detected(&self) -> bool {
        self.detected
    }

    pub fn set_temperature(&mut self, temperature: u32) {
        self.temperature = temperature;
    }

    pub fn get_temperature(&self) -> u32 {
        self.temperature
    }

    pub fn add_ingredient(&mut self, ingredient: String) {
        self.ingredients.push(ingredient);
    }

    pub fn get_ingredients(&self) -> &Vec<String> {
        &self.ingredients
    }

    pub fn set_cooking_time(&mut self, cooking_time: u32) {
        self.cooking_time = cooking_time;
    }

    pub fn get_cooking_time(&self) -> u32 {
        self.cooking_time
    }

    pub fn set_recipe_name(&mut self, recipe_name: String) {
        self.recipe_name = recipe_name;
    }

    pub fn get_recipe_name(&self) -> &str {
        &self.recipe_name
    }
}
