extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FileAutoArchiveOld {
    file_path: String,
    archive_path: String,
}

impl FileAutoArchiveOld {
    pub fn new(file_path: &str, archive_path: &str) -> Self {
        FileAutoArchiveOld {
            file_path: String::from(file_path),
            archive_path: String::from(archive_path),
        }
    }

    pub fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn set_archive_path(&mut self, path: &str) {
        self.archive_path = String::from(path);
    }

    pub fn archive_file(&self) {
        // Logic to archive the file
        println!("Archiving file {}", self.get_file_path());
    }

    pub fn unarchive_file(&self) {
        // Logic to unarchive the file
        println!("Unarchiving file {}", self.get_file_path());
    }

    pub fn check_archive_status(&self) -> bool {
        // Logic to check archive status
        true
    }
}

pub struct FileAutoArchiveOldList {
    files: Vec<FileAutoArchiveOld>,
}

impl FileAutoArchiveOldList {
    pub fn new() -> Self {
        FileAutoArchiveOldList { files: Vec::new() }
    }

    pub fn add_file(&mut self, file: FileAutoArchiveOld) {
        self.files.push(file);
    }

    pub fn get_files(&self) -> &Vec<FileAutoArchiveOld> {
        &self.files
    }

    pub fn archive_all_files(&self) {
        for file in &self.files {
            file.archive_file();
        }
    }

    pub fn unarchive_all_files(&self) {
        for file in &self.files {
            file.unarchive_file();
        }
    }
}