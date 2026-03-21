extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PeopleNicknameRemember {
    names: Vec<String>,
}

impl PeopleNicknameRemember {
    pub fn new() -> Self {
        PeopleNicknameRemember {
            names: Vec::new(),
        }
    }

    pub fn add_nickname(&mut self, nickname: String) {
        if !self.names.contains(&nickname) {
            self.names.push(nickname);
        }
    }

    pub fn remove_nickname(&mut self, nickname: &str) -> bool {
        let position = self.names.iter().position(|n| n == nickname);
        if let Some(pos) = position {
            self.names.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_all_nicknames(&self) -> Vec<String> {
        self.names.clone()
    }

    pub fn contains_nickname(&self, nickname: &str) -> bool {
        self.names.contains(&String::from(nickname))
    }

    pub fn count_nicknames(&self) -> usize {
        self.names.len()
    }
}
