extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct TaxCategory {
    name: String,
    rate: f64,
}

impl TaxCategory {
    pub fn new(name: &str, rate: f64) -> Self {
        TaxCategory {
            name: String::from(name),
            rate,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_rate(&self) -> f64 {
        self.rate
    }

    pub fn set_rate(&mut self, new_rate: f64) {
        self.rate = new_rate;
    }

    pub fn calculate_tax(&self, amount: f64) -> f64 {
        amount * self.rate
    }
}

struct FinanceTaxCategorize {
    categories: Vec<TaxCategory>,
}

impl FinanceTaxCategorize {
    pub fn new() -> Self {
        FinanceTaxCategorize {
            categories: Vec::new(),
        }
    }

    pub fn add_category(&mut self, category: TaxCategory) {
        self.categories.push(category);
    }

    pub fn get_categories(&self) -> &Vec<TaxCategory> {
        &self.categories
    }

    pub fn find_category_by_name(&self, name: &str) -> Option<&TaxCategory> {
        self.categories.iter().find(|&c| c.get_name() == name)
    }

    pub fn calculate_total_tax(&self, amounts: &[f64]) -> f64 {
        if amounts.len() != self.categories.len() {
            return 0.0;
        }
        self.categories
            .iter()
            .zip(amounts.iter())
            .map(|(category, &amount)| category.calculate_tax(amount))
            .sum()
    }
}
