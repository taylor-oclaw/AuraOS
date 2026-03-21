extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

struct MovingObject {
    position: (i32, i32),
    velocity: (i32, i32),
    acceleration: (i32, i32),
    mass: f32,
    name: String,
}

impl MovingObject {
    pub fn new(position: (i32, i32), velocity: (i32, i32), acceleration: (i32, i32), mass: f32, name: &str) -> Self {
        MovingObject {
            position,
            velocity,
            acceleration,
            mass,
            name: String::from(name),
        }
    }

    pub fn update_position(&mut self, time: f32) {
        let (vx, vy) = self.velocity;
        let (ax, ay) = self.acceleration;
        self.position.0 += (vx as f32 + 0.5 * ax * time.powi(2)) as i32;
        self.position.1 += (vy as f32 + 0.5 * ay * time.powi(2)) as i32;
    }

    pub fn update_velocity(&mut self, time: f32) {
        let (vx, vy) = self.velocity;
        let (ax, ay) = self.acceleration;
        self.velocity.0 = vx + ax * time as i32;
        self.velocity.1 = vy + ay * time as i32;
    }

    pub fn apply_force(&mut self, force: (f32, f32)) {
        let (fx, fy) = force;
        self.acceleration.0 += (fx / self.mass) as i32;
        self.acceleration.1 += (fy / self.mass) as i32;
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    pub fn get_velocity(&self) -> (i32, i32) {
        self.velocity
    }
}

pub extern "C" fn create_moving_object(position: (i32, i32), velocity: (i32, i32), acceleration: (i32, i32), mass: f32, name: *const u8) -> *mut MovingObject {
    let c_str = unsafe { core::ffi::CStr::from_ptr(name as *const _) };
    let name_str = String::from_utf8_lossy(c_str.to_bytes()).into_owned();
    Box::leak(Box::new(MovingObject::new(position, velocity, acceleration, mass, &name_str)))
}

pub extern "C" fn update_object_position(obj: *mut MovingObject, time: f32) {
    unsafe { (*obj).update_position(time); }
}

pub extern "C" fn update_object_velocity(obj: *mut MovingObject, time: f32) {
    unsafe { (*obj).update_velocity(time); }
}

pub extern "C" fn apply_force_to_object(obj: *mut MovingObject, force: (f32, f32)) {
    unsafe { (*obj).apply_force(force); }
}

pub extern "C" fn get_object_position(obj: *const MovingObject) -> (i32, i32) {
    unsafe { (*obj).get_position() }
}

pub extern "C" fn get_object_velocity(obj: *const MovingObject) -> (i32, i32) {
    unsafe { (*obj).get_velocity() }
}
