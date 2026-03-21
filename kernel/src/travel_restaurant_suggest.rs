extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod travel_restaurant_suggest {
    use super::*;

    pub struct RestaurantSuggestion {
        restaurants: Vec<String>,
        current_index: usize,
    }

    impl RestaurantSuggestion {
        pub fn new(restaurants: Vec<String>) -> Self {
            RestaurantSuggestion {
                restaurants,
                current_index: 0,
            }
        }

        pub fn add_restaurant(&mut self, restaurant_name: String) {
            self.restaurants.push(restaurant_name);
        }

        pub fn remove_restaurant(&mut self, restaurant_name: &str) -> bool {
            if let Some(index) = self.restaurants.iter().position(|r| r == restaurant_name) {
                self.restaurants.remove(index);
                true
            } else {
                false
            }
        }

        pub fn get_current_suggestion(&self) -> Option<&String> {
            self.restaurants.get(self.current_index)
        }

        pub fn next_suggestion(&mut self) -> Option<&String> {
            if self.current_index < self.restaurants.len() {
                let suggestion = &self.restaurants[self.current_index];
                self.current_index += 1;
                Some(suggestion)
            } else {
                None
            }
        }

        pub fn reset_suggestions(&mut self) {
            self.current_index = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::travel_restaurant_suggest::*;

    #[test]
    fn test_restaurant_suggestion() {
        let mut suggestions = RestaurantSuggestion::new(vec![
            String::from("Italian Bistro"),
            String::from("Japanese Sushi Bar"),
            String::from("Mexican Taqueria"),
        ]);

        assert_eq!(suggestions.get_current_suggestion(), Some(&String::from("Italian Bistro")));
        assert_eq!(suggestions.next_suggestion(), Some(&String::from("Japanese Sushi Bar")));
        assert_eq!(suggestions.next_suggestion(), Some(&String::from("Mexican Taqueria")));
        assert_eq!(suggestions.next_suggestion(), None);

        suggestions.reset_suggestions();
        assert_eq!(suggestions.get_current_suggestion(), Some(&String::from("Italian Bistro")));

        suggestions.add_restaurant(String::from("French Bistro"));
        assert_eq!(suggestions.restaurants.len(), 4);
        assert_eq!(suggestions.remove_restaurant("Japanese Sushi Bar"), true);
        assert_eq!(suggestions.restaurants.len(), 3);
    }
}
