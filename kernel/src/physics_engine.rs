extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_physics_engine_init() {
}

pub extern "C" fn rust_physics_engine_exit() {
}

pub struct PhysicsEngine {
    objects: Vec<Object>,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine { objects: Vec::new() }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, index: usize) -> Option<Object> {
        if index < self.objects.len() {
            Some(self.objects.remove(index))
        } else {
            None
        }
    }

    pub fn update_positions(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.position.x += object.velocity.x * delta_time;
            object.position.y += object.velocity.y * delta_time;
        }
    }

    pub fn get_objects(&self) -> &[Object] {
        &self.objects
    }
}

pub struct Object {
    name: String,
    position: Vector2,
    velocity: Vector2,
}

impl Object {
    pub fn new(name: String, position: Vector2, velocity: Vector2) -> Self {
        Object { name, position, velocity }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_velocity(&mut self, velocity: Vector2) {
        self.velocity = velocity;
    }
}

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn subtract(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
