extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub enum Protocol {
    Tcp,
    Udp,
    Http,
    Https,
    Ssh,
    Ftp,
    Smb,
    Nfs,
    AgentMesh,
    Bluetooth,
    Usb,
}

pub enum TargetDevice {
    AuraOs(String),
    Linux(String),
    MacOs(String),
    Windows(String),
    Unknown(String),
}

pub struct ConnectionInfo {
    pub target: TargetDevice,
    pub protocol: Protocol,
    pub address: String,
    pub port: u16,
    pub encrypted: bool,
    pub authenticated: bool,
}

pub struct CompatBridge {
    pub connections: Vec<ConnectionInfo>,
    pub protocol_preference: Vec<Protocol>,
}

impl CompatBridge {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            protocol_preference: vec![
                Protocol::AgentMesh,
                Protocol::Https,
                Protocol::Ssh,
                Protocol::Smb,
                Protocol::Http,
                Protocol::Ftp,
                Protocol::Tcp,
            ],
        }
    }

    pub fn detect_target(&self, address: &str) -> TargetDevice {
        if address.contains("aura") {
            TargetDevice::AuraOs(String::from(address))
        } else if address.contains("linux") || address.contains("ubuntu") {
            TargetDevice::Linux(String::from(address))
        } else if address.contains("mac") || address.contains("apple") {
            TargetDevice::MacOs(String::from(address))
        } else if address.contains("win") {
            TargetDevice::Windows(String::from(address))
        } else {
            TargetDevice::Unknown(String::from(address))
        }
    }

    pub fn select_protocol(&self, target: &TargetDevice) -> Protocol {
        match target {
            TargetDevice::AuraOs(_) => Protocol::AgentMesh,
            TargetDevice::Linux(_) => Protocol::Ssh,
            TargetDevice::MacOs(_) => Protocol::Smb,
            TargetDevice::Windows(_) => Protocol::Smb,
            TargetDevice::Unknown(_) => Protocol::Tcp,
        }
    }

    pub fn connect(&mut self, address: &str, port: u16) -> &ConnectionInfo {
        let target = self.detect_target(address);
        let protocol = self.select_protocol(&target);
        let encrypted = matches!(protocol, Protocol::AgentMesh | Protocol::Https | Protocol::Ssh);

        self.connections.push(ConnectionInfo {
            target,
            protocol,
            address: String::from(address),
            port,
            encrypted,
            authenticated: false,
        };

        self.connections.last().unwrap()
    }

    pub fn active_connections(&self) -> usize {
        self.connections.len()
    }
)}
