extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct EnterpriseSSOOIDC {
    client_id: String,
    client_secret: String,
    token_endpoint: String,
    scopes: Vec<String>,
    access_token: Option<String>,
}

impl EnterpriseSSOOIDC {
    pub fn new(client_id: &str, client_secret: &str, token_endpoint: &str) -> Self {
        EnterpriseSSOOIDC {
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
            token_endpoint: String::from(token_endpoint),
            scopes: Vec::new(),
            access_token: None,
        }
    }

    pub fn add_scope(&mut self, scope: &str) {
        self.scopes.push(String::from(scope));
    }

    pub fn get_scopes(&self) -> &[String] {
        &self.scopes
    }

    pub fn set_access_token(&mut self, token: &str) {
        self.access_token = Some(String::from(token));
    }

    pub fn get_access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    pub fn clear_access_token(&mut self) {
        self.access_token = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sso = EnterpriseSSOOIDC::new("client_id", "client_secret", "token_endpoint");
        assert_eq!(sso.client_id, "client_id");
        assert_eq!(sso.client_secret, "client_secret");
        assert_eq!(sso.token_endpoint, "token_endpoint");
        assert!(sso.scopes.is_empty());
        assert!(sso.access_token.is_none());
    }

    #[test]
    fn test_add_scope() {
        let mut sso = EnterpriseSSOOIDC::new("client_id", "client_secret", "token_endpoint");
        sso.add_scope("scope1");
        sso.add_scope("scope2");
        assert_eq!(sso.get_scopes(), &["scope1".to_string(), "scope2".to_string()]);
    }

    #[test]
    fn test_set_access_token() {
        let mut sso = EnterpriseSSOOIDC::new("client_id", "client_secret", "token_endpoint");
        sso.set_access_token("access_token");
        assert_eq!(sso.get_access_token(), Some("access_token"));
    }

    #[test]
    fn test_clear_access_token() {
        let mut sso = EnterpriseSSOOIDC::new("client_id", "client_secret", "token_endpoint");
        sso.set_access_token("access_token");
        sso.clear_access_token();
        assert_eq!(sso.get_access_token(), None);
    }
}