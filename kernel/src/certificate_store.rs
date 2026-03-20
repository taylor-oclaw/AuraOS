extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod certificate_store {
    use super::*;

    pub struct CertificateStore {
        certificates: Vec<String>,
    }

    impl CertificateStore {
        pub fn new() -> Self {
            CertificateStore {
                certificates: Vec::new(),
            }
        }

        pub fn add_certificate(&mut self, cert: String) {
            if !self.certificates.contains(&cert) {
                self.certificates.push(cert);
            }
        }

        pub fn remove_certificate(&mut self, cert: &str) -> bool {
            let index = self.certificates.iter().position(|c| c == cert);
            if let Some(i) = index {
                self.certificates.remove(i);
                true
            } else {
                false
            }
        }

        pub fn contains_certificate(&self, cert: &str) -> bool {
            self.certificates.contains(&String::from(cert))
        }

        pub fn get_certificates(&self) -> Vec<String> {
            self.certificates.clone()
        }

        pub fn count_certificates(&self) -> usize {
            self.certificates.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_store() {
        let mut store = certificate_store::CertificateStore::new();
        assert_eq!(store.count_certificates(), 0);

        store.add_certificate(String::from("cert1"));
        assert_eq!(store.count_certificates(), 1);
        assert!(store.contains_certificate("cert1"));

        store.add_certificate(String::from("cert2"));
        assert_eq!(store.count_certificates(), 2);
        assert!(store.contains_certificate("cert2"));

        assert!(!store.remove_certificate("cert3"));
        assert!(store.remove_certificate("cert1"));
        assert_eq!(store.count_certificates(), 1);

        let certs = store.get_certificates();
        assert_eq!(certs.len(), 1);
        assert_eq!(certs[0], "cert2");
    }
}
