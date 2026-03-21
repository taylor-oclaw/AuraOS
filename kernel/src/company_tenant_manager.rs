extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct CompanyTenantManager {
    tenants: Vec<String>,
}

impl CompanyTenantManager {
    pub fn new() -> Self {
        CompanyTenantManager {
            tenants: Vec::new(),
        }
    }

    pub fn add_tenant(&mut self, tenant_name: &str) {
        if !self.tenants.contains(&tenant_name.to_string()) {
            self.tenants.push(tenant_name.to_string());
        }
    }

    pub fn remove_tenant(&mut self, tenant_name: &str) {
        self.tenants.retain(|t| t != tenant_name);
    }

    pub fn list_tenants(&self) -> Vec<String> {
        self.tenants.clone()
    }

    pub fn has_tenant(&self, tenant_name: &str) -> bool {
        self.tenants.contains(&tenant_name.to_string())
    }

    pub fn count_tenants(&self) -> usize {
        self.tenants.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_tenant_manager() {
        let mut manager = CompanyTenantManager::new();
        assert_eq!(manager.count_tenants(), 0);

        manager.add_tenant("tenant1");
        assert_eq!(manager.count_tenants(), 1);
        assert!(manager.has_tenant("tenant1"));

        manager.add_tenant("tenant2");
        assert_eq!(manager.count_tenants(), 2);
        assert!(manager.has_tenant("tenant2"));

        let tenants = manager.list_tenants();
        assert_eq!(tenants.len(), 2);
        assert!(tenants.contains(&String::from("tenant1")));
        assert!(tenants.contains(&String::from("tenant2")));

        manager.remove_tenant("tenant1");
        assert_eq!(manager.count_tenants(), 1);
        assert!(!manager.has_tenant("tenant1"));

        manager.remove_tenant("tenant2");
        assert_eq!(manager.count_tenants(), 0);
        assert!(!manager.has_tenant("tenant2"));
    }
}
