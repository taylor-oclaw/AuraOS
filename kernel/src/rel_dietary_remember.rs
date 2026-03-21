extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut diet = DietaryRemember::new();
    diet.add_food("Apple", 100);
    diet.add_food("Banana", 200);
    diet.remove_food("Apple");
    diet.update_calories("Banana", 150);
    let total_calories = diet.total_calories();
    // Assuming there's a way to print or log in the kernel
    // unsafe { printk!("Total Calories: {}\n", total_calories); }
    loop {}
}

pub struct DietaryRemember {
    foods: Vec<(String, u32)>,
}

impl DietaryRemember {
    pub fn new() -> Self {
        DietaryRemember {
            foods: Vec::new(),
        }
    }

    pub fn add_food(&mut self, name: &str, calories: u32) {
        let food_name = String::from(name);
        self.foods.push((food_name, calories));
    }

    pub fn remove_food(&mut self, name: &str) {
        self.foods.retain(|(food_name, _)| food_name != name);
    }

    pub fn update_calories(&mut self, name: &str, new_calories: u32) {
        if let Some((_, calories)) = self.foods.iter_mut().find(|(food_name, _)| food_name == name) {
            *calories = new_calories;
        }
    }

    pub fn total_calories(&self) -> u32 {
        self.foods.iter().map(|(_, calories)| calories).sum()
    }

    pub fn list_foods(&self) -> Vec<String> {
        self.foods.iter().map(|(food_name, _)| food_name.clone()).collect()
    }
}
