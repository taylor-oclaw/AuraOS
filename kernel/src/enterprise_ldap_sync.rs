extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseLdapSync {
    server_url: String,
    bind_dn: String,
    bind_password: String,
    search_base: String,
    attributes: Vec<String>,
}

impl EnterpriseLdapSync {
    pub fn new(server_url: &str, bind_dn: &str, bind_password: &str, search_base: &str) -> Self {
        EnterpriseLdapSync {
            server_url: String::from(server_url),
            bind_dn: String::from(bind_dn),
            bind_password: String::from(bind_password),
            search_base: String::from(search_base),
            attributes: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, attribute: &str) {
        self.attributes.push(String::from(attribute));
    }

    pub fn get_server_url(&self) -> &str {
        &self.server_url
    }

    pub fn get_bind_dn(&self) -> &str {
        &self.bind_dn
    }

    pub fn get_search_base(&self) -> &str {
        &self.search_base
    }

    pub fn list_attributes(&self) -> Vec<&str> {
        self.attributes.iter().map(|attr| attr.as_str()).collect()
    }
}
