extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn health_breathing_exercise_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn health_breathing_exercise_exit() {
    // Cleanup logic for the module
}

pub struct BreathingExercise {
    name: String,
    duration: u32, // in seconds
    steps: Vec<String>,
    current_step: usize,
    timer: u32,
}

impl BreathingExercise {
    pub fn new(name: &str, duration: u32, steps: Vec<&str>) -> Self {
        BreathingExercise {
            name: String::from(name),
            duration,
            steps: steps.into_iter().map(String::from).collect(),
            current_step: 0,
            timer: 0,
        }
    }

    pub fn start(&mut self) {
        self.current_step = 0;
        self.timer = 0;
    }

    pub fn next_step(&mut self) -> Option<&str> {
        if self.timer >= self.duration {
            return None;
        }

        let step = &self.steps[self.current_step];
        self.current_step += 1;

        Some(step)
    }

    pub fn is_finished(&self) -> bool {
        self.timer >= self.duration || self.current_step >= self.steps.len()
    }

    pub fn update_timer(&mut self, elapsed: u32) {
        self.timer += elapsed;
    }
}

#[no_mangle]
pub extern "C" fn health_breathing_exercise_create(name: *const u8, duration: u32, steps_ptr: *const *const u8, steps_count: usize) -> *mut BreathingExercise {
    let name_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, 0)) };
    let mut steps_vec = Vec::new();
    for i in 0..steps_count {
        let step_ptr = unsafe { *steps_ptr.offset(i as isize) };
        let step_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(step_ptr, 0)) };
        steps_vec.push(step_str);
    }

    Box::into_raw(Box::new(BreathingExercise::new(name_str, duration, steps_vec)))
}

#[no_mangle]
pub extern "C" fn health_breathing_exercise_destroy(exercise: *mut BreathingExercise) {
    unsafe { drop(Box::from_raw(exercise)); }
}
