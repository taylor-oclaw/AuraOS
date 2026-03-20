extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TlsHandshake {
    // Example fields for a TLS handshake module
    client_hello: Vec<u8>,
    server_hello: Vec<u8>,
    session_id: Vec<u8>,
    cipher_suite: String,
    compression_method: u8,
}

impl TlsHandshake {
    pub fn new() -> Self {
        TlsHandshake {
            client_hello: Vec::new(),
            server_hello: Vec::new(),
            session_id: Vec::new(),
            cipher_suite: String::from("TLS_AES_128_GCM_SHA256"),
            compression_method: 0, // No compression
        }
    }

    pub fn set_client_hello(&mut self, hello: Vec<u8>) {
        self.client_hello = hello;
    }

    pub fn get_client_hello(&self) -> &Vec<u8> {
        &self.client_hello
    }

    pub fn set_server_hello(&mut self, hello: Vec<u8>) {
        self.server_hello = hello;
    }

    pub fn get_server_hello(&self) -> &Vec<u8> {
        &self.server_hello
    }

    pub fn set_session_id(&mut self, session_id: Vec<u8>) {
        self.session_id = session_id;
    }

    pub fn get_session_id(&self) -> &Vec<u8> {
        &self.session_id
    }

    pub fn set_cipher_suite(&mut self, cipher_suite: String) {
        self.cipher_suite = cipher_suite;
    }

    pub fn get_cipher_suite(&self) -> &String {
        &self.cipher_suite
    }

    pub fn set_compression_method(&mut self, method: u8) {
        self.compression_method = method;
    }

    pub fn get_compression_method(&self) -> u8 {
        self.compression_method
    }
}
