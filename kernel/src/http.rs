extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub host: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn get(host: &str, path: &str) -> Self {
        Self {
            method: String::from("GET"),
            path: String::from(path),
            host: String::from(host),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn post(host: &str, path: &str, body: Vec<u8>) -> Self {
        Self {
            method: String::from("POST"),
            path: String::from(path),
            host: String::from(host),
            headers: Vec::new(),
            body: Some(body),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut req = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", self.method, self.path, self.host);
        for (k, v) in &self.headers {
            req.push_str(&format!("{}: {}\r\n", k, v));
        }
        if let Some(ref body) = self.body {
            req.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }
        req.push_str("\r\n");
        let mut bytes = req.into_bytes();
        if let Some(ref body) = self.body {
            bytes.extend_from_slice(body);
        }
        bytes
    }
}

impl HttpResponse {
    pub fn parse(data: &[u8]) -> Option<Self> {
        // Find \r\n\r\n separator
        let mut sep = 0;
        for i in 0..data.len().saturating_sub(3) {
            if &data[i..i+4] == b"\r\n\r\n" {
                sep = i;
                break;
            }
        }
        if sep == 0 { return None; }

        let header_str = core::str::from_utf8(&data[..sep]).ok()?;
        let body = data[sep+4..].to_vec();

        let mut lines = header_str.split("\r\n");
        let status_line = lines.next()?;
        let status_code = status_line.split(' ').nth(1)?.parse::<u16>().ok()?;

        let mut headers = Vec::new();
        for line in lines {
            if let Some(pos) = line.find(':') {
                let key = String::from(line[..pos].trim());
                let val = String::from(line[pos+1..].trim());
                headers.push((key, val));
            }
        }

        Some(Self { status_code, headers, body })
    }

    pub fn body_as_string(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}
