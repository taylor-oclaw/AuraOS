extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum Easing { Linear, EaseIn, EaseOut, EaseInOut }

pub struct Animation {
    pub id: u64, pub property: String,
    pub from: f32, pub to: f32,
    pub duration_ms: u64, pub elapsed_ms: u64,
    pub easing: Easing, pub active: bool,
}

pub struct AnimationEngine {
    pub animations: Vec<Animation>,
    pub next_id: u64,
    pub reduce_motion: bool,
}

impl AnimationEngine {
    pub fn new() -> Self { Self { animations: Vec::new(), next_id: 1, reduce_motion: false } }
    pub fn animate(&mut self, prop: &str, from: f32, to: f32, dur: u64, easing: Easing) -> u64 {
        if self.reduce_motion { return 0; }
        let id = self.next_id; self.next_id += 1;
        self.animations.push(Animation { id, property: String::from(prop), from, to, duration_ms: dur, elapsed_ms: 0, easing, active: true });
        id
    }
    pub fn tick(&mut self, delta_ms: u64) {
        for a in &mut self.animations {
            if !a.active { continue; }
            a.elapsed_ms += delta_ms;
            if a.elapsed_ms >= a.duration_ms { a.active = false; }
        }
        self.animations.retain(|a| a.active);
    }
    pub fn value(&self, id: u64) -> Option<f32> {
        self.animations.iter().find(|a| a.id == id).map(|a| {
            let t = a.elapsed_ms as f32 / a.duration_ms as f32;
            let t = if t > 1.0 { 1.0 } else { t };
            a.from + (a.to - a.from) * t
        })
    }
    pub fn cancel(&mut self, id: u64) { self.animations.retain(|a| a.id != id); }
    pub fn active_count(&self) -> usize { self.animations.iter().filter(|a| a.active).count() }
}
