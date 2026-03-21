extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ShoppingMealPlan {
    meals: Vec<String>,
}

impl ShoppingMealPlan {
    pub fn new() -> Self {
        ShoppingMealPlan { meals: Vec::new() }
    }

    pub fn add_meal(&mut self, meal: String) {
        self.meals.push(meal);
    }

    pub fn remove_meal(&mut self, index: usize) -> Option<String> {
        if index < self.meals.len() {
            Some(self.meals.remove(index))
        } else {
            None
        }
    }

    pub fn get_meal(&self, index: usize) -> Option<&String> {
        self.meals.get(index)
    }

    pub fn list_meals(&self) -> &Vec<String> {
        &self.meals
    }

    pub fn clear_meals(&mut self) {
        self.meals.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shopping_meal_plan() {
        let mut plan = ShoppingMealPlan::new();

        plan.add_meal(String::from("Breakfast"));
        plan.add_meal(String::from("Lunch"));
        plan.add_meal(String::from("Dinner"));

        assert_eq!(plan.list_meals().len(), 3);
        assert_eq!(plan.get_meal(1), Some(&String::from("Lunch")));
        assert_eq!(plan.remove_meal(0), Some(String::from("Breakfast")));
        assert_eq!(plan.list_meals().len(), 2);
        plan.clear_meals();
        assert_eq!(plan.list_meals().len(), 0);
    }
}
