extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
    job_role: String,
}

impl Person {
    pub fn new(name: &str, age: u32, job_role: &str) -> Self {
        Person {
            name: String::from(name),
            age,
            job_role: String::from(job_role),
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

    pub fn get_job_role(&self) -> &str {
        &self.job_role
    }

    pub fn set_job_role(&mut self, new_job_role: &str) {
        self.job_role = String::from(new_job_role);
    }
}

#[derive(Debug)]
pub struct PeopleDetailJobRole {
    people: Vec<Person>,
}

impl PeopleDetailJobRole {
    pub fn new() -> Self {
        PeopleDetailJobRole {
            people: Vec::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|p| p.get_name() != name);
    }

    pub fn get_people_count(&self) -> usize {
        self.people.len()
    }

    pub fn find_person_by_name(&self, name: &str) -> Option<&Person> {
        self.people.iter().find(|p| p.get_name() == name)
    }
}
