extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct HallucinationScore {
    scores: Vec<f64>,
    threshold: f64,
}

impl HallucinationScore {
    pub fn new(threshold: f64) -> Self {
        HallucinationScore {
            scores: Vec::new(),
            threshold,
        }
    }

    pub fn add_score(&mut self, score: f64) {
        self.scores.push(score);
    }

    pub fn average_score(&self) -> Option<f64> {
        if self.scores.is_empty() {
            None
        } else {
            let sum: f64 = self.scores.iter().sum();
            Some(sum / self.scores.len() as f64)
        }
    }

    pub fn highest_score(&self) -> Option<f64> {
        self.scores.iter().cloned().max()
    }

    pub fn lowest_score(&self) -> Option<f64> {
        self.scores.iter().cloned().min()
    }

    pub fn is_above_threshold(&self, score: f64) -> bool {
        score > self.threshold
    }
}

pub extern "C" fn ai_sec_hallucination_score_new(threshold: f64) -> *mut HallucinationScore {
    Box::into_raw(Box::new(HallucinationScore::new(threshold)))
}

pub extern "C" fn ai_sec_hallucination_score_add_score(score_ptr: *mut HallucinationScore, score: f64) {
    unsafe {
        (*score_ptr).add_score(score);
    }
}

pub extern "C" fn ai_sec_hallucination_score_average_score(score_ptr: *const HallucinationScore) -> Option<f64> {
    unsafe { (*score_ptr).average_score() }
}

pub extern "C" fn ai_sec_hallucination_score_highest_score(score_ptr: *const HallucinationScore) -> Option<f64> {
    unsafe { (*score_ptr).highest_score() }
}

pub extern "C" fn ai_sec_hallucination_score_lowest_score(score_ptr: *const HallucinationScore) -> Option<f64> {
    unsafe { (*score_ptr).lowest_score() }
}

pub extern "C" fn ai_sec_hallucination_score_is_above_threshold(score_ptr: *const HallucinationScore, score: f64) -> bool {
    unsafe { (*score_ptr).is_above_threshold(score) }
}

pub extern "C" fn ai_sec_hallucination_score_free(score_ptr: *mut HallucinationScore) {
    if !score_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(score_ptr));
        }
    }
}
