extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct AccelerometerDriver {
    // Simulated sensor data
    x: i32,
    y: i32,
    z: i32,
}

impl AccelerometerDriver {
    pub fn new() -> Self {
        AccelerometerDriver { x: 0, y: 0, z: 0 }
    }

    pub fn read_x(&self) -> i32 {
        self.x
    }

    pub fn read_y(&self) -> i32 {
        self.y
    }

    pub fn read_z(&self) -> i32 {
        self.z
    }

    pub fn update_data(&mut self, x: i32, y: i32, z: i32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn get_acceleration_vector(&self) -> Vec<i32> {
        vec![self.x, self.y, self.z]
    }
}
