extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseSSOOIDC {
    client_id: String,
    client_secret: String,
    token_endpoint: String,
    redirect_uri: String,
    scopes: Vec<String>,
}

impl EnterpriseSSOOIDC {
    pub fn new(client_id: &str, client_secret: &str, token_endpoint: &str, redirect_uri: &str) -> Self {
        EnterpriseSSOOIDC {
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
            token_endpoint: String::from(token_endpoint),
            redirect_uri: String::from(redirect_uri),
            scopes: Vec::new(),
        }
    }

    pub fn add_scope(&mut self, scope: &str) {
        self.scopes.push(String::from(scope));
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_token_endpoint(&self) -> &str {
        &self.token_endpoint
    }

    pub fn get_redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub fn get_scopes(&self) -> &[String] {
        &self.scopes
    }
}
