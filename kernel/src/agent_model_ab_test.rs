extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct AgentModelABTest {
    model_a: String,
    model_b: String,
    results: Vec<(String, u32)>,
    current_model: String,
}

impl AgentModelABTest {
    pub fn new(model_a: &str, model_b: &str) -> Self {
        AgentModelABTest {
            model_a: String::from(model_a),
            model_b: String::from(model_b),
            results: Vec::new(),
            current_model: String::from(model_a),
        }
    }

    pub fn switch_model(&mut self) {
        if self.current_model == self.model_a {
            self.current_model = self.model_b.clone();
        } else {
            self.current_model = self.model_a.clone();
        }
    }

    pub fn get_current_model(&self) -> &str {
        &self.current_model
    }

    pub fn record_result(&mut self, model: &str, score: u32) {
        if model == self.model_a || model == self.model_b {
            self.results.push((String::from(model), score));
        }
    }

    pub fn get_results(&self) -> &Vec<(String, u32)> {
        &self.results
    }

    pub fn analyze_results(&self) -> (u32, u32) {
        let mut a_score = 0;
        let mut b_score = 0;

        for (model, score) in &self.results {
            if *model == self.model_a {
                a_score += score;
            } else if *model == self.model_b {
                b_score += score;
            }
        }

        (a_score, b_score)
    }
}
