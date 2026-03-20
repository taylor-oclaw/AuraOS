extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct TensorOps {
    data: Vec<f32>,
    rows: usize,
    cols: usize,
}

impl TensorOps {
    pub fn new(rows: usize, cols: usize) -> Self {
        TensorOps {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn from_data(data: Vec<f32>, rows: usize, cols: usize) -> Self {
        TensorOps { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: f32) {
        self.data[row * self.cols + col] = val;
    }

    pub fn add(&self, other: &TensorOps) -> Option<TensorOps> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }
        let mut result = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            result.push(self.data[i] + other.data[i]);
        }
        Some(TensorOps { data: result, rows: self.rows, cols: self.cols })
    }

    pub fn multiply(&self, other: &TensorOps) -> Option<TensorOps> {
        if self.cols != other.rows {
            return None;
        }
        let mut result = vec![0.0; self.rows * other.cols];
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0f32;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result[i * other.cols + j] = sum;
            }
        }
        Some(TensorOps { data: result, rows: self.rows, cols: other.cols })
    }

    pub fn transpose(&self) -> TensorOps {
        let mut result = vec![0.0; self.data.len()];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[j * self.rows + i] = self.get(i, j);
            }
        }
        TensorOps { data: result, rows: self.cols, cols: self.rows }
    }

    pub fn scale(&mut self, factor: f32) {
        for val in self.data.iter_mut() {
            *val *= factor;
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn sum(&self) -> f32 {
        let mut total = 0.0f32;
        for val in &self.data {
            total += *val;
        }
        total
    }
}
