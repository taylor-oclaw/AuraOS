extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct UnitConverter {
    conversion_rates: Vec<(String, String, f64)>,
}

impl UnitConverter {
    pub fn new() -> Self {
        UnitConverter {
            conversion_rates: Vec::new(),
        }
    }

    pub fn add_conversion_rate(&mut self, from_unit: &str, to_unit: &str, rate: f64) {
        let from_unit = String::from(from_unit);
        let to_unit = String::from(to_unit);
        self.conversion_rates.push((from_unit, to_unit, rate));
    }

    pub fn convert(&self, value: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
        for &(ref f_unit, ref t_unit, rate) in &self.conversion_rates {
            if f_unit == from_unit && t_unit == to_unit {
                return Some(value * rate);
            }
        }
        None
    }

    pub fn list_conversion_rates(&self) -> Vec<(String, String, f64)> {
        self.conversion_rates.clone()
    }

    pub fn remove_conversion_rate(&mut self, from_unit: &str, to_unit: &str) {
        self.conversion_rates.retain(|&(ref f_unit, ref t_unit, _)| {
            !(f_unit == from_unit && t_unit == to_unit)
        });
    }
}
