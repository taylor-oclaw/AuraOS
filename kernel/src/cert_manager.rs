extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Certificate {
    pub id: u64,
    pub subject: String,
    pub issuer: String,
    pub public_key: [u8; 32],
    pub not_before: u64,
    pub not_after: u64,
    pub serial: u64,
    pub trusted: bool,
    pub self_signed: bool,
}

pub struct CertManager {
    pub certificates: Vec<Certificate>,
    pub trusted_roots: Vec<u64>,
    pub next_id: u64,
}

impl CertManager {
    pub fn new() -> Self {
        Self {
            certificates: Vec::new(),
            trusted_roots: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_cert(&mut self, subject: &str, issuer: &str, valid_days: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.certificates.push(Certificate {
            id,
            subject: String::from(subject),
            issuer: String::from(issuer),
            public_key: [0; 32],
            not_before: 0,
            not_after: valid_days * 86400,
            serial: id,
            trusted: false,
            self_signed: subject == issuer,
        };
        id
    }

    pub fn trust(&mut self, id: u64) {
        if let Some(c) = self.certificates.iter_mut().find(|c| c.id == id) {
            c.trusted = true;
            self.trusted_roots.push(id);
        }
    }

    pub fn verify(&self, id: u64) -> bool {
        self.certificates
            .iter()
            .find(|c| c.id == id)
            .map(|c| {
                c.trusted || self.trusted_roots.iter().any(|r| {
                    self.certificates.iter().any(|rc| rc.id == *r && rc.subject == c.issuer)
                })
            })
            .unwrap_or(false)
    }

    pub fn expired(&self, now: u64) -> Vec<&Certificate> {
        self.certificates
            .iter()
            .filter(|c| c.not_after > 0 && now > c.not_after)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.certificates.len()
    }
)}
