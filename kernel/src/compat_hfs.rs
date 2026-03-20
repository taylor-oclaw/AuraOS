extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct HfsVolume {
    name: String,
    files: Vec<String>,
}

impl HfsVolume {
    pub fn new(name: &str) -> Self {
        HfsVolume {
            name: String::from(name),
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_name: &str) {
        self.files.push(String::from(file_name));
    }

    pub fn remove_file(&mut self, file_name: &str) -> bool {
        if let Some(index) = self.files.iter().position(|f| f == file_name) {
            self.files.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_files(&self) -> Vec<String> {
        self.files.clone()
    }

    pub fn has_file(&self, file_name: &str) -> bool {
        self.files.contains(&String::from(file_name))
    }

    pub fn get_volume_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hfs_volume() {
        let mut volume = HfsVolume::new("Test Volume");
        assert_eq!(volume.get_volume_name(), "Test Volume");

        volume.add_file("file1.txt");
        volume.add_file("file2.txt");
        assert_eq!(volume.list_files(), vec![String::from("file1.txt"), String::from("file2.txt")]);
        assert!(volume.has_file("file1.txt"));
        assert!(!volume.has_file("file3.txt"));

        assert!(volume.remove_file("file1.txt"));
        assert!(!volume.has_file("file1.txt"));
        assert_eq!(volume.list_files(), vec![String::from("file2.txt")]);
    }
}
