extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub extern "C" fn rust_ffi_example() -> i32 {
    42
}

struct AuraAssistantMemory {
    data: Vec<u8>,
}

impl AuraAssistantMemory {
    pub fn new(size: usize) -> Self {
        AuraAssistantMemory {
            data: vec![0; size],
        }
    }

    pub fn read(&self, offset: usize, length: usize) -> Option<&[u8]> {
        if offset + length <= self.data.len() {
            Some(&self.data[offset..offset + length])
        } else {
            None
        }
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> bool {
        if offset + data.len() <= self.data.len() {
            self.data[offset..offset + data.len()].copy_from_slice(data);
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    pub fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, 0);
    }
}

pub extern "C" fn aura_assistant_memory_new(size: usize) -> *mut AuraAssistantMemory {
    Box::into_raw(Box::new(AuraAssistantMemory::new(size)))
}

pub unsafe extern "C" fn aura_assistant_memory_read(
    memory: *const AuraAssistantMemory,
    offset: usize,
    length: usize,
    buffer: *mut u8,
 -> bool {
    if let Some(data) = (*memory).read(offset, length) {
        core::ptr::copy_nonoverlapping(data.as_ptr(), buffer, length);
        true
    } else {
        false
    }
}

pub unsafe extern "C" fn aura_assistant_memory_write(
    memory: *mut AuraAssistantMemory,
    offset: usize,
    data: *const u8,
    length: usize,
 -> bool {
    (*memory).write(offset, core::slice::from_raw_parts(data, length))
}

pub extern "C" fn aura_assistant_memory_size(memory: *const AuraAssistantMemory) -> usize {
    unsafe { (*memory).size() }
}

pub unsafe extern "C" fn aura_assistant_memory_clear(memory: *mut AuraAssistantMemory) {
    (*memory).clear();
}

pub unsafe extern "C" fn aura_assistant_memory_resize(
    memory: *mut AuraAssistantMemory,
    new_size: usize,
 {
    (*memory).resize(new_size);
}

pub unsafe extern "C" fn aura_assistant_memory_free(memory: *mut AuraAssistantMemory) {
    drop(Box::from_raw(memory));
}
