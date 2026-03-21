extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(C)]
pub struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    pub fn new(name: &str, age: u32, email: &str) -> Self {
        Person {
            name: String::from(name),
            age,
            email: String::from(email),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

#[repr(C)]
pub struct PeopleRememberPrompt {
    people: Vec<Person>,
}

impl PeopleRememberPrompt {
    pub fn new() -> Self {
        PeopleRememberPrompt {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.get_name() != name);
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.get_name() == name)
    }

    pub fn list_people(&self) -> &[Person] {
        &self.people
    }
}
