extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct MCPRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl MCPRequest {
    pub fn new(method: &str, path: &str) -> Self {
        MCPRequest {
            method: String::from(method),
            path: String::from(path),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((String::from(key), String::from(value)));
    }

    pub fn set_body(&mut self, body: &[u8]) {
        self.body.extend_from_slice(body);
    }

    pub fn get_method(&self) -> &str {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_headers(&self) -> &Vec<(String, String)> {
        &self.headers
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }
}

struct MCPResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl MCPResponse {
    pub fn new(status_code: u16) -> Self {
        MCPResponse {
            status_code,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((String::from(key), String::from(value)));
    }

    pub fn set_body(&mut self, body: &[u8]) {
        self.body.extend_from_slice(body);
    }

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub fn get_headers(&self) -> &Vec<(String, String)> {
        &self.headers
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }
}

struct MCPServer;

impl MCPServer {
    pub fn new() -> Self {
        MCPServer
    }

    pub fn handle_request(&self, request: MCPRequest) -> MCPResponse {
        let mut response = MCPResponse::new(200);
        response.add_header("Content-Type", "text/plain");

        if request.get_method() == "GET" && request.get_path() == "/hello" {
            response.set_body(b"Hello, World!");
        } else {
            response.set_body(b"Not Found");
            response.status_code = 404;
        }

        response
    }
}
