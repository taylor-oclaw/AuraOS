extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct GiftAllergyAwareGift {
    name: String,
    description: String,
    allergens: Vec<String>,
}

impl GiftAllergyAwareGift {
    pub fn new(name: &str, description: &str) -> Self {
        GiftAllergyAwareGift {
            name: String::from(name),
            description: String::from(description),
            allergens: Vec::new(),
        }
    }

    pub fn add_allergen(&mut self, allergen: &str) {
        if !self.allergens.contains(&String::from(allergen)) {
            self.allergens.push(String::from(allergen));
        }
    }

    pub fn remove_allergen(&mut self, allergen: &str) {
        self.allergens.retain(|a| a != allergen);
    }

    pub fn has_allergen(&self, allergen: &str) -> bool {
        self.allergens.contains(&String::from(allergen))
    }

    pub fn list_allergens(&self) -> Vec<String> {
        self.allergens.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gift_allergy_aware_gift() {
        let mut gift = GiftAllergyAwareGift::new("Chocolate Box", "A box filled with chocolates.");

        assert_eq!(gift.get_name(), "Chocolate Box");
        assert_eq!(gift.get_description(), "A box filled with chocolates.");
        assert!(!gift.has_allergen("Nuts"));

        gift.add_allergen("Nuts");
        assert!(gift.has_allergen("Nuts"));
        let allergens = gift.list_allergens();
        assert_eq!(allergens, vec![String::from("Nuts")]);

        gift.remove_allergen("Nuts");
        assert!(!gift.has_allergen("Nuts"));
        let allergens = gift.list_allergens();
        assert!(allergens.is_empty());
    }
}
