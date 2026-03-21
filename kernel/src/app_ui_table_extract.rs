extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppUITableExtract {
    data: Vec<Vec<String>>,
}

impl AppUITableExtract {
    pub fn new() -> Self {
        AppUITableExtract {
            data: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.data.push(row);
    }

    pub fn get_num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn get_num_columns(&self) -> Option<usize> {
        if let Some(first_row) = self.data.first() {
            Some(first_row.len())
        } else {
            None
        }
    }

    pub fn get_cell(&self, row_index: usize, col_index: usize) -> Option<&String> {
        self.data.get(row_index).and_then(|row| row.get(col_index))
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.data {
            for (i, cell) in row.iter().enumerate() {
                if i > 0 {
                    result.push_str(" | ");
                }
                result.push_str(cell);
            }
            result.push('\n');
        }
        result
    }
}
