extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let avatar = Avatar::new("Qwen", 100);
    println!("Avatar Name: {}", avatar.get_name());
    println!("Avatar Level: {}", avatar.get_level());
    println!("Avatar Experience: {}", avatar.get_experience());
    println!("Avatar Health: {}", avatar.get_health());
    println!("Avatar Mana: {}", avatar.get_mana());

    avatar.level_up();
    println!("After leveling up:");
    println!("Avatar Level: {}", avatar.get_level());
    println!("Avatar Experience: {}", avatar.get_experience());
}

pub struct Avatar {
    name: String,
    level: u32,
    experience: u32,
    health: u32,
    mana: u32,
}

impl Avatar {
    pub fn new(name: &str, level: u32) -> Self {
        Avatar {
            name: String::from(name),
            level,
            experience: 0,
            health: 100 + (level * 10),
            mana: 50 + (level * 5),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_experience(&self) -> u32 {
        self.experience
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn get_mana(&self) -> u32 {
        self.mana
    }

    pub fn level_up(&mut self) {
        if self.experience >= 100 * self.level {
            self.level += 1;
            self.experience = 0;
            self.health = 100 + (self.level * 10);
            self.mana = 50 + (self.level * 5);
        }
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        if self.experience >= 100 * self.level {
            self.level_up();
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        if damage < self.health {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }

    pub fn use_mana(&mut self, mana_cost: u32) {
        if mana_cost <= self.mana {
            self.mana -= mana_cost;
        } else {
            // Handle out of mana case
        }
    }

    pub fn restore_health(&mut self, amount: u32) {
        self.health += amount;
        if self.health > 100 + (self.level * 10) {
            self.health = 100 + (self.level * 10);
        }
    }

    pub fn restore_mana(&mut self, amount: u32) {
        self.mana += amount;
        if self.mana > 50 + (self.level * 5) {
            self.mana = 50 + (self.level * 5);
        }
    }
}
