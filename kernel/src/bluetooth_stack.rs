extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut stack = BluetoothStack::new();
    stack.initialize();
    stack.add_device("device1".into());
    stack.connect_device("device1");
    stack.send_data("device1", b"Hello, World!");
    stack.disconnect_device("device1");
    stack.shutdown();
}

pub struct BluetoothStack {
    devices: Vec<String>,
    connected_devices: Vec<String>,
}

impl BluetoothStack {
    pub fn new() -> Self {
        BluetoothStack {
            devices: Vec::new(),
            connected_devices: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        // Initialize the Bluetooth stack
        println!("Bluetooth stack initialized");
    }

    pub fn add_device(&mut self, device_name: String) {
        // Add a new device to the stack
        if !self.devices.contains(&device_name) {
            self.devices.push(device_name);
            println!("Device added: {}", &self.devices.last().unwrap());
        } else {
            println!("Device already exists");
        }
    }

    pub fn connect_device(&mut self, device_name: &str) {
        // Connect to a device
        if self.devices.contains(&device_name.into()) && !self.connected_devices.contains(&device_name.into()) {
            self.connected_devices.push(device_name.into());
            println!("Connected to device: {}", device_name);
        } else {
            println!("Device not found or already connected");
        }
    }

    pub fn send_data(&mut self, device_name: &str, data: &[u8]) {
        // Send data to a connected device
        if self.connected_devices.contains(&device_name.into()) {
            println!("Sending data to {}: {:?}", device_name, data);
        } else {
            println!("Device not connected");
        }
    }

    pub fn disconnect_device(&mut self, device_name: &str) {
        // Disconnect from a device
        if let Some(index) = self.connected_devices.iter().position(|x| x == device_name) {
            self.connected_devices.remove(index);
            println!("Disconnected from device: {}", device_name);
        } else {
            println!("Device not connected");
        }
    }

    pub fn shutdown(&mut self) {
        // Shutdown the Bluetooth stack
        self.connected_devices.clear();
        println!("Bluetooth stack shut down");
    }
}
