extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SpeechSentenceComplete {
    sentences: Vec<String>,
}

impl SpeechSentenceComplete {
    pub fn new() -> Self {
        SpeechSentenceComplete {
            sentences: Vec::new(),
        }
    }

    pub fn add_sentence(&mut self, sentence: String) {
        self.sentences.push(sentence);
    }

    pub fn get_sentences(&self) -> &Vec<String> {
        &self.sentences
    }

    pub fn find_sentence(&self, keyword: &str) -> Option<&String> {
        self.sentences.iter().find(|sentence| sentence.contains(keyword))
    }

    pub fn remove_sentence(&mut self, index: usize) -> Option<String> {
        if index < self.sentences.len() {
            Some(self.sentences.remove(index))
        } else {
            None
        }
    }

    pub fn count_sentences(&self) -> usize {
        self.sentences.len()
    }
}
