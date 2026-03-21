extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoQualityScore {
    image_data: Vec<u8>,
    score: u32,
}

impl PhotoQualityScore {
    pub fn new(image_data: Vec<u8>) -> Self {
        let score = calculate_score(&image_data);
        PhotoQualityScore { image_data, score }
    }

    pub fn get_image_data(&self) -> &Vec<u8> {
        &self.image_data
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn set_image_data(&mut self, new_data: Vec<u8>) {
        self.image_data = new_data;
        self.score = calculate_score(&self.image_data);
    }

    pub fn is_high_quality(&self) -> bool {
        self.score > 75
    }

    pub fn analyze(&self) -> String {
        if self.is_high_quality() {
            String::from("info")
        } else {
            String::from("info")
        }
    }
}

fn calculate_score(image_data: &[u8]) -> u32 {
    // Simple heuristic for demonstration purposes
    let mut sum = 0;
    for &byte in image_data {
        sum += byte as u32;
    }
    (sum / image_data.len() as u32) % 100
}
