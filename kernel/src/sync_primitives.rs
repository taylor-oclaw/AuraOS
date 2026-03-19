use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;

pub struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Mutex<T> {}
unsafe impl<T: Send> Send for Mutex<T> {}

pub struct MutexGuard<'a, T>
where
{
    lock: *const AtomicBool,
    data: *mut T,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self.locked.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        MutexGuard {
            lock: &self.locked as *const _,
            data: self.data.get(),
            _marker: core::marker::PhantomData,
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            Some(MutexGuard {
                lock: &self.locked as *const _,
                data: self.data.get(),
                _marker: core::marker::PhantomData,
            })
        } else {
            None
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }
}

impl<T> core::ops::Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T> core::ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe { (*self.lock).store(false, Ordering::Release); }
    }
}
