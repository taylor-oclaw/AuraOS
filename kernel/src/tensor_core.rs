extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

#[no_std]
mod tensor_core {
    extern crate alloc;
    use alloc::alloc::{alloc, dealloc};
    use core::ptr;
    use core::mem;

    pub struct Tensor<T> {
        data: *mut T,
        shape: Vec<usize>,
        strides: Vec<usize>,
        size: usize,
    }

    impl<T> Tensor<T> {
        pub fn new(shape: Vec<usize>) -> Self {
            let size = shape.iter().product();
            let layout = alloc::alloc::Layout::array::<T>(size).unwrap();
            let data = unsafe { alloc(layout) as *mut T };
            Tensor {
                data,
                shape,
                strides: vec![1; shape.len()],
                size,
            }
        }

        pub fn fill(&mut self, value: T)
        where
            T: Copy,
        {
            for i in 0..self.size {
                unsafe { ptr::write(self.data.offset(i as isize), value) };
            }
        }

        pub fn get(&self, indices: &[usize]) -> Option<&T> {
            if indices.len() != self.shape.len() || !indices.iter().zip(&self.shape).all(|(i, s)| *i < *s) {
                return None;
            }
            let mut index = 0;
            for (i, &stride) in indices.iter().rev().zip(self.strides.iter().rev()) {
                index += i * stride;
            }
            unsafe { Some(&*self.data.offset(index as isize)) }
        }

        pub fn set(&mut self, indices: &[usize], value: T)
        where
            T: Copy,
        {
            if let Some(elem) = self.get(indices) {
                unsafe { ptr::write(elem as *const T as *mut T, value) };
            }
        }

        pub fn shape(&self) -> &Vec<usize> {
            &self.shape
        }

        pub fn size(&self) -> usize {
            self.size
        }
    }

    impl<T> Drop for Tensor<T> {
        fn drop(&mut self) {
            let layout = alloc::alloc::Layout::array::<T>(self.size).unwrap();
            unsafe { dealloc(self.data as *mut u8, layout) };
        }
    }
}
