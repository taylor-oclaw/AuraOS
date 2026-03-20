extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DhcpClient {
    ip_address: Option<String>,
    subnet_mask: Option<String>,
    gateway: Option<String>,
    dns_servers: Vec<String>,
    lease_time: u32,
}

impl DhcpClient {
    pub fn new() -> Self {
        DhcpClient {
            ip_address: None,
            subnet_mask: None,
            gateway: None,
            dns_servers: Vec::new(),
            lease_time: 0,
        }
    }

    pub fn set_ip_address(&mut self, ip_address: String) {
        self.ip_address = Some(ip_address);
    }

    pub fn get_ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }

    pub fn set_subnet_mask(&mut self, subnet_mask: String) {
        self.subnet_mask = Some(subnet_mask);
    }

    pub fn get_subnet_mask(&self) -> Option<&String> {
        self.subnet_mask.as_ref()
    }

    pub fn set_gateway(&mut self, gateway: String) {
        self.gateway = Some(gateway);
    }

    pub fn get_gateway(&self) -> Option<&String> {
        self.gateway.as_ref()
    }

    pub fn add_dns_server(&mut self, dns_server: String) {
        self.dns_servers.push(dns_server);
    }

    pub fn get_dns_servers(&self) -> &Vec<String> {
        &self.dns_servers
    }

    pub fn set_lease_time(&mut self, lease_time: u32) {
        self.lease_time = lease_time;
    }

    pub fn get_lease_time(&self) -> u32 {
        self.lease_time
    }
}
