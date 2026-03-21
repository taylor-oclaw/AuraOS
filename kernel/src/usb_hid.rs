extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

pub enum UsbHidType {
    Keyboard,
    Mouse,
    Gamepad,
    Touchscreen,
    Unknown,
}

pub struct UsbHidDevice {
    pub address: u8,
    pub endpoint: u8,
    pub hid_type: UsbHidType,
    pub vendor_id: u16,
    pub product_id: u16,
    pub name: String,
    pub poll_interval_ms: u8,
}

pub struct KeyboardReport {
    pub modifiers: u8,
    pub keys: [u8; 6],
}

pub struct MouseReport {
    pub buttons: u8,
    pub x_rel: i8,
    pub y_rel: i8,
    pub wheel: i8,
}

pub struct UsbHidDriver {
    pub devices: Vec<UsbHidDevice>,
    pub keyboard_buffer: Vec<KeyboardReport>,
    pub mouse_buffer: Vec<MouseReport>,
}

impl UsbHidDriver {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            keyboard_buffer: Vec::new(),
            mouse_buffer: Vec::new(),
        }
    }

    pub fn register_device(&mut self, dev: UsbHidDevice) {
        self.devices.push(dev);
    }

    pub fn process_keyboard(&mut self, report: KeyboardReport) {
        self.keyboard_buffer.push(report);
    }

    pub fn process_mouse(&mut self, report: MouseReport) {
        self.mouse_buffer.push(report);
    }

    pub fn next_key(&mut self) -> Option<KeyboardReport> {
        if self.keyboard_buffer.is_empty() {
            None
        } else {
            Some(self.keyboard_buffer.remove(0))
        }
    }

    pub fn next_mouse(&mut self) -> Option<MouseReport> {
        if self.mouse_buffer.is_empty() {
            None
        } else {
            Some(self.mouse_buffer.remove(0))
        }
    }

    pub fn keyboard_count(&self) -> usize {
        self.devices.iter().filter(|d| matches!(d.hid_type, UsbHidType::Keyboard)).count()
    }

    pub fn mouse_count(&self) -> usize {
        self.devices.iter().filter(|d| matches!(d.hid_type, UsbHidType::Mouse)).count()
    }
}
