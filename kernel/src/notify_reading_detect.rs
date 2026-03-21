extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut detector = NotifyReadingDetect::new();
    detector.add_book("The Great Gatsby");
    detector.add_book("1984");
    detector.log_books();
    if detector.is_reading("The Great Gatsby") {
        println!("User is reading The Great Gatsby");
    } else {
        println!("User is not reading The Great Gatsby");
    }
    detector.remove_book("1984");
    detector.log_books();
}

pub struct NotifyReadingDetect {
    books: Vec<String>,
}

impl NotifyReadingDetect {
    pub fn new() -> Self {
        NotifyReadingDetect { books: Vec::new() }
    }

    pub fn add_book(&mut self, title: &str) {
        self.books.push(String::from(title));
    }

    pub fn remove_book(&mut self, title: &str) {
        if let Some(index) = self.books.iter().position(|book| book == title) {
            self.books.remove(index);
        }
    }

    pub fn is_reading(&self, title: &str) -> bool {
        self.books.contains(&String::from(title))
    }

    pub fn log_books(&self) {
        for book in &self.books {
            println!("Book: {}", book);
        }
    }

    pub fn get_book_count(&self) -> usize {
        self.books.len()
    }
}
