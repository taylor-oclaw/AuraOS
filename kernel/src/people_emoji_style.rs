extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    emoji: char,
}

impl Person {
    pub fn new(name: &str, age: u8, emoji: char) -> Self {
        Person {
            name: String::from(name),
            age,
            emoji,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn set_age(&mut self, new_age: u8) {
        self.age = new_age;
    }

    pub fn get_emoji(&self) -> char {
        self.emoji
    }

    pub fn set_emoji(&mut self, new_emoji: char) {
        self.emoji = new_emoji;
    }
}

#[derive(Debug)]
pub struct PeopleEmojiStyle {
    people: Vec<Person>,
}

impl PeopleEmojiStyle {
    pub fn new() -> Self {
        PeopleEmojiStyle {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.get_name() != name);
    }

    pub fn get_people(&self) -> &Vec<Person> {
        &self.people
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.get_name() == name)
    }

    pub fn count_people(&self) -> usize {
        self.people.len()
    }
}
