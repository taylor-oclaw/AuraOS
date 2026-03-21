extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let mut people = PeopleGoodbyeStyle::new();
    people.add_person("Alice");
    people.add_person("Bob");
    people.add_person("Charlie");

    println!("People count: {}", people.count());

    if let Some(name) = people.remove_person("Bob") {
        println!("Removed person: {}", name);
    }

    for person in people.iter() {
        println!("Person: {}", person);
    }

    loop {}
}

pub struct PeopleGoodbyeStyle {
    names: Vec<String>,
}

impl PeopleGoodbyeStyle {
    pub fn new() -> Self {
        PeopleGoodbyeStyle { names: Vec::new() }
    }

    pub fn add_person(&mut self, name: &str) {
        self.names.push(String::from(name));
    }

    pub fn remove_person(&mut self, name: &str) -> Option<String> {
        if let Some(index) = self.names.iter().position(|n| n == name) {
            return Some(self.names.remove(index));
        }
        None
    }

    pub fn count(&self) -> usize {
        self.names.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.names.iter()
    }

    pub fn clear(&mut self) {
        self.names.clear();
    }
}
