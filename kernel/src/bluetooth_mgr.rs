extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum BtDeviceType {
    Headphones,
    Keyboard,
    Mouse,
    Speaker,
    Phone,
    Watch,
    Unknown,
}

pub enum BtState {
    Off,
    Discovering,
    Connected,
    Pairing,
}

pub struct BtDevice {
    pub address: [u8; 6],
    pub name: String,
    pub device_type: BtDeviceType,
    pub paired: bool,
    pub connected: bool,
    pub signal: i8,
    pub battery_pct: Option<u8>,
}

pub struct BluetoothManager {
    pub state: BtState,
    pub devices: Vec<BtDevice>,
    pub paired_devices: Vec<[u8; 6]>,
    pub enabled: bool,
}

impl BluetoothManager {
    pub fn new() -> Self {
        Self {
            state: BtState::Off,
            devices: Vec::new(),
            paired_devices: Vec::new(),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.state = BtState::Off;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.state = BtState::Off;
        self.disconnect_all();
    }

    pub fn start_discovery(&mut self) {
        if self.enabled {
            self.state = BtState::Discovering;
            self.devices.clear();
        }
    }

    pub fn stop_discovery(&mut self) {
        self.state = BtState::Off;
    }

    pub fn pair(&mut self, addr: [u8; 6]) -> bool {
        if let Some(d) = self.devices.iter_mut().find(|d| d.address == addr) {
            d.paired = true;
            self.paired_devices.push(addr);
            true
        } else {
            false
        }
    }

    pub fn connect(&mut self, addr: [u8; 6]) -> bool {
        if let Some(d) = self.devices.iter_mut().find(|d| d.address == addr && d.paired) {
            d.connected = true;
            self.state = BtState::Connected;
            true
        } else {
            false
        }
    }

    pub fn disconnect_all(&mut self) {
        for d in &mut self.devices {
            d.connected = false;
        }
    }

    pub fn connected_devices(&self) -> Vec<&BtDevice> {
        self.devices.iter().filter(|d| d.connected).collect()
    }
}
