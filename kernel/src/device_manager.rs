extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum DeviceType {
    Storage,
    Display,
    Input,
    Network,
    Audio,
    Usb,
    Gpu,
    Serial,
    Unknown,
}

pub enum DeviceStatus {
    Detected,
    Initializing,
    Ready,
    Error(String),
    Disabled,
}

pub struct Device {
    pub id: u64,
    pub name: String,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
    pub driver: Option<String>,
    pub io_base: Option<u16>,
    pub irq: Option<u8>,
    pub capabilities: Vec<String>,
}

pub struct DeviceManager {
    pub devices: Vec<Device>,
    pub next_id: u64,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            next_id: 1,
        }
    }

    pub fn register(&mut self, name: &str, dt: DeviceType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.devices.push(Device {
            id,
            name: String::from(name),
            device_type: dt,
            status: DeviceStatus::Detected,
            driver: None,
            io_base: None,
            irq: None,
            capabilities: Vec::new(),
        };
        id
    }

    pub fn init_device(&mut self, id: u64, driver: &str) {
        if let Some(d) = self.devices.iter_mut().find(|d| d.id == id) {
            d.driver = Some(String::from(driver));
            d.status = DeviceStatus::Ready;
        }
    }

    pub fn probe_all(&mut self) {
        self.register("ATA Primary", DeviceType::Storage);
        self.register("VGA Framebuffer", DeviceType::Display);
        self.register("PS/2 Keyboard", DeviceType::Input);
        self.register("PS/2 Mouse", DeviceType::Input);
        self.register("Serial COM1", DeviceType::Serial);
        self.register("PCI Bus", DeviceType::Unknown);
        self.register("E1000 NIC", DeviceType::Network);
    }

    pub fn by_type(&self, dt_name: &str) -> Vec<&Device> {
        self.devices.iter().filter(|d| {
            let type_name = match d.device_type {
                DeviceType::Storage => "storage",
                DeviceType::Display => "display",
                DeviceType::Input => "input",
                DeviceType::Network => "network",
                DeviceType::Audio => "audio",
                DeviceType::Usb => "usb",
                DeviceType::Gpu => "gpu",
                DeviceType::Serial => "serial",
                DeviceType::Unknown => "unknown",
            };
            type_name == dt_name
        }).collect()
    }

    pub fn ready_count(&self) -> usize {
        self.devices.iter().filter(|d| matches!(d.status, DeviceStatus::Ready)).count()
    }
)}
