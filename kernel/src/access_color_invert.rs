extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut color_inverter = ColorInverter::new();
    color_inverter.add_color("red");
    color_inverter.add_color("green");
    color_inverter.add_color("blue");

    if color_inverter.is_color_available("red") {
        color_inverter.invert_color("red");
    }

    let inverted_colors = color_inverter.get_inverted_colors();
    for color in inverted_colors.iter() {
        // Simulate printing the inverted colors
        unsafe { printk(b"Inverted Color: {}\n", color.as_ptr()) };
    }
}

#[no_mangle]
pub extern "C" fn rust_stop() {
    // Cleanup code if needed
}

struct ColorInverter {
    colors: Vec<String>,
    inverted_colors: Vec<String>,
}

impl ColorInverter {
    pub fn new() -> Self {
        ColorInverter {
            colors: Vec::new(),
            inverted_colors: Vec::new(),
        }
    }

    pub fn add_color(&mut self, color: &str) {
        self.colors.push(String::from(color));
    }

    pub fn is_color_available(&self, color: &str) -> bool {
        self.colors.contains(&String::from(color))
    }

    pub fn invert_color(&mut self, color: &str) {
        if let Some(index) = self.colors.iter().position(|c| c == color) {
            let inverted_color = format!("inverted_{}", color);
            self.inverted_colors.push(inverted_color);
            self.colors.remove(index);
        }
    }

    pub fn get_inverted_colors(&self) -> &Vec<String> {
        &self.inverted_colors
    }

    pub fn clear_colors(&mut self) {
        self.colors.clear();
        self.inverted_colors.clear();
    }
}

// Simulate a printk function for demonstration purposes
extern "C" {
    fn printk(fmt: *const u8, ...);
}
