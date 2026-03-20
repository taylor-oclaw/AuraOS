extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module
}

struct AuraPhotos {
    photos: Vec<String>,
}

impl AuraPhotos {
    pub fn new() -> Self {
        AuraPhotos {
            photos: Vec::new(),
        }
    }

    pub fn add_photo(&mut self, photo_name: &str) {
        self.photos.push(String::from(photo_name));
    }

    pub fn remove_photo(&mut self, photo_name: &str) {
        if let Some(index) = self.photos.iter().position(|p| p == photo_name) {
            self.photos.remove(index);
        }
    }

    pub fn get_photos(&self) -> Vec<String> {
        self.photos.clone()
    }

    pub fn has_photo(&self, photo_name: &str) -> bool {
        self.photos.contains(&String::from(photo_name))
    }

    pub fn count_photos(&self) -> usize {
        self.photos.len()
    }
}

#[no_mangle]
pub extern "C" fn aura_photos_new() -> *mut AuraPhotos {
    Box::into_raw(Box::new(AuraPhotos::new()))
}

#[no_mangle]
pub extern "C" fn aura_photos_add_photo(module: *mut AuraPhotos, photo_name: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(photo_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*module).add_photo(name);
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_photos_remove_photo(module: *mut AuraPhotos, photo_name: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(photo_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*module).remove_photo(name);
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_photos_get_photos(module: *const AuraPhotos, photos: *mut *mut u8, sizes: *mut usize, count: *mut usize) -> isize {
    unsafe {
        let module = &*module;
        let photos_vec = module.get_photos();
        let num_photos = photos_vec.len();

        if num_photos > 0 {
            let mut photo_pointers: Vec<*mut u8> = Vec::with_capacity(num_photos);
            let mut sizes_vec: Vec<usize> = Vec::with_capacity(num_photos);

            for photo in photos_vec.iter() {
                let bytes = photo.as_bytes();
                let ptr = core::alloc::alloc(core::alloc::Layout::from_size_align(bytes.len(), 1).unwrap());
                if ptr.is_null() {
                    return -1;
                }
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
                photo_pointers.push(ptr);
                sizes_vec.push(bytes.len());
            }

            *photos = photo_pointers.as_mut_ptr();
            *sizes = sizes_vec.as_mut_ptr();
            *count = num_photos;

            core::mem::forget(photo_pointers);
            core::mem::forget(sizes_vec);

            num_photos as isize
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_photos_has_photo(module: *const AuraPhotos, photo_name: *const u8, len: usize) -> bool {
    unsafe {
        let slice = core::slice::from_raw_parts(photo_name, len);
        if let Ok(name) = core::str::from_utf8(slice) {
            (*module).has_photo(name)
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn aura_photos_count_photos(module: *const AuraPhotos) -> usize {
    unsafe { (*module).count_photos() }
}

#[no_mangle]
pub extern "C" fn aura_photos_free(photos: *mut *mut u8, sizes: *mut usize, count: usize) {
    unsafe {
        for i in 0..count {
            core::alloc::dealloc((*photos).add(i), core::alloc::Layout::from_size_align(*sizes.add(i), 1).unwrap());
        }
        Vec::<*mut u8>::from_raw_parts(photos, count, count);
        Vec::<usize>::from_raw_parts(sizes, count, count);
    }
}
