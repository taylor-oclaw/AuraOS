extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[no_std]
mod tensor_ops {
    use core::ops::{Add, Mul};

    #[derive(Debug)]
    pub struct Tensor {
        data: Vec<Vec>,
        shape: (usize, usize),
    }

    impl<T: Copy + Add<Output = T> + Mul<Output = T>> Tensor {
        pub fn new(data: Vec<Vec>) -> Self {
            let rows = data.len();
            let cols = if rows > 0 { data[0].len() } else { 0 };
            Tensor { data, shape: (rows, cols) }
        }

        pub fn add(&self, other: &Tensor) -> Result<Self, String> {
            if self.shape != other.shape {
                return Err(String::from("Shape mismatch"));
            }
            let mut result = Vec::with_capacity(self.shape.0);
            for i in 0..self.shape.0 {
                let row = (0..self.shape.1)
                    .map(|j| self.data[i][j] + other.data[i][j])
                    .collect();
                result.push(row);
            }
            Ok(Tensor { data: result, shape: self.shape })
        }

        pub fn multiply(&self, other: &Tensor) -> Result<Self, String> {
            if self.shape != other.shape {
                return Err(String::from("Shape mismatch"));
            }
            let mut result = Vec::with_capacity(self.shape.0);
            for i in 0..self.shape.0 {
                let row = (0..self.shape.1)
                    .map(|j| self.data[i][j] * other.data[i][j])
                    .collect();
                result.push(row);
            }
            Ok(Tensor { data: result, shape: self.shape })
        }

        pub fn transpose(&self) -> Self {
            let mut transposed = Vec::with_capacity(self.shape.1);
            for j in 0..self.shape.1 {
                let row = (0..self.shape.0)
                    .map(|i| self.data[i][j])
                    .collect();
                transposed.push(row);
            }
            Tensor { data: transposed, shape: (self.shape.1, self.shape.0) }
        }

        pub fn scalar_add(&self, value: f32) -> Self {
            let mut result = Vec::with_capacity(self.shape.0);
            for row in &self.data {
                let new_row = row.iter().map(|&x| x + value).collect();
                result.push(new_row);
            }
            Tensor { data: result, shape: self.shape }
        }

        pub fn scalar_multiply(&self, value: f32) -> Self {
            let mut result = Vec::with_capacity(self.shape.0);
            for row in &self.data {
                let new_row = row.iter().map(|&x| x * value).collect();
                result.push(new_row);
            }
            Tensor { data: result, shape: self.shape }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            let t1 = Tensor::new(vec![vec![1, 2], vec![3, 4]]);
            let t2 = Tensor::new(vec![vec![5, 6], vec![7, 8]]);
            let result = t1.add(&t2).unwrap();
            assert_eq!(result.data, vec![vec![6, 8], vec![10, 12]]);
        }

        #[test]
        fn test_multiply() {
            let t1 = Tensor::new(vec![vec![1, 2], vec![3, 4]]);
            let t2 = Tensor::new(vec![vec![5, 6], vec![7, 8]]);
            let result = t1.multiply(&t2).unwrap();
            assert_eq!(result.data, vec![vec![5, 12], vec![21, 32]]);
        }

        #[test]
        fn test_transpose() {
            let t = Tensor::new(vec![vec![1, 2], vec![3, 4]]);
            let result = t.transpose();
            assert_eq!(result.data, vec![vec![1, 3], vec![2, 4]]);
        }

        #[test]
        fn test_scalar_add() {
            let t = Tensor::new(vec![vec![1, 2], vec![3, 4]]);
            let result = t.scalar_add(5);
            assert_eq!(result.data, vec![vec![6, 7], vec![8, 9]]);
        }

        #[test]
        fn test_scalar_multiply() {
            let t = Tensor::new(vec![vec![1, 2], vec![3, 4]]);
            let result = t.scalar_multiply(2);
            assert_eq!(result.data, vec![vec![2, 4], vec![6, 8]]);
        }
    }
}
