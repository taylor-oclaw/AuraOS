extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechConsonantCluster {
    consonants: Vec<char>,
}

impl SpeechConsonantCluster {
    pub fn new() -> Self {
        SpeechConsonantCluster {
            consonants: Vec::new(),
        }
    }

    pub fn add_consonant(&mut self, consonant: char) {
        if is_consonant(consonant) && !self.consonants.contains(&consonant) {
            self.consonants.push(consonant);
        }
    }

    pub fn remove_consonant(&mut self, consonant: char) {
        self.consonants.retain(|&c| c != consonant);
    }

    pub fn get_consonants(&self) -> &Vec<char> {
        &self.consonants
    }

    pub fn is_empty(&self) -> bool {
        self.consonants.is_empty()
    }

    pub fn count_consonants(&self) -> usize {
        self.consonants.len()
    }
}

fn is_consonant(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z')
}
