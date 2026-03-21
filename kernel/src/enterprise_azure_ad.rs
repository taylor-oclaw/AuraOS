extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct EnterpriseAzureAD {
    client_id: String,
    tenant_id: String,
    access_token: Option<String>,
}

impl EnterpriseAzureAD {
    pub fn new(client_id: &str, tenant_id: &str) -> Self {
        EnterpriseAzureAD {
            client_id: String::from(client_id),
            tenant_id: String::from(tenant_id),
            access_token: None,
        }
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn set_access_token(&mut self, token: &str) {
        self.access_token = Some(String::from(token));
    }

    pub fn has_valid_token(&self) -> bool {
        self.access_token.is_some()
    }

    pub fn get_user_info(&self) -> Option<String> {
        if let Some(token) = &self.access_token {
            // Simulate fetching user info from Azure AD
            Some(format!("User Info for token: {}", token))
        } else {
            None
        }
    }
}
