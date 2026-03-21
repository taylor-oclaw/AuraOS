extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshCountryProfileUS {
    name: String,
    population: u64,
    capital: String,
    states: Vec<String>,
    gdp_per_capita: f64,
}

impl MeshCountryProfileUS {
    pub fn new(name: &str, population: u64, capital: &str, states: &[&str], gdp_per_capita: f64) -> Self {
        let state_vec = states.iter().map(|s| String::from(*s)).collect();
        MeshCountryProfileUS {
            name: String::from(name),
            population,
            capital: String::from(capital),
            states: state_vec,
            gdp_per_capita,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_population(&self) -> u64 {
        self.population
    }

    pub fn get_capital(&self) -> &str {
        &self.capital
    }

    pub fn get_states(&self) -> &[String] {
        &self.states
    }

    pub fn get_gdp_per_capita(&self) -> f64 {
        self.gdp_per_capita
    }

    pub fn add_state(&mut self, state: &str) {
        self.states.push(String::from(state));
    }

    pub fn remove_state(&mut self, state: &str) {
        if let Some(pos) = self.states.iter().position(|s| s == state) {
            self.states.remove(pos);
        }
    }

    pub fn update_gdp_per_capita(&mut self, new_gdp: f64) {
        self.gdp_per_capita = new_gdp;
    }

    pub fn has_state(&self, state: &str) -> bool {
        self.states.contains(&String::from(state))
    }
}
