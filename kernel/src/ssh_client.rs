extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SshClient {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl SshClient {
    pub fn new(host: &str, port: u16, username: &str, password: &str) -> Self {
        SshClient {
            host: String::from(host),
            port,
            username: String::from(username),
            password: String::from(password),
        }
    }

    pub fn connect(&self) -> Result<(), &'static str> {
        // Simulate a connection attempt
        if self.host.is_empty() || self.username.is_empty() || self.password.is_empty() {
            Err("Invalid credentials or host")
        } else {
            Ok(())
        }
    }

    pub fn execute_command(&self, command: &str) -> Result<String, &'static str> {
        // Simulate executing a command
        if command.is_empty() {
            Err("Empty command")
        } else {
            Ok(format!("Command executed: {}", command))
        }
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn set_host(&mut self, host: &str) {
        self.host = String::from(host);
    }

    pub fn list_files(&self, directory: &str) -> Result<Vec<String>, &'static str> {
        // Simulate listing files in a directory
        if directory.is_empty() {
            Err("Empty directory")
        } else {
            Ok(vec![String::from("file1.txt"), String::from("file2.txt")])
        }
    }
}
